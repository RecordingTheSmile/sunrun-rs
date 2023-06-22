use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::FromRequest;
use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};

use crate::errors::BusinessError;
use crate::models::datas::session_user_id::JwtUserId;
use crate::services::database::database::Database;

pub struct AdminLoginWrap;

pub struct AdminLoginWrapService<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Transform<S, ServiceRequest> for AdminLoginWrap
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::error::Error>
        + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::error::Error;
    type Transform = AdminLoginWrapService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AdminLoginWrapService {
            service: Rc::new(RefCell::new(service)),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AdminLoginWrapService<S>
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
        let session = JwtUserId::from_request(req.request(), &mut Payload::None);
        let svc = self.service.to_owned();

        Box::pin(async move {
            let user_id = session.await?;

            #[derive(FromQueryResult)]
            struct QueryResult {
                pub can_login: bool,
                pub is_admin: bool,
            }

            let user_query = match entity::user::Entity::find()
                .filter(entity::user::Column::Id.eq(*user_id))
                .select_only()
                .column(entity::user::Column::CanLogin)
                .column(entity::user::Column::IsAdmin)
                .into_model::<QueryResult>()
                .one(Database::get_conn())
                .await
                .map_err(|e| {
                    log::error!("DbErr: {:?}", e);
                    BusinessError::default()
                })? {
                Some(u) => u,
                None => return Err(BusinessError::new_code("用户信息有误，请重新登录", 401).into()),
            };

            if !user_query.can_login {
                return Err(BusinessError::new_code("您的账户已被封禁", 401).into());
            }

            if !user_query.is_admin {
                return Err(BusinessError::new_code("您没有访问该页面的权限", 403).into());
            }
            Ok(svc.call(req).await?)
        })
    }
}
