use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
          .alter_table(
              sea_query::Table::alter()
                .table(RiotAccount::Table)
                .add_column_if_not_exists(
                    ColumnDef::new(RiotAccount::UpdatedAt)
                      .timestamp()
                      .not_null()
                )
                .to_owned()
          ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
          .alter_table(
              sea_query::Table::alter()
                .table(RiotAccount::Table)
                .drop_column(RiotAccount::UpdatedAt)
                .to_owned()
          ).await
    }
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