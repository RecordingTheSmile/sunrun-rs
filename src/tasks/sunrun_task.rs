use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use sea_orm::EntityTrait;
use serde_json::Value;
use tokio_scheduler_rs::ScheduleJob;

use crate::services::database::database::Database;
use crate::services::managers::email_manager::EmailManager;
use crate::services::managers::sunrun_tasklog_manager::SunrunTasklogManager;
use crate::services::scheduler::scheduler::Scheduler;
use crate::services::sunrun::models::SunrunUserInfo;
use crate::services::sunrun::sunrun::Sunrun;

pub const TASK_NAME: &str = "SunrunTask";

pub struct SunrunTask;

impl ScheduleJob for SunrunTask {
    fn get_job_name(&self) -> String {
        String::from(TASK_NAME)
    }

    fn execute(&self, id: String, args: Option<Value>) -> Pin<Box<dyn Future<Output=()> + Send>> {
        Box::pin(async move {
            let user_info_id = match args {
                Some(a) => match a["id"].as_i64() {
                    Some(a) => a,
                    None => {
                        let _ = Scheduler::get().delete_job(id).await;
                        return;
                    }
                },
                None => {
                    let _ = Scheduler::get().delete_job(id).await;
                    return;
                }
            };

            let sunrun_userinfo = match entity::sunrun_userinfo::Entity::find_by_id(user_info_id)
                .one(Database::get_conn())
                .await {
                Ok(s) => match s {
                    Some(s) => s,
                    None => {
                        let _ = Scheduler::get().delete_job(id).await;
                        return;
                    }
                },
                Err(e) => {
                    log::error!("Database Error: {:?}",e);
                    return;
                }
            };

            for i in 1..=3 {
                let mut sunrun = Sunrun::new(&sunrun_userinfo.imeicode);
                let _ = match sunrun.get_token(sunrun_userinfo.is_iphone).await {
                    Ok(_) => (),
                    Err(e) => {
                        if i >= 3 {
                            let _ = SunrunTasklogManager::create_log(sunrun_userinfo.id, false, &e.message).await;
                            if let Some(email) = sunrun_userinfo.email.to_owned() {
                                EmailManager::send_result_email(email, false, Some(e.message));
                            }
                        }
                        tokio::time::sleep(Duration::from_secs(5 * i)).await;
                        continue;
                    }
                };

                match sunrun.start_run(SunrunUserInfo {
                    length: sunrun_userinfo.length,
                    max_speed: sunrun_userinfo.max_speed as f64,
                    min_speed: sunrun_userinfo.min_speed as f64,
                    school_name: sunrun_userinfo.school_name.to_owned(),
                    nick_name: sunrun_userinfo.nick_name.to_owned(),
                    user_id: sunrun_userinfo.user_id,
                    latitude: sunrun_userinfo.latitude.to_owned(),
                    longitude: sunrun_userinfo.longitude.to_owned(),
                    step: sunrun_userinfo.step,
                }).await {
                    Ok(_) => (),
                    Err(e) => {
                        if i >= 3 {
                            let _ = SunrunTasklogManager::create_log(sunrun_userinfo.id, false, &e.message).await;
                            if let Some(email) = sunrun_userinfo.email.to_owned() {
                                EmailManager::send_result_email(email, false, Some(e.message));
                            }
                        }
                        tokio::time::sleep(Duration::from_secs(5 * i)).await;
                        continue;
                    }
                };
                break;
            }

            let _ = SunrunTasklogManager::create_log(sunrun_userinfo.id, true, "").await;
            if let Some(email) = sunrun_userinfo.email.to_owned() {
                EmailManager::send_result_email(email, true, None);
            }
        })
    }
}