pub use sea_orm_migration::prelude::*;

mod m20230311_000001_create_user_table;
mod m20230311_000002_create_riot_table;
mod m20230311_000003_create_maps_table;
mod m20230311_000004_create_agents_table;
mod m20230311_000005_create_tiers_table;
mod m20230321_000001_create_session_table;
mod m20230322_000001_alter_agents_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230311_000001_create_user_table::Migration),
            Box::new(m20230311_000002_create_riot_table::Migration),
            Box::new(m20230311_000003_create_maps_table::Migration),
            Box::new(m20230311_000004_create_agents_table::Migration),
            Box::new(m20230311_000005_create_tiers_table::Migration),
            Box::new(m20230321_000001_create_session_table::Migration),
            Box::new(m20230322_000001_alter_agents_table::Migration),
        ]
    }
}
