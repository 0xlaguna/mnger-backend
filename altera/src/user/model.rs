use sea_orm_migration::prelude::*;

#[allow(dead_code)]
#[derive(DeriveIden)]
pub enum Company {
    Table,
    Id,
    Name,
    Address,
    PhoneNumber,
    Email,
    Logo,
    Website,
    CreatedAt,
    UpdatedAt
}

#[allow(dead_code)]
#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
    FirstName,
    MiddleName,
    LastName,
    DoB,
    Avatar,
    Timezone,
    Password,
    Enabled,
    CompanyId,
    CreatedAt,
    UpdatedAt
}

#[allow(dead_code)]
#[derive(DeriveIden)]
pub enum Session {
    Table,
    Id,
    Token,
    Name,
    UserId
}

#[allow(dead_code)]
#[derive(DeriveIden)]
pub enum Team {
    Table,
    Id,
    Name,
    Description,
    CreatedBy,
    CreatedAt,
    UpdatedAt
}

#[allow(dead_code)]
#[derive(DeriveIden)]
pub enum TeamParticipant {
    Table,
    Id,
    TeamId,
    UserId,
    JoinedAt
}

#[allow(dead_code)]
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
