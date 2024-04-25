use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230311_000005_create_tiers_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(Table::create()
                .table(Tier::Table)
                .col(ColumnDef::new(Tier::Tier).integer().not_null().primary_key())
                .col(ColumnDef::new(Tier::TierName).string().not_null())
                .col(ColumnDef::new(Tier::Icon).string().not_null())
                .col(ColumnDef::new(Tier::DivisionId).string().not_null())
                .col(ColumnDef::new(Tier::DivisionName).string().not_null())
                .col(ColumnDef::new(Tier::Color).string().not_null())
                .col(ColumnDef::new(Tier::BackgroundColor).string().not_null())
                .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tier::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Tier {
    Table,
    // Tierデータ
    Tier,
    TierName,
    Icon,
    // ランクDivision共通データ
    DivisionId,
    DivisionName,
    Color,
    BackgroundColor,
}