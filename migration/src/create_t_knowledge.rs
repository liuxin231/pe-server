use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TKnowledge::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TKnowledge::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TKnowledge::FuncName).string().not_null())
                    .col(ColumnDef::new(TKnowledge::FuncDesc).string())
                    .col(
                        ColumnDef::new(TKnowledge::IsSensitive)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TKnowledge::CreateTime)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TKnowledge::ModifyTime)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TKnowledge::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TKnowledge {
    Table,
    Id,
    FuncName,
    FuncDesc,
    IsSensitive,
    CreateTime,
    ModifyTime,
}
