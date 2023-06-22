use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::errors::BusinessError;
use crate::services::managers::session_manager::SessionManager;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::HeaderValue;
use actix_web::HttpMessage;

pub struct SessionWrap;

pub struct SessionWrapService<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Transform<S, ServiceRequest> for SessionWrap
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::error::Error>
        + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::error::Error;
    type Transform = SessionWrapService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(SessionWrapService {
            service: Rc::new(RefCell::new(service)),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for SessionWrapService<S>
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
        let session_cookie = req.cookie("Session");
        Box::pin(async move {
            let session_manager = match session_cookie {
                Some(v) => SessionManager::get_or_create(v.value()).await?,
                None => SessionManager::create().await?,
            };

            let session_id = session_manager.get_session_id().to_owned();

            req.extensions_mut().insert(session_manager);

            let mut response: ServiceResponse<B> = svc.call(req).await?;

            let set_cookie_value = format!(
                "Session={}; Max-Age={}; SameSite=Lax; HttpOnly; Path=/",
                urlencoding::encode(&session_id),
                60 * 60
            );

            let set_cookie_value = HeaderValue::from_str(&set_cookie_value).map_err(|e| {
                log::error!("{:#?}", e);
                BusinessError::new("Session设置失败，请联系管理员")
            })?;
            response
                .headers_mut()
                .insert(actix_web::http::header::SET_COOKIE, set_cookie_value);
            Ok(response)
        })
    }
}
