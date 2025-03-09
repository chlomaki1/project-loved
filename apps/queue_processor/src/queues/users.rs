struct UserUpdateQueueHandler;

#[async_trait::async_trait]
impl QueueHandler for UserUpdateQueueHandler {
    fn queue_name(&self) -> &'static str {
        "loved:queues:user_update"
    }

    async fn handle_message(&self, message: String) -> Result<(), Box<dyn Error>> {
        // TODO: Implement message handling logic
        Ok(())
    }
}