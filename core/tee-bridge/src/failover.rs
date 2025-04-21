impl EnclaveMonitor {
    pub async fn watch_enclave(&self, enclave_id: EnclaveId) -> Result<(), TeeError> {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            
            if !self.check_heartbeat(enclave_id).await? {
                self.relaunch_enclave(enclave_id).await?;
                self.rotate_attestation(enclave_id).await?;
            }
        }
    }
}
