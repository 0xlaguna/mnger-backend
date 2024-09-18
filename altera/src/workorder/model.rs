use sea_orm_migration::prelude::*;

#[allow(dead_code)]
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
    UpdatedAt,
}

#[allow(dead_code)]
#[derive(DeriveIden)]
pub enum WorkOrderAssignment {
    Table,
    WorkOrderId,
    UserId,
    TeamId,
    AssignedAt,
}
