pub use sea_orm_migration::prelude::*;

mod create_t_file;
mod create_t_knowledge;
mod create_t_test;
mod seed_t_knowledge;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_t_test::Migration),
            Box::new(create_t_knowledge::Migration),
            Box::new(create_t_file::Migration),
            Box::new(seed_t_knowledge::Migration),
        ]
    }
}
