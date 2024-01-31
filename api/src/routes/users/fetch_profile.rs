use sea_orm_rocket::Connection;
use rocket::serde::json::Json;

use mnger_preon::Result;

use mnger_api_service::operations::users::user::AbstractUser;

use crate::pool::Db;

pub use entity::user;

/// # Fetch User Profile
///
/// Retrieve a user's profile data.
///
/// Will fail if you do not have permission to access the other user's profile.

#[get("/<target>/profile")]
pub async fn req(conn: Connection<'_, Db>, target: i32) -> Result<Json<user::Model>> {
    let db = conn.into_inner();

    let user = AbstractUser::fetch_user(db, target).await?;

    Ok(Json(user))
}