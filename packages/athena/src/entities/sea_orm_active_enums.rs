//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "rating_type")]
pub enum RatingType {
    #[sea_orm(string_value = "submission")]
    Submission,
    #[sea_orm(string_value = "review")]
    Review,
}
