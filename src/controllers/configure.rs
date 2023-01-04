use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::HttpResponse;

use crate::common::actix_exts::CommonHttpRequestExts;
use crate::controllers::{admin_api_controller, console_api_controller, console_controller, console_iframes_controller, fallback, public_api_controller, root_controller, task_api_controller, user_api_controller};
use crate::errors::BusinessError;
use crate::services::template::template::ActixHttpResponseExt;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .app_data(actix_web::web::JsonConfig::default().error_handler(fallback::handle_json_error))
        .app_data(actix_web::web::FormConfig::default().error_handler(fallback::handle_form_error))
        .app_data(actix_web::web::QueryConfig::default().error_handler(fallback::handle_query_error))
        .app_data(actix_web::web::PathConfig::default().error_handler(fallback::handle_path_error))
        .app_data(actix_web_validator::JsonConfig::default().error_handler(fallback::handle_validate_error))
        .app_data(actix_web_validator::FormConfig::default().error_handler(fallback::handle_validate_error))
        .app_data(actix_web_validator::QueryConfig::default().error_handler(fallback::handle_validate_error))
        .app_data(actix_web_validator::PathConfig::default().error_handler(fallback::handle_validate_error))
        .service(user_api_controller::post_login)
        .service(user_api_controller::post_register)
        .service(user_api_controller::post_forget_password)
        .service(user_api_controller::put_username)
        .service(user_api_controller::put_password)
        .service(user_api_controller::put_email)
        .service(public_api_controller::get_captcha)
        .service(public_api_controller::post_send_email_code)
        .service(task_api_controller::get_task)
        .service(task_api_controller::post_task)
        .service(task_api_controller::put_task)
        .service(task_api_controller::delete_task)
        .service(task_api_controller::get_task_log)
        .service(task_api_controller::delete_task_log)
        .service(task_api_controller::post_execute_now)
        .service(task_api_controller::get_task_status)
        .service(admin_api_controller::get_user)
        .service(admin_api_controller::put_can_login)
        .service(admin_api_controller::put_is_admin)
        .service(console_controller::get_login)
        .service(console_controller::get_index)
        .service(console_controller::get_register)
        .service(console_controller::get_forget_password)
        .service(console_controller::get_exit_login)
        .service(console_api_controller::get_menu)
        .service(console_iframes_controller::get_index)
        .service(console_iframes_controller::get_sunrun_tasks)
        .service(console_iframes_controller::get_add_sunrun_task)
        .service(console_iframes_controller::get_edit_sunrun_task)
        .service(console_iframes_controller::get_sunrun_task_logs)
        .service(console_iframes_controller::get_users)
        .service(console_iframes_controller::get_edit_username)
        .service(console_iframes_controller::get_edit_password)
        .service(console_iframes_controller::get_edit_email)
        .service(console_iframes_controller::get_task_status)
        .service(root_controller::get_root)
        .service(actix_files::Files::new("/", "./statics").prefer_utf8(true)
            .default_handler(fn_service(|req: ServiceRequest| async {
                let (req, _) = req.into_parts();
                let res = match req.method() {
                    &actix_web::http::Method::GET => {
                        let mut ctx = tera::Context::new();
                        ctx.insert("message", "页面不存在");
                        ctx.insert("code", &404);
                        HttpResponse::NotFound().html("error.html", &ctx)
                    }
                    _ => return Err(BusinessError::new_code("请求方法不正确", 405).set_html(!req.is_ajax()).into())
                };
                Ok(ServiceResponse::new(req, res))
            })))
    ;
}