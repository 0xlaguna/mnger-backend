#[macro_use]
extern crate schemars;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

pub use iso8601_timestamp::Timestamp;
pub use sea_orm;

pub mod r#impl;

pub mod util;

pub mod models;

pub mod auth;

pub use util::{
    result::{Error, Result},
    variables,
};
