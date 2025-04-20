impl TaskMonitor {
    pub async fn watch_task(&self, receipt: AllocationReceipt) -> TaskOutcome {
        let mut retries = 0;
        
        loop {
            match self.check_completion(&receipt).await {
                Ok(result) => return result,
                Err(_) if retries < MAX_RETRIES => {
                    self.reschedule_failed_task(&receipt).await;
                    retries += 1;
                }
                Err(e) => return TaskOutcome::Failed(e),
            }
        }
    }
}
