use mnger_preon::dto::users::{DataLoginAccount, LoginResponse};
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use mnger_preon::r#impl::postgres::pool::Db;
use mnger_preon::r#impl::postgres::users::account::AbstractAccount;
use mnger_preon::Result;

/// Login user account
#[utoipa::path(
    context_path = "/account",
    request_body = DataLoginAccount,
    responses(
        (status = 200, description = "Logged in successfully", body = LoginResponse),
    ),
    security(
        (),
        ("x-session-token" = [])
    )
)]
#[post("/login", data = "<data>")]
pub async fn req(
    conn: Connection<'_, Db>,
    data: Json<DataLoginAccount>,
) -> Result<Json<LoginResponse>> {
    let db = conn.into_inner();

    let data = data.into_inner();

    let session =
        AbstractAccount::login(db, &data.email, &data.password, data.name.as_deref()).await?;

    let response: LoginResponse = session.into();

    Ok(Json(response))
}
