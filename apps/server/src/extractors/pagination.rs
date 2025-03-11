use std::future::Future;

use actix_web::{FromRequest, HttpRequest, Responder};
use futures_util::future::LocalBoxFuture;
use serde::Serialize;

use crate::{errors::LovedError, service::Response};

pub struct Pagination<Output, const HARD_LIMIT: usize> 
    where Output: Serialize
{
    pub page: u32,
    pub limit: usize,
    pub total: usize,
    pub data: Vec<Output>,
}

impl<Output, const HARD_LIMIT: usize> FromRequest for Pagination<Output, HARD_LIMIT>
    where Output: Serialize
{
    type Error = LovedError;
    type Future = LocalBoxFuture<'static, Result<Pagination<Output, HARD_LIMIT>, LovedError>>;

    fn from_request(
        req: &HttpRequest,
        _payload: &mut actix_web::dev::Payload
    ) -> Self::Future {
        // The request wont outlive the extractor
        // unless we clone it
        let req = HttpRequest::clone(req);

        Box::pin(async move {
            let query_string = querystring::querify(req.query_string());
            let mut page = 1;
            let mut limit = HARD_LIMIT;
    
            for (key, value) in query_string {
                if key == "page" {
                    let parsed_page = value.parse().unwrap();
    
                    if parsed_page > 0 {
                        page = parsed_page;
                    } else {
                        return Err(LovedError::InvalidPage);
                    }
                } else if key == "limit" {
                    let parsed_limit: usize = value.parse().unwrap();
    
                    if parsed_limit > HARD_LIMIT || parsed_limit < 1 {
                        return Err(LovedError::PaginationLimitInvalid { min: 1, max: HARD_LIMIT });
                    }

                    limit = parsed_limit;
                }
            }
    
            Ok(Pagination { page, limit, total: 0, data: Vec::new() })
        })
    }
}

impl<Output, const HARD_LIMIT: usize> Pagination<Output, HARD_LIMIT>
    where Output: Serialize
{
    pub fn get_page_offset(&self) -> usize {
        (self.page - 1) * self.limit
    }

    pub async fn provide<Fut>(mut self, fun: impl FnOnce(&Pagination<Output, HARD_LIMIT>) -> Fut) -> Result<Self, LovedError>
        where Fut: Future<Output = Result<Vec<Output>, LovedError>>
    {
        let data = fun(&self).await;

        if let Err(e) = data {
            return Err(e);
        }

        self.data = data.unwrap();
        Ok(self)
    }

    pub fn respond(
        self,
    ) -> Result<Response<Vec<Output>>, LovedError> {
        Ok(Response {
            status: 200,
            message: None,
            data: Some(self.data)
        })
    }
}