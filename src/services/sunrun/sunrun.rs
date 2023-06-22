use async_trait::async_trait;
use rand::Rng;

use crate::errors::{BusinessError, BusinessResult};
use crate::services::http_client::http_client::HttpClient;
use crate::services::sunrun::crypto::SunrunCrypto;
use crate::services::sunrun::models::{SunrunRunTimes, SunrunUserInfo};

const BASE_URL: &str = "http://client3.aipao.me/api";

#[derive(Clone, Debug)]
pub struct Sunrun {
    imeicode: String,
    token: Option<String>,
}

impl Sunrun {
    pub fn new(imeicode: impl ToString) -> Self {
        Self {
            imeicode: imeicode.to_string(),
            token: None,
        }
    }

    pub fn new_with_token<ToStr>(imeicode: ToStr, token: ToStr) -> Self
    where
        ToStr: ToString,
    {
        Self {
            imeicode: imeicode.to_string(),
            token: Some(token.to_string()),
        }
    }

    pub async fn new_by_wx_token(token: &str) -> BusinessResult<Self> {
        let response = HttpClient::client()
            .get(format!("{BASE_URL}/{token}/QM_Users/Login_Android"))
            .query(&[("wxCode", token), ("IMEI", "Android:")])
            .set_version_header()
            .send()
            .await?
            .ensure_success_json()
            .await?;

        let token = response["Data"]["Token"].as_str().unwrap_or_default();
        let imeicode = response["Data"]["IMEICode"].as_str().unwrap_or_default();

        if token.len() == 0 || imeicode.len() == 0 {
            return Err(BusinessError::new("获取到的Token为空"));
        }
        Ok(Self {
            imeicode: imeicode.to_owned(),
            token: Some(token.to_owned()),
        })
    }

    pub fn get_imeicode(&self) -> String {
        self.imeicode.to_owned()
    }

    pub async fn get_token(&mut self, is_iphone: bool) -> BusinessResult<String> {
        let path = if is_iphone {
            "/%7Btoken%7D/QM_Users/LoginSchool"
        } else {
            "/%7Btoken%7D/QM_Users/Login_AndroidSchool"
        };
        let result = HttpClient::client()
            .get(format!("{BASE_URL}{path}"))
            .query(&[("IMEICode", &self.imeicode)])
            .set_version_header()
            .send()
            .await?
            .ensure_success_json()
            .await?;

        let token = match result["Data"]["Token"].as_str() {
            Some(t) => t,
            None => return Err(BusinessError::new("获取阳光体育Token为空")),
        };
        self.token = Some(token.to_owned());
        Ok(token.to_string())
    }

    pub async fn get_userinfo(&self) -> BusinessResult<SunrunUserInfo> {
        let token = match self.token.to_owned() {
            Some(t) => t,
            None => return Err(anyhow::anyhow!("未初始化Token").into()),
        };

        let result = HttpClient::client()
            .get(format!("{BASE_URL}/{token}/QM_Users/GS"))
            .set_auth_header()
            .set_version_header()
            .send()
            .await?
            .ensure_success_json()
            .await?;

        let user = &result["Data"]["User"];
        let school_run = &result["Data"]["SchoolRun"];

        Ok(SunrunUserInfo {
            length: school_run["Lengths"]
                .as_i64()
                .ok_or(BusinessError::new("跑步长度为空"))?,
            max_speed: school_run["MaxSpeed"]
                .as_f64()
                .ok_or(BusinessError::new("跑步最大速度为空"))?,
            min_speed: school_run["MinSpeed"]
                .as_f64()
                .ok_or(BusinessError::new("跑步最小速度为空"))?,
            school_name: school_run["SchoolName"].as_str().unwrap_or("").to_owned(),
            nick_name: user["NickName"].as_str().unwrap_or("").to_owned(),
            user_id: user["UserID"]
                .as_i64()
                .ok_or(BusinessError::new("用户ID为空"))?,
            latitude: String::new(),
            longitude: String::new(),
            step: 0,
        })
    }

    pub async fn get_run_times(&self) -> BusinessResult<SunrunRunTimes> {
        let token = match &self.token {
            Some(t) => t,
            None => return Err(anyhow::anyhow!("未初始化Token").into()),
        };

        let user_info = self.get_userinfo().await?;

        let result = HttpClient::client()
            .get(format!(
                "{BASE_URL}/{token}/QM_Runs/getResultsofValidByUser"
            ))
            .query(&[
                ("UserId", user_info.user_id),
                ("pageIndex", 1),
                ("pageSize", u8::MAX as i64),
            ])
            .send()
            .await?
            .ensure_success_json()
            .await?;

        Ok(SunrunRunTimes {
            morning_run_times: result["RaceMNums"]
                .as_i64()
                .ok_or(BusinessError::new("晨跑次数不存在"))?,
            run_times: result["RaceNums"]
                .as_i64()
                .ok_or(BusinessError::new("总跑步次数不存在"))?,
        })
    }

    pub async fn start_run(&self, userinfo: SunrunUserInfo) -> BusinessResult<()> {
        let token = match &self.token {
            Some(t) => t,
            None => return Err(anyhow::anyhow!("未初始化Token").into()),
        };

        let run_info = HttpClient::client()
            .get(format!("{BASE_URL}/{token}/QM_Runs/SRS"))
            .query(&[
                ("S1", &userinfo.latitude),
                ("S2", &userinfo.longitude),
                ("S3", &userinfo.length.to_string()),
            ])
            .set_version_header()
            .send()
            .await?
            .ensure_success_json()
            .await?;

        let run_id = run_info["Data"]["RunId"]
            .as_str()
            .ok_or(BusinessError::new("RunId为空"))?;
        let route = run_info["Data"]["Routes"]
            .as_str()
            .ok_or(BusinessError::new("Routes为空"))?;

        let speed = rand::thread_rng().gen_range(userinfo.min_speed + 0.01..userinfo.max_speed);

        let run_length = rand::thread_rng().gen_range(userinfo.length..=userinfo.length + 2);

        let run_time = (run_length as f64 / speed).round() as i64;
        let step = userinfo.step + rand::thread_rng().gen_range(-100..100);
        let encrypt_key = SunrunCrypto::get_encrypt_key();
        let encrypt_key_str: String = encrypt_key.iter().map(|x| x.to_string()).collect();

        let _ = HttpClient::client()
            .get(format!("{BASE_URL}/{token}/QM_Runs/ES"))
            .query(&[
                ("S1", run_id),
                ("S4", &SunrunCrypto::encrypt_integer(&encrypt_key, run_time)),
                (
                    "S5",
                    &SunrunCrypto::encrypt_integer(&encrypt_key, run_length),
                ),
                ("S6", route),
                ("S7", "1"),
                ("S8", &encrypt_key_str),
                ("S9", &SunrunCrypto::encrypt_integer(&encrypt_key, step)),
            ])
            .send()
            .await?
            .ensure_success_json()
            .await?;

        Ok(())
    }
}

#[async_trait]
trait SunrunHttpResponseExt {
    async fn ensure_success_json(self) -> BusinessResult<serde_json::Value>;
}

#[async_trait]
impl SunrunHttpResponseExt for reqwest::Response {
    async fn ensure_success_json(self) -> BusinessResult<serde_json::Value> {
        let json = self.json::<serde_json::Value>().await?;

        if !json["Success"].as_bool().unwrap_or(false) {
            return Err(BusinessError::new(
                json["ErrMsg"].as_str().unwrap_or("阳光体育服务器请求失败"),
            ));
        }

        Ok(json)
    }
}

trait SunrunHttpRequestBuilderExt {
    fn set_auth_header(self) -> Self;
    fn set_version_header(self) -> Self;
}

impl SunrunHttpRequestBuilderExt for reqwest::RequestBuilder {
    fn set_auth_header(self) -> Self {
        self.header("auth", SunrunCrypto::user_info_encrypt())
    }

    fn set_version_header(self) -> Self {
        self.header("version", "9.9")
    }
}
