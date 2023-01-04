use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

use actix_session::SessionExt;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use sea_orm::{EntityTrait, FromQueryResult, QuerySelect};

use crate::common::actix_exts::CommonHttpRequestExts;
use crate::errors::business_error::ResultExts;
use crate::errors::BusinessError;
use crate::services::database::database::Database;

pub struct IsAdmin(bool);

impl FromRequest for IsAdmin {
    type Error = BusinessError;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let session = req.get_session();
        let is_ajax = req.is_ajax();
        Box::pin(async move {
            let user_id = match session.get::<i64>("user_id").set_html(is_ajax)? {
                Some(u) => u,
                None => return Err(BusinessError::new_code("请先登录", 401).set_html(is_ajax))
            };
            #[derive(FromQueryResult)]
            struct QueryResult {
                pub is_admin: bool,
            }

            let query_result = match entity::user::Entity::find_by_id(user_id)
                .select_only()
                .column(entity::user::Column::IsAdmin)
                .into_model::<QueryResult>()
                .one(Database::get_conn())
                .await.set_html(is_ajax)?
            {
                Some(q) => q,
                None => return Err(BusinessError::new_code("登录信息失效，请重新登录", 401).set_html(is_ajax))
            };

            Ok(Self(query_result.is_admin))
        })
    }
}

impl Deref for IsAdmin {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}