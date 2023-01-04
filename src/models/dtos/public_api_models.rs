use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct PostSendEmailCodeBody {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4))]
    pub captcha: String,
}