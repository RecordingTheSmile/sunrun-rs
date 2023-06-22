use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "qr_scan")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub qr_id: String,
    pub qr_uuid: String,
    pub imeicode: Option<String>,
    pub create_at: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
