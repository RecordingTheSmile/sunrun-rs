use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ActiveModelTrait;
use sea_orm_migration::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SunrunUserinfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SunrunUserinfo::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(SunrunUserinfo::CreateAt)
                            .not_null()
                            .big_integer(),
                    )
                    .col(ColumnDef::new(SunrunUserinfo::Email).text().null())
                    .col(ColumnDef::new(SunrunUserinfo::Hour).integer().not_null())
                    .col(ColumnDef::new(SunrunUserinfo::Minute).integer().not_null())
                    .col(ColumnDef::new(SunrunUserinfo::Imeicode).text().not_null())
                    .col(ColumnDef::new(SunrunUserinfo::Latitude).text().not_null())
                    .col(ColumnDef::new(SunrunUserinfo::Longitude).text().not_null())
                    .col(
                        ColumnDef::new(SunrunUserinfo::Length)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SunrunUserinfo::MaxSpeed).float().not_null())
                    .col(ColumnDef::new(SunrunUserinfo::MinSpeed).float().not_null())
                    .col(
                        ColumnDef::new(SunrunUserinfo::UpdateAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SunrunUserinfo::NickName).text().not_null())
                    .col(ColumnDef::new(SunrunUserinfo::SchoolName).text().not_null())
                    .col(
                        ColumnDef::new(SunrunUserinfo::Step)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SunrunUserinfo::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SunrunUserinfo::IsIphone)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SunrunUserinfo::IsEnable)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SunrunUserinfo::CreateBy)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SunrunTasklog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SunrunTasklog::Id)
                            .big_integer()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(SunrunTasklog::TaskInfoId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SunrunTasklog::CreateAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SunrunTasklog::IsSuccess)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SunrunTasklog::Description).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(
                        ColumnDef::new(User::Id)
                            .primary_key()
                            .auto_increment()
                            .big_integer(),
                    )
                    .col(ColumnDef::new(User::Username).text().not_null())
                    .col(ColumnDef::new(User::Password).text().not_null())
                    .col(ColumnDef::new(User::Email).text().not_null())
                    .col(ColumnDef::new(User::CanLogin).boolean().not_null())
                    .col(ColumnDef::new(User::CreateAt).big_integer().not_null())
                    .col(ColumnDef::new(User::IsAdmin).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(JobStorage::Table)
                    .col(ColumnDef::new(JobStorage::Id).uuid().primary_key())
                    .col(ColumnDef::new(JobStorage::TaskName).text().not_null())
                    .col(ColumnDef::new(JobStorage::Cron).text().not_null())
                    .col(
                        ColumnDef::new(JobStorage::LastCheckAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JobStorage::NextExecuteAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(JobStorage::Args).json().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SunrunTask::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SunrunTask::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SunrunTask::TaskInfoId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SunrunTask::CreateAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SunrunTask::TaskId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        entity::user::ActiveModel {
            username: Set("admin123".into()),
            password: Set(bcrypt::hash("admin123", bcrypt::DEFAULT_COST)
                .map_err(|e| DbErr::Custom(format!("bcrypt error: {:#?}", e)))?),
            email: Set("admin@admin.com".into()),
            can_login: Set(true),
            create_at: Set(chrono::Local::now().timestamp()),
            is_admin: Set(true),
            ..Default::default()
        }
        .insert(conn)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SunrunUserinfo::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SunrunTasklog::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(JobStorage::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SunrunTask::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum SunrunUserinfo {
    Table,
    Id,
    Length,
    MaxSpeed,
    MinSpeed,
    SchoolName,
    NickName,
    UserId,
    Latitude,
    Longitude,
    Step,
    Hour,
    Minute,
    Email,
    Imeicode,
    CreateAt,
    UpdateAt,
    IsIphone,
    IsEnable,
    CreateBy,
}

#[derive(Iden)]
enum SunrunTasklog {
    Table,
    Id,
    TaskInfoId,
    CreateAt,
    IsSuccess,
    Description,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Password,
    Email,
    CanLogin,
    CreateAt,
    IsAdmin,
}

#[derive(Iden)]
enum JobStorage {
    Table,
    Id,
    TaskName,
    Cron,
    LastCheckAt,
    NextExecuteAt,
    Args,
}

#[derive(Iden)]
enum SunrunTask {
    Table,
    Id,
    TaskInfoId,
    CreateAt,
    TaskId,
}
