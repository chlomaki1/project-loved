use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Submissions::Table)
                    .if_not_exists()
                    .col(pk_auto(Submissions::Id))
                    .col(integer(Submissions::BeatmapsetId))
                    .col(integer(Submissions::SubmitterId))
                    .col(small_integer(Submissions::GameMode))
                    .col(timestamp(Submissions::SubmittedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submissions_beatmapset")
                            .from(Submissions::Table, Submissions::BeatmapsetId)
                            .to(Beatmapsets::Table, Beatmapsets::Id)
                    )
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SubmissionReviews::Table)
                    .if_not_exists()
                    .col(pk_auto(SubmissionReviews::Id))
                    .col(integer(SubmissionReviews::SubmissionId))
                    .col(integer(SubmissionReviews::ReviewerId))
                    .col(integer_null(SubmissionReviews::ParentId))
                    .col(small_integer(SubmissionReviews::GameMode).unique_key())
                    .col(text(SubmissionReviews::Content))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_reviews_submission")
                            .from(SubmissionReviews::Table, SubmissionReviews::SubmissionId)
                            .to(Submissions::Table, Submissions::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_reviews_reviewer")
                            .from(SubmissionReviews::Table, SubmissionReviews::ReviewerId)
                            .to(Users::Table, Users::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_reviews_parent")
                            .from(SubmissionReviews::Table, SubmissionReviews::ParentId)
                            .to(SubmissionReviews::Table, SubmissionReviews::Id)
                    )
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ReviewRatings::Table)
                    .if_not_exists()
                    .col(pk_auto(ReviewRatings::Id))
                    .col(enumeration(ReviewRatings::ReviewType, "ReviewType", vec!["submission", "review"]))
                    .col(integer(ReviewRatings::ObjectId))
                    .col(integer(ReviewRatings::ReviewerId))
                    .col(small_integer(ReviewRatings::Value))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_review_ratings_reviewer")
                            .from(ReviewRatings::Table, ReviewRatings::ReviewerId)
                            .to(Users::Table, Users::Id)
                    )
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SubmissionReviews::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Submissions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ReviewRatings::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Submissions {
    Table,
    Id,
    BeatmapsetId,
    SubmitterId,
    GameMode,
    SubmittedAt
}

#[derive(DeriveIden)]
pub enum SubmissionReviews {
    Table,
    Id,
    SubmissionId,
    ReviewerId,
    ParentId,
    GameMode,
    Content
}

#[derive(DeriveIden)]
pub enum ReviewRatings {
    Table,
    Id,
    ReviewType, // This will be of type ReviewType
    ObjectId,   // This will replace SubmissionId and ReviewId
    ReviewerId,
    Value,
}

#[derive(DeriveIden)]
enum Beatmapsets {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id
}