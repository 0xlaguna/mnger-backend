use mnger_preon::dto::users::UserGetMeData;
use sea_orm_rocket::Connection;
use rocket::serde::json::Json;

use mnger_preon::Result;
use mnger_preon::models::Session as Session;
use mnger_preon::r#impl::postgres::users::user::AbstractUser;
use mnger_preon::r#impl::postgres::pool::Db;


/// Getme
#[utoipa::path(
    context_path = "/users",
    responses(
        (status = 200, body = UserGetMeData, description = "Getme information"),
    ),
)]
#[get("/me")]
pub async fn req(conn: Connection<'_, Db>, mut _session: Session) -> Result<Json<UserGetMeData>> {
    let db = conn.into_inner();

    let user = AbstractUser
        ::fetch_user(
            db,
            _session.user_id
        ).await?;

    let user: UserGetMeData = user.into();

    Ok(Json(user))
}
