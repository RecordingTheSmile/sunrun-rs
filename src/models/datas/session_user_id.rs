use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};

use crate::errors::{BusinessError, BusinessResult};
use crate::models::datas::jwt_data::JwtData;
use crate::services::database::database::Database;

pub struct JwtUserId(i64);

impl FromRequest for JwtUserId {
    type Error = BusinessError;
    type Future = Pin<Box<dyn Future<Output = BusinessResult<Self>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let jwt_data = JwtData::from_request(req, payload);
        Box::pin(async move {
            let jwt_data = jwt_data.await?;
            let user_id = match jwt_data["user_id"].as_i64() {
                Some(u) => u,
                None => return Err(BusinessError::new_code("请先登录", 401)),
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
                .await?
            {
                Some(u) => u,
                None => {
                    return Err(BusinessError::new_code("用户信息有误，请重新登录", 401));
                }
            };

            if !user_query.can_login {
                return Err(BusinessError::new_code("您的账户已被封禁", 401));
            }

            Ok(Self(user_id))
        })
    }
}

impl Deref for JwtUserId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
