use actix_web::{get, HttpResponse, post};
use captcha::Difficulty;
use rand::Rng;

use crate::common::r::R;
use crate::errors::{BusinessError, BusinessResult};
use crate::models::dtos::public_api_models::PostSendEmailCodeBody;
use crate::services::managers::email_manager::EmailManager;

#[get("/public/api/captcha")]
pub async fn get_captcha(session: actix_session::Session) -> BusinessResult<HttpResponse> {
    let (captcha, pic) = captcha::gen(Difficulty::Easy)
        .as_tuple().ok_or(BusinessError::new_code("生成验证码失败", 500))?;

    session.insert("captcha", captcha.to_lowercase())?;
    Ok(HttpResponse::Ok().content_type(actix_web::http::header::ContentType::png()).body(pic))
}

#[post("/public/api/sendEmailCode")]
pub async fn post_send_email_code(body: actix_web_validator::Json<PostSendEmailCodeBody>, session: actix_session::Session) -> BusinessResult<HttpResponse> {
    let captcha = match session.get::<String>("captcha")? {
        Some(c) => c,
        None => return Err(BusinessError::new_code("图片验证码已过期，请重新获取", 400))
    };

    if captcha != body.captcha {
        return Err(BusinessError::new_code("图片验证码不正确", 400));
    }

    let code = rand::thread_rng().gen_range(10000..99999);
    session.insert("email_code", &code.to_string())?;
    session.insert("email_code_create_at", &chrono::Local::now().timestamp())?;
    EmailManager::send_verify_email(body.email.to_owned(), code.to_string());

    Ok(HttpResponse::Ok().json(R::json_success()))
}