use athena_macros::generate_display;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};

use crate::{entities::{beatmaps, beatmapsets}, errors::AthenaError};

generate_display! {
    #[display(beatmaps::Model)]
    DisplayBeatmap {
        id = i32: base.id
    }
}

generate_display! {
    #[display(beatmapsets::Model)]
    DisplayBeatmapset {
        id = i32: base.id
    }
}

pub struct FullBeatmap {
    pub base: beatmaps::Model,
    pub beatmapset: DisplayBeatmapset
}

impl FullBeatmap {
    pub async fn create(beatmap: beatmaps::ActiveModel, conn: &sea_orm::DatabaseConnection) -> Result<Self, DbErr> {
        let base = beatmap.insert(conn).await?;
        let beatmapset = DisplayBeatmapset::new(beatmapsets::Entity::find_by_id(base.beatmapset_id).one(conn).await?.unwrap());
        Ok(FullBeatmap { base, beatmapset })
    }

    pub async fn create_all(beatmaps: Vec<beatmaps::ActiveModel>, conn: &sea_orm::DatabaseConnection) -> Result<Vec<Self>, DbErr> {
        let mut full_beatmaps = Vec::new();
        
        for beatmap in beatmaps {
            full_beatmaps.push(FullBeatmap::create(beatmap, conn).await?);
        }

        Ok(full_beatmaps)
    }

    pub async fn fetch(beatmap_id: i32, conn: &sea_orm::DatabaseConnection) -> Result<Self, AthenaError> {
        let base = beatmaps::Entity::find_by_id(beatmap_id)
            .one(conn)
            .await?;

        if let Some(base) = base {
            let beatmapset = DisplayBeatmapset::new(beatmapsets::Entity::find_by_id(base.beatmapset_id).one(conn).await?.unwrap());
            Ok(FullBeatmap { base, beatmapset })
        } else {
            Err(AthenaError::ModelNotFound("beatmap"))
        }
    }
}

pub struct FullBeatmapset {
    pub base: beatmapsets::Model,
    pub beatmaps: Vec<FullBeatmap>
}

impl FullBeatmapset {
    pub async fn create(beatmapset: beatmapsets::ActiveModel, beatmaps: Vec<beatmaps::ActiveModel>, conn: &sea_orm::DatabaseConnection) -> Result<Self, DbErr> {
        let base = beatmapset.insert(conn).await?;
        let beatmaps = FullBeatmap::create_all(beatmaps, conn).await?;
        
        Ok(FullBeatmapset { base, beatmaps })
    }
}
