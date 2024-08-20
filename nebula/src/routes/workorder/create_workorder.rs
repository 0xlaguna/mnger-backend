use mnger_preon::dto::work_order::DataCreateWorkOrder;
use rocket::response::status;
use sea_orm_rocket::Connection;
use rocket::serde::json::Json;

use mnger_preon::Result;
use mnger_preon::models::Session as Session;
use mnger_preon::r#impl::postgres::workorders::workorder::AbstractWorkOrder;
use mnger_preon::r#impl::postgres::pool::Db;


/// Create WorkOrder
#[utoipa::path(
    context_path = "/workorder",
    request_body = DataCreateWorkOrder,
    responses(
        (status = 201, description = "Workorder created successfully"),
    ),
)]
#[post("/create", data = "<data>")]
pub async fn req(conn: Connection<'_, Db>, mut _session: Session, data: Json<DataCreateWorkOrder>) -> Result<status::NoContent> {
    let db = conn.into_inner();

    let data = data.into_inner();

    let _ = AbstractWorkOrder
        ::create_work_order(
            db,
            data.title, 
            data.description,
            data.start_date, 
            data.end_date, 
            _session.user_id,
        ).await?;

    Ok(status::NoContent)
}
