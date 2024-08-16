use rocket::form::Form;
use rocket::response::status;
use sea_orm_rocket::Connection;

use mnger_preon::{Error, Result};
use mnger_preon::models::Session as Session;
use mnger_preon::r#impl::postgres::users::user::AbstractUser;
use mnger_preon::r#impl::postgres::pool::Db;
use mnger_preon::dto::users::DataEditUser;


/// Edit User account
#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 201, description = "User edited successfully"),
    ),
)]
#[patch("/<target>", data = "<data>")]
pub async fn req(
    conn: Connection<'_, Db>,
    mut _session: Session,
    target: i32, 
    data: Form<DataEditUser<'_>>
) -> Result<status::NoContent>  {
    let db = conn.into_inner();
    let data = data.into_inner();

    if _session.user_id != target {
        return Err(Error::NotPrivileged)
    }

    AbstractUser
        ::update_user(
            db,
            target,
            data
        ).await?;

    Ok(status::NoContent)
}
