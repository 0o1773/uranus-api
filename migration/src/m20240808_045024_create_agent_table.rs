use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
          .create_table(
              Table::create()
                .table(Agent::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Agent::ID)
                      .binary()
                      .not_null()
                      .primary_key()
                )
                .col(
                    ColumnDef::new(Agent::AgentID)
                      .string()
                      .unique_key()
                      .not_null()
                )
                .col(
                    ColumnDef::new(Agent::AgentName)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(Agent::AgentRole)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(Agent::AgentIcon)
                      .string()
                      .not_null()
                )
                .to_owned(),
          )
          .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
          .drop_table(Table::drop().table(Agent::Table).to_owned())
          .await
    }
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
