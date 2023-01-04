use actix_web::{get, HttpResponse};

use crate::errors::BusinessResult;
use crate::services::template::template::ActixHttpResponseExt;

#[get("/")]
pub async fn get_root() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("index.html", &tera::Context::new()))
}