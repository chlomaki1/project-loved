use actix_web::{get, web, Responder};
use athena::{entities::submissions, prelude::submissions::FullSubmission};
use sea_orm::{EntityTrait, QueryOrder};
use crate::{extractors::pagination::Pagination, state::LovedState};

#[get("/")]
pub async fn index(
    state: web::Data<LovedState>,
    pagination: Pagination<FullSubmission, 100>,
) -> impl Responder {
    let a = submissions::Entity::find()
        .order_by_desc(submissions::Column::Id)
        .limit(pagination.limit)
        .offset(pagination.get_page_offset())
        .all(&state.db_pool)
        .await;

    Ok(())
}