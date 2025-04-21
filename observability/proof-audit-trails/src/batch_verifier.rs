impl BatchVerifier {
    pub fn verify_batch_groth16(
        proofs: &[Proof<Bls12_381>],
        roots: &[[u8; 32]],
        vk: &VerifyingKey,
    ) -> bool {
        let mut batch = vec![];
        
        for (proof, root) in proofs.iter().zip(roots) {
            let public_input = Fr::from_le_bytes_mod_order(root);
            batch.push((proof, vec![public_input]));
        }
        
        Groth16::verify_batch(&vk, &mut rand::thread_rng(), batch).is_ok()
    }
}
