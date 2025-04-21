use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::ristretto::CompressedRistretto;
use merlin::Transcript;

pub struct RedactedLogger {
    pc_gens: PedersenGens,
    bp_gens: BulletproofGens,
}

impl RedactedLogger {
    pub fn new() -> Self {
        Self {
            pc_gens: PedersenGens::default(),
            bp_gens: BulletproofGens::new(128, 1),
        }
    }

    pub fn create_redacted_entry(
        &self,
        secret_value: u64,
        redacted_mask: [u8; 32]
    ) -> (CompressedRistretto, RangeProof) {
        let mut transcript = Transcript::new(b"RedactedAudit");
        let (commitment, range_proof) = RangeProof::prove_single(
            &self.bp_gens,
            &self.pc_gens,
            &mut transcript,
            secret_value,
            &redacted_mask,
            64
        ).unwrap();
        
        (commitment, range_proof)
    }

    pub fn verify_redacted_entry(
        &self,
        commitment: &CompressedRistretto,
        proof: &RangeProof,
    ) -> Result<(), ProofError> {
        let mut transcript = Transcript::new(b"RedactedAudit");
        proof.verify_single(
            &self.bp_gens,
            &self.pc_gens,
            &mut transcript,
            commitment,
            64
        )
    }
}
