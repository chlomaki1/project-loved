use actix_web::{error::JsonPayloadError, http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use rosu_v2::error::OsuError;
use sea_orm::DbErr;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Display, Serialize, Clone)]
#[serde(untagged)]
pub enum LovedError {
    //* `4xx REQUEST ERROR` error codes *//
    
    /// `400 BAD REQUEST
    /// An error that occurs because the client sent a general request that
    /// the application could not understand.
    #[display(fmt = "The application was unable to understand this request.")]
    BadRequest,

    /// AUTHORIZATION ///
    
    /// `400 BAD REQUEST`
    /// 
    /// An error that occurs because the client sent an invalid token
    /// authentication type.
    #[display(fmt = "The provided token authentication type is invalid.")]
    InvalidTokenAuthType,

    /// `400 BAD REQUEST`
    /// 
    /// An error that occurs because the client sent an invalid token state.
    #[display(fmt = "The provided token state is invalid.")]
    InvalidTokenState,

    /// GENERIC ///
    
    /// `404 NOT FOUND`
    /// The client tried to request an endpoint that doesn't exist.
    #[display(fmt = "The requested endpoint was not found.")]
    NotFound,

    /// `405 METHOD NOT ALLOWED`
    /// An error that occurs because the  client tried to access an endpoint 
    /// with a method that isn't accepted by the endpoint itself.
    #[display(fmt = "This endpoint does not support this request method.")]
    MethodNotAccepted /*{
        // ...how do i implement this?
        allowed: [?]
    }*/,

    /// `413 PAYLOAD TOO LARGE`
    /// The client sent a request with a body that's larger than the application or
    /// endpoint can even handle.
    #[display(fmt = "The provided request body is too large.")]
    RequestTooLarge {
        limit: usize
    },

    /// `415 UNSUPPORTED MEDIA TYPE`
    /// An error that occurs when the client sends a content body with a type that 
    /// isnt accepted by the endpoint.
    #[display(fmt = "This endpoint does not accept the provided body under this content type, as it expects a different kind.")]
    UnsupportedContentType {
        expected: &'static str
    },

    //* `5xx SERVER ERROR` error codes *//

    /// `500 INTERNAL SERVER ERROR`
    /// An error that occurs whenever the application has encountered an unrecoverable
    /// error, and it is thus completely impossible to continue the request.
    #[display(fmt = "An internal application error has occurred.")]
    InternalError,

    /// `500 INTERNAL SERVER ERROR`
    /// An error that occurs whenever the database encounters an unrecoverable error,
    /// and nothing can be done to keep the request going.
    #[display(fmt = "An internal database error has occurred.")]
    DatabaseError,

    /// `500 INTERNAL SERVER ERROR`
    /// An error that occurs whenever a connection cannot be made to the database.
    #[display(fmt = "The application was unable to make a connection to the database.")]
    DatabaseConnectionError,

    /// An error that isn't naturally handled by the application's error handler,
    /// acting as a fallback with a generic message and status code.
    #[display(fmt = "An unknown application error has occurred.",)]
    Unknown {
        #[serde(skip)]
        status: StatusCode,
    } 
}

impl ResponseError for LovedError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            // 4xx CLIENT ERROR
            // 400 BAD REQUEST
            LovedError::BadRequest => actix_web::http::StatusCode::BAD_REQUEST,
            LovedError::InvalidTokenAuthType => actix_web::http::StatusCode::BAD_REQUEST,
            LovedError::InvalidTokenState => actix_web::http::StatusCode::BAD_REQUEST,

            // 404 NOT FOUND,
            LovedError::NotFound => actix_web::http::StatusCode::NOT_FOUND,

            // 405 METHOD NOT ALLOWED
            LovedError::MethodNotAccepted => actix_web::http::StatusCode::METHOD_NOT_ALLOWED,

            // 413 PAYLOAD TOO LARGE
            LovedError::RequestTooLarge { .. } => actix_web::http::StatusCode::PAYLOAD_TOO_LARGE,

            // 415 UNSUPPORTED MEDIA TYPE
            LovedError::UnsupportedContentType { .. } => actix_web::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,

            // 5xx SERVER ERROR
            // 500 INTERNAL SERVER ERROR
            LovedError::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            LovedError::DatabaseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            LovedError::DatabaseConnectionError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            
            // UNKNOWN
            LovedError::Unknown { status, .. } => status,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(json!({
            "status": self.status_code().as_u16(),
            "message": self.to_string(),
            "data": self
        }))
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
        match error {
            _ => LovedError::DatabaseError
        }
    }
}

impl From<redis::RedisError> for LovedError {
    fn from(error: redis::RedisError) -> LovedError {
        match error {
            _ => LovedError::DatabaseError
        }
    }
}