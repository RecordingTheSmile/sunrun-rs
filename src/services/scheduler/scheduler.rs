use std::sync::Arc;

use once_cell::sync::OnceCell;
use tokio_scheduler_rs::JobScheduler;

use crate::services::scheduler::job_executor::SqlJobExecutor;
use crate::services::scheduler::job_storage::SqlJobStorage;
use crate::tasks::sunrun_task::SunrunTask;

static SCHEDULER: OnceCell<JobScheduler<chrono_tz::Tz>> = OnceCell::new();

pub struct Scheduler;

impl Scheduler {
    pub async fn init() {
        let job_storage = Arc::new(SqlJobStorage::new(chrono_tz::PRC));
        let job_executor = SqlJobExecutor::new(job_storage.to_owned());
        let job_scheduler = JobScheduler::new(job_storage, job_executor);

        job_scheduler.register_job(Box::new(SunrunTask)).await.unwrap();
        let _ = SCHEDULER.set(job_scheduler);
    }

    pub fn get() -> &'static JobScheduler<'static, chrono_tz::Tz> {
        SCHEDULER.get().unwrap()
    }
}