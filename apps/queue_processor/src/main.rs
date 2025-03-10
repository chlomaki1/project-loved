use std::env;
use dotenvy::dotenv;
use handling::{HandlerRegistry, TaskManager};

mod handling;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("A proper environmental file has not been found");
    
    let mut queue_registry = HandlerRegistry::new();
    let mut task_registry = TaskManager::new();
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    queue_registry
        .start_all(&redis_url)
        .await?;

    task_registry
        .start()
        .await?;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}