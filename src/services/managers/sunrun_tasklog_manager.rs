use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::errors::BusinessResult;
use crate::services::database::database::Database;

pub struct SunrunTasklogManager;

impl SunrunTasklogManager {
    pub async fn create_log(info_id: i64, is_success: bool, description: impl ToString) -> BusinessResult<i64> {
        let model = entity::sunrun_tasklog::ActiveModel {
            task_info_id: Set(info_id),
            create_at: Set(chrono::Local::now().timestamp()),
            is_success: Set(is_success),
            description: Set(description.to_string()),
            ..Default::default()
        }.insert(Database::get_conn()).await?;
        Ok(model.id)
    }

    pub async fn delete_log_by_user_info_id(user_info_id: i64) -> BusinessResult<()> {
        entity::sunrun_tasklog::Entity::delete_many()
            .filter(entity::sunrun_tasklog::Column::TaskInfoId.eq(user_info_id))
            .exec(Database::get_conn())
            .await?;

        Ok(())
    }
}