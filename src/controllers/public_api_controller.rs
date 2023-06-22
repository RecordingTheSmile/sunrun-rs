use actix_web::{get, post, HttpResponse};
use captcha::Difficulty;
use rand::Rng;

use crate::common::r::R;
use crate::errors::{BusinessError, BusinessResult};
use crate::models::datas::session::Session;
use crate::models::dtos::public_api_models::{
    GetWxStatusQuery, GetWxUuidQuery, PostSendEmailCodeBody,
};

use crate::services::managers::email_manager::EmailManager;
use crate::services::managers::jwt_manager::JwtManager;
use crate::services::managers::qr_manager::QrManager;
use crate::services::sunrun::wx_login::WxQrcodeStatus;

#[get("/public/api/captcha")]
pub async fn get_captcha(session: Session) -> BusinessResult<HttpResponse> {
    let (captcha, pic) = captcha::gen(Difficulty::Easy)
        .as_tuple()
        .ok_or(BusinessError::new_code("生成验证码失败", 500))?;

    session.set("captcha", &captcha.to_lowercase()).await?;
    Ok(HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::png())
        .body(pic))
}

#[post("/public/api/sendEmailCode")]
pub async fn post_send_email_code(
    body: actix_web_validator::Json<PostSendEmailCodeBody>,
    session: Session,
) -> BusinessResult<HttpResponse> {
    let captcha = match session.get::<String>("captcha").await? {
        Some(c) => c,
        None => return Err(BusinessError::new_code("图片验证码已过期，请重新获取", 400)),
    };

    if captcha != body.captcha {
        return Err(BusinessError::new_code("图片验证码不正确", 400));
    }

    let code = rand::thread_rng().gen_range(10000..99999);
    session.set("email_code", code.to_string()).await?;
    EmailManager::send_verify_email(body.email.to_owned(), code.to_string());

    session.delete("captcha").await?;
    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[get("/public/api/wxUuid")]
pub async fn get_wx_uuid(
    query: actix_web::web::Query<GetWxUuidQuery>,
) -> BusinessResult<HttpResponse> {
    let value = JwtManager::get_global()
        .get_verified_data::<serde_json::Value>(&query.grant_code)
        .map_err(|_| BusinessError::new_code("二维码请求不合法", 403))?;

    let id = value["id"].as_str();

    if let Some(id) = id {
        let wx_info = QrManager::create_qr(id).await?;

        Ok(HttpResponse::Ok().json(R::json_success_data(wx_info)))
    } else {
        Err(BusinessError::new_code("二维码请求不合法", 403))
    }
}

#[get("/public/api/wxStatus")]
pub async fn get_wx_status(
    query: actix_web::web::Query<GetWxStatusQuery>,
) -> BusinessResult<HttpResponse> {
    let value = JwtManager::get_global()
        .get_verified_data::<serde_json::Value>(&query.grant_code)
        .map_err(|_| BusinessError::new_code("二维码请求不合法", 403))?;

    let id = value["id"].as_str();

    if id.is_none() {
        return Err(BusinessError::new_code("二维码请求不合法", 403));
    }

    let id = id.unwrap();

    let status = QrManager::get_qr_status(&query.uuid, id).await?;

    let (response_code, response_message) = match status {
        WxQrcodeStatus::Success(_) => (200, "二维码扫描完成"),
        WxQrcodeStatus::Expire => (400, "二维码已过期，请重新扫描"),
        WxQrcodeStatus::Error => (406, "二维码错误，请重新扫描"),
        WxQrcodeStatus::Cancel => (402, "二维码扫描被取消"),
        WxQrcodeStatus::Scanned => (201, "您已扫描二维码，请点击确认授权按钮"),
        WxQrcodeStatus::WaitingForScan => (202, "请使用手机微信扫描二维码"),
    };
    Ok(HttpResponse::Ok().json(R::json(response_message, response_code, ())))
}
