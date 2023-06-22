use crate::services::database::database::Database;
use once_cell::sync::OnceCell;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct TimerTaskManager(Mutex<Vec<JoinHandle<()>>>);

static TIMER_TASK_MANAGER_GLOBAL: OnceCell<TimerTaskManager> = OnceCell::new();

impl TimerTaskManager {
    pub async fn init() {
        let tasks = Mutex::new(Vec::new());

        let scan_expire_session_handle = tokio::spawn(async move {
            scan_expire_session().await;
        });

        let scan_expire_qr_handle = tokio::spawn(async move {
            scan_expire_qr().await;
        });
        {
            let mut tasks_locked = tasks.lock().await;
            tasks_locked.push(scan_expire_session_handle);
            tasks_locked.push(scan_expire_qr_handle);
        }

        let _ = TIMER_TASK_MANAGER_GLOBAL.set(Self(tasks));
    }

    pub async fn wait_for_stop(self) {
        let mut tasks = self.0.lock().await;
        while let Some(task) = tasks.pop() {
            task.abort();
            let _ = task.await;
        }
    }
}

async fn scan_expire_session() {
    loop {
        let time_now = chrono::Local::now().timestamp();
        if let Err(e) = entity::session::Entity::delete_many()
            .filter(entity::session::Column::ExpiresAt.lte(time_now))
            .exec(Database::get_conn())
            .await
        {
            log::error!("{:#?}", e);
        }
        tokio::time::sleep(Duration::from_secs(5 * 60)).await;
    }
}

async fn scan_expire_qr() {
    loop {
        let time_exp = chrono::Local::now().timestamp() - 15 * 60; // 15分钟之前创建的全部过期
        if let Err(e) = entity::qr_scan::Entity::delete_many()
            .filter(entity::qr_scan::Column::CreateAt.lte(time_exp))
            .exec(Database::get_conn())
            .await
        {
            log::error!("{:#?}", e);
        }
        tokio::time::sleep(Duration::from_secs(5 * 60)).await;
    }
}
