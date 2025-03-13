use actix_web::{error::JsonPayloadError, http::StatusCode};
use athena::{errors::AthenaError, RequestError};
use rosu_v2::error::OsuError;
use sea_orm::DbErr;

#[athena::request_error]
pub enum LovedError {
    //* `4xx REQUEST ERROR` error codes *//

    /// AUTHORIZATION ///

    /// `400 BAD REQUEST`
    /// An error that occurs because the client sent an invalid token
    /// authentication type.
    #[error(StatusCode::BAD_REQUEST, "ERR_INVALID_TOKEN_AUTH_TYPE", "The provided token authentication type is invalid.")]
    InvalidTokenAuthType,

    /// `400 BAD REQUEST`
    /// An error that occurs because the client sent an invalid token state.
    #[error(StatusCode::BAD_REQUEST, "ERR_INVALID_TOKEN_STATE", "The provided token state is invalid.")]
    InvalidTokenState,

    /// `400 BAD REQUEST`
    /// An error that occurs because the client sent an invalid limit.
    #[error(StatusCode::BAD_REQUEST, "ERR_INVALID_LIMIT", "The provided limit is invalid.")]    
    PaginationLimitInvalid {
        min: usize,
        max: usize
    },

    /// PAGINATION ///
    
    /// `400 BAD REQUEST`
    /// An error that occurs because the client sent an invalid page number.
    #[error(StatusCode::BAD_REQUEST, "ERR_INVALID_PAGE", "The provided page number is invalid.")]
    InvalidPage,

    /// GENERIC ///

    /// `400 BAD REQUEST`
    /// An error that occurs because the client sent a general request that
    /// the application could not understand.
    #[error(StatusCode::BAD_REQUEST, "ERR_BAD_REQUEST", "The application was unable to understand this request.")]
    BadRequest,
    
    /// `404 NOT FOUND`
    /// The client tried to request an endpoint that doesn't exist.
    #[error(StatusCode::NOT_FOUND, "ERR_NOT_FOUND", "The requested endpoint was not found.")]
    NotFound,
    
    /// `404 NOT FOUND`
    /// An error that occurs because the client tried to access a resource
    /// that doesn't exist.
    #[error(StatusCode::NOT_FOUND, "ERR_RESOURCE_NOT_FOUND", "No {model} with this identifier could be found.")]
    ModelNotFound {
        #[serde(skip)]
        model: &'static str
    },

    /// `405 METHOD NOT ALLOWED`
    /// An error that occurs because the client tried to access an endpoint 
    /// with a method that isn't accepted by the endpoint itself.
    #[error(StatusCode::METHOD_NOT_ALLOWED, "ERR_METHOD_NOT_ACCEPTED", "This endpoint does not support this request method.")]
    MethodNotAccepted,

    /// `413 PAYLOAD TOO LARGE`
    /// The client sent a request with a body that's larger than the application or
    /// endpoint can even handle.
    #[error(StatusCode::PAYLOAD_TOO_LARGE, "ERR_REQUEST_TOO_LARGE", "The provided request body is too large.")]
    RequestTooLarge {
        limit: usize
    },

    /// `415 UNSUPPORTED MEDIA TYPE`
    /// An error that occurs when the client sends a content body with a type that 
    /// isn't accepted by the endpoint.
    #[error(StatusCode::UNSUPPORTED_MEDIA_TYPE, "ERR_UNSUPPORTED_MEDIA_TYPE", "This endpoint does not accept the provided body under this content type, as it expects a different kind.")]
    UnsupportedContentType {
        expected: &'static str
    },

    //* `5xx SERVER ERROR` error codes *//

    /// `500 INTERNAL SERVER ERROR`
    /// An error that occurs whenever the application has encountered an unrecoverable
    /// error, and it is thus completely impossible to continue the request.
    #[error(StatusCode::INTERNAL_SERVER_ERROR, "ERR_INTERNAL_ERROR", "An internal application error has occurred.")]
    InternalError,

    /// `500 INTERNAL SERVER ERROR`
    /// An error that occurs whenever the database encounters an unrecoverable error,
    /// and nothing can be done to keep the request going.
    #[error(StatusCode::INTERNAL_SERVER_ERROR, "ERR_DATABASE_ERROR", "An internal database error has occurred.")]
    DatabaseError,

    /// `500 INTERNAL SERVER ERROR`
    /// An error that occurs whenever a connection cannot be made to the database.
    #[error(StatusCode::INTERNAL_SERVER_ERROR, "ERR_DATABASE_CONNECTION_ERROR", "The application was unable to make a connection to the database.")]
    DatabaseConnectionError,

    /// An error that isn't naturally handled by the application's error handler,
    /// acting as a fallback with a generic message and status code.
    #[error(StatusCode::INTERNAL_SERVER_ERROR, "ERR_UNKNOWN", "An unknown application error has occurred.")]
    Unknown {
        #[serde(skip)]
        status: StatusCode,
    } 
}

impl From<JsonPayloadError> for LovedError {
    fn from(error: JsonPayloadError) -> LovedError {
        match error {
            JsonPayloadError::ContentType => LovedError::UnsupportedContentType { expected: "application/json" },
            JsonPayloadError::OverflowKnownLength { length: _, limit } => LovedError::RequestTooLarge { limit },
            JsonPayloadError::Overflow { limit } => LovedError::RequestTooLarge { limit },
            _ => LovedError::BadRequest
        }
    }
}

impl From<OsuError> for LovedError {
    fn from(error: OsuError) -> Self {
        println!("osu error: {:?}", error);
        match error {
            OsuError::UpdateToken { source: _} => LovedError::InvalidTokenState,
            _ => LovedError::BadRequest
        }
    }
}

impl From<DbErr> for LovedError {
    fn from(error: DbErr) -> Self {
        println!("db error: {:?}", error);
        
        LovedError::DatabaseError
    }
}

impl From<redis::RedisError> for LovedError {
    fn from(error: redis::RedisError) -> LovedError {
        println!("redis error: {:?}", error);
        
        LovedError::DatabaseError
    }
}

impl From<AthenaError> for LovedError {
    fn from(error: AthenaError) -> Self {
        match error {
            AthenaError::DbErr(_) => LovedError::DatabaseError,
            AthenaError::ModelNotFound(model) => LovedError::ModelNotFound { model }
        }
    }
}