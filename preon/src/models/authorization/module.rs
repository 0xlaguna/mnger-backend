use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "module")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub name: String,

    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_module::Entity")]
    RoleModule,
}

impl Related<super::role_module::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleModule.def()
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_module::Relation::Role.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_module::Relation::Module.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
