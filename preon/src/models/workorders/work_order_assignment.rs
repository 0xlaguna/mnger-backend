use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize,
)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "work_order_assignment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    #[sea_orm(column_type = "Text")]
    pub work_order_id: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub user_id: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub team_id: Option<String>,

    pub assigned_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::team::Entity",
        from = "Column::TeamId",
        to = "crate::models::team::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Team,
    #[sea_orm(
        belongs_to = "crate::models::user::Entity",
        from = "Column::UserId",
        to = "crate::models::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::workorder::Entity",
        from = "Column::WorkOrderId",
        to = "super::workorder::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    WorkOrder,
}

impl Related<crate::models::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl Related<crate::models::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::workorder::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkOrder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
