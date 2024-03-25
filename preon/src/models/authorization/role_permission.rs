use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "role_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub role_id: i32,
    
    #[sea_orm(primary_key)]
    pub permission_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
