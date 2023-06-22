use actix_web::{get, put, web, HttpResponse};
use sea_orm::{
    ActiveModelTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryOrder, QuerySelect, Set,
};
use serde::Serialize;
use serde_json::json;

use crate::common::r::R;
use crate::errors::{BusinessError, BusinessResult};
use crate::models::datas::session_user_id::JwtUserId;
use crate::models::dtos::admin_api_models::{PutCanLoginBody, PutIsAdminBody};
use crate::models::dtos::common_models::PageInfo;
use crate::services::database::database::Database;
use crate::wraps::admin_login_wrap::AdminLoginWrap;

#[get("/admin/api/user", wrap = "AdminLoginWrap")]
pub async fn get_user(query: actix_web_validator::Query<PageInfo>) -> BusinessResult<HttpResponse> {
    #[derive(FromQueryResult, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct QueryResult {
        pub id: i64,
        pub username: String,
        pub email: String,
        pub can_login: bool,
        pub is_admin: bool,
        pub create_at: i64,
    }

    let rows = entity::user::Entity::find()
        .order_by_asc(entity::user::Column::CreateAt)
        .limit(query.limit)
        .offset((query.page - 1) * query.limit)
        .select_only()
        .column(entity::user::Column::Id)
        .column(entity::user::Column::Username)
        .column(entity::user::Column::Email)
        .column(entity::user::Column::CanLogin)
        .column(entity::user::Column::IsAdmin)
        .column(entity::user::Column::CreateAt)
        .into_model::<QueryResult>()
        .all(Database::get_conn())
        .await?;

    let total = entity::user::Entity::find()
        .count(Database::get_conn())
        .await?;

    Ok(HttpResponse::Ok().json(R::json_success_data(json!({
        "total":total,
        "rows":rows
    }))))
}

#[put("/admin/api/user/{id}/loginStatus", wrap = "AdminLoginWrap")]
pub async fn put_can_login(
    id: web::Path<i64>,
    body: web::Json<PutCanLoginBody>,
    user_id: JwtUserId,
) -> BusinessResult<HttpResponse> {
    if *user_id == *id {
        return Err(BusinessError::new_code("您不能操作自己", 400));
    }

    entity::user::ActiveModel {
        id: Set(*id),
        can_login: Set(body.can_login),
        ..Default::default()
    }
    .update(Database::get_conn())
    .await?;
    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[put("/admin/api/user/{id}/isAdmin", wrap = "AdminLoginWrap")]
pub async fn put_is_admin(
    id: web::Path<i64>,
    body: web::Json<PutIsAdminBody>,
    user_id: JwtUserId,
) -> BusinessResult<HttpResponse> {
    if *user_id == *id {
        return Err(BusinessError::new_code("您不能操作自己", 403));
    }

    entity::user::ActiveModel {
        id: Set(*id),
        is_admin: Set(body.is_admin),
        ..Default::default()
    }
    .update(Database::get_conn())
    .await?;
    Ok(HttpResponse::Ok().json(R::json_success()))
}
