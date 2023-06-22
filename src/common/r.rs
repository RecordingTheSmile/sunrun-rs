use serde::Serialize;
use serde_json::{json, Value};

pub struct R;

impl R {
    pub fn json<T: Serialize>(message: impl ToString, code: u16, data: T) -> Value {
        json!({
            "code":code,
            "message":message.to_string(),
            "data":data,
            "success": code <= 299 && code >= 200
        })
    }

    pub fn json_success() -> Value {
        Self::json("请求成功", 200, ())
    }

    pub fn json_success_message(message: impl ToString) -> Value {
        Self::json(message, 200, ())
    }

    pub fn json_success_data<T: Serialize>(data: T) -> Value {
        Self::json("请求成功", 200, data)
    }

    pub fn json_fail(message: impl ToString, code: u16) -> Value {
        Self::json(message, code, ())
    }
}
