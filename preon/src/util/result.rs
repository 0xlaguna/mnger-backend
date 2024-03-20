use rocket::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

use std::io::Cursor;

/// API Errors
#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(tag = "type")]
pub enum Error {
    // Permissions
    NotOwner,

    // General errors
    DatabaseError {
        operation: &'static str,
        with: &'static str,
        info: String,
    },
    InternalError,
    InvalidOperation,
    InvalidCredentials,
    NotFound,
    InvalidSession,
    FailedValidation {
        #[serde(skip_serializing, skip_deserializing)]
        error: ValidationErrors,
    },

    MissingHeaders,
}

/// Result type with custom Error
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// HTTP response builder for Error enum
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status = match self {
            Error::NotOwner => Status::Forbidden,

            Error::DatabaseError { .. } => Status::InternalServerError,
            Error::InternalError => Status::InternalServerError,
            Error::InvalidOperation => Status::BadRequest,
            Error::InvalidCredentials => Status::Unauthorized,
            Error::NotFound => Status::NotFound,
            Error::InvalidSession => Status::Unauthorized,
            Error::FailedValidation { .. } => Status::BadRequest,
            Error::MissingHeaders => Status::Unauthorized,
        };

        // Serialize the error data structure into JSON.
        let string = serde_json::to_string(&self).unwrap();

        // Build and send the request.
        Response::build()
            .sized_body(string.len(), Cursor::new(string))
            .header(ContentType::new("application", "json"))
            .status(status)
            .ok()
    }
}
