use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::Responder;

use crate::controllers::{
    admin_api_controller, fallback, public_api_controller, task_api_controller, user_api_controller,
};
use crate::errors::BusinessError;

use super::root_controller;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.app_data(actix_web::web::JsonConfig::default().error_handler(fallback::handle_json_error))
        .app_data(actix_web::web::FormConfig::default().error_handler(fallback::handle_form_error))
        .app_data(
            actix_web::web::QueryConfig::default().error_handler(fallback::handle_query_error),
        )
        .app_data(actix_web::web::PathConfig::default().error_handler(fallback::handle_path_error))
        .app_data(
            actix_web_validator::JsonConfig::default()
                .error_handler(fallback::handle_validate_error),
        )
        .app_data(
            actix_web_validator::FormConfig::default()
                .error_handler(fallback::handle_validate_error),
        )
        .app_data(
            actix_web_validator::QueryConfig::default()
                .error_handler(fallback::handle_validate_error),
        )
        .app_data(
            actix_web_validator::PathConfig::default()
                .error_handler(fallback::handle_validate_error),
        )
        .service(user_api_controller::post_login)
        .service(user_api_controller::post_register)
        .service(user_api_controller::post_forget_password)
        .service(user_api_controller::put_username)
        .service(user_api_controller::put_password)
        .service(user_api_controller::put_email)
        .service(public_api_controller::get_captcha)
        .service(public_api_controller::post_send_email_code)
        .service(public_api_controller::get_wx_uuid)
        .service(public_api_controller::get_wx_status)
        .service(task_api_controller::get_task)
        .service(task_api_controller::get_task_detail)
        .service(task_api_controller::post_task)
        .service(task_api_controller::put_task)
        .service(task_api_controller::put_task_status)
        .service(task_api_controller::delete_task)
        .service(task_api_controller::get_task_log)
        .service(task_api_controller::delete_task_log)
        .service(task_api_controller::post_execute_now)
        .service(task_api_controller::get_task_status)
        .service(task_api_controller::get_imeicode)
        .service(task_api_controller::create_wx_qr_token)
        .service(admin_api_controller::get_user)
        .service(admin_api_controller::put_can_login)
        .service(admin_api_controller::put_is_admin)
        .service(root_controller::get_root)
        .service(
            actix_files::Files::new("/", "./statics")
                .prefer_utf8(true)
                .default_handler(fn_service(|req: ServiceRequest| async {
                    let (req, _) = req.into_parts();
                    let res = match req.method() {
                        &actix_web::http::Method::GET => {
                            actix_files::NamedFile::open_async("./statics/index.html")
                                .await?
                                .respond_to(&req)
                        }
                        _ => return Err(BusinessError::new_code("请求方法不正确", 405).into()),
                    };
                    Ok(ServiceResponse::new(req, res))
                })),
        );
}
