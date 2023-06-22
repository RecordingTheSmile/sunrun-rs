use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct PostSendEmailCodeBody {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4))]
    pub captcha: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWxUuidQuery {
    pub grant_code: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWxStatusQuery {
    pub uuid: String,
    pub grant_code: String,
}
