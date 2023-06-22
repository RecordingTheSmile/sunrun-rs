use actix_web::middleware::DefaultHeaders;
use actix_web::{App, HttpServer};

use migration::MigratorTrait;
use sunrun_rs::controllers::configure::configure;
use sunrun_rs::services::database::database::Database;
use sunrun_rs::services::managers::jwt_manager::JwtManager;
use sunrun_rs::services::managers::timer_task_manager::TimerTaskManager;
use sunrun_rs::services::scheduler::scheduler::Scheduler;
use sunrun_rs::wraps::auto_renew_jwt::AutoRenewJwt;
use sunrun_rs::wraps::session_wrap::SessionWrap;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main]
async fn main() {
    // 读取配置文件
    #[cfg(not(debug_assertions))]
    {
        dotenv::dotenv().ok().expect("Cannot read .env");
    }
    #[cfg(debug_assertions)]
    {
        dotenv::from_filename(".env.development").expect("Cannot read .env.development");
    }
    // 初始化日志
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Warn),
    )
    .unwrap();

    // 初始化数据库
    Database::init(std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    migration::Migrator::up(Database::get_conn(), None)
        .await
        .unwrap();

    // 初始化任务
    Scheduler::init().await;
    Scheduler::get().start();

    // 初始化Session
    JwtManager::init(
        &std::env::var("SESSION_KEY").expect("无法读取SessionKey，请确认SESSION_KEY是否已正确设置"),
    );

    // 初始化简单定期任务
    TimerTaskManager::init().await;

    // 初始化并运行HTTP服务器
    HttpServer::new(move || {
        App::new()
        .wrap(DefaultHeaders::new().add((actix_web::http::header::ACCESS_CONTROL_EXPOSE_HEADERS,"x-authorization")))
            .wrap(AutoRenewJwt)
            .wrap(SessionWrap)
            .configure(configure)
    })
    .bind(std::env::var("LISTEN_ADDR").expect("Cannot find LISTEN_ADDR"))
    .unwrap()
    .run()
    .await
    .unwrap()
}
