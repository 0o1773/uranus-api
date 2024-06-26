use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230311_000004_create_agents_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Agent::Table)
                    .col(
                        ColumnDef::new(Agent::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Agent::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Agent::Icon)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Agent::LogImg)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop().table(Agent::Table).to_owned()
            )
            .await
    }
}

#[derive(Iden)]
pub enum Agent {
    Table,
    Id,
    Name,
    Icon,
    LogImg,
    Role,
}