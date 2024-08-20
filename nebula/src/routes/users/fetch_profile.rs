use mnger_preon::dto::users::FetchProfileResponse;
use mnger_preon::models::Session as Session;
use sea_orm_rocket::Connection;
use rocket::serde::json::Json;

use mnger_preon::Result;
use mnger_preon::r#impl::postgres::users::user::AbstractUser;
use mnger_preon::r#impl::postgres::pool::Db;


/// Fetch user profile
#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, description = "Account created successfully", body = FetchProfileResponse),
    ),
)]
#[get("/<target>/profile")]
pub async fn req(conn: Connection<'_, Db>, mut _session: Session, target: i32) -> Result<Json<FetchProfileResponse>> {
    let db = conn.into_inner();

    let user = AbstractUser::fetch_user(db, target).await?;

    let response: FetchProfileResponse = user.into();

    Ok(Json(response))
}
