use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sunrun_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub task_info_id: i64,
    pub create_at: i64,
    pub task_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}