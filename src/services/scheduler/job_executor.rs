use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio_scheduler_rs::{JobExecutor, JobStorage};

pub struct SqlJobExecutor<Tz>
    where Tz: chrono::TimeZone + Send + Sync,
          Tz::Offset: Send + Sync
{
    jobs: Arc<dyn JobStorage<Tz>>,
    tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
    shutdown_channel: tokio::sync::broadcast::Sender<()>,
    should_next: Arc<std::sync::RwLock<bool>>,
}

impl<Tz> SqlJobExecutor<Tz>
    where Tz: chrono::TimeZone + Send + Sync,
          Tz::Offset: Send + Sync
{
    pub fn new(jobs: Arc<dyn JobStorage<Tz>>) -> Self {
        let shutdown_chan = tokio::sync::broadcast::channel(1);
        Self {
            jobs,
            tasks: Arc::new(RwLock::new(vec![])),
            shutdown_channel: shutdown_chan.0,
            should_next: Arc::new(std::sync::RwLock::new(true)),
        }
    }
}

impl<Tz> JobExecutor for SqlJobExecutor<Tz>
    where Tz: chrono::TimeZone + Send + Sync + 'static,
          Tz::Offset: Send + Sync
{
    fn start(&self) -> JoinHandle<()> {
        let storage = self.jobs.to_owned();
        let shutdown_sender = self.shutdown_channel.to_owned();
        let should_next = self.should_next.to_owned();
        let tasks = self.tasks.to_owned();
        tokio::spawn(async move {
            loop {
                let should_next = match should_next.read() {
                    Ok(s) => *s,
                    Err(_) => false
                };

                if !should_next {
                    let _ = shutdown_sender.send(());
                    break;
                }
                let should_exec = match storage.get_all_should_execute_jobs().await {
                    Ok(t) => t,
                    Err(_) => continue
                };
                for job in should_exec {
                    let handle = tokio::spawn(async move {
                        job.await;
                        ()
                    });
                    let mut task_vec = tasks.write().await;
                    task_vec.push(handle);
                }

                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
            }
        })
    }

    fn stop(&self) -> Pin<Box<dyn Future<Output=()>>> {
        let mut shutdown_recv = self.shutdown_channel.subscribe();
        let tasks = self.tasks.to_owned();
        *self.should_next.write().unwrap() = false;
        Box::pin(async move {
            let _ = shutdown_recv.recv().await;
            let tasks = tasks.read().await;
            for i in tasks.iter() {
                loop {
                    if i.is_finished() {
                        break;
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        })
    }
}