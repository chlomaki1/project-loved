use actix_web::{get, post, web, Responder};
use athena::{entities::submissions, prelude::submissions::FullSubmission};
use sea_orm::{EntityTrait, QueryOrder, QuerySelect};
use serde_json::json;
use crate::{extractors::pagination::Pagination, state::LovedState};

#[get("/")]
pub async fn index(
    state: web::Data<LovedState>,
    pagination: Pagination<100>,
) -> impl Responder {
    pagination
        .provide(|p: &Pagination<100>| {
            let limit = p.limit.try_into().unwrap();
            let offset = p.get_page_offset().into();
            let db_pool = state.db_pool.clone();

            async move {
                let submissions = FullSubmission::find(&db_pool, |query| {
                    query
                        .order_by_desc(submissions::Column::Id)
                        .limit(Some(limit))
                        .offset(Some(offset))
                })
                .await?;

                Ok(submissions.into_iter().map(|s| json!({ "hi": 1 })).collect())
            }
        })
        .await?
        .respond()
}

/*#[post("/")]
pub async fn create(
    state: web::Data<LovedState>,
    payload: web::Json<FullSubmission>,
) -> impl Responder {
    
}*/