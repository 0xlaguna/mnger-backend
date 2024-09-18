use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "team")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,

    #[sea_orm(unique)]
    pub name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub created_by: Option<String>,

    pub created_at: DateTimeWithTimeZone,

    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::team_participant::Entity")]
    TeamParticipant,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedBy",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(has_many = "crate::models::work_order_assignment::Entity")]
    WorkOrderAssignment,
}

impl Related<super::team_participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamParticipant.def()
    }
}

impl Related<super::user::Entity> for Entity {
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
