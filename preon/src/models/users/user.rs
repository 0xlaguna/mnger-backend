use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize,
)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Text"
    )]
    pub id: String,

    /// Username
    pub username: Option<String>,
    
    /// Email
    #[sea_orm(unique)]
    pub email: String,

    /// First name
    pub first_name: String,

    /// Middle name
    pub middle_name: Option<String>,

    /// Last Name
    pub last_name: String,

    /// Dob
    pub dob: Option<Date>,

    /// Avatar
    pub avatar: Option<String>,

    pub timezone: Option<String>,
    
    /// Argon2 hashed password
    pub password: String,

    /// Is account disabled ?
    pub enabled: bool,

    #[sea_orm(column_type = "Text", nullable)]
    pub company_id: Option<String>,

    pub created_at: DateTimeWithTimeZone,
    
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::company::Entity",
        from = "Column::CompanyId",
        to = "super::company::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Company,

    #[sea_orm(has_many = "super::session::Entity")]
    Session,

    #[sea_orm(has_many = "crate::models::workorder::Entity")]
    WorkOrder,

    #[sea_orm(has_many = "crate::models::work_order_assignment::Entity")]
    WorkOrderAssignment,
}

impl Related<super::company::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Company.def()
    }
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

impl Related<crate::models::work_order_assignment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkOrderAssignment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
