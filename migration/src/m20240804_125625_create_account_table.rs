use sea_orm_migration::prelude::*;
use sea_orm_migration::seaql_migrations::Column;
use crate::async_trait::async_trait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
          .create_table(
              Table::create()
                .table(DiscordAccount::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(DiscordAccount::Id)
                      .binary()
                      .not_null()
                      .primary_key()
                )
                .col(
                    ColumnDef::new(DiscordAccount::DiscordId)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(DiscordAccount::Username)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(DiscordAccount::AccessToken)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(DiscordAccount::RefreshToken)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(DiscordAccount::ExpiresAt)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(DiscordAccount::Avatar)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(DiscordAccount::CreatedAt)
                      .timestamp()
                      .not_null()
                )
                .col(
                    ColumnDef::new(DiscordAccount::UpdatedAt)
                      .timestamp()
                      .not_null()
                )
                .to_owned()
          )
          .await
          .unwrap();

        manager
          .create_table(
              Table::create()
                .table(RiotAccount::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(RiotAccount::Id)
                      .binary_len(16)
                      .not_null()
                      .primary_key()
                )
                .col(
                    ColumnDef::new(RiotAccount::Puuid)
                      .string_len(255)
                      .not_null()
                )
                .col(
                    ColumnDef::new(RiotAccount::GameName)
                      .string_len(255)
                      .not_null()
                )
                .col(
                    ColumnDef::new(RiotAccount::TagLine)
                      .string_len(255)
                      .not_null()
                )
                .col(
                    ColumnDef::new(RiotAccount::PlayerCard)
                      .string_len(255)
                      .not_null()
                )
                .col(
                    ColumnDef::new(RiotAccount::CreatedAt)
                      .timestamp()
                      .not_null()
                )
                .to_owned()
          )
          .await.unwrap();

        manager
          .create_table(
              Table::create()
                .table(Account::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Account::Id)
                      .binary()
                      .not_null()
                      .primary_key()
                )
                .col(
                    ColumnDef::new(Account::DiscordAccountId)
                      .binary()
                      .not_null()
                )
                .col(
                    ColumnDef::new(Account::RiotAccountId)
                      .binary()
                      .not_null()
                )
                .foreign_key(
                    ForeignKeyCreateStatement::new()
                      .name("fk-account_discord_account_id-discord_account_id")
                      .from_tbl(Account::Table)
                      .from_col(Account::DiscordAccountId)
                      .to_tbl(DiscordAccount::Table)
                      .to_col(DiscordAccount::Id)
                      .on_delete(ForeignKeyAction::Cascade)
                )
                .foreign_key(
                    ForeignKeyCreateStatement::new()
                      .name("fk-account_riot_account_id-riot_account_id")
                      .from_tbl(Account::Table)
                      .from_col(Account::RiotAccountId)
                      .to_tbl(RiotAccount::Table)
                      .to_col(RiotAccount::Id)
                      .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned(),
          )
          .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
          .drop_table(
              Table::drop()
                .table(DiscordAccount::Table)
                .if_exists()
                .to_owned()
          ).await
          .unwrap();
        manager
          .drop_table(
              Table::drop()
                .table(RiotAccount::Table)
                .if_exists()
                .to_owned()
          ).await
          .unwrap();

        manager
          .drop_table(
              Table::drop()
                .table(Account::Table)
                .if_exists()
                .to_owned()
          ).await
    }
}

#[derive(DeriveIden)]
pub enum Account {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "riot_account_id")]
    RiotAccountId,
    #[sea_orm(iden = "discord_account_id")]
    DiscordAccountId,
}

#[derive(DeriveIden)]
pub enum DiscordAccount {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "discord_id")]
    DiscordId,
    #[sea_orm(iden = "username")]
    Username,
    #[sea_orm(iden = "access_token")]
    AccessToken,
    #[sea_orm(iden = "refresh_token")]
    RefreshToken,
    #[sea_orm(iden = "expires_at")]
    ExpiresAt,
    #[sea_orm(iden = "avatar_url")]
    Avatar,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}

#[derive(DeriveIden)]
enum RiotAccount {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "puuid")]
    Puuid,
    #[sea_orm(iden = "game_name")]
    GameName,
    #[sea_orm(iden = "tag_line")]
    TagLine,
    #[sea_orm(iden = "player_card")]
    PlayerCard,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}