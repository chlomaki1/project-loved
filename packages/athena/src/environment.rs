use dotenvy::{var, Error};
use std::{fmt::Debug, str::FromStr};

#[derive(Default, Clone)]
pub struct LovedEnvironment {
    pub database_url: String,
    pub redis_url: String,
}

impl LovedEnvironment {
    pub fn new() -> LovedEnvironment {
        let mut env = LovedEnvironment::default();
        env.initialize();

        env
    }

    pub fn initialize(&mut self) {
        self.database_url = self.get("DATABASE_URL").unwrap();
        self.redis_url = self.get("REDIS_URL").unwrap();
    }

    pub fn get<T: FromStr>(&self, key: &str) -> Result<T, Error>
    where
        <T as FromStr>::Err: Debug,
    {
        match var(key) {
            Ok(value) => Ok(value.parse().unwrap()),
            Err(err) => Err(err),
        }
    }

    pub fn get_default<T: FromStr>(&self, key: &str, or_default: T) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        match var(key) {
            Ok(value) => value.parse().unwrap(),
            Err(_) => or_default,
        }
    }
}