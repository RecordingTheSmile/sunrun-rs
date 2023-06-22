use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter, QuerySelect, Set,
};

use crate::errors::{BusinessError, BusinessResult};
use crate::services::database::database::Database;

pub struct UserManager;

pub struct UserLoginStatus {
    pub username: String,
    pub email: String,
    pub id: i64,
    pub is_admin: bool,
}

impl UserManager {
    pub async fn create_user<ToStr>(
        username: ToStr,
        password: ToStr,
        email: ToStr,
    ) -> BusinessResult<i64>
    where
        ToStr: ToString,
    {
        let user_count = entity::user::Entity::find()
            .filter(entity::user::Column::Username.eq(username.to_string()))
            .count(Database::get_conn())
            .await?;

        if user_count != 0 {
            return Err(BusinessError::new_code("用户名已被使用", 400));
        }

        let user_count = entity::user::Entity::find()
            .filter(entity::user::Column::Email.eq(email.to_string()))
            .count(Database::get_conn())
            .await?;

        if user_count != 0 {
            return Err(BusinessError::new_code("邮箱已被使用", 400));
        }

        let model = entity::user::ActiveModel {
            username: Set(username.to_string()),
            password: Set(bcrypt::hash(password.to_string(), bcrypt::DEFAULT_COST)?),
            email: Set(email.to_string()),
            can_login: Set(true),
            create_at: Set(chrono::Local::now().timestamp()),
            is_admin: Set(false),
            ..Default::default()
        }
        .insert(Database::get_conn())
        .await?;

        Ok(model.id)
    }

    pub async fn update_is_admin(id: i64, is_admin: bool) -> BusinessResult<()> {
        entity::user::ActiveModel {
            id: Set(id),
            is_admin: Set(is_admin),
            ..Default::default()
        }
        .update(Database::get_conn())
        .await?;

        Ok(())
    }

    pub async fn ensure_can_login<ToStr>(
        username: ToStr,
        password: ToStr,
    ) -> BusinessResult<UserLoginStatus>
    where
        ToStr: ToString,
    {
        #[derive(FromQueryResult)]
        struct QueryResult {
            pub id: i64,
            pub password: String,
            pub can_login: bool,
            pub username: String,
            pub email: String,
            pub is_admin: bool,
        }

        let user_result = match entity::user::Entity::find()
            .filter(entity::user::Column::Username.eq(username.to_string()))
            .select_only()
            .column(entity::user::Column::Password)
            .column(entity::user::Column::CanLogin)
            .column(entity::user::Column::Id)
            .column(entity::user::Column::Username)
            .column(entity::user::Column::Email)
            .column(entity::user::Column::IsAdmin)
            .into_model::<QueryResult>()
            .one(Database::get_conn())
            .await?
        {
            Some(u) => u,
            None => return Err(BusinessError::new_code("用户名或密码错误", 400)),
        };

        if !bcrypt::verify(password.to_string(), &user_result.password)? {
            return Err(BusinessError::new_code("用户名或密码错误", 400));
        }

        if !user_result.can_login {
            return Err(BusinessError::new_code("您的账户已被封禁", 400));
        }

        Ok(UserLoginStatus {
            id: user_result.id,
            username: user_result.username,
            email: user_result.email,
            is_admin: user_result.is_admin,
        })
    }

    pub async fn update_username(id: i64, username: impl ToString) -> BusinessResult<()> {
        let user_count = entity::user::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::user::Column::Id.ne(id))
                    .add(entity::user::Column::Username.eq(username.to_string())),
            )
            .count(Database::get_conn())
            .await?;

        if user_count != 0 {
            return Err(BusinessError::new_code("用户名已被使用", 400));
        }

        entity::user::ActiveModel {
            id: Set(id),
            username: Set(username.to_string()),
            ..Default::default()
        }
        .update(Database::get_conn())
        .await?;

        Ok(())
    }

    pub async fn update_password(id: i64, password: impl ToString) -> BusinessResult<()> {
        entity::user::ActiveModel {
            id: Set(id),
            password: Set(bcrypt::hash(password.to_string(), bcrypt::DEFAULT_COST)?),
            ..Default::default()
        }
        .update(Database::get_conn())
        .await?;
        Ok(())
    }

    pub async fn update_email(id: i64, email: impl ToString) -> BusinessResult<()> {
        let user_count = entity::user::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::user::Column::Id.ne(id))
                    .add(entity::user::Column::Email.eq(email.to_string())),
            )
            .count(Database::get_conn())
            .await?;

        if user_count != 0 {
            return Err(BusinessError::new_code("邮箱已被使用", 400));
        }

        entity::user::ActiveModel {
            id: Set(id),
            email: Set(email.to_string()),
            ..Default::default()
        }
        .update(Database::get_conn())
        .await?;

        Ok(())
    }

    pub async fn update_can_login(id: i64, can_login: bool) -> BusinessResult<()> {
        entity::user::ActiveModel {
            id: Set(id),
            can_login: Set(can_login),
            ..Default::default()
        }
        .update(Database::get_conn())
        .await?;

        Ok(())
    }
}
