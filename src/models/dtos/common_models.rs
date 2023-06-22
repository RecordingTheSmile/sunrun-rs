use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct PageInfo {
    #[validate(range(min = 1))]
    pub page: u64,
    #[validate(range(min = 1, max = 50))]
    pub limit: u64,
}
