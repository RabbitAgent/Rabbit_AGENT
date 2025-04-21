impl AuditSigner {
    pub fn sign_entry(&self, entry: &mut AuditEntry) {
        let context = SignatureContext::new(b"RabbitAgentAudit");
        let sig = self.keypair.sign(
            &context,
            &entry.data_hash,
            &entry.parties,
        );
        
        entry.zk_proof = Some(sig.to_bytes().to_vec());
    }

    pub fn verify_entry(&self, entry: &AuditEntry) -> Result<(), SignatureError> {
        let context = SignatureContext::new(b"RabbitAgentAudit");
        let sig = Signature::from_bytes(&entry.zk_proof.as_ref().unwrap())?;
        
        self.keypair.verify(
            &context,
            &entry.data_hash,
            &entry.parties,
            &sig
        )
    }
}
