use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "timezone")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_type = "Text")]
    pub value: String,

    #[sea_orm(column_type = "Text")]
    pub abbr: String,

    #[sea_orm(column_type = "Double")]
    pub offset: f64,

    pub isdst: bool,

    #[sea_orm(column_type = "Text")]
    pub text: String,

    #[sea_orm(column_type = "JsonBinary")]
    pub utc: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
