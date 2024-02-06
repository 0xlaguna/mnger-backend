#[macro_use]
extern crate schemars;

#[macro_use]
extern crate rocket;

pub use authifier;
pub use iso8601_timestamp::Timestamp;
pub use sea_orm;

pub mod r#impl;

pub mod util;

pub mod models;

pub use util::{
    result::{Error, Result},
    variables,
};
