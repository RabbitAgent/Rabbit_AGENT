use ed25519_dalek::{Signer, Verifier};

impl SignedLatencyReport {
    pub fn sign_metrics(keypair: &Keypair, metrics: &LatencyStats) -> Self {
        let mut buf = Vec::new();
        buf.extend(metrics.mean.to_le_bytes());
        buf.extend(metrics.std_dev.to_le_bytes());
        buf.extend(metrics.min.to_le_bytes());
        buf.extend(metrics.max.to_le_bytes());
        
        Self {
            signature: keypair.sign(&buf).to_bytes().to_vec(),
            public_key: keypair.public.to_bytes().to_vec(),
            metrics: metrics.clone(),
        }
    }
    
    pub fn verify(&self) -> bool {
        let public = PublicKey::from_bytes(&self.public_key).unwrap();
        let sig = Signature::from_bytes(&self.signature).unwrap();
        
        let mut buf = Vec::new();
        buf.extend(self.metrics.mean.to_le_bytes());
        buf.extend(self.metrics.std_dev.to_le_bytes());
        buf.extend(self.metrics.min.to_le_bytes());
        buf.extend(self.metrics.max.to_le_bytes());
        
        public.verify(&buf, &sig).is_ok()
    }
}
