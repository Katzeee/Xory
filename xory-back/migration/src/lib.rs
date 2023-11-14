pub use sea_orm_migration::prelude::*;

mod m20231107_084318_init_table;
mod db_utils;
static DATA_DIR: &str = "migration/data/";

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20231107_084318_init_table::Migration)]
    }
}
