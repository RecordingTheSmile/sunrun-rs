use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_session::SessionExt;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};

use crate::common::actix_exts::CommonHttpRequestExts;
use crate::errors::BusinessError;
use crate::services::database::database::Database;

pub struct AdminLoginWrap;

pub struct AdminLoginWrapService<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Transform<S, ServiceRequest> for AdminLoginWrap
    where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::error::Error> + 'static,
          B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::error::Error;
    type Transform = AdminLoginWrapService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AdminLoginWrapService {
            service: Rc::new(RefCell::new(service))
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AdminLoginWrapService<S>
    where S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=actix_web::error::Error> + 'static,
          B: 'static,
          S::Future: 'static
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::error::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let svc = self.service.to_owned();
        let is_html = !req.is_ajax();
        Box::pin(async move {
            let user_id = match session.get::<i64>("user_id")? {
                Some(u) => u,
                None => return Err(BusinessError::new_code("请先登录", 401).set_html(is_html).into())
            };

            #[derive(FromQueryResult)]
            struct QueryResult {
                pub can_login: bool,
                pub is_admin: bool,
            }

            let user_query = match entity::user::Entity::find()
                .filter(entity::user::Column::Id.eq(user_id))
                .select_only()
                .column(entity::user::Column::CanLogin)
                .column(entity::user::Column::IsAdmin)
                .into_model::<QueryResult>()
                .one(Database::get_conn())
                .await.map_err(|e| {
                log::error!("DbErr: {:?}",e);
                BusinessError::default().set_html(is_html)
            })?
            {
                Some(u) => u,
                None => return Err(BusinessError::new_code("用户信息有误，请重新登录", 401).set_html(is_html).into())
            };

            if !user_query.can_login {
                return Err(BusinessError::new_code("您的账户已被封禁", 401).set_html(is_html).into());
            }

            if !user_query.is_admin {
                return Err(BusinessError::new_code("您没有访问该页面的权限", 403).set_html(is_html).into());
            }
            Ok(svc.call(req).await?)
        })
    }
}