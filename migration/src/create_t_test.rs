use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TTest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TTest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TTest::Title).string().not_null())
                    .col(ColumnDef::new(TTest::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TTest::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TTest {
    Table,
    Id,
    Title,
    Text,
}
