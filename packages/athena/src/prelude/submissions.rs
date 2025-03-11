use athena_macros::generate_display;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};
use crate::{entities::submissions, errors::AthenaError};
use super::{users::{DisplayUser, FullUser}, AsyncFromDatabase};

generate_display! {
    #[display(submissions::Model)]
    DisplaySubmission {
        id = i32: base.id,
        submitter = DisplayUser: DisplayUser::default()
    } 
}

pub struct FullSubmission {
    pub base: submissions::Model,
    pub submitter: FullUser
}

impl FullSubmission {
    pub async fn create(submission: submissions::ActiveModel, conn: &sea_orm::DatabaseConnection) -> Result<Self, AthenaError> {
        let base = submission.insert(conn).await?;
        let submitter = FullUser::fetch(base.submitter_id, conn).await?;
        Ok(FullSubmission { base, submitter })
    }

    pub async fn fetch(submission_id: i32, conn: &sea_orm::DatabaseConnection) -> Result<Self, AthenaError> {
        let base = submissions::Entity::find_by_id(submission_id)
            .one(conn)
            .await?;

        if let Some(base) = base {
            let submitter = FullUser::fetch(base.submitter_id, conn).await?;
            
            Ok(FullSubmission { base, submitter })
        } else {
            Err(AthenaError::ModelNotFound("submission".to_string()))
        }
    }
}

impl AsyncFromDatabase<submissions::Model> for FullSubmission {
    async fn from_async(value: submissions::Model, conn: &sea_orm::DatabaseConnection) -> Self {
        FullSubmission {
            base: value.clone(),
            submitter: FullUser::fetch(value.submitter_id, conn)
            .await
            .unwrap()
        }
    }
}