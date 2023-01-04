use std::env::VarError;
use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use lettre::address::AddressError;
use reqwest::Error;

use crate::common::r::R;
use crate::services::template::template::ActixHttpResponseExt;

#[derive(Debug)]
pub struct BusinessError {
    pub code: u16,
    pub message: String,
    is_html: bool,
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
        write!(f, "BusinessError:\nCode:{}\nMessage:{}", self.code, self.message)
    }
}

impl std::error::Error for BusinessError {}

impl Default for BusinessError {
    fn default() -> Self {
        Self {
            code: 500,
            message: "未知的系统错误".into(),
            is_html: false,
        }
    }
}

impl BusinessError {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
            code: 503,
            is_html: false,
        }
    }

    pub fn new_code(message: impl ToString, code: u16) -> Self {
        Self {
            message: message.to_string(),
            code,
            is_html: false,
        }
    }

    pub fn set_html(self, is_html: bool) -> Self {
        Self {
            is_html,
            ..self
        }
    }
}

from_type_to_business_error!(anyhow::Error,
    bcrypt::BcryptError,
    sea_orm::DbErr,
    actix_session::SessionGetError,
    actix_session::SessionInsertError,
    tokio_scheduler_rs::errors::SchedulerError,
    uuid::Error,
    lettre::error::Error,
    lettre::transport::smtp::Error
);

impl From<reqwest::Error> for BusinessError {
    fn from(value: Error) -> Self {
        log::error!("ReqwestError: {:#?}",value);
        Self::new("网络请求错误")
    }
}

impl From<std::env::VarError> for BusinessError {
    fn from(value: VarError) -> Self {
        log::error!("VarError: {}",value.to_string());
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
        if self.is_html {
            let mut ctx = tera::Context::new();
            ctx.insert("message", &self.message);
            ctx.insert("code", &self.code);
            HttpResponseBuilder::new(self.status_code())
                .html("error.html", &ctx)
        } else {
            HttpResponseBuilder::new(self.status_code())
                .json(R::json_fail(&self.message, self.code))
        }
    }
}

pub trait ResultExts<T> {
    fn set_html(self, is_ajax: bool) -> Result<T, BusinessError>;
}

impl<T, ToBusinessError> ResultExts<T> for Result<T, ToBusinessError>
    where ToBusinessError: Into<BusinessError> + std::fmt::Debug
{
    fn set_html(self, is_ajax: bool) -> Result<T, BusinessError> {
        if let Err(e) = self {
            let business_error = e.into().set_html(is_ajax);
            return Err(business_error);
        }
        Ok(self.unwrap())
    }
}