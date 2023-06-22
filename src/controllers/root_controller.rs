use actix_web::{get, Responder};

use crate::errors::{BusinessError};

#[get("/")]
pub async fn get_root() -> impl Responder {
    actix_files::NamedFile::open_async("./statics/index.html")
        .await
        .map_err(|_| BusinessError::new("首页文件不存在，请联系管理员修复"))
}
