use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
pub enum Permission {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
pub enum RolePermission {
    Table,
    RoleId,
    PermissionId,
}

#[derive(DeriveIden)]
pub enum Module {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
pub enum RoleModule {
    Table,
    RoleId,
    ModuleId,
}
