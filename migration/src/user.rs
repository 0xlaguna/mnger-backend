use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    FirstName,
    LastName,
}

// -- Users Table with Company Reference
// CREATE TABLE users (
//     user_id SERIAL PRIMARY KEY,
//     username VARCHAR(50) UNIQUE NOT NULL,
//     email VARCHAR(100) UNIQUE NOT NULL,
//     password_hash VARCHAR(100) NOT NULL,
//     first_name VARCHAR(50),
//     last_name VARCHAR(50),
//     date_of_birth DATE,
//     company_id INT NOT NULL,
//     created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
//     updated_at TIMESTAMPTZ,
//     FOREIGN KEY (company_id) REFERENCES companies(company_id)
//     -- Add more fields as needed for your application's requirements
// );
