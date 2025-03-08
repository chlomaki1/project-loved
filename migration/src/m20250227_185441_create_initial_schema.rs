use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string(Users::Username))
                    .col(
                        ColumnDef::new(Users::Country)
                            .char_len(2)
                    )
                    .col(boolean(Users::Banned))
                    .col(timestamp(Users::ApiFetchedAt))
                    .col(json(Users::Tokens).default("{}"))
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(pk_auto(Roles::Id))
                    .col(string(Roles::Name))
                    .col(small_integer(Roles::Type).default(0))
                    .col(big_integer(Roles::Permissions).default(0))
                    .col(boolean(Roles::HasGamemode).default(false))
                    .to_owned()
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RoleAssignments::Table)
                    .if_not_exists()
                    .col(integer(RoleAssignments::UserId))
                    .col(integer(RoleAssignments::RoleId))
                    .col(tiny_integer(RoleAssignments::GameMode))
                    .col(boolean(RoleAssignments::Alumni))
                    .primary_key(
                        Index::create()
                            .col(RoleAssignments::UserId)
                            .col(RoleAssignments::RoleId)
                            .col(RoleAssignments::GameMode)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_assignments_user")
                            .from(RoleAssignments::Table, RoleAssignments::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_assignments_role")
                            .from(RoleAssignments::Table, RoleAssignments::RoleId)
                            .to(Roles::Table, Roles::Id)
                    )
                    .to_owned()
            )
            .await?;

        // Beatmapsets table
        manager
            .create_table(
                Table::create()
                    .table(Beatmapsets::Table)
                    .if_not_exists()
                    .col(pk_auto(Beatmapsets::Id))
                    .col(timestamp(Beatmapsets::ApiFetchedAt))
                    .col(string(Beatmapsets::Artist))
                    .col(integer(Beatmapsets::CreatorId))
                    .col(string(Beatmapsets::CreatorName))
                    .col(timestamp_null(Beatmapsets::DeletedAt))
                    .col(integer(Beatmapsets::FavoriteCount))
                    .col(integer(Beatmapsets::PlayCount))
                    .col(tiny_integer(Beatmapsets::RankedStatus))
                    .col(timestamp(Beatmapsets::SubmittedAt))
                    .col(string(Beatmapsets::Title))
                    .col(timestamp(Beatmapsets::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_beatmapsets_creator")
                            .from(Beatmapsets::Table, Beatmapsets::CreatorId)
                            .to(Users::Table, Users::Id)
                    )
                    .to_owned()
            )
            .await?;

        // Beatmaps table
        manager
            .create_table(
                Table::create()
                    .table(Beatmaps::Table)
                    .if_not_exists()
                    .col(pk_auto(Beatmaps::Id))
                    .col(integer(Beatmaps::BeatmapsetId))
                    .col(decimal(Beatmaps::Bpm))
                    .col(integer(Beatmaps::CreatorId))
                    .col(timestamp_null(Beatmaps::DeletedAt))
                    .col(tiny_integer(Beatmaps::GameMode))
                    .col(tiny_integer_null(Beatmaps::KeyCount))
                    .col(integer(Beatmaps::PlayCount))
                    .col(tiny_integer(Beatmaps::RankedStatus))
                    .col(decimal(Beatmaps::StarRating))
                    .col(integer(Beatmaps::TotalLength))
                    .col(string(Beatmaps::Version))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_beatmaps_beatmapset")
                            .from(Beatmaps::Table, Beatmaps::BeatmapsetId)
                            .to(Beatmapsets::Table, Beatmapsets::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_beatmaps_creator")
                            .from(Beatmaps::Table, Beatmaps::CreatorId)
                            .to(Users::Table, Users::Id)
                    )
                    .to_owned()
            )
            .await?;

        // Submissions table
        manager
            .create_table(
                Table::create()
                    .table(Submissions::Table)
                    .if_not_exists()
                    .col(pk_auto(Submissions::Id))
                    .col(integer(Submissions::SubmitterId))
                    .col(integer(Submissions::BeatmapsetId))
                    .col(timestamp(Submissions::SubmittedAt))
                    .col(tiny_integer(Submissions::GameMode))
                    .col(string(Submissions::Reason))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submissions_submitter")
                            .from(Submissions::Table, Submissions::SubmitterId)
                            .to(Users::Table, Users::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submissions_beatmapset")
                            .from(Submissions::Table, Submissions::BeatmapsetId)
                            .to(Beatmapsets::Table, Beatmapsets::Id)
                    )
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order of creation
        manager
            .drop_table(Table::drop().table(Submissions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Beatmaps::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Beatmapsets::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(RoleAssignments::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Country,
    Banned,
    ApiFetchedAt,
    Tokens,
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    Name,
    Type,
    Permissions,
    HasGamemode,
}

#[derive(DeriveIden)]
enum RoleAssignments {
    Table,
    UserId,
    RoleId,
    GameMode,
    Alumni,
}

#[derive(DeriveIden)]
enum Beatmapsets {
    Table,
    Id,
    ApiFetchedAt,
    Artist,
    CreatorId,
    CreatorName,
    DeletedAt,
    FavoriteCount,
    PlayCount,
    RankedStatus,
    SubmittedAt,
    Title,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Beatmaps {
    Table,
    Id,
    BeatmapsetId,
    Bpm,
    CreatorId,
    DeletedAt,
    GameMode,
    KeyCount,
    PlayCount,
    RankedStatus,
    StarRating,
    TotalLength,
    Version,
}

#[derive(DeriveIden)]
enum Submissions {
    Table,
    Id,
    SubmitterId,
    BeatmapsetId,
    SubmittedAt,
    GameMode,
    Reason,
}