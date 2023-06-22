use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use sea_orm::{EntityTrait, FromQueryResult, QuerySelect};

use crate::errors::BusinessError;
use crate::models::datas::session_user_id::JwtUserId;
use crate::services::database::database::Database;

pub struct IsAdmin(bool);

impl FromRequest for IsAdmin {
    type Error = BusinessError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let session_user_id = JwtUserId::from_request(req, payload);
        Box::pin(async move {
            let user_id = session_user_id.await?;
            #[derive(FromQueryResult)]
            struct QueryResult {
                pub is_admin: bool,
            }

            let query_result = match entity::user::Entity::find_by_id(*user_id)
                .select_only()
                .column(entity::user::Column::IsAdmin)
                .into_model::<QueryResult>()
                .one(Database::get_conn())
                .await?
            {
                Some(q) => q,
                None => return Err(BusinessError::new_code("登录信息失效，请重新登录", 401)),
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
