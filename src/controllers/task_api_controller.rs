use actix_web::{delete, get, HttpResponse, post, put, web};
use chrono::Local;
use lazy_static::lazy_static;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde::Serialize;
use serde_json::json;
use tokio_scheduler_rs::ScheduleJob;

use migration::{JoinType, Query};

use crate::common::r::R;
use crate::errors::{BusinessError, BusinessResult};
use crate::models::datas::session_user_id::SessionUserId;
use crate::models::dtos::common_models::PageInfo;
use crate::models::dtos::task_api_models::PostTaskBody;
use crate::services::database::database::Database;
use crate::services::managers::sunrun_task_manager::SunrunTaskManager;
use crate::services::managers::sunrun_tasklog_manager::SunrunTasklogManager;
use crate::services::sunrun::sunrun::Sunrun;
use crate::tasks::sunrun_task::SunrunTask;
use crate::wraps::login_wrap::LoginWrap;

lazy_static! {
    static ref EMAIL_REGEX: regex::Regex = regex::Regex::new("(.+)@(.+)\\.(.+)").unwrap();
}

#[get("/task/api/task", wrap = "LoginWrap")]
pub async fn get_task(query: actix_web_validator::Query<PageInfo>, user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    #[derive(FromQueryResult, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct QueryResult {
        pub id: i64,
        pub nick_name: String,
        pub hour: i32,
        pub minute: i32,
        pub length: i64,
        pub min_speed: f32,
        pub max_speed: f32,
        pub is_enable: bool,
        pub create_at: i64,
        pub school_name: String,
        pub latitude: String,
        pub longitude: String,
    }

    let rows = entity::sunrun_userinfo::Entity::find()
        .filter(entity::sunrun_userinfo::Column::CreateBy.eq(*user_id))
        .limit(query.limit)
        .offset((query.page - 1) * query.limit)
        .order_by_asc(entity::sunrun_userinfo::Column::CreateAt)
        .select_only()
        .column(entity::sunrun_userinfo::Column::Id)
        .column(entity::sunrun_userinfo::Column::NickName)
        .column(entity::sunrun_userinfo::Column::Hour)
        .column(entity::sunrun_userinfo::Column::Minute)
        .column(entity::sunrun_userinfo::Column::Length)
        .column(entity::sunrun_userinfo::Column::MinSpeed)
        .column(entity::sunrun_userinfo::Column::MaxSpeed)
        .column(entity::sunrun_userinfo::Column::IsEnable)
        .column(entity::sunrun_userinfo::Column::CreateAt)
        .column(entity::sunrun_userinfo::Column::SchoolName)
        .column(entity::sunrun_userinfo::Column::Latitude)
        .column(entity::sunrun_userinfo::Column::Longitude)
        .into_model::<QueryResult>()
        .all(Database::get_conn())
        .await?;

    let total = entity::sunrun_userinfo::Entity::find()
        .filter(entity::sunrun_userinfo::Column::CreateBy.eq(*user_id))
        .count(Database::get_conn())
        .await?;
    Ok(HttpResponse::Ok().json(R::json_success_data(json!({
        "rows":rows,
        "total":total
    }))))
}

#[post("/task/api/task", wrap = "LoginWrap")]
pub async fn post_task(body: actix_web_validator::Json<PostTaskBody>, user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    if let Some(email) = &body.email {
        if !EMAIL_REGEX.is_match(email) {
            return Err(BusinessError::new_code("邮箱格式不正确", 400));
        }
    }

    let mut sunrun = Sunrun::new(&body.imeicode);

    let _ = sunrun.get_token(body.is_iphone)
        .await.map_err(|e| BusinessError::new_code(format!("IMEICODE无效：{}", e.message), 400))?;

    let userinfo = sunrun.get_userinfo().await?;

    let model = entity::sunrun_userinfo::ActiveModel {
        length: Set(userinfo.length),
        max_speed: Set(userinfo.max_speed as f32),
        min_speed: Set(userinfo.min_speed as f32),
        school_name: Set(userinfo.school_name),
        nick_name: Set(userinfo.nick_name),
        user_id: Set(userinfo.user_id),
        latitude: Set(body.latitude.to_owned()),
        longitude: Set(body.longitude.to_owned()),
        step: Set(body.step),
        hour: Set(body.hour),
        minute: Set(body.minute),
        email: Set(body.email.to_owned()),
        imeicode: Set(body.imeicode.to_owned()),
        is_iphone: Set(body.is_iphone),
        is_enable: Set(body.is_enable),
        create_at: Set(Local::now().timestamp()),
        update_at: Set(Local::now().timestamp()),
        create_by: Set(*user_id),
        ..Default::default()
    }.insert(Database::get_conn()).await?;

    if body.is_enable {
        SunrunTaskManager::create_task(model.id, body.hour, body.minute).await?;
    }

    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[put("/task/api/task/{id}", wrap = "LoginWrap")]
pub async fn put_task(id: web::Path<i64>, body: actix_web_validator::Json<PostTaskBody>, user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    #[derive(FromQueryResult)]
    struct QueryResult {
        pub create_by: i64,
        pub hour: i32,
        pub minute: i32,
        pub is_enable: bool,
    }

    let query_result = match entity::sunrun_userinfo::Entity::find_by_id(*id)
        .select_only()
        .column(entity::sunrun_userinfo::Column::CreateBy)
        .column(entity::sunrun_userinfo::Column::Hour)
        .column(entity::sunrun_userinfo::Column::Minute)
        .column(entity::sunrun_userinfo::Column::IsEnable)
        .into_model::<QueryResult>()
        .one(Database::get_conn())
        .await? {
        Some(s) => s,
        None => return Err(BusinessError::new_code("任务信息不存在", 404))
    };

    if query_result.create_by != *user_id {
        return Err(BusinessError::new_code("您不可以编辑不属于自己的任务信息", 403));
    }

    if let Some(email) = &body.email {
        if !EMAIL_REGEX.is_match(email) {
            return Err(BusinessError::new_code("邮箱格式不正确", 400));
        }
    }

    let mut sunrun = Sunrun::new(&body.imeicode);

    let _ = sunrun.get_token(body.is_iphone).await
        .map_err(|e| BusinessError::new_code(format!("IMEICODE无效：{}", e.message), 400))?;

    let userinfo = sunrun.get_userinfo().await?;

    entity::sunrun_userinfo::ActiveModel {
        id: Set(*id),
        length: Set(userinfo.length),
        max_speed: Set(userinfo.max_speed as f32),
        min_speed: Set(userinfo.min_speed as f32),
        school_name: Set(userinfo.school_name),
        nick_name: Set(userinfo.nick_name),
        user_id: Set(userinfo.user_id),
        latitude: Set(body.latitude.to_owned()),
        longitude: Set(body.longitude.to_owned()),
        step: Set(body.step),
        hour: Set(body.hour),
        minute: Set(body.minute),
        email: Set(body.email.to_owned()),
        imeicode: Set(body.imeicode.to_owned()),
        is_iphone: Set(body.is_iphone),
        is_enable: Set(body.is_enable),
        update_at: Set(Local::now().timestamp()),
        ..Default::default()
    }.update(Database::get_conn()).await?;

    if query_result.is_enable != body.is_enable || query_result.hour != body.hour || query_result.minute != body.minute {
        SunrunTaskManager::delete_task(*id).await?;
        if body.is_enable {
            SunrunTaskManager::create_task(*id, body.hour, body.minute).await?;
        }
    }

    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[delete("/task/api/task/{id}", wrap = "LoginWrap")]
pub async fn delete_task(id: web::Path<i64>, user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    #[derive(FromQueryResult)]
    struct QueryResult {
        pub create_by: i64,
    }

    let query_result = match entity::sunrun_userinfo::Entity::find_by_id(*id)
        .select_only()
        .column(entity::sunrun_userinfo::Column::CreateBy)
        .into_model::<QueryResult>()
        .one(Database::get_conn())
        .await? {
        Some(u) => u,
        None => return Err(BusinessError::new_code("任务不存在", 404))
    };

    if query_result.create_by != *user_id {
        return Err(BusinessError::new_code("您不能删除不属于自己的任务", 403));
    }

    entity::sunrun_userinfo::Entity::delete_by_id(*id).exec(Database::get_conn()).await?;

    SunrunTaskManager::delete_task(*id).await?;
    SunrunTasklogManager::delete_log_by_user_info_id(*id).await?;

    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[post("/task/api/task/{id}/executeNow", wrap = "LoginWrap")]
pub async fn post_execute_now(id: web::Path<i64>, user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    #[derive(FromQueryResult)]
    struct QueryResult {
        pub create_by: i64,
        pub args: Option<serde_json::Value>,
        pub task_id: uuid::Uuid,
    }

    let query_result = match entity::sunrun_userinfo::Entity::find_by_id(*id)
        .join_rev(JoinType::InnerJoin, entity::sunrun_task::Entity::belongs_to(entity::sunrun_userinfo::Entity)
            .from(entity::sunrun_task::Column::TaskInfoId)
            .to(entity::sunrun_userinfo::Column::Id)
            .into())
        .join_rev(JoinType::InnerJoin, entity::job_storage::Entity::belongs_to(entity::sunrun_task::Entity)
            .from(entity::job_storage::Column::Id)
            .to(entity::sunrun_task::Column::TaskId)
            .into())
        .select_only()
        .column(entity::sunrun_userinfo::Column::CreateBy)
        .column(entity::job_storage::Column::Args)
        .column_as(entity::job_storage::Column::Id, "task_id")
        .into_model::<QueryResult>()
        .one(Database::get_conn())
        .await? {
        Some(q) => q,
        None => return Err(BusinessError::new_code("任务不存在", 404))
    };

    if query_result.create_by != *user_id {
        return Err(BusinessError::new_code("您不能操作不属于自己的任务", 403));
    }

    let task = SunrunTask {}.execute(query_result.task_id.to_string(), query_result.args);
    tokio::spawn(async move {
        task.await;
    });
    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[get("/task/api/taskLog", wrap = "LoginWrap")]
pub async fn get_task_log(query: actix_web_validator::Query<PageInfo>, user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    #[derive(Serialize, FromQueryResult)]
    #[serde(rename_all = "camelCase")]
    struct QueryResult {
        pub id: i64,
        pub create_at: i64,
        pub description: String,
        pub is_success: bool,
        pub nick_name: String,
        pub school_name: String,
    }

    let query_result = entity::sunrun_tasklog::Entity::find()
        .join_rev(JoinType::InnerJoin, entity::sunrun_userinfo::Entity::belongs_to(entity::sunrun_tasklog::Entity)
            .from(entity::sunrun_userinfo::Column::Id)
            .to(entity::sunrun_tasklog::Column::TaskInfoId)
            .into())
        .filter(entity::sunrun_userinfo::Column::CreateBy.eq(*user_id))
        .order_by_desc(entity::sunrun_tasklog::Column::CreateAt)
        .limit(query.limit)
        .offset((query.page - 1) * query.limit)
        .select_only()
        .column(entity::sunrun_tasklog::Column::Id)
        .column(entity::sunrun_tasklog::Column::CreateAt)
        .column(entity::sunrun_tasklog::Column::Description)
        .column(entity::sunrun_tasklog::Column::IsSuccess)
        .column(entity::sunrun_userinfo::Column::NickName)
        .column(entity::sunrun_userinfo::Column::SchoolName)
        .into_model::<QueryResult>()
        .all(Database::get_conn())
        .await?;

    let total = entity::sunrun_tasklog::Entity::find()
        .join_rev(JoinType::InnerJoin, entity::sunrun_userinfo::Entity::belongs_to(entity::sunrun_tasklog::Entity)
            .from(entity::sunrun_userinfo::Column::Id)
            .to(entity::sunrun_tasklog::Column::TaskInfoId)
            .into())
        .filter(entity::sunrun_userinfo::Column::CreateBy.eq(*user_id))
        .count(Database::get_conn())
        .await?;

    Ok(HttpResponse::Ok().json(R::json_success_data(json!({
        "total":total,
        "rows":query_result
    }))))
}

#[delete("/task/api/taskLog", wrap = "LoginWrap")]
pub async fn delete_task_log(user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    entity::sunrun_tasklog::Entity::delete_many()
        .filter(entity::sunrun_tasklog::Column::TaskInfoId.in_subquery(
            Query::select()
                .column(entity::sunrun_userinfo::Column::Id)
                .from(entity::sunrun_userinfo::Entity)
                .and_where(entity::sunrun_userinfo::Column::CreateBy.eq(*user_id))
                .distinct()
                .to_owned()
        )).exec(Database::get_conn()).await?;
    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[get("/task/api/task/{id}/status")]
pub async fn get_task_status(user_id: SessionUserId, id: web::Path<i64>) -> BusinessResult<HttpResponse> {
    #[derive(FromQueryResult)]
    struct QueryResult {
        pub imeicode: String,
        pub create_by: i64,
        pub is_iphone: bool,
    }

    let query_result = match entity::sunrun_userinfo::Entity::find_by_id(*id)
        .select_only()
        .column(entity::sunrun_userinfo::Column::Imeicode)
        .column(entity::sunrun_userinfo::Column::CreateBy)
        .column(entity::sunrun_userinfo::Column::IsIphone)
        .into_model::<QueryResult>()
        .one(Database::get_conn())
        .await? {
        Some(q) => q,
        None => return Err(BusinessError::new_code("任务不存在", 404))
    };

    if query_result.create_by != *user_id {
        return Err(BusinessError::new_code("您不可以查看不属于自己的任务", 403));
    }

    let mut sunrun = Sunrun::new(query_result.imeicode);
    sunrun.get_token(query_result.is_iphone).await?;

    let run_result = sunrun.get_run_times().await?;
    Ok(HttpResponse::Ok().json(R::json_success_data(json!({
        "totalTimes":run_result.run_times,
        "morningTimes":run_result.morning_run_times
    }))))
}