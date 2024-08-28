use mnger_preon::r#impl::storage::s3::S3;
use mnger_preon::models::Session as Session;
use sea_orm_rocket::Connection;
use rocket::response::Redirect;

use mnger_preon::Result;
use mnger_preon::r#impl::postgres::pool::Db;


/// Fetch avatar
#[utoipa::path(
    context_path = "/media",
)]
#[get("/avatars/<filename>")]
pub async fn req(conn: Connection<'_, Db>, mut _session: Session, filename: String) -> Result<Redirect> {
    let _db = conn.into_inner();

    let key = format!(
        "user-images/{}.{}",
        filename,
        "jpeg"
    );

    let presigned_url = S3::generate_presigned_url(key).await?;

    Ok(Redirect::to(presigned_url))
}
