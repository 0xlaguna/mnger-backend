use entity::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct UsersDto {
    pub page: u64,
    pub users_per_page: u64,
    pub num_pages: u64,
    pub users: Vec<user::Model>,
}