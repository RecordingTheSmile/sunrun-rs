use actix_web::{get, HttpResponse};

use crate::errors::business_error::ResultExts;
use crate::errors::BusinessResult;
use crate::services::template::template::ActixHttpResponseExt;
use crate::wraps::login_wrap::LoginWrap;

#[get("/console/login")]
pub async fn get_login(session: actix_session::Session) -> BusinessResult<HttpResponse> {
    if let Some(_) = session.get::<i64>("user_id").set_html(true)? {
        return Ok(HttpResponse::Found().insert_header((actix_web::http::header::LOCATION, "/console/index")).finish());
    }
    Ok(HttpResponse::Ok().html("console/login.html", &tera::Context::new()))
}

#[get("/console/index")]
pub async fn get_index() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/index.html", &tera::Context::new()))
}

#[get("/console/exitLogin", wrap = "LoginWrap")]
pub async fn get_exit_login(session: actix_session::Session) -> BusinessResult<HttpResponse> {
    session.clear();
    Ok(HttpResponse::Found().insert_header((actix_web::http::header::LOCATION, "/")).finish())
}

#[get("/console/register")]
pub async fn get_register() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/register.html", &tera::Context::new()))
}

#[get("/console/forgetPassword")]
pub async fn get_forget_password() -> BusinessResult<HttpResponse> {
    Ok(HttpResponse::Ok().html("console/forget_password.html", &tera::Context::new()))
}