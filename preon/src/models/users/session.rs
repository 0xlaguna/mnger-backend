use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, FromForm,
)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    /// Session token
    pub token: String,

    /// Display name
    pub name: String,

    /// User id
    pub user_id: i32

}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
