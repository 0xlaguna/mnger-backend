use mnger_preon::models::Session as Session;
use sea_orm_rocket::Connection;
use rocket::serde::json::Json;

use mnger_preon::Result;

use mnger_preon::r#impl::postgres::users::user::AbstractUser;
use mnger_preon::models::user;

use mnger_preon::r#impl::postgres::pool::Db;

/// # Fetch User Profile
///
/// Retrieve a user's profile data.
///
/// Will fail if you do not have permission to access the other user's profile.

#[get("/<target>/profile")]
pub async fn req(conn: Connection<'_, Db>, mut _session: Session, target: i32) -> Result<Json<user::Model>> {
    let db = conn.into_inner();

    let user = AbstractUser::fetch_user(db, target).await?;

    Ok(Json(user))
}
