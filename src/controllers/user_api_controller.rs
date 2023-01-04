use actix_web::{HttpResponse, post, put};
use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect};

use crate::common::r::R;
use crate::errors::{BusinessError, BusinessResult};
use crate::models::datas::session_user_id::SessionUserId;
use crate::models::dtos::user_api_models::{PostForgetPasswordBody, PostLoginBody, PostRegisterBody, PutEmailBody, PutPasswordBody, PutUsernameBody};
use crate::services::database::database::Database;
use crate::services::managers::user_manager::UserManager;
use crate::wraps::login_wrap::LoginWrap;

#[post("/user/api/login")]
pub async fn post_login(body: actix_web_validator::Json<PostLoginBody>, session: actix_session::Session) -> BusinessResult<HttpResponse> {
    let user_id = UserManager::ensure_can_login(&body.username, &body.password).await?;

    session.insert("user_id", user_id)?;
    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[post("/user/api/register")]
pub async fn post_register(body: actix_web_validator::Json<PostRegisterBody>, session: actix_session::Session) -> BusinessResult<HttpResponse> {
    let email_code = match session.get::<String>("email_code")? {
        Some(e) => e,
        None => return Err(BusinessError::new_code("邮箱验证码已过期，请重新获取", 400))
    };

    if email_code != body.code {
        return Err(BusinessError::new_code("邮箱验证码不正确，请重新输入", 400));
    }

    UserManager::create_user(&body.username, &body.password, &body.email).await?;

    session.remove("email_code");
    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[post("/user/api/forgetPassword")]
pub async fn post_forget_password(body: actix_web_validator::Json<PostForgetPasswordBody>, session: actix_session::Session) -> BusinessResult<HttpResponse> {
    let email_code = match session.get::<String>("email_code")? {
        Some(e) => e,
        None => return Err(BusinessError::new_code("邮箱验证码已过期，请重新获取", 400))
    };

    if email_code != body.code {
        return Err(BusinessError::new_code("邮箱验证码不正确，请重新输入", 400));
    }

    #[derive(FromQueryResult)]
    struct QueryResult {
        pub id: i64,
    }

    let user_query = match entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(body.email.to_owned()))
        .select_only()
        .column(entity::user::Column::Id)
        .into_model::<QueryResult>()
        .one(Database::get_conn())
        .await? {
        Some(u) => u,
        None => return Err(BusinessError::new_code("用户不存在", 404))
    };

    UserManager::update_password(user_query.id, &body.password).await?;
    session.remove("email_code");
    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[put("/user/api/username", wrap = "LoginWrap")]
pub async fn put_username(body: actix_web_validator::Json<PutUsernameBody>, user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    UserManager::update_username(*user_id, &body.username).await?;
    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[put("/user/api/password", wrap = "LoginWrap")]
pub async fn put_password(body: actix_web_validator::Json<PutPasswordBody>, user_id: SessionUserId) -> BusinessResult<HttpResponse> {
    UserManager::update_password(*user_id, &body.password).await?;
    Ok(HttpResponse::Ok().json(R::json_success()))
}

#[put("/user/api/email", wrap = "LoginWrap")]
pub async fn put_email(body: actix_web_validator::Json<PutEmailBody>, user_id: SessionUserId, session: actix_session::Session) -> BusinessResult<HttpResponse> {
    let email_code = match session.get::<String>("email_code")? {
        Some(e) => e,
        None => return Err(BusinessError::new_code("邮箱验证码已过期，请重新获取", 400))
    };

    if email_code != body.code {
        return Err(BusinessError::new_code("邮箱验证码不正确，请重新输入", 400));
    }

    UserManager::update_email(*user_id, &body.email).await?;
    Ok(HttpResponse::Ok().json(R::json_success()))
}