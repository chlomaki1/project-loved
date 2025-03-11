use athena_macros::generate_display;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, TryIntoModel};
use crate::{entities::{self, role_assignments, roles, users}, errors::AthenaError};
use super::roles::{DisplayRole, FullRole};

generate_display! {
    #[display(users::Model)]
    DisplayUser {
        roles = Vec<DisplayRole>: Vec::new()
    }

    pub fn default() -> Self {
        DisplayUser {
            base: <users::ActiveModel as ActiveModelTrait>::default()
                .try_into_model()
                .unwrap(),
            roles: Vec::new()
        }
    }
}

impl DisplayUser {
    pub async fn obtain_roles(&mut self, conn: &sea_orm::DatabaseConnection) -> Result<(), DbErr> {
        if let Ok(roles) = get_user_roles(self.base.id, conn).await {
            self.roles = roles.into_iter().map(|r| r.into_display()).collect();
        }

        Ok(())
    }
}

pub struct FullUser {
    pub base: users::Model,
    pub roles: Vec<FullRole>
}

impl FullUser {
    pub async fn create(user: users::ActiveModel, conn: &sea_orm::DatabaseConnection) -> Result<Self, AthenaError> {
        let base = user.insert(conn).await?;

        Ok(FullUser { base, roles: Vec::new() })
    }

    pub async fn fetch(user_id: i32, conn: &sea_orm::DatabaseConnection) -> Result<Self, AthenaError> {
        let base = users::Entity::find_by_id(user_id)
            .one(conn)
            .await?;

        if let Some(base) = base {
            Ok(FullUser {
                base: base.clone(),
                roles: get_user_roles(base.id, conn).await.unwrap_or(Vec::new())
            })
        } else {
            Err(AthenaError::ModelNotFound("User".to_string()))
        }
    }

    pub async fn update(model: users::ActiveModel, conn: &sea_orm::DatabaseConnection) -> Result<Self, AthenaError> {
        let base = model.update(conn).await?;

        Ok(FullUser {
            base: base.clone(),
            roles: get_user_roles(base.id, conn).await.unwrap_or(Vec::new())
        })
    }

    pub fn from(model: users::Model) -> Self {
        FullUser {
            base: model.clone(),
            roles: Vec::new()
        }
    }

    pub fn into_display(self) -> DisplayUser {
        DisplayUser { base: self.base, roles: self.roles.into_iter().map(|r| r.into_display()).collect() }
    }
}

async fn get_user_roles(user_id: i32, conn: &sea_orm::DatabaseConnection) -> Result<Vec<FullRole>, DbErr> {
    let role_tuple = role_assignments::Entity::find()
        .filter(role_assignments::Column::UserId.eq(user_id))
        .find_also_related(roles::Entity)
        .all(conn)
        .await?;

    if role_tuple.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(role_tuple.into_iter().map(|(_, role)| FullRole::from(role.unwrap())).collect())
    }
}