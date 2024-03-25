pub use sea_orm_migration::prelude::*;

mod user;
mod authorization;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(user::m20220101_000001_create_table::Migration),
            Box::new(authorization::m20240321_001343_rbac_initial::Migration),
        ]
    }
}
