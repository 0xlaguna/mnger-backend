use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize,
)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    /// Username
    pub username: Option<String>,
    
    /// Email
    pub email: String,

    /// First name
    pub first_name: String,

    /// Middle name
    pub middle_name: String,

    /// Last Name
    pub last_name: String,
    
    /// Argon2 hashed password
    pub password: String,

    /// Is account disabled ?
    pub disabled: bool,

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::session::Entity")]
    Session,

    #[sea_orm(has_many = "crate::models::workorder::Entity")]
    WorkOrder,
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}
impl Related<crate::models::workorder::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkOrder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
