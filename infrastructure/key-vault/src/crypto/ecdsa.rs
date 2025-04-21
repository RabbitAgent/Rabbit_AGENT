impl ThresholdEcdsa {
    pub fn sign_distributed(
        shares: &[KeyShare],
        message: &[u8],
        threshold: usize
    ) -> Result<Signature> {
        let mut rng = rand::thread_rng();
        let (commitments, witnesses) = frost::preprocess(shares, threshold, &mut rng)?;
        
        let binding = frost::compute_binding_factor(commitments, message);
        let signature_shares = frost::sign(message, shares, witnesses, binding)?;
        
        frost::aggregate(commitments, signature_shares)
    }
}
