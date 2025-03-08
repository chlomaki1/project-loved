use actix_web::HttpRequest;
use crate::{errors::LovedError, service::Response};

pub mod oauth;

pub async fn handle_default(request: HttpRequest) -> Result<Response, LovedError> {
    if request.resource_map().has_resource(request.path()) {
        return Err(LovedError::MethodNotAccepted);
    }

    Err(LovedError::NotFound)
}