use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Links::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Links::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Links::Link).string().not_null())
                    .col(ColumnDef::new(Links::Count).integer().default(1).not_null())
                    .col(
                        ColumnDef::new(Links::CreatedAt)
                            .timestamp()
                            .default("CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(Links::UpdatedAt)
                            .timestamp()
                            .default("CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Links::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Links {
    Table,
    Id,
    Link,
    Count,
    CreatedAt,
    UpdatedAt,
}
