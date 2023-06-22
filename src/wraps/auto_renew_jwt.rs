use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::str::FromStr;
use std::task::{Context, Poll};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};

use crate::services::managers::jwt_manager::JwtManager;

pub struct AutoRenewJwt;

pub struct AutoRenewJwtService<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Transform<S, ServiceRequest> for AutoRenewJwt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::error::Error>
        + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::error::Error;
    type Transform = AutoRenewJwtService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AutoRenewJwtService {
            service: Rc::new(RefCell::new(service)),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AutoRenewJwtService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::error::Error>
        + 'static,
    B: 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.to_owned();
        let auth_header = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|v| Some(v.to_str().unwrap_or_default().to_owned()))
            .unwrap_or(String::new());

        Box::pin(async move {
            let mut response: Self::Response = svc.call(req).await?;
            if let Some((auth_type, auth_token)) = auth_header.split_once(' ') {
                if auth_type.to_lowercase() != "bearer" {
                    return Ok(response);
                }
                let jwt_manager = JwtManager::get_global();
                let data = match jwt_manager.get_verified_claims(auth_token) {
                    Ok(v) => v,
                    Err(_) => return Ok(response),
                };

                let time_now = chrono::Local::now().timestamp();

                if let Some(exp) = data.expires_at {
                    // 10分钟之内过期则自动刷新token
                    if exp.as_secs() - time_now as u64 <= 10 * 60 {
                        match JwtManager::get_global().get_signed_token(data.custom) {
                            Ok(token) => {
                                let header_value =
                                    HeaderValue::from_str(&urlencoding::encode(&token));
                                match header_value {
                                    Ok(v) => {
                                        let headers = response.headers_mut();
                                        headers.insert(
                                            HeaderName::from_str("x-authorization").unwrap(),
                                            v,
                                        );
                                    }
                                    Err(e) => {
                                        log::error!("{:#?}", e);
                                    }
                                }
                            }
                            Err(_) => (),
                        };
                    }
                }
                return Ok(response);
            } else {
                return Ok(response);
            }
        })
    }
}
