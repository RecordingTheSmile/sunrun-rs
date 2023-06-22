use crate::errors::{BusinessError, BusinessResult};
use jwt_simple::claims::{Claims, JWTClaims};
use jwt_simple::common::VerificationOptions;
use jwt_simple::prelude::{Duration, HS256Key, MACLike, UnixTimeStamp};
use jwt_simple::JWTError;
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashSet;

static JWT_MANAGER_GLOBAL: OnceCell<JwtManager> = OnceCell::new();

pub struct JwtManager {
    key: String,
}

impl JwtManager {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
        }
    }

    pub fn init(key: &str) {
        let _ = JWT_MANAGER_GLOBAL.set(Self::new(key));
    }

    pub fn get_global() -> &'static Self {
        JWT_MANAGER_GLOBAL.get().unwrap()
    }

    pub fn get_signed_token_with_exp<T>(&self, claims: T, exp: i64) -> BusinessResult<String>
    where
        T: Serialize + DeserializeOwned,
    {
        let time_now = chrono::Local::now().timestamp();
        let key = HS256Key::from_bytes(self.key.as_bytes());

        let val = serde_json::to_value(claims).map_err(|_| BusinessError::new("无法转换值"))?;

        let claims = Claims::with_custom_claims(val, Duration::from_secs(exp as u64))
            .with_issuer("SUNRUN-RS")
            .invalid_before(UnixTimeStamp::new(time_now as u64, 0));

        let jwt = key.authenticate(claims).map_err(|e| {
            log::error!("{:#?}", e);
            BusinessError::new_code("Token签发失败", 500)
        })?;

        Ok(jwt)
    }

    pub fn get_signed_token<T>(&self, claims: T) -> BusinessResult<String>
    where
        T: Serialize + DeserializeOwned,
    {
        let key = HS256Key::from_bytes(self.key.as_bytes());

        let val = serde_json::to_value(claims).map_err(|_| BusinessError::new("无法转换值"))?;

        let claims = Claims::with_custom_claims(val, Duration::from_hours(1))
            .with_issuer("SUNRUN-RS")
            .invalid_before(UnixTimeStamp::new(
                chrono::Local::now().timestamp() as u64,
                0,
            ));

        let jwt = key.authenticate(claims).map_err(|e| {
            log::error!("{:#?}", e);
            BusinessError::new_code("Token签发失败", 500)
        })?;

        Ok(jwt)
    }

    pub fn get_verified_data<T>(&self, token: &str) -> BusinessResult<T>
    where
        T: Serialize + DeserializeOwned,
    {
        let key = HS256Key::from_bytes(self.key.as_bytes());

        let mut opts = VerificationOptions::default();

        opts.accept_future = false;

        let mut iss = HashSet::new();

        iss.insert("SUNRUN-RS".to_owned());

        opts.allowed_issuers = Some(iss);

        opts.time_tolerance = Some(Duration::from_secs(30));

        let claims = key.verify_token::<T>(token, Some(opts)).map_err(|e| {
            e.downcast::<JWTError>()
                .unwrap_or(JWTError::InternalError(String::from("Cannot convert")))
        })?;
        Ok(claims.custom)
    }

    pub fn get_verified_data_rbf<T>(&self, token: &str, rbf: i64) -> BusinessResult<T>
    where
        T: Serialize + DeserializeOwned,
    {
        let key = HS256Key::from_bytes(self.key.as_bytes());

        let mut opts = VerificationOptions::default();

        opts.accept_future = false;

        let mut iss = HashSet::new();

        iss.insert("SUNRUN-RS".to_owned());

        opts.allowed_issuers = Some(iss);

        opts.reject_before = Some(UnixTimeStamp::new(rbf as u64, 0));

        opts.time_tolerance = Some(Duration::from_secs(30));

        let claims = key.verify_token::<T>(token, Some(opts))?;
        Ok(claims.custom)
    }

    pub fn get_verified_claims(&self, token: &str) -> BusinessResult<JWTClaims<Value>> {
        let key = HS256Key::from_bytes(self.key.as_bytes());

        let mut opts = VerificationOptions::default();

        opts.accept_future = false;

        let mut iss = HashSet::new();

        iss.insert("SUNRUN-RS".to_owned());

        opts.allowed_issuers = Some(iss);

        opts.time_tolerance = Some(Duration::from_secs(30));

        let claims = key.verify_token(token, Some(opts)).map_err(|e| {
            e.downcast::<JWTError>()
                .unwrap_or(JWTError::InternalError(String::from("Cannot convert")))
        })?;
        Ok(claims)
    }
}
