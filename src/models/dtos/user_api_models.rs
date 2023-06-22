use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct PostLoginBody {
    #[validate(length(min = 4, max = 20))]
    pub username: String,
    #[validate(length(min = 4, max = 20))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PostRegisterBody {
    #[validate(length(min = 4, max = 20))]
    pub username: String,
    #[validate(length(min = 4, max = 20))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 6))]
    pub code: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PostForgetPasswordBody {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 5))]
    pub code: String,
    #[validate(length(min = 4, max = 20))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PutUsernameBody {
    #[validate(length(min = 1, max = 20))]
    pub username: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PutPasswordBody {
    #[validate(length(min = 1, max = 20))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PutEmailBody {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 6))]
    pub code: String,
}
