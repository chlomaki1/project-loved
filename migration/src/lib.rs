pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250227_185441_create_initial_schema::Migration)]
    }
}
mod m20250227_185441_create_initial_schema;
