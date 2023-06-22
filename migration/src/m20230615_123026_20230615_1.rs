use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::SessionId).text().not_null())
                    .col(ColumnDef::new(Session::Data).json().not_null())
                    .col(ColumnDef::new(Session::ExpiresAt).big_integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(QrScan::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QrScan::Id)
                            .big_integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(QrScan::QrId).text().not_null())
                    .col(ColumnDef::new(QrScan::QrUuid).text().not_null())
                    .col(ColumnDef::new(QrScan::Imeicode).text().null())
                    .col(ColumnDef::new(QrScan::CreateAt).big_integer().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(QrScan::Table).to_owned())
            .await?;
        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Session {
    Table,
    Id,
    SessionId,
    Data,
    ExpiresAt,
}

#[derive(Iden)]
enum QrScan {
    Table,
    Id,
    QrUuid,
    QrId,
    Imeicode,
    CreateAt,
}
