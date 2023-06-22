use crate::errors::{BusinessError, BusinessResult};
use crate::services::managers::jwt_manager::JwtManager;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use serde_json::Value;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

pub struct JwtData(Value);

impl FromRequest for JwtData {
    type Error = BusinessError;
    type Future = Pin<Box<dyn Future<Output = BusinessResult<Self>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|v| Some(v.to_str().unwrap_or_default().to_owned()))
            .unwrap_or(String::new());

        Box::pin(async move {
            if let Some((auth_type, auth_token)) = auth_header.split_once(' ') {
                if auth_type.to_lowercase() != "bearer" {
                    return Err(BusinessError::new_code("验证方式无效", 400));
                }
                let jwt_manager = JwtManager::get_global();
                let data = jwt_manager.get_verified_data::<Value>(auth_token)?;
                Ok(Self(data))
            } else {
                Err(BusinessError::new_code("Token无效，请重新登录", 401))
            }
        })
    }
}

impl Deref for JwtData {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
