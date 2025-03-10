use std::env;
use dotenvy::dotenv;
use handling::{HandlerRegistry, TaskManager};

pub mod queues;
pub mod tasks;
pub mod handling;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("A proper environmental file has not been found");
    
    let mut queue_registry = HandlerRegistry::new();
    let task_registry = TaskManager::new();
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    queue_registry.register_handler(queues::users::UserUpdateQueueHandler);
    queue_registry.start_all(&redis_url).await?;

    //task_registry
    task_registry.start().await?;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}