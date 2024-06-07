use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum WorkOrder {
    Table,
    Id,
    Title,
    Description,
    Status,
    StartDate,
    EndDate,
    CreatedBy,
    CreatedAt,
    UpdatedAt
}
