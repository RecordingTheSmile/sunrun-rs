use actix_web::{get, HttpResponse, web};
use sea_orm::{EntityTrait, FromQueryResult, QuerySelect};
use serde::Serialize;

use crate::errors::{BusinessError, BusinessResult};
use crate::errors::business_error::ResultExts;
use crate::models::datas::session_user_id::SessionUserId;
use crate::services::database::database::Database;
use crate::services::template::template::ActixHttpResponseExt;
use crate::wraps::admin_login_wrap::AdminLoginWrap;
use crate::wraps::login_wrap::LoginWrap;

#[get("/console/iframes/index", wrap = "LoginWrap")]
pub async fn get_index() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/iframes/index.html", &tera::Context::new()))
}

#[get("/console/iframes/sunrunTasks", wrap = "LoginWrap")]
pub async fn get_sunrun_tasks() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/iframes/sunrun_tasks.html", &tera::Context::new()))
}

#[get("/console/iframes/addSunrunTask", wrap = "LoginWrap")]
pub async fn get_add_sunrun_task() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/iframes/add_sunrun_task.html", &tera::Context::new()))
}

#[get("/console/iframes/editSunrunTask/{id}", wrap = "LoginWrap")]
pub async fn get_edit_sunrun_task(user_id: SessionUserId, id: web::Path<i64>) -> BusinessResult<HttpResponse> {
    #[derive(FromQueryResult, Serialize)]
    struct QueryResult {
        pub id: i64,
        pub imeicode: String,
        pub hour: i32,
        pub minute: i32,
        pub create_by: i64,
        pub is_enable: bool,
        pub is_iphone: bool,
        pub longitude: String,
        pub latitude: String,
        pub step: i64,
        pub email: Option<String>,
    }

    let query_result = match entity::sunrun_userinfo::Entity::find_by_id(*id)
        .select_only()
        .column(entity::sunrun_userinfo::Column::Id)
        .column(entity::sunrun_userinfo::Column::Imeicode)
        .column(entity::sunrun_userinfo::Column::Hour)
        .column(entity::sunrun_userinfo::Column::Minute)
        .column(entity::sunrun_userinfo::Column::CreateBy)
        .column(entity::sunrun_userinfo::Column::IsEnable)
        .column(entity::sunrun_userinfo::Column::IsIphone)
        .column(entity::sunrun_userinfo::Column::Longitude)
        .column(entity::sunrun_userinfo::Column::Latitude)
        .column(entity::sunrun_userinfo::Column::Step)
        .column(entity::sunrun_userinfo::Column::Email)
        .into_model::<QueryResult>()
        .one(Database::get_conn())
        .await.set_html(true)? {
        Some(q) => q,
        None => return Err(BusinessError::new_code("任务不存在", 404).set_html(true))
    };

    if query_result.create_by != *user_id {
        return Err(BusinessError::new_code("您不能编辑不属于自己的任务", 403).set_html(true));
    }

    let mut context = tera::Context::new();

    context.insert("info", &query_result);

    Ok(HttpResponse::Ok().html("console/iframes/edit_sunrun_task.html", &context))
}

#[get("/console/iframes/sunrunTasklogs", wrap = "LoginWrap")]
pub async fn get_sunrun_task_logs() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/iframes/sunrun_task_logs.html", &tera::Context::new()))
}

#[get("/console/iframes/users", wrap = "AdminLoginWrap")]
pub async fn get_users() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/iframes/users.html", &tera::Context::new()))
}

#[get("/console/iframes/editUsername")]
pub async fn get_edit_username(user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    #[derive(FromQueryResult)]
    struct QueryResult {
        pub username: String,
    }

    let query_result = match entity::user::Entity::find_by_id(*user_id)
        .select_only()
        .column(entity::user::Column::Username)
        .into_model::<QueryResult>()
        .one(Database::get_conn())
        .await.set_html(true)? {
        Some(q) => q,
        None => return Err(BusinessError::new_code("用户不存在", 404).set_html(true))
    };

    let mut context = tera::Context::new();
    context.insert("username", &query_result.username);
    Ok(HttpResponse::Ok().html("console/iframes/edit_username.html", &context))
}

#[get("/console/iframes/editPassword", wrap = "LoginWrap")]
pub async fn get_edit_password() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/iframes/edit_password.html", &tera::Context::new()))
}

#[get("/console/iframes/editEmail", wrap = "LoginWrap")]
pub async fn get_edit_email() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/iframes/edit_email.html", &tera::Context::new()))
}

#[get("/console/iframes/taskStatus", wrap = "LoginWrap")]
pub async fn get_task_status() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/iframes/sunrun_task_status.html", &tera::Context::new()))
}