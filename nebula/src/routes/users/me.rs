use mnger_preon::dto::users::User;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use mnger_preon::models::Session;
use mnger_preon::r#impl::postgres::pool::Db;
use mnger_preon::r#impl::postgres::users::user::AbstractUser;
use mnger_preon::Result;

/// Getme
#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, body = UserGetMeData, description = "Getme information"),
    ),
)]
#[get("/me")]
pub async fn req(conn: Connection<'_, Db>, mut _session: Session) -> Result<Json<User>> {
    let db = conn.into_inner();

    let user = AbstractUser::fetch_me(db, &_session.user_id).await?;

    let user: User = user.into();

    Ok(Json(user))
}
