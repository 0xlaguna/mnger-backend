use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

use strong_id::strong_uuid;

strong_uuid!(pub struct WorkOrderId(pub Uuid => "wo"));

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "work_order")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,

    /// Title
    pub title: String,

    /// Description
    pub description: String,

    /// Status
    pub status: Option<i16>,

    /// Start Date
    pub start_date: DateTimeWithTimeZone,

    /// End Date
    pub end_date: Option<DateTimeWithTimeZone>,

    pub created_by: Option<String>,

    pub created_at: DateTimeWithTimeZone,

    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::user::Entity",
        from = "Column::CreatedBy",
        to = "crate::models::user::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    User,

    #[sea_orm(has_many = "crate::models::work_order_assignment::Entity")]
    WorkOrderAssignment,
}

impl Related<crate::models::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<crate::models::work_order_assignment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkOrderAssignment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
