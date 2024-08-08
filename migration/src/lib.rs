pub use sea_orm_migration::prelude::*;

mod m20240804_125625_create_account_table;
mod m20240806_004350_alter_riot_account_table;
mod m20240808_044445_create_map_table;
mod m20240808_045024_create_agent_table;
mod m20240808_054002_alter_riot_account_table;
mod m20240808_054418_create_match_result_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240804_125625_create_account_table::Migration),
            Box::new(m20240806_004350_alter_riot_account_table::Migration),
            Box::new(m20240808_044445_create_map_table::Migration),
            Box::new(m20240808_045024_create_agent_table::Migration),
            Box::new(m20240808_054002_alter_riot_account_table::Migration),
            Box::new(m20240808_054418_create_match_result_table::Migration),
        ]
    }
}
