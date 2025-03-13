use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AthenaError {
    #[error("{0}")]
    DbErr(DbErr),

    #[error("Failed to acquire model of type {0}")]
    ModelNotFound(&'static str)
}

impl From<DbErr> for AthenaError {
    fn from(err: DbErr) -> Self {
        AthenaError::DbErr(err)
    }
}
