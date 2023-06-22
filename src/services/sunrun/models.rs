use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunrunUserInfo {
    pub length: i64,
    pub max_speed: f64,
    pub min_speed: f64,
    pub school_name: String,
    pub nick_name: String,
    pub user_id: i64,
    pub latitude: String,
    pub longitude: String,
    pub step: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SunrunRunTimes {
    pub morning_run_times: i64,
    pub run_times: i64,
}
