use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

use actix_session::SessionExt;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};

use crate::common::actix_exts::CommonHttpRequestExts;
use crate::errors::{BusinessError, BusinessResult};
use crate::errors::business_error::ResultExts;
use crate::services::database::database::Database;

pub struct SessionUserId(i64);

impl FromRequest for SessionUserId {
    type Error = BusinessError;
    type Future = Pin<Box<dyn Future<Output=BusinessResult<Self>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let session = req.get_session();
        let is_html = !req.is_ajax();
        Box::pin(async move {
            let user_id = match session.get::<i64>("user_id")? {
                Some(u) => u,
                None => return Err(BusinessError::new_code("请先登录", 401).set_html(is_html))
            };

            #[derive(FromQueryResult)]
            struct QueryResult {
                pub can_login: bool,
            }

            let user_query = match entity::user::Entity::find()
                .filter(entity::user::Column::Id.eq(user_id))
                .select_only()
                .column(entity::user::Column::CanLogin)
                .into_model::<QueryResult>()
                .one(Database::get_conn())
                .await.set_html(is_html)? {
                Some(u) => u,
                None => {
                    session.remove("user_id");
                    return Err(BusinessError::new_code("用户信息有误，请重新登录", 401).set_html(is_html));
                }
            };

            if !user_query.can_login {
                return Err(BusinessError::new_code("您的账户已被封禁", 401).set_html(is_html));
            }

            Ok(Self(user_id))
        })
    }
}

impl Deref for SessionUserId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}