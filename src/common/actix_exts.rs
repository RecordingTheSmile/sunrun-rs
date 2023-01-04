use actix_web::dev::ServiceRequest;
use actix_web::HttpRequest;

pub trait CommonHttpRequestExts {
    fn is_ajax(&self) -> bool;
}

impl CommonHttpRequestExts for HttpRequest {
    fn is_ajax(&self) -> bool {
        let headers = self.headers().get("x-requested-with")
            .and_then(|v| Some(v.to_str().unwrap_or("")))
            .unwrap_or("");

        if headers == "XMLHttpRequest" {
            return true;
        }

        false
    }
}

impl CommonHttpRequestExts for ServiceRequest {
    fn is_ajax(&self) -> bool {
        let headers = self.headers().get("x-requested-with")
            .and_then(|v| Some(v.to_str().unwrap_or("")))
            .unwrap_or("");
        if headers == "XMLHttpRequest" {
            return true;
        }

        false
    }
}