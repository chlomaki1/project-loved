use redis::AsyncCommands;
use std::{collections::HashMap, sync::Arc, error::Error};
use tokio;
use async_trait::async_trait;

#[async_trait]
pub trait QueueHandler: Send + Sync {
    /// Return the queue name this handler is responsible for.
    fn queue_name(&self) -> &'static str;

    /// Process a message asynchronously.
    async fn handle_message(&self, message: String) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait ScheduledTask: Send + Sync {
    /// Returns the cron expression that defines when the task should run.
    fn schedule(&self) -> &'static str;

    /// Executes the scheduled task asynchronously.
    async fn execute(&self) -> Result<(), Box<dyn Error>>;
}

pub struct HandlerRegistry {
    // Key is the queue name, value is the handler.
    handlers: HashMap<String, Arc<dyn QueueHandler>>,
}

impl HandlerRegistry {
    /// Create a new, empty registry.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register a handler for a given queue.
    pub fn register_handler<H: QueueHandler + 'static>(&mut self, handler: H) {
        self.handlers
            .insert(handler.queue_name().to_string(), Arc::new(handler));

    }

    /// Start a listener task for each registered handler.
    ///
    /// Each task will connect to Redis, and continuously poll its assigned queue
    /// (using a blocking pop with timeout) to process new messages.
    pub async fn start_all(&self, redis_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        for (queue_name, handler) in self.handlers.iter() {
            let redis_url = redis_url.to_string();
            let queue = queue_name.clone();
            let handler = Arc::clone(handler);

            // Spawn a task for each queue handler.
            tokio::spawn(async move {
                // Create a new Redis client and async connection.
                let client = redis::Client::open(redis_url.as_str())
                    .expect("Failed to create Redis client");

                let mut con = client
                    .get_async_connection()
                    .await
                    .expect("Failed to get Redis connection");

                loop {
                    // Use BRPOP with a timeout (e.g., 5 seconds).
                    let result: Option<(String, String)> = match con.brpop(queue.as_str(), 5).await {
                        Ok(res) => res,
                        Err(e) => {
                            eprintln!("Error on BRPOP for queue {}: {}", queue, e);
                            continue;
                        }
                    };

                    if let Some((_queue, message)) = result {
                        if let Err(e) = handler.handle_message(message).await {
                            eprintln!("Error handling message in {}: {}", queue, e);
                        }
                    } else {
                        // Timeout reached – you could check for shutdown signals here.
                        println!("No message on '{}' queue, retrying...", queue);
                    }
                }
            });
        }
        Ok(())
    }
}

pub struct TaskManager {
    scheduled_tasks: Vec<Arc<dyn ScheduledTask>>,
}


impl TaskManager {
    /// Creates a new TaskManager.
    pub fn new(redis_url: String) -> Self {
        Self {
            scheduled_tasks: Vec::new(),
        }
    }

    /// Registers a scheduled task.
    pub fn register_scheduled_task<T: ScheduledTask + 'static>(&mut self, task: T) {
        self.scheduled_tasks.push(Arc::new(task));
    }

    /// Starts both the queue handlers and the scheduled tasks.
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize the cron scheduler.
        let scheduler = JobScheduler::new().await?;
        
        for task in self.scheduled_tasks {
            let cron_expr = task.schedule();
            let task_clone = Arc::clone(&task);

            let job = Job::new_async(cron_expr, move |_uuid, _l| {
                let task_clone = Arc::clone(&task_clone);
                Box::pin(async move {
                    if let Err(e) = task_clone.execute().await {
                        eprintln!("Error executing scheduled task: {}", e);
                    }
                })
            })?;
            
            scheduler.add(job).await?;
        }

        scheduler.start().await?;
    }
}