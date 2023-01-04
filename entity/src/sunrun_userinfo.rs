use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sunrun_userinfo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub length: i64,
    pub max_speed: f32,
    pub min_speed: f32,
    pub school_name: String,
    pub nick_name: String,
    pub user_id: i64,
    pub latitude: String,
    pub longitude: String,
    pub step: i64,
    pub hour: i32,
    pub minute: i32,
    pub email: Option<String>,
    pub imeicode: String,
    pub is_iphone: bool,
    pub is_enable: bool,
    pub create_at: i64,
    pub update_at: i64,
    pub create_by: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}