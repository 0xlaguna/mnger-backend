pub use sea_orm_migration::prelude::*;

mod user;
mod authorization;
mod workorder;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(user::m20220101_000001_create_table::Migration),
            Box::new(authorization::m20240321_001343_rbac_initial::Migration),
            Box::new(workorder::m20240325_143045_workorder::Migration),
            Box::new(workorder::m20240616_164610_workorder_user_fk::Migration),
            Box::new(user::m20240819_145713_add_verification_table::Migration),
            Box::new(user::m20240820_105510_seed_verfication_type_data::Migration),
        ]
    }
}
