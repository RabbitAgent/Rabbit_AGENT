impl Auditor {
    pub fn log_operation(&self, event: AuditEvent) -> Result<()> {
        let mut entry = AuditEntry {
            timestamp: SystemTime::now(),
            event,
            principal: self.current_context.principal.clone(),
            device_fingerprint: self.get_hardware_id()?,
            blockchain_proof: self.generate_merkle_proof()?,
        };
        
        self.append_immutable_log(&entry)?;
        self.post_to_blockchain(entry)
    }
    
    fn generate_merkle_proof(&self) -> Result<MerkleProof> {
        let leaf = self.calculate_entry_hash()?;
        self.tree.generate_proof(leaf)
    }
}
