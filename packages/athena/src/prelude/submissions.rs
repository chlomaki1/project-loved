use athena_macros::generate_display;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, Select};
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
            Err(AthenaError::ModelNotFound("submission"))
        }
    }

    pub async fn find(
        conn: &sea_orm::DatabaseConnection,
        fun: impl FnOnce(Select<submissions::Entity>) -> Select<submissions::Entity>,
    ) -> Result<Vec<Self>, AthenaError> {
        let base = submissions::Entity::find();
        let base = fun(base);
    
        let base = base.all(conn).await?;
        let mut full_submissions = Vec::new();
    
        for submission in base {
            full_submissions.push(FullSubmission::from_async(submission, conn).await);
        }
    
        Ok(full_submissions)
    }

    pub fn into_display(self) -> DisplaySubmission {
        DisplaySubmission { 
            base: self.base.clone(), 
            id: self.base.id,
            submitter: self.submitter.into_display()
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