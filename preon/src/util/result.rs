use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

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
    },
    InternalError,
    InvalidOperation,
    InvalidCredentials,
    NotFound,
    FailedValidation {
        #[serde(skip_serializing, skip_deserializing)]
        error: ValidationErrors,
    },
}
