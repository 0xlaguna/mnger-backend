use rocket::form::Form;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use mnger_preon::dto::users::{DataEditUser, User};
use mnger_preon::models::Session;
use mnger_preon::r#impl::postgres::pool::Db;
use mnger_preon::r#impl::postgres::users::user::AbstractUser;
use mnger_preon::{Error, Result};

/// Edit User account
#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 201, description = "User edited successfully", body = User),
    ),
)]
#[patch("/<target>", data = "<data>")]
pub async fn req(
    conn: Connection<'_, Db>,
    mut _session: Session,
    target: String,
    data: Form<DataEditUser<'_>>,
) -> Result<Json<User>> {
    let db = conn.into_inner();
    let data = data.into_inner();

    if _session.user_id != target {
        return Err(Error::NotPrivileged);
    }

    let user = AbstractUser::update_user(db, &target, data).await?;

    let user: User = user.into();

    Ok(Json(user))
}
