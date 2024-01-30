#[macro_use]
extern crate schemars;

pub use authifier;
pub use iso8601_timestamp::Timestamp;

pub mod r#impl;

#[cfg(feature = "rocket_impl")]
pub mod web;
#[cfg(feature = "rocket_impl")]
pub use web::{Db, EmptyResponse};

pub mod util;

pub use util::{
    result::{Error, Result},
    variables,
};
