use actix_web::error::{JsonPayloadError, PathError, QueryPayloadError, UrlencodedError};
use actix_web::HttpRequest;

use crate::common::actix_exts::CommonHttpRequestExts;
use crate::errors::business_error::BusinessError;

pub fn handle_json_error(_: JsonPayloadError, req: &HttpRequest) -> actix_web::Error {
    let is_html = !req.is_ajax();

    BusinessError::new_code("请求参数不正确", 400).set_html(is_html).into()
}

pub fn handle_form_error(_: UrlencodedError, req: &HttpRequest) -> actix_web::Error {
    let is_html = !req.is_ajax();

    BusinessError::new_code("请求参数不正确", 400).set_html(is_html).into()
}

pub fn handle_query_error(_: QueryPayloadError, req: &HttpRequest) -> actix_web::Error {
    let is_html = !req.is_ajax();

    BusinessError::new_code("请求参数不正确", 400).set_html(is_html).into()
}

pub fn handle_path_error(_: PathError, req: &HttpRequest) -> actix_web::Error {
    let is_html = !req.is_ajax();

    BusinessError::new_code("请求参数不正确", 400).set_html(is_html).into()
}

pub fn handle_validate_error(_: actix_web_validator::error::Error, req: &HttpRequest) -> actix_web::Error {
    let is_html = !req.is_ajax();
    BusinessError::new_code("请求参数不正确", 400).set_html(is_html).into()
}