use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PostTaskBody {
    #[validate(custom = "validate_lan_lon")]
    pub latitude: String,
    #[validate(custom = "validate_lan_lon")]
    pub longitude: String,
    #[validate(range(min = 1))]
    pub step: i64,
    #[validate(range(min = 0, max = 23))]
    pub hour: i32,
    #[validate(range(min = 0, max = 59))]
    pub minute: i32,
    pub email: Option<String>,
    #[validate(length(min = 1))]
    pub imeicode: String,
    pub is_iphone: bool,
    pub is_enable: bool,
}

fn validate_lan_lon(lan_lon: &str) -> Result<(), ValidationError> {
    if lan_lon.contains(".") {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid Lan Lon Format"))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutTaskStatusBody {
    pub is_enable: bool,
}
