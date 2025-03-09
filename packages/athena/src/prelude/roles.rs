use bitflags::bitflags;
use athena_macros::generate_display;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};
use crate::{entities::roles, errors::AthenaError};

generate_display! {
    #[display(roles::Model)]
    DisplayRole {
        id = i32: base.id
    }
}

pub struct FullRole {
    pub base: roles::Model,
    pub permissions: Permissions
}

impl FullRole {
    pub async fn create(role: roles::ActiveModel, conn: &sea_orm::DatabaseConnection) -> Result<Self, DbErr> {
        let base = role.insert(conn).await?;

        Ok(FullRole { base, permissions: Permissions::empty() })
    }

    pub async fn fetch(role_id: i32, conn: &sea_orm::DatabaseConnection) -> Result<Self, AthenaError> {
        let base = roles::Entity::find_by_id(role_id)
            .one(conn)
            .await?;

        if let Some(base) = base {
            Ok(FullRole {
                base: base.clone(),
                permissions: Permissions::from_bits(base.permissions).unwrap_or(Permissions::empty())
            })
        } else {
            Err(AthenaError::ModelNotFound("Role".to_string()))
        }
    }

    pub fn from(model: roles::Model) -> Self {
        FullRole {
            base: model.clone(),
            permissions: Permissions::from_bits(model.permissions).unwrap_or(Permissions::empty())
        }
    }

    pub async fn update(model: roles::ActiveModel, conn: &sea_orm::DatabaseConnection) -> Result<Self, DbErr> {
        let base = model.update(conn).await?;

        Ok(FullRole {
            base: base.clone(),
            permissions: Permissions::from_bits(base.permissions).unwrap_or(Permissions::empty())
        })
    }

    pub fn into_display(self) -> DisplayRole {
        DisplayRole {
            base: self.base.clone(),
            id: self.base.id
        }
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Permissions: i64 {
        // Administrator permissions
        const ADMIN                     = 1 << 0;
        const MANAGE_ROLES              = 1 << 1;
        const MANAGE_SITE_SETTINGS      = 1 << 2;

        // General team permissions
        const VIEW_ROUNDS               = 1 << 3;
        const MANAGE_ROUNDS             = 1 << 4;
        const MANAGE_PICKS              = 1 << 5;
        const MANAGE_GAMEMODE_PICKS     = 1 << 6;
        const MANAGE_METADATA           = 1 << 7;
        const MANAGE_MODERATION         = 1 << 8;
    }
}
