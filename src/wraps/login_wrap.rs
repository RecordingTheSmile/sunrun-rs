use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::FromRequest;

use crate::models::datas::session_user_id::JwtUserId;

pub struct LoginWrap;

pub struct LoginWrapService<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Transform<S, ServiceRequest> for LoginWrap
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::error::Error>
        + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::error::Error;
    type Transform = LoginWrapService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(LoginWrapService {
            service: Rc::new(RefCell::new(service)),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for LoginWrapService<S>
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
        Box::pin(async move {
            let _ = JwtUserId::from_request(req.request(), &mut Payload::None).await;
            Ok(svc.call(req).await?)
        })
    }
}
