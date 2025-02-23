pub use sea_orm_migration::prelude::*;

mod m20250222_190000_create_table;
mod m20250222_190001_seed_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250222_190000_create_table::Migration),
            Box::new(m20250222_190001_seed_data::Migration),
        ]
    }
}
