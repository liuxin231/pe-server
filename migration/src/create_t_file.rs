use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TFile::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TFile::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(TFile::FileName).string().not_null())
                    .col(ColumnDef::new(TFile::FileMd5).string().not_null())
                    .col(ColumnDef::new(TFile::FileBuf).binary().not_null())
                    .col(ColumnDef::new(TFile::FileReport).binary())
                    .col(ColumnDef::new(TFile::CreateTime).timestamp().not_null())
                    .col(ColumnDef::new(TFile::ModifyTime).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TFile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TFile {
    Table,
    Id,
    FileName,
    FileMd5,
    FileBuf,
    FileReport,
    CreateTime,
    ModifyTime,
}
