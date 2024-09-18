use rocket::data::ToByteUnit;
use rocket::form::{self, DataField, FromForm, FromFormField};
use rocket::http::ContentType;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{user, Session};
use crate::r#impl::storage::s3::S3;

/// # User

/// # Login Account Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DataLoginAccount {
    /// Valid email address
    pub email: String,

    /// Password
    pub password: String,

    /// Session name
    pub name: Option<String>,
}

/// # Login-Response Account Data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,

    pub name: Option<String>,

    pub user_id: String,
}

impl From<Session> for LoginResponse {
    fn from(session: Session) -> Self {
        LoginResponse {
            token: session.token,
            name: session.name,
            user_id: session.user_id,
        }
    }
}

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
    pub middle_name: Option<String>,

    // Last Name
    pub last_name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
    /// User identifier
    pub id: String,

    /// Username
    pub username: Option<String>,

    /// Email
    pub email: String,

    /// First Name
    pub first_name: String,

    /// Middle Name
    pub middle_name: Option<String>,

    /// Last Name
    pub last_name: String,

    /// Avatar
    pub avatar: Option<String>,
}

impl From<user::Model> for User {
    fn from(user: user::Model) -> Self {
        User {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            middle_name: user.middle_name,
            last_name: user.last_name,
            avatar: S3::extract_filename(user.avatar),
        }
    }
}

/// User getme data
#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserGetMeData {
    /// Email
    pub email: String,

    /// First name
    pub first_name: String,

    /// Middle name
    pub middle_name: Option<String>,

    /// Last Name
    pub last_name: String,

    pub avatar: Option<String>,
}

impl From<user::Model> for UserGetMeData {
    fn from(model: user::Model) -> Self {
        UserGetMeData {
            email: model.email,
            first_name: model.first_name,
            middle_name: model.middle_name,
            last_name: model.last_name,
            avatar: S3::extract_filename(model.avatar),
        }
    }
}

/// # Edit Account Data
pub struct DataEditUserAvatar<'r> {
    pub file_name: &'r str,
    pub content_type: ContentType,
    pub extension: String,
    pub data: &'r [u8],
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for DataEditUserAvatar<'r> {
    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `256KiB` as default.
        let limit = field
            .request
            .limits()
            .get("avatar")
            .unwrap_or(256.kibibytes());

        let sanitized_filename = match field.file_name {
            Some(i) => i.as_str(),
            None => Some("default"),
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
            data: bytes,
        })
    }
}

/// # Edit User Data
#[derive(ToSchema, FromForm)]
pub struct DataEditUser<'r> {
    /// First Name
    pub first_name: Option<String>,
    /// Middle Name
    pub middle_name: Option<String>,

    // Last Name
    pub last_name: Option<String>,

    // Avatar
    pub avatar: Option<DataEditUserAvatar<'r>>,
}
