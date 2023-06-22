use crate::errors::{BusinessError, BusinessResult};
use crate::services::http_client::http_client::HttpClient;
use lazy_static::lazy_static;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

pub struct WxLogin;

lazy_static! {
    static ref UUID_REGEX: regex::Regex = regex::Regex::new(r#".+\/(.+)$"#).unwrap();
    static ref WX_QRRESULT_REGEX: regex::Regex =
        regex::Regex::new(r#"^window\.wx_errcode=(\d+);window.wx_code=\'(.*)\';$"#).unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct WxQrcodeInfo {
    pub url: String,
    pub uuid: String,
}

pub enum WxQrcodeStatus {
    Success(String),
    WaitingForScan,
    Scanned,
    Cancel,
    Expire,
    Error,
}

impl WxLogin {
    const SUNRUN_APP_ID: &'static str = "wxf83de11533c17d91";

    pub async fn get_qr_login_info() -> BusinessResult<WxQrcodeInfo> {
        let response = HttpClient::client().get("https://open.weixin.qq.com/connect/app/qrconnect")
            .query(&[("appid",Self::SUNRUN_APP_ID),("bundleid","cn.lwpro.admin"),("scope","snsapi_userinfo"),("state","sunrun_login"),("supportcontentfromwx","8191")])
            .header(reqwest::header::USER_AGENT,"Mozilla/5.0 (iPhone; CPU iPhone OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/7.0.0(0x17000024) NetType/WIFI Language/zh_CN")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(BusinessError::new(format!(
                "远程请求失败: {}",
                response.status()
            )));
        }

        let response = response.text().await?;

        let error_message_selector = Selector::parse(".weui_msg_title").unwrap();

        let html = Html::parse_document(&response);

        if let Some(error_title_node) = html.select(&error_message_selector).next() {
            let error_message = error_title_node.text().collect::<String>();
            return Err(BusinessError::new(format!(
                "微信登录失败: {}",
                error_message
            )));
        }

        let qr_code_selector = Selector::parse(".auth_qrcode").unwrap();

        if let Some(qr_code_selector) = html.select(&qr_code_selector).next() {
            let qr_src = qr_code_selector.value().attr("src");

            if let Some(qr_src) = qr_src {
                let uuid = match UUID_REGEX.captures(qr_src) {
                    Some(v) => v,
                    None => return Err(BusinessError::new("微信二维码UUID为空")),
                };

                let uuid = match uuid.get(1) {
                    Some(v) => v,
                    None => return Err(BusinessError::new("微信二维码UUID为空")),
                };
                Ok(WxQrcodeInfo {
                    url: qr_src.to_owned(),
                    uuid: uuid.as_str().to_owned(),
                })
            } else {
                Err(BusinessError::new("微信二维码获取为空"))
            }
        } else {
            Err(BusinessError::new("无法获取微信二维码"))
        }
    }

    pub async fn get_qr_code_status(uuid: &str) -> BusinessResult<WxQrcodeStatus> {
        let response = HttpClient::client()
            .get("https://long.open.weixin.qq.com/connect/l/qrconnect")
            .query(&[("uuid", uuid)])
            .send()
            .await?
            .text()
            .await?;

        let regex_match_result = match WX_QRRESULT_REGEX.captures(&response) {
            Some(v) => v,
            None => return Err(BusinessError::new("微信二维码返回不正确")),
        };

        let wx_errcode = match regex_match_result.get(1) {
            Some(v) => v,
            None => return Err(BusinessError::new("微信二维码错误码不存在")),
        };
        let wx_code = match regex_match_result.get(2) {
            Some(v) => v,
            None => return Err(BusinessError::new("微信二维码结果不存在")),
        };

        let wx_errcode = wx_errcode.as_str();

        let wx_code = wx_code.as_str();

        let qr_status = match wx_errcode {
            "405" => {
                if wx_code.len() == 0 {
                    return Err(BusinessError::new("微信二维码结果为空，请重新扫描"));
                }
                WxQrcodeStatus::Success(wx_code.to_owned())
            }
            "404" => WxQrcodeStatus::Scanned,
            "403" => WxQrcodeStatus::Cancel,
            "402" => WxQrcodeStatus::Expire,
            "500" => WxQrcodeStatus::Error,
            "408" => WxQrcodeStatus::WaitingForScan,
            _ => WxQrcodeStatus::Error,
        };
        Ok(qr_status)
    }
}
