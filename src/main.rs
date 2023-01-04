use actix_session::config::{CookieContentSecurity, PersistentSession, TtlExtensionPolicy};
use actix_session::SessionMiddleware;
use actix_web::{App, HttpServer};
use actix_web::cookie::SameSite;

use migration::MigratorTrait;
use sunrun_rs::controllers::configure::configure;
use sunrun_rs::services::database::database::Database;
use sunrun_rs::services::scheduler::scheduler::Scheduler;

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
    fast_log::init(fast_log::Config::default().level(log::LevelFilter::Warn)).unwrap();

    // 初始化数据库
    Database::init(std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    migration::Migrator::up(Database::get_conn(), None).await.unwrap();

    // 初始化任务
    Scheduler::init().await;
    Scheduler::get().start();

    // 初始化并运行HTTP服务器
    HttpServer::new(move || App::new()
        .wrap(SessionMiddleware::builder(actix_session::storage::CookieSessionStore::default(), actix_web::cookie::Key::from(std::env::var("SESSION_KEY")
            .expect("Cannot read SESSION_KEY").as_bytes()))
            .cookie_name(String::from("SessionID"))
            .session_lifecycle(PersistentSession::default().session_ttl(actix_web::cookie::time::Duration::days(7))
                .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest))
            .cookie_http_only(false)
            .cookie_same_site(SameSite::Lax)
            .cookie_content_security(CookieContentSecurity::Private)
            .build())
        .configure(configure))
        .bind(std::env::var("LISTEN_ADDR").expect("Cannot find LISTEN_ADDR"))
        .unwrap()
        .run()
        .await
        .unwrap()
}