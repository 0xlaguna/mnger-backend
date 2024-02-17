use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
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
