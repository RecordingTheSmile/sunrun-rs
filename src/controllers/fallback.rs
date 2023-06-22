use actix_web::error::{JsonPayloadError, PathError, QueryPayloadError, UrlencodedError};
use actix_web::HttpRequest;

use crate::errors::business_error::BusinessError;

pub fn handle_json_error(_e: JsonPayloadError, _: &HttpRequest) -> actix_web::Error {
    #[cfg(debug_assertions)]
    {
        log::error!("{:#?}", _e);
    }
    BusinessError::new_code("请求参数不正确", 400).into()
}

pub fn handle_form_error(_e: UrlencodedError, _: &HttpRequest) -> actix_web::Error {
    #[cfg(debug_assertions)]
    {
        log::error!("{:#?}", _e);
    }
    BusinessError::new_code("请求参数不正确", 400).into()
}

pub fn handle_query_error(_e: QueryPayloadError, _: &HttpRequest) -> actix_web::Error {
    #[cfg(debug_assertions)]
    {
        log::error!("{:#?}", _e);
    }
    BusinessError::new_code("请求参数不正确", 400).into()
}

pub fn handle_path_error(_e: PathError, _: &HttpRequest) -> actix_web::Error {
    #[cfg(debug_assertions)]
    {
        log::error!("{:#?}", _e);
    }
    BusinessError::new_code("请求参数不正确", 400).into()
}

pub fn handle_validate_error(
    _: actix_web_validator::error::Error,
    _: &HttpRequest,
) -> actix_web::Error {
    BusinessError::new_code("请求参数不正确", 400).into()
}
