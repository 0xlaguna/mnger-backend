use rocket::serde::{Serialize, Deserialize};
use sea_orm::prelude::DateTimeWithTimeZone;
use utoipa::ToSchema;

use crate::models::workorder;

/// WorkOrder Create Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DataCreateWorkOrder {
    /// WorkOrder name
    pub title: String,

    /// Descripion
    pub description: String,

    /// Start Date
    pub start_date: DateTimeWithTimeZone,

    /// End Date
    pub end_date: Option<DateTimeWithTimeZone>,

}

/// WorkOrder Iitem
#[derive(Serialize, Deserialize, ToSchema)]
pub struct WorkOrderItem {
    /// Id
    pub id: String,

    /// WorkOrder name
    pub title: String,

    /// Descripion
    pub description: String,

    /// Start Date
    pub start_date: DateTimeWithTimeZone,

    /// End Date
    pub end_date: Option<DateTimeWithTimeZone>,

    /// Created At
    pub created_at: DateTimeWithTimeZone
}

/// WorkOrder List Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct WorkOrderListData {
    /// WorkOrder List
    pub list: Vec<WorkOrderItem>,

    /// Total pages
    pub pages: u64
}

impl From<workorder::Model> for WorkOrderItem {
    fn from(model: workorder::Model) -> Self {
        WorkOrderItem {
            id: model.id,
            title: model.title,
            description: model.description,
            start_date: model.start_date,
            end_date: model.end_date,
            created_at: model.created_at
        }
    }
}

impl From<(Vec<workorder::Model>, u64)> for WorkOrderListData {
    fn from(tuple: (Vec<workorder::Model>, u64)) -> Self {
        WorkOrderListData {
            list: tuple.0.into_iter().map(WorkOrderItem::from).collect(),
            pages: tuple.1,
        }
    }
}
