use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize,
)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "company")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Text"
    )]
    pub id: String,

    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub address: Option<String>,

    pub phone_number: Option<String>,

    pub email: Option<String>,
    
    pub logo: Option<String>,

    pub website: Option<String>,

    pub created_at: Option<DateTimeWithTimeZone>,

    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user::Entity")]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
