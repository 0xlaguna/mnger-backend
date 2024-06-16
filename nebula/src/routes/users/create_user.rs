use rocket::response::status;
use sea_orm_rocket::Connection;
use rocket::serde::{Serialize, Deserialize, json::Json};
use utoipa::ToSchema;

use mnger_preon::Result;
use mnger_preon::r#impl::postgres::users::user::AbstractUser;
use mnger_preon::r#impl::postgres::pool::Db;

/// # Account Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DataCreateAccount {
    /// Valid email address
    pub email: String,

    /// Password
    pub password: String,

    /// First Name
    pub first_name: String,

    /// Middle Name
    pub middle_name: String,

    // Last Name
    pub last_name: String,
}

/// Create user account
#[utoipa::path(
    context_path = "/users",
    request_body = DataCreateAccount,
    responses(
        (status = 201, description = "Account created successfully"),
    ),
)]
#[post("/create", data = "<data>")]
pub async fn req(conn: Connection<'_, Db>, data: Json<DataCreateAccount>) -> Result<status::NoContent> {

    let db = conn.into_inner();

    let data = data.into_inner();

    let _ = AbstractUser
        ::create_user(
            db,
            data.email,
            data.password,
            data.first_name,
            data.middle_name,
            data.last_name
        ).await?;
    
    Ok(status::NoContent)
}
