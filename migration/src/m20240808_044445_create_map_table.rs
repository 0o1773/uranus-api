use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
          .create_table(
              Table::create()
                .table(Map::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Map::ID)
                      .binary()
                      .not_null()
                      .primary_key()
                )
                .col(
                    ColumnDef::new(Map::MapID)
                      .string()
                      .unique_key()
                      .not_null()
                )
                .col(
                    ColumnDef::new(Map::Name)
                      .string()
                      .not_null()
                )
                .col(
                    ColumnDef::new(Map::Image)
                      .string()
                      .not_null()
                )
                .to_owned()
          )
          .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
          .drop_table(
              Table::drop()
                .table(Map::Table)
                .if_exists()
                .to_owned()
          )
          .await
    }
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