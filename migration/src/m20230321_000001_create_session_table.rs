use sea_orm_migration::prelude::*;

use crate::m20230311_000001_create_user_table::User;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230321_000001_create_session_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(Table::create()
                .table(Session::Table)
                .col(ColumnDef::new(Session::Id).string().not_null().primary_key())
                .col(ColumnDef::new(Session::UserId).string().not_null())
                .col(ColumnDef::new(Session::ExpiresAt).timestamp().not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-session-user_id")
                        .from(Session::Table, Session::UserId)
                        .to(User::Table, User::Id)
                )
                .to_owned()
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Session {
    Table,
    Id,
    UserId,
    ExpiresAt
}