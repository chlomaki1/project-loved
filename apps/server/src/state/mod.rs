use athena::environment::LovedEnvironment;
use rosu_v2::error::OsuError;
use rosu_v2::prelude::Scopes;
use rosu_v2::Osu;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr, ExecResult, Statement};
use settings::LovedSettingsManager;
use tokio::sync::OnceCell;

pub mod settings;

static OSU_CLIENT: OnceCell<Osu> = OnceCell::const_new();

#[derive(Clone)]
pub struct LovedState {
    pub env: LovedEnvironment,
    pub settings: LovedSettingsManager,
    pub db_pool: DatabaseConnection,
    pub redis_pool: redis::Client
}

impl LovedState {
    pub async fn new() -> Self {
        // Get the current application environment
        let env = LovedEnvironment::new();
        let mut options = ConnectOptions::new(&env.database_url);

        options.max_connections(10)
            .min_connections(5);

        unsafe {
            // Has to be unsafe because `Osu` doesn't implement `Debug`
            // ...or really anything.
            OSU_CLIENT.set(
                Osu::builder()
                    .url(env.get::<String>("OSU_URL").unwrap())
                    .client_id(env.get::<u64>("OSU_CLIENT_ID").unwrap())
                    .client_secret(env.get::<String>("OSU_CLIENT_SECRET").unwrap())
                    .build()
                    .await
                    .expect("Failed to create osu! API client")
            ).unwrap_unchecked();
        }

        LovedState {
            env: env.clone(),
            settings: LovedSettingsManager::new(),
            db_pool: Database::connect(options)
                .await
                .expect("Failed to connect to database"),
            redis_pool: redis::Client::open(&*env.redis_url.clone()).unwrap()
        }
    }

    pub async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        self.db_pool.execute(stmt).await
    }

    pub async fn execute_osu<T>(&self, func: impl Fn(&Osu) -> Result<T, OsuError>) -> Result<T, OsuError> {
        func(OSU_CLIENT.get().unwrap())
    }

    pub async fn run(&self, query: &str) -> Result<ExecResult, DbErr> {
        self.db_pool.execute_unprepared(query).await
    }

    pub async fn cache<T>(&self, func: impl Fn(&mut redis::Connection) -> Result<T, redis::RedisError>) -> Result<T, redis::RedisError> {
        let mut con = self.redis_pool.get_connection().unwrap();

        func(&mut con)
    }

    pub async fn get_osu_client(&self, user_token: String, oauth_type: &str) -> Result<Osu, OsuError> {
        Osu::builder()
            .url(self.env.get::<String>("OSU_URL").unwrap())
            .client_id(self.env.get::<u64>("OSU_CLIENT_ID").unwrap())
            .client_secret(self.env.get::<String>("OSU_CLIENT_SECRET").unwrap())
            .with_authorization(user_token, self.env.get::<String>("OSU_REDIRECT_URI").unwrap() + oauth_type, Scopes::default())
            .build()
            .await
    }
}