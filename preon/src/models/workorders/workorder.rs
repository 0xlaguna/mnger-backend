use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize
)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "workorder")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: String,

    /// Title
    pub title: String,

    /// Description
    pub description: String,

    /// Status
    pub status: i8,

    /// Start Date
    pub start_date: TimeDateTimeWithTimeZone,

    /// End Date
    pub end_date: TimeDateTimeWithTimeZone,

    /// Created By
    pub created_by: i32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
