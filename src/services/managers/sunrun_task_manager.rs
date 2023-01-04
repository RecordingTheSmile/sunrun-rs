use std::str::FromStr;

use chrono::Local;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect, Set};
use sea_orm::sea_query::Expr;
use serde_json::json;

use crate::errors::{BusinessError, BusinessResult};
use crate::services::database::database::Database;
use crate::services::scheduler::scheduler::Scheduler;

pub struct SunrunTaskManager;

impl SunrunTaskManager {
    pub async fn create_task(user_info_id: i64, hour: i32, minute: i32) -> BusinessResult<i64> {
        let id = Scheduler::get().add_job(crate::tasks::sunrun_task::TASK_NAME.to_owned(), format!("0 {minute} {hour} * * * *"), Some(json!({
            "id":user_info_id
        }))).await?;

        let uuid = uuid::Uuid::from_str(&id)?;
        let model = entity::sunrun_task::ActiveModel {
            task_info_id: Set(user_info_id),
            create_at: Set(Local::now().timestamp()),
            task_id: Set(uuid),
            ..Default::default()
        }.insert(Database::get_conn()).await?;

        Ok(model.id)
    }

    pub async fn update_task_enable(user_info_id: i64, is_enable: bool) -> BusinessResult<()> {
        let task_info = match entity::sunrun_task::Entity::find()
            .filter(entity::sunrun_task::Column::TaskInfoId.eq(user_info_id))
            .one(Database::get_conn())
            .await? {
            Some(s) => s,
            None => return Err(BusinessError::new_code("任务不存在", 404))
        };

        if is_enable {
            if !Scheduler::get().has_job(task_info.task_id.to_string()).await? {
                #[derive(FromQueryResult)]
                struct QueryResult {
                    pub hour: i32,
                    pub minute: i32,
                }

                let task_info = match entity::sunrun_userinfo::Entity::find_by_id(user_info_id)
                    .select_only()
                    .column(entity::sunrun_userinfo::Column::Hour)
                    .column(entity::sunrun_userinfo::Column::Minute)
                    .into_model::<QueryResult>()
                    .one(Database::get_conn())
                    .await? {
                    Some(s) => s,
                    None => return Err(BusinessError::new_code("任务信息不存在", 404))
                };
                Scheduler::get().add_job(crate::tasks::sunrun_task::TASK_NAME.to_owned(), format!("0 {} {} * * * *", task_info.minute, task_info.hour), Some(json!({
                    "id":user_info_id
                }))).await?;
            }
        } else {
            Scheduler::get().delete_job(task_info.task_id.to_string()).await?;
        }
        entity::sunrun_userinfo::Entity::update_many()
            .filter(entity::sunrun_userinfo::Column::Id.eq(user_info_id))
            .col_expr(entity::sunrun_userinfo::Column::IsEnable, Expr::value(is_enable))
            .exec(Database::get_conn())
            .await?;
        Ok(())
    }

    pub async fn delete_task(user_info_id: i64) -> BusinessResult<()> {
        let task_info = match entity::sunrun_task::Entity::find()
            .filter(entity::sunrun_task::Column::TaskInfoId.eq(user_info_id))
            .one(Database::get_conn())
            .await? {
            Some(s) => s,
            None => return Ok(())
        };

        Scheduler::get().delete_job(task_info.task_id.to_string()).await?;

        entity::sunrun_task::Entity::delete_many()
            .filter(entity::sunrun_task::Column::TaskInfoId.eq(user_info_id))
            .exec(Database::get_conn())
            .await?;
        Ok(())
    }
}