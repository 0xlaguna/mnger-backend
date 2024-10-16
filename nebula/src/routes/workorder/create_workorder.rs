use mnger_preon::dto::work_order::DataCreateWorkOrder;
use rocket::response::status;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use mnger_preon::models::Session;
use mnger_preon::r#impl::postgres::pool::Db;
use mnger_preon::r#impl::postgres::workorders::workorder::AbstractWorkOrder;
use mnger_preon::Result;

/// Create WorkOrder
#[utoipa::path(
    context_path = "/workorder",
    request_body = DataCreateWorkOrder,
    responses(
        (status = 201, description = "Workorder created successfully"),
    ),
)]
#[post("/create", data = "<data>")]
pub async fn req(
    conn: Connection<'_, Db>,
    mut _session: Session,
    data: Json<DataCreateWorkOrder>,
) -> Result<status::NoContent> {
    let db = conn.into_inner();

    let data = data.into_inner();

    let _ = AbstractWorkOrder::create_work_order(db, data, &_session.user_id).await?;

    Ok(status::NoContent)
}
