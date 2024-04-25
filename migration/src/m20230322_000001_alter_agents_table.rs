use sea_orm_migration::prelude::*;
use crate::m20230311_000004_create_agents_table::Agent;

pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230322_000001_alter_agents_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(Table::alter()
                .table(Agent::Table)
                .add_column(ColumnDef::new(Agent::Role).string().not_null())
                .to_owned()
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(Table::alter()
                .table(Agent::Table)
                .drop_column(Agent::Role)
                .to_owned()
            )
            .await
    }
}

