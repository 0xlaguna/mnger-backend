use mnger_preon::r#impl::postgres::users::account::AbstractAccount;
use rocket::futures::stream::StreamExt;
use rocket::response::stream::ByteStream;
use sea_orm_rocket::Connection;

use mnger_preon::r#impl::postgres::pool::Db;
use mnger_preon::r#impl::storage::s3::S3;
use mnger_preon::Error;
use mnger_preon::Result;

/// Fetch avatar
#[utoipa::path(context_path = "/media")]
#[get("/avatars?<filename>&<token>")]
pub async fn req(
    conn: Connection<'_, Db>,
    filename: &str,
    token: &str,
) -> Result<ByteStream![Vec<u8>]> {
    let db = conn.into_inner();

    let _session = AbstractAccount::find_session_by_token(db, token.to_string()).await?;

    let key = format!("user-images/{}", filename);

    let presigned_url = S3::generate_presigned_url(key).await?;

    let response = reqwest::get(presigned_url)
        .await
        .map_err(|e| Error::InternalError {
            info: e.to_string(),
        })?;

    if !response.status().is_success() {
        return Err(Error::NotFound);
    }

    let mut bytes_stream = response.bytes_stream();

    Ok(ByteStream! {
        while let Some(chunk) = bytes_stream.next().await {
            match chunk {
                Ok(bytes) => yield bytes.to_vec(),
                Err(_) => yield vec![],
            }
        }
    })
}
