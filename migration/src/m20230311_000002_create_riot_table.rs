use sea_orm_migration::prelude::*;

use crate::m20230311_000001_create_user_table::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230311_000002_create_riot_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RiotAccount::Table)
                    .col(
                        ColumnDef::new(RiotAccount::Puuid)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RiotAccount::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RiotAccount::Tag)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RiotAccount::UserId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-riot_account-user_id")
                            .from(RiotAccount::Table, RiotAccount::UserId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop().table(RiotAccount::Table).to_owned()
            )
            .await
    }
}

#[derive(Iden)]
pub enum RiotAccount {
    Table,
    Puuid,
    Name,
    Tag,
    UserId,
}