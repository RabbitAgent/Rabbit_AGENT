impl KeyRotator {
    pub async fn rotate_keys(&self, schedule: RotationPolicy) -> Result<RotationReport> {
        let mut new_keys = HashMap::new();
        
        for key in self.keystore.list_active_keys().await? {
            if schedule.should_rotate(&key.metadata).await? {
                let new_key = self.keystore.generate(key.spec).await?;
                self.keystore.archive(key.id).await?;
                new_keys.insert(key.id, new_key);
            }
        }
        
        Ok(RotationReport {
            rotated_keys: new_keys,
            timestamp: Utc::now(),
        })
    }
}
