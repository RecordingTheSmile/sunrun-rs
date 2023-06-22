use crate::errors::{BusinessError, BusinessResult};
use crate::services::database::database::Database;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, IsolationLevel, PaginatorTrait,
    QueryFilter, QuerySelect, TransactionTrait,
};

#[derive(Debug, Clone)]
pub struct SessionManager(String);

impl SessionManager {
    pub fn get_session_id(&self) -> &str {
        &self.0
    }

    pub async fn get_or_create(origin_id: &str) -> BusinessResult<Self> {
        let txn = Database::get_conn()
            .begin_with_config(Some(IsolationLevel::RepeatableRead), None)
            .await?;

        let session_count = entity::session::Entity::find()
            .filter(entity::session::Column::SessionId.eq(origin_id))
            .count(&txn)
            .await?;

        if session_count == 0 {
            let new_session_id = uuid::Uuid::new_v4().to_string();
            let time_exp = chrono::Local::now().timestamp() + 60 * 60;

            entity::session::ActiveModel {
                session_id: Set(new_session_id.to_owned()),
                data: Set(serde_json::Value::default()),
                expires_at: Set(time_exp),
                ..Default::default()
            }
            .insert(&txn)
            .await?;

            txn.commit().await?;

            Ok(Self(new_session_id))
        } else {
            let time_exp = chrono::Local::now().timestamp() + 60 * 60;

            entity::session::Entity::update_many()
                .filter(entity::session::Column::SessionId.eq(origin_id))
                .col_expr(entity::session::Column::ExpiresAt, Expr::value(time_exp))
                .exec(&txn)
                .await?;

            txn.commit().await?;
            Ok(Self(origin_id.to_owned()))
        }
    }

    pub async fn create() -> BusinessResult<Self> {
        let txn = Database::get_conn()
            .begin_with_config(Some(IsolationLevel::RepeatableRead), None)
            .await?;

        let new_session_id = uuid::Uuid::new_v4().to_string();
        let time_exp = chrono::Local::now().timestamp() + 60 * 60;

        entity::session::ActiveModel {
            session_id: Set(new_session_id.to_owned()),
            data: Set(serde_json::Value::default()),
            expires_at: Set(time_exp),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;

        Ok(Self(new_session_id))
    }

    pub async fn get<T>(&self, key: &str) -> BusinessResult<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let txn = Database::get_conn()
            .begin_with_config(Some(IsolationLevel::RepeatableRead), None)
            .await?;

        #[derive(FromQueryResult)]
        struct QueryResult {
            pub data: serde_json::Value,
        }

        let current_session = entity::session::Entity::find()
            .filter(entity::session::Column::SessionId.eq(&self.0))
            .select_only()
            .column(entity::session::Column::Data)
            .into_model::<QueryResult>()
            .one(&txn)
            .await?;

        if let Some(current_session) = current_session {
            let dt = &current_session.data[key];

            if dt.is_null() {
                return Ok(None);
            }
            let data_result =
                T::deserialize(dt).map_err(|_| BusinessError::new("会话数据类型错误"))?;

            txn.commit().await?;
            Ok(Some(data_result))
        } else {
            txn.commit().await?;
            Err(BusinessError::new("会话已过期，请重新获取"))
        }
    }

    pub async fn set<T>(&self, key: &str, value: T) -> BusinessResult<()>
    where
        T: serde::Serialize,
    {
        let txn = Database::get_conn()
            .begin_with_config(Some(IsolationLevel::RepeatableRead), None)
            .await?;

        #[derive(FromQueryResult)]
        struct QueryResult {
            pub data: serde_json::Value,
        }

        let current_session = entity::session::Entity::find()
            .filter(entity::session::Column::SessionId.eq(&self.0))
            .select_only()
            .column(entity::session::Column::Data)
            .into_model::<QueryResult>()
            .one(&txn)
            .await?;

        if let Some(current_session) = current_session {
            let mut data = current_session.data;
            data[key] = serde_json::value::to_value(value).map_err(|e| {
                log::error!("{:#?}", e);
                BusinessError::new("Session数据存储失败")
            })?;
            let time_exp = chrono::Local::now().timestamp() + 60 * 60;
            entity::session::Entity::update_many()
                .filter(entity::session::Column::SessionId.eq(&self.0))
                .col_expr(entity::session::Column::Data, Expr::value(data))
                .col_expr(entity::session::Column::ExpiresAt, Expr::value(time_exp))
                .exec(&txn)
                .await?;

            txn.commit().await?;

            Ok(())
        } else {
            Err(BusinessError::new("会话已过期，请重新获取"))
        }
    }

    pub async fn clear(&self) -> BusinessResult<()> {
        entity::session::Entity::update_many()
            .filter(entity::session::Column::SessionId.eq(&self.0))
            .col_expr(
                entity::session::Column::Data,
                Expr::value(serde_json::Value::default()),
            )
            .exec(Database::get_conn())
            .await?;

        Ok(())
    }

    pub async fn delete(&self, key: &str) -> BusinessResult<()> {
        let txn = Database::get_conn()
            .begin_with_config(Some(IsolationLevel::RepeatableRead), None)
            .await?;

        #[derive(FromQueryResult)]
        struct QueryResult {
            pub data: serde_json::Value,
        }

        let current_session = entity::session::Entity::find()
            .filter(entity::session::Column::SessionId.eq(&self.0))
            .select_only()
            .column(entity::session::Column::Data)
            .into_model::<QueryResult>()
            .one(&txn)
            .await?;

        if let Some(current_session) = current_session {
            let mut data = current_session.data;
            data[key] = Option::<()>::None.into();
            entity::session::Entity::update_many()
                .filter(entity::session::Column::SessionId.eq(&self.0))
                .col_expr(entity::session::Column::Data, Expr::value(data))
                .exec(&txn)
                .await?;
            txn.commit().await?;
            Ok(())
        } else {
            txn.commit().await?;
            Err(BusinessError::new("会话已过期，请重新获取"))
        }
    }
}
