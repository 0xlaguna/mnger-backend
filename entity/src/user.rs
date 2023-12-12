use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use sea_orm::entity::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, FromForm, JsonSchema,
)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}