use std::future::Future;

pub mod users;
pub mod roles;
pub mod submissions;
pub mod beatmaps;

pub trait AsyncFromDatabase<T>: Sized {
    fn from_async(value: T, conn: &sea_orm::DatabaseConnection) -> impl Future<Output = Self>;
}