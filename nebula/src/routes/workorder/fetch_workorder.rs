use sea_orm_rocket::Connection;
use rocket::serde::json::Json;

use mnger_preon::Result;
use mnger_preon::models::Session as Session;
use mnger_preon::r#impl::postgres::workorders::workorder::AbstractWorkOrder;
use mnger_preon::r#impl::postgres::pool::Db;

use crate::routes::workorder::list_workorders::WorkOrderItem;

/// Fetch single workorder
#[utoipa::path(
    context_path = "/workorder",
    responses(
        (status = 200, body = WorkOrderItem),
    ),
)]
#[get("/<id>")]
pub async fn req(
    conn: Connection<'_, Db>, 
    mut _session: Session, 
    id: String
) -> Result<Json<WorkOrderItem>> {
    let db = conn.into_inner();

    let work_order = AbstractWorkOrder::fetch_work_order(db, id).await?;

    let response: WorkOrderItem = work_order.into();

    Ok(Json(response))
}
