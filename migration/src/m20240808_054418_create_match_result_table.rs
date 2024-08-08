use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
          .create_table(
              Table::create()
                .table(MatchResult::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(MatchResult::Id)
                      .binary()
                      .not_null()
                      .primary_key()
                )
                .col(
                    ColumnDef::new(MatchResult::MatchID)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::Puuid)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::AccountID)
                      .binary()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::AgentID)
                      .binary()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::MapID)
                      .binary()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::Kill)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::Death)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::Assist)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::HSRate)
                      .float()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::FirstBlood)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::FirstDeath)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::CombatScore)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::Damage)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::PlayedRounds)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::WinRounds)
                      .integer()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::Win)
                      .boolean()
                      .not_null()
                )
                .col(
                    ColumnDef::new(MatchResult::MatchStartTime)
                      .timestamp()
                      .not_null()
                )
                .foreign_key(
                    ForeignKeyCreateStatement::new()
                      .name("fk-match_result_account_id-account_id")
                      .from_tbl(MatchResult::Table)
                      .from_col(MatchResult::AccountID)
                      .to_tbl(Account::Table)
                      .to_col(Account::Id)
                      .on_delete(ForeignKeyAction::Cascade)
                )
                .foreign_key(
                    ForeignKeyCreateStatement::new()
                      .name("fk-match_result_agent_id-agent_id")
                      .from_tbl(MatchResult::Table)
                      .from_col(MatchResult::AgentID)
                      .to_tbl(Agent::Table)
                      .to_col(Agent::ID)
                      .on_delete(ForeignKeyAction::Cascade)
                )
                .foreign_key(
                    ForeignKeyCreateStatement::new()
                      .name("fk-match_result_map_id-map_id")
                      .from_tbl(MatchResult::Table)
                      .from_col(MatchResult::MapID)
                      .to_tbl(Map::Table)
                      .to_col(Map::ID)
                      .on_delete(ForeignKeyAction::Cascade)
                )
                .foreign_key(
                    ForeignKeyCreateStatement::new()
                      .name("fk-match_result_puuid-riot_account_puuid")
                      .from_tbl(MatchResult::Table)
                      .from_col(MatchResult::Puuid)
                      .to_tbl(RiotAccount::Table)
                      .to_col(RiotAccount::Puuid)
                      .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned(),
          )
          .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
          .drop_table(Table::drop().table(MatchResult::Table).to_owned())
          .await
    }
}

#[derive(DeriveIden)]
enum MatchResult {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "match_id")]
    MatchID,
    #[sea_orm(iden = "puuid")]
    Puuid,
    #[sea_orm(iden = "account_id")]
    AccountID,
    #[sea_orm(iden = "agent_id")]
    AgentID,
    #[sea_orm(iden = "map_id")]
    MapID,
    #[sea_orm(iden = "kill")]
    Kill,
    #[sea_orm(iden = "death")]
    Death,
    #[sea_orm(iden = "assist")]
    Assist,
    #[sea_orm(iden = "hs_rate")]
    HSRate,
    #[sea_orm(iden = "first_blood")]
    FirstBlood,
    #[sea_orm(iden = "first_death")]
    FirstDeath,
    #[sea_orm(iden = "combat_score")]
    CombatScore,
    #[sea_orm(iden = "damage")]
    Damage,
    #[sea_orm(iden = "played_rounds")]
    PlayedRounds,
    #[sea_orm(iden = "win_rounds")]
    WinRounds,
    #[sea_orm(iden = "win")]
    Win,
    #[sea_orm(iden = "match_start_time")]
    MatchStartTime,
}

#[derive(DeriveIden)]
enum Agent {
    Table,
    #[sea_orm(iden = "id")]
    ID,
    #[sea_orm(iden = "agent_id")]
    AgentID,
    #[sea_orm(iden = "name")]
    AgentName,
    #[sea_orm(iden = "role")]
    AgentRole,
    #[sea_orm(iden = "icon")]
    AgentIcon,
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
enum Map {
    Table,
    #[sea_orm(iden = "id")]
    ID,
    #[sea_orm(iden = "map_id")]
    MapID,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "image")]
    Image,
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