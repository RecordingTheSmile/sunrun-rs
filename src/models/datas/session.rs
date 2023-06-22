use crate::errors::{BusinessError, BusinessResult};
use crate::services::managers::session_manager::SessionManager;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

pub struct Session(SessionManager);

impl FromRequest for Session {
    type Error = BusinessError;
    type Future = Pin<Box<dyn Future<Output = BusinessResult<Self>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let exts = req
            .extensions()
            .get::<SessionManager>()
            .and_then(|v| Some(v.to_owned()));
        Box::pin(async move {
            match exts {
                Some(v) => Ok(Self(v)),
                None => Err(BusinessError::new("Session未初始化")),
            }
        })
    }
}

impl Deref for Session {
    type Target = SessionManager;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
