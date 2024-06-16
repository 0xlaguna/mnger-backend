use mnger_preon::models::Session as Session;
use sea_orm_rocket::Connection;
use rocket::serde::{Serialize, Deserialize, json::Json};

use utoipa::ToSchema;

use mnger_preon::Result;

use mnger_preon::r#impl::postgres::users::user::AbstractUser;
use mnger_preon::models::user;

use mnger_preon::r#impl::postgres::pool::Db;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FetchProfileResponse {
    /// User identifier
    pub id: i32,

    /// Username
    pub username: Option<String>,

    /// Email
    pub email: String,

    /// First Name
    pub first_name: String,

    /// Middle Name
    pub middle_name: String,

    /// Last Name
    pub last_name: String,
}

impl From<user::Model> for FetchProfileResponse {
    fn from(user: user::Model) -> Self {
        FetchProfileResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            middle_name: user.middle_name,
            last_name: user.last_name
        }
    }
}

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
