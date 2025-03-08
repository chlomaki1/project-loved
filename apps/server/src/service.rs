use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;
use uuid::Uuid;

fn respond_to_impl<T>(data: &T, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<BoxBody>
where T : Serialize {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(data).unwrap())
}


#[derive(Debug, Serialize)]
pub struct Response<T=()>
where T : Serialize {
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

impl<T> Responder for Response<T>
where T : Serialize {
    type Body = BoxBody;
    
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        respond_to_impl::<Self>(&self, req)
    }
}
#[derive(Serialize)]
pub struct ResponseWithMetadata<T, M>
where
    T : Serialize,
    M : Serialize
{
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub meta: M
}

impl<T, M> Responder for ResponseWithMetadata<T, M>
where
    T : Serialize,
    M : Serialize
{
    type Body = BoxBody;
    
    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        respond_to_impl::<Self>(&self, req)
    }
}

pub fn generate_token() -> String {
    Uuid::new_v4().to_string() // Generates a new UUID as a token
}