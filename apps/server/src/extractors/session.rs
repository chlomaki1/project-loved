use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use athena::prelude::users::FullUser;
use futures_util::future::LocalBoxFuture;

use crate::{errors::LovedError, state::LovedState};

pub struct Session {
    pub user: FullUser
}

impl FromRequest for Session {
    type Error = LovedError;
    type Future = LocalBoxFuture<'static, Result<Session, LovedError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = HttpRequest::clone(req);
        
        Box::pin(async move {
            let state = req.app_data::<web::Data<LovedState>>().unwrap();

            if let Some(session) = req.cookie("session") {
                return Ok(Session {
                    user: FullUser::from_session(session.value(), &state.db_pool).await?
                })
            }

            Err(LovedError::Unauthorized)
        })
    }
}