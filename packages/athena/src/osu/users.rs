use athena_macros::generate_display;
use sea_orm::{ColumnTrait, Condition, DbErr, EntityOrSelect, EntityTrait, JoinType, LoaderTrait, QueryFilter, QuerySelect, Related};
use crate::entities::{role_assignments, roles, users};

use super::roles::DisplayRole;

generate_display! {
    #[display(users::Model)]
    DisplayUser {
        roles = Vec<DisplayRole>: Vec::new()
    }
}

impl DisplayUser {
    pub async fn obtain_roles(&mut self, conn: &sea_orm::DatabaseConnection) -> Result<(), DbErr> {
        let role_tuple = role_assignments::Entity::find()
            .filter(role_assignments::Column::UserId.eq(self.base.id))
            .find_also_related(roles::Entity)
            .one(conn)
            .await?;

        if let Some((_, roles)) = role_tuple {
            self.roles = roles.into_iter().map(|r| DisplayRole::new(r)).collect();
        }

        Ok(())
    }
}