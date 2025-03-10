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
                    .table(SubmissionRatings::Table)
                    .if_not_exists()
                    .col(pk_auto(SubmissionRatings::Id))
                    .col(integer(SubmissionRatings::SubmissionId))
                    .col(integer(SubmissionRatings::ReviewerId))
                    .col(small_integer(SubmissionRatings::Value))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_ratings_submission")
                            .from(SubmissionRatings::Table, SubmissionRatings::SubmissionId)
                            .to(Submissions::Table, Submissions::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_ratings_reviewer")
                            .from(SubmissionRatings::Table, SubmissionRatings::ReviewerId)
                            .to(Users::Table, Users::Id)
                    )
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SubmissionReviewRatings::Table)
                    .if_not_exists()
                    .col(pk_auto(SubmissionReviewRatings::Id))
                    .col(integer(SubmissionReviewRatings::ReviewId))
                    .col(integer(SubmissionReviewRatings::ReviewerId))
                    .col(small_integer(SubmissionReviewRatings::Value))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_review_ratings_review")
                            .from(SubmissionReviewRatings::Table, SubmissionReviewRatings::ReviewId)
                            .to(SubmissionReviews::Table, SubmissionReviews::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_review_ratings_reviewer")
                            .from(SubmissionReviewRatings::Table, SubmissionReviewRatings::ReviewerId)
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
            .drop_table(Table::drop().table(SubmissionRatings::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(SubmissionReviewRatings::Table).to_owned())
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
pub enum SubmissionRatings {
    Table,
    Id,
    SubmissionId,
    ReviewerId,
    Value
}

#[derive(DeriveIden)]
pub enum SubmissionReviewRatings {
    Table,
    Id,
    ReviewId,
    ReviewerId,
    Value
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