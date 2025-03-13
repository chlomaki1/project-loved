use actix_web::{get, web, Responder};
use athena::{entities::submissions, prelude::{submissions::{DisplaySubmission, FullSubmission}, AsyncFromDatabase}};
use sea_orm::{EntityTrait, QueryOrder, QuerySelect};
use crate::{extractors::pagination::Pagination, state::LovedState};

#[get("/")]
pub async fn index(
    state: web::Data<LovedState>,
    pagination: Pagination<DisplaySubmission, 100>,
) -> impl Responder {
    pagination
        .provide(|p: &Pagination<DisplaySubmission, 100>| {
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

                Ok(submissions.into_iter().map(|s| s.into_display()).collect())
            }
        })
        .await?
        .respond()
}