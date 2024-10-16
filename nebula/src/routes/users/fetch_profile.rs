use mnger_preon::dto::users::User;
use mnger_preon::models::Session;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use mnger_preon::r#impl::postgres::pool::Db;
use mnger_preon::r#impl::postgres::users::user::AbstractUser;
use mnger_preon::Result;

/// Fetch user profile
#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, description = "Account created successfully", body = User),
    ),
)]
#[get("/<target>/profile")]
pub async fn req(
    conn: Connection<'_, Db>,
    mut _session: Session,
    target: String,
) -> Result<Json<User>> {
    let db = conn.into_inner();

    let user = AbstractUser::fetch_user(db, &target).await?;

    let response: User = user.into();

    Ok(Json(response))
}
