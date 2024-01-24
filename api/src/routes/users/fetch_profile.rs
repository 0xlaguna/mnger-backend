use sea_orm_rocket::Connection;
use rocket::serde::json::Json;
use rocket::http::Status;

use mnger_api_service::Query;

use crate::pool::Db;

pub use entity::user;

/// # Fetch User Profile
///
/// Retrieve a user's profile data.
///
/// Will fail if you do not have permission to access the other user's profile.

#[openapi]
#[get("/<target>/profile")]
pub async fn req(conn: Connection<'_, Db>, target: i32) -> Result<Json<user::Model>, Status> {
    let db = conn.into_inner();

    let user: Option<user::Model> = Query::find_user_by_id(db, target)
        .await
        .expect("could not find user");

    let json = Json(user);
    Ok(json)
}