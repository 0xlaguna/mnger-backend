use rocket::response::status;
use sea_orm_rocket::Connection;
use rocket::serde::{Serialize, Deserialize, json::Json};
use utoipa::ToSchema;

use mnger_preon::Result;
use mnger_preon::r#impl::postgres::pool::Db;

/// WorkOrder Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DataCreateWorkOrder {
    /// WorkOrder name
    pub name: String,
}

/// Create WorkOrder
#[utoipa::path(
    context_path = "/workorder",
    request_body = DataCreateWorkOrder,
    responses(
        (status = 201, description = "Workorder created successfully"),
    ),
)]
#[post("/create", data = "<data>")]
pub async fn req(conn: Connection<'_, Db>, data: Json<DataCreateWorkOrder>) -> Result<status::NoContent> {
    let db = conn.into_inner();

    let data = data.into_inner();

    Ok(status::NoContent)
}
