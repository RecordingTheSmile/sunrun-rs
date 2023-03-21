use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Local;
use sea_orm::sea_query::Expr;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde_json::Value;
use tokio::sync::RwLock;
use tokio_scheduler_rs::errors::{SchedulerError, SchedulerErrorKind};
use tokio_scheduler_rs::{JobStorage, ScheduleJob};
use uuid::Uuid;

use crate::services::database::database::Database;

pub struct SqlJobStorage<Tz>
where
    Tz: chrono::TimeZone + Send + Sync,
{
    timezone: Tz,
    jobs: Arc<RwLock<HashMap<String, Box<dyn ScheduleJob>>>>,
}

unsafe impl<Tz: chrono::TimeZone + Sync + Send> Send for SqlJobStorage<Tz> {}

unsafe impl<Tz: chrono::TimeZone + Sync + Send> Sync for SqlJobStorage<Tz> {}

impl<Tz> SqlJobStorage<Tz>
where
    Tz: chrono::TimeZone + Send + Sync,
    Tz::Offset: Send + Sync,
{
    pub fn new(timezone: Tz) -> Self {
        Self {
            timezone,
            jobs: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl<Tz> JobStorage<Tz> for SqlJobStorage<Tz>
where
    Tz: chrono::TimeZone + Send + Sync,
    Tz::Offset: Send + Sync,
{
    async fn register_job(&self, job: Box<dyn ScheduleJob>) -> Result<(), SchedulerError> {
        self.jobs.write().await.insert(job.get_job_name(), job);
        Ok(())
    }

    async fn add_job(
        &self,
        job_name: String,
        cron: String,
        args: Option<Value>,
    ) -> Result<String, SchedulerError> {
        if !self.jobs.read().await.contains_key(&job_name) {
            return Err(SchedulerError::new(SchedulerErrorKind::JobNotExists));
        }

        let id = Uuid::new_v4();
        let time_now = Local::now().with_timezone(&self.timezone);

        let cron_scheduler = cron::Schedule::from_str(&cron)
            .map_err(|_| SchedulerError::new(SchedulerErrorKind::CronInvalid))?;

        let next_execute_at = match cron_scheduler.after(&time_now).take(1).into_iter().next() {
            Some(e) => e,
            None => return Ok(id.to_string()),
        };
        let time_now = time_now.timestamp();
        let next_execute_at = next_execute_at.timestamp();
        entity::job_storage::ActiveModel {
            id: Set(id),
            task_name: Set(job_name),
            cron: Set(cron),
            last_check_at: Set(time_now),
            next_execute_at: Set(next_execute_at),
            args: Set(args),
        }
        .insert(Database::get_conn())
        .await
        .map_err(|e| {
            log::error!("Database Error: {:#?}", e);
            SchedulerError::new(SchedulerErrorKind::CustomErr("数据库存储错误".into()))
        })?;
        Ok(id.to_string())
    }

    async fn delete_job(&self, id: String) -> Result<(), SchedulerError> {
        entity::job_storage::Entity::delete_by_id(
            Uuid::from_str(&id).map_err(|_| {
                SchedulerError::new(SchedulerErrorKind::CustomErr("Id不合法".into()))
            })?,
        )
        .exec(Database::get_conn())
        .await
        .map_err(|_| SchedulerError::new(SchedulerErrorKind::CustomErr("数据库删除失败".into())))?;

        Ok(())
    }

    async fn has_job(&self, id: String) -> Result<bool, SchedulerError> {
        let count =
            entity::job_storage::Entity::find_by_id(Uuid::from_str(&id).map_err(|_| {
                SchedulerError::new(SchedulerErrorKind::CustomErr("Id不合法".into()))
            })?)
            .count(Database::get_conn())
            .await
            .map_err(|e| {
                log::error!("Database Error: {:#?}", e);
                SchedulerError::new(SchedulerErrorKind::CustomErr("数据库读取失败".into()))
            })?;

        Ok(count != 0)
    }

    async fn get_all_should_execute_jobs(
        &self,
    ) -> Result<Vec<Pin<Box<dyn Future<Output = ()> + Send>>>, SchedulerError> {
        let self_jobs = self.jobs.read().await;
        let mut result = vec![];

        let time_now = Local::now().with_timezone(&self.timezone);

        let timestamp_now = time_now.timestamp();

        let tasks = entity::job_storage::Entity::find()
            .filter(entity::job_storage::Column::NextExecuteAt.lte(timestamp_now))
            .all(Database::get_conn())
            .await
            .map_err(|e| {
                log::error!("Database Error: {:#?}", e);
                SchedulerError::new(SchedulerErrorKind::CustomErr("数据库读取失败".into()))
            })?;

        for task in tasks {
            let job = match self_jobs.get(&task.task_name) {
                Some(j) => j,
                None => continue,
            };

            let cron_expr = match cron::Schedule::from_str(&task.cron) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let last_execute_at =
                match chrono::NaiveDateTime::from_timestamp_opt(task.last_check_at, 0) {
                    Some(e) => e,
                    None => continue,
                };

            let last_execute_at =
                chrono::DateTime::<chrono::Utc>::from_utc(last_execute_at, chrono::Utc)
                    .with_timezone(&self.timezone);
            let mut next_execute_at = time_now.timestamp();

            for i in cron_expr.after(&last_execute_at) {
                if i <= time_now {
                    let delta_time = i - time_now.to_owned();
                    // 30分钟内的任务重新执行
                    if delta_time.num_seconds() <= 30 * 60 {
                        result.push(job.execute(task.id.to_string(), task.args.to_owned()))
                    }
                } else {
                    next_execute_at = i.timestamp();
                    break;
                }
            }

            entity::job_storage::Entity::update_many()
                .filter(entity::job_storage::Column::Id.eq(task.id.to_owned()))
                .col_expr(
                    entity::job_storage::Column::NextExecuteAt,
                    Expr::value(next_execute_at),
                )
                .col_expr(
                    entity::job_storage::Column::LastCheckAt,
                    Expr::value(time_now.timestamp()),
                )
                .exec(Database::get_conn())
                .await
                .map_err(|e| {
                    log::error!("Database Error: {:#?}", e);
                    SchedulerError::new(SchedulerErrorKind::CustomErr("数据库更新失败".into()))
                })?;
        }

        Ok(result)
    }

    async fn restore_jobs(&self) -> Result<(), SchedulerError> {
        Ok(())
    }
}
