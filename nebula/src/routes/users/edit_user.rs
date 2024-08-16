use rocket::form::{self, DataField, Form, FromFormField, ValueField};
use rocket::data::ToByteUnit;
use rocket::response::status;
use sea_orm_rocket::Connection;
use utoipa::ToSchema;
use memchr::memchr;

use mnger_preon::{Error, Result};
use mnger_preon::models::Session as Session;
use mnger_preon::r#impl::postgres::pool::Db;


/// # Edit User Data
pub struct Avatar<'r> {
    name: &'r str,
    data: &'r [u8]
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Avatar<'r> {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> { 
        match field.value.find(':') { 
            Some(i) => Ok(Avatar { 
                name: &field.value[..i],
                data: field.value[(i + 1)..].as_bytes()
            }),
            None => Err(form::Error::validation("does not contain ':'"))?
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `256KiB` as default.
        let limit = field.request.limits()
            .get("avatar")
            .unwrap_or(256.kibibytes());

        // Read the capped data stream, returning a limit error as needed.
        let bytes = field.data.open(limit).into_bytes().await?;
        
        if !bytes.is_complete() { 
            Err((None, Some(limit)))?
        }
        
        // Store the bytes in request-local cache and split at ':'.
        let bytes = bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);

        let (raw_name, data) = match memchr(b':', bytes) { 
            Some(i) => (&bytes[..i], &bytes[(i + 1)..]),
            None => Err(form::Error::validation("does not contain ':'"))?
        };

        let name = std::str::from_utf8(raw_name)?;
        Ok(Avatar { name, data })
    }
}


#[derive(ToSchema, FromForm)]
pub struct DataEditUser<'r> {
    /// First Name

    pub first_name: String,
    /// Middle Name

    pub middle_name: Option<String>,

    // Last Name
    pub last_name: String,

    // Avatar
    pub avatar: Avatar<'r>,
}

/// Edit User account
#[utoipa::path(
    context_path = "/users",
    request_body = DataEditUser,
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
    let _db = conn.into_inner();
    let data = data.into_inner();

    if _session.user_id != target {
        return Err(Error::NotPrivileged)
    }

    Ok(status::NoContent)
}
