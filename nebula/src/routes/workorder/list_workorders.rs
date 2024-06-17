use altera::sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm_rocket::Connection;
use rocket::serde::{Serialize, Deserialize, json::Json};
use utoipa::ToSchema;

use mnger_preon::Result;
use mnger_preon::models::Session as Session;
use mnger_preon::models::workorder;
use mnger_preon::r#impl::postgres::workorders::workorder::AbstractWorkOrder;
use mnger_preon::r#impl::postgres::pool::Db;


/// WorkOrder List Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct WorkOrderListData {
    /// WorkOrder List
    pub list: Vec<WorkOrderItem>,

    /// Total pages
    pub pages: u64
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
}

impl From<workorder::Model> for WorkOrderItem {
    fn from(model: workorder::Model) -> Self {
        WorkOrderItem {
            id: model.id,
            title: model.title,
            description: model.description,
            start_date: model.start_date,
            end_date: model.end_date
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


/// Fetch WorkOrders
#[utoipa::path(
    context_path = "/workorder",
    responses(
        (status = 200, description = "List returned", body = WorkOrderListData),
    ),
)]
#[get("/?<page>&<per_page>")]
pub async fn req(
    conn: Connection<'_, Db>, 
    mut _session: Session, 
    page: Option<u64>, 
    per_page: Option<u64>
) -> Result<Json<WorkOrderListData>> { 
    let db = conn.into_inner();

    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(4);
    
    let workorders = AbstractWorkOrder
        ::workorder_pagination(db, _session.user_id, page, per_page).await?;

    let response: WorkOrderListData = workorders.into();

    Ok(Json(response))
}
