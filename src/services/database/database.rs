use once_cell::sync::OnceCell;

use crate::errors::BusinessResult;

static DATABASE_CONN: OnceCell<sea_orm::DatabaseConnection> = OnceCell::new();

pub struct Database;

impl Database {
    pub async fn init(dsn: impl ToString) -> BusinessResult<()> {
        let _ = DATABASE_CONN.set(sea_orm::Database::connect(dsn.to_string()).await?);
        Ok(())
    }

    pub fn get_conn() -> &'static sea_orm::DatabaseConnection {
        DATABASE_CONN.get().unwrap()
    }
}