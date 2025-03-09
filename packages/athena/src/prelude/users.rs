use athena_macros::generate_display;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, ActiveModelTrait, QueryFilter,};
use crate::{entities::{role_assignments, roles, users}, errors::AthenaError};
use super::roles::DisplayRole;

generate_display! {
    #[display(users::Model)]
    DisplayUser {
        roles = Vec<DisplayRole>: Vec::new()
    }
}

impl DisplayUser {
    pub async fn obtain_roles(&mut self, conn: &sea_orm::DatabaseConnection) -> Result<(), DbErr> {
        if let Ok(roles) = get_user_roles(self.base.id, conn).await {
            self.roles = roles.into_iter().map(|r| DisplayRole::new(r)).collect();
        }

        Ok(())
    }
}

pub struct FullUser {
    pub base: users::Model,
    pub roles: Vec<roles::Model>
}

impl FullUser {
    async fn fetch(user_id: i32, conn: &sea_orm::DatabaseConnection) -> Result<Self, AthenaError> {
        let base = users::Entity::find_by_id(user_id)
            .one(conn)
            .await?;

        if let Some(base) = base {
            Ok(FullUser { base, roles: get_user_roles(user_id, conn).await.unwrap_or(Vec::new())})
        } else {
            Err(AthenaError::ModelNotFound("User".to_string()))
        }
    }

    async fn create(user: users::ActiveModel, conn: &sea_orm::DatabaseConnection) -> Result<Self, DbErr> {
        let base = user.insert(conn).await?;

        Ok(FullUser { base, roles: Vec::new() })
    }

    fn into_display(self) -> DisplayUser {
        DisplayUser { base: self.base, roles: self.roles.into_iter().map(|r| DisplayRole::new(r)).collect() }
    }
}

async fn get_user_roles(user_id: i32, conn: &sea_orm::DatabaseConnection) -> Result<Vec<roles::Model>, DbErr> {
    let role_tuple = role_assignments::Entity::find()
        .filter(role_assignments::Column::UserId.eq(user_id))
        .find_also_related(roles::Entity)
        .all(conn)
        .await?;

    if role_tuple.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(role_tuple.into_iter().map(|(_, role)| role.unwrap()).collect())
    }
}