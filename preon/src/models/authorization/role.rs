use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "role")]
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
    #[sea_orm(has_many = "super::role_permission::Entity")]
    RolePermission,
}

impl Related<super::role_module::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoleModule.def()
    }
}

impl Related<super::role_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolePermission.def()
    }
}

impl Related<super::module::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_module::Relation::Module.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_module::Relation::Role.def().rev())
    }
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_permission::Relation::Permission.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_permission::Relation::Role.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
