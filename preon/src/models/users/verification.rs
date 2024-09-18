use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "verification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub user_id: i32,

    pub type_id: i32,

    pub token: Option<String>,

    pub pending: Option<bool>,

    pub expires_at: Option<DateTimeWithTimeZone>,

    pub enabled: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
