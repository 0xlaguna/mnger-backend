use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum WorkOrder {
    Table,
    Ulid,
    Title,
    Description,
    Status,
    StartDate,
    EndDate,
    CreatedBy
}
