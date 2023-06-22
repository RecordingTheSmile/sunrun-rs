use std::env::VarError;
use std::fmt::{Display, Formatter};

use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use jwt_simple::JWTError;
use lettre::address::AddressError;
use reqwest::Error;

use crate::common::r::R;

#[derive(Debug)]
pub struct BusinessError {
    pub code: u16,
    pub message: String,
}

pub type BusinessResult<T> = Result<T, BusinessError>;

macro_rules! from_type_to_business_error {
    ($($t:ty),*) => {
        $(
            impl From<$t> for BusinessError{
                fn from(e: $t) -> Self {
                    log::error!("error: {:#?}", e);
                    Self::default()
                }
            }
        )*
    };
}

impl Display for BusinessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BusinessError:\nCode:{}\nMessage:{}",
            self.code, self.message
        )
    }
}

impl std::error::Error for BusinessError {}

impl Default for BusinessError {
    fn default() -> Self {
        Self {
            code: 500,
            message: "未知的系统错误".into(),
        }
    }
}

impl BusinessError {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
            code: 503,
        }
    }

    pub fn new_code(message: impl ToString, code: u16) -> Self {
        Self {
            message: message.to_string(),
            code,
        }
    }
}

from_type_to_business_error!(
    anyhow::Error,
    bcrypt::BcryptError,
    sea_orm::DbErr,
    tokio_scheduler_rs::errors::SchedulerError,
    uuid::Error,
    lettre::error::Error,
    lettre::transport::smtp::Error
);

impl From<reqwest::Error> for BusinessError {
    fn from(value: Error) -> Self {
        log::error!("ReqwestError: {:#?}", value);
        Self::new("网络请求错误")
    }
}

impl From<std::env::VarError> for BusinessError {
    fn from(value: VarError) -> Self {
        log::error!("VarError: {}", value.to_string());
        Self::default()
    }
}

impl From<lettre::address::AddressError> for BusinessError {
    fn from(_: AddressError) -> Self {
        Self::new_code("邮箱格式不正确", 400)
    }
}

impl ResponseError for BusinessError {
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(self.status_code()).json(R::json_fail(&self.message, self.code))
    }
}
impl From<jwt_simple::JWTError> for BusinessError {
    fn from(value: JWTError) -> Self {
        let (code, message) = match value {
            JWTError::ClockDrift => (401, "JWT签发时间错误"),
            JWTError::OldTokenReused => (401, "登录信息失效，请重新登录"),
            JWTError::TokenHasExpired => (401, "登录信息过期，请重新登录"),
            _ => (401, "Token无效，请重新登录"),
        };

        Self::new_code(message, code)
    }
}
