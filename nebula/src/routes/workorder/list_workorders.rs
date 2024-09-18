use mnger_preon::dto::work_order::WorkOrderListData;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use mnger_preon::models::Session;
use mnger_preon::r#impl::postgres::pool::Db;
use mnger_preon::r#impl::postgres::workorders::workorder::AbstractWorkOrder;
use mnger_preon::Result;

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
    per_page: Option<u64>,
) -> Result<Json<WorkOrderListData>> {
    let db = conn.into_inner();

    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(4);

    let workorders =
        AbstractWorkOrder::workorder_pagination(db, _session.user_id, page, per_page).await?;

    let response: WorkOrderListData = workorders.into();

    Ok(Json(response))
}
