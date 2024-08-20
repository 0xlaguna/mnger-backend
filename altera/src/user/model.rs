use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
    FirstName,
    MiddleName,
    LastName,
    Avatar,
    Password,
    Disabled
}

#[derive(DeriveIden)]
pub enum Session {
    Table,
    Id,
    Token,
    Name,
    UserId
}

#[derive(DeriveIden)]
pub enum Verification {
    Table,
    Id,
    UserId,
    TypeId,
    Token,
    Pending,
    ExpiresAt,
    Enabled
}

#[derive(DeriveIden)]
pub enum VerificationType {
    Table,
    Id,
    Name,
    Description,
    Enabled
}
