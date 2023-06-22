use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sunrun_tasklog")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub task_info_id: i64,
    pub create_at: i64,
    pub is_success: bool,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
