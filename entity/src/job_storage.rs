use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "job_storage")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub task_name: String,
    pub cron: String,
    pub last_check_at: i64,
    pub next_execute_at: i64,
    pub args: Option<serde_json::Value>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}