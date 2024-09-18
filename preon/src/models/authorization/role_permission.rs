use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "role_permission")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: i32,

    #[sea_orm(primary_key, auto_increment = false)]
    pub permission_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::permission::Entity",
        from = "Column::PermissionId",
        to = "super::permission::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Permission,
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Role,
}

impl Related<super::permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
