pub use sea_orm;

pub mod operations;

/// Result type with custom Error
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error information
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemas", derive(JsonSchema))]
#[derive(Debug, Clone)]
pub struct Error {
    /// Type of error and additional information
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub error_type: ErrorType,

    /// Where this error occurred
    pub location: String,
}

/// Possible error types
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[cfg_attr(feature = "schemas", derive(JsonSchema))]
#[derive(Debug, Clone)]
pub enum ErrorType {
    MissingPermission {
        permission: String,
    },
    MissingUserPermission {
        permission: String,
    },

    NotOwner,
    InternalError,
    InvalidOperation,
    InvalidCredentials,
    NotFound,

    // ? General errors
    DatabaseError {
        info: String,
    },

    FailedValidation {
        error: String,
    },
}

#[macro_export]
macro_rules! create_error {
    ( $error: ident $( $tt:tt )? ) => {
        $crate::Error {
            error_type: $crate::ErrorType::$error $( $tt )?,
            location: format!("{}:{}:{}", file!(), line!(), column!()),
        }
    };
}

#[macro_export]
macro_rules! create_database_error {
    ( $info: expr ) => {
        create_error!(DatabaseError {
            info: $info.to_string()
        })
    };
}
