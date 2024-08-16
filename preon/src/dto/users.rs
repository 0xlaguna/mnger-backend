use rocket::http::ContentType;
use rocket::data::ToByteUnit;
use rocket::form::{self, FromForm, DataField, FromFormField};
use utoipa::ToSchema;

pub struct DataEditUserAvatar<'r> {
    pub file_name: &'r str,
    pub content_type: ContentType,
    pub extension: String,
    pub data: &'r [u8]
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for DataEditUserAvatar<'r> {
    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `256KiB` as default.
        let limit = field.request.limits()
            .get("avatar")
            .unwrap_or(256.kibibytes());

        let sanitized_filename = match field.file_name {
            Some(i) => i.as_str(),
            None => Some("default")
        };

        let file_name = sanitized_filename.unwrap_or("default");
        let extension = field.content_type.extension().unwrap().to_string();
        
        // Read the capped data stream, returning a limit error as needed.
        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }
        let bytes = bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);

        Ok(DataEditUserAvatar { 
            file_name,
            content_type: field.content_type,
            extension,
            data: bytes 
        })
    }
}

/// # Edit User Data
#[derive(ToSchema, FromForm)]
pub struct DataEditUser<'r> {
    /// First Name

    pub first_name: String,
    /// Middle Name

    pub middle_name: Option<String>,

    // Last Name
    pub last_name: String,

    // Avatar
    pub avatar: DataEditUserAvatar<'r>,
}
