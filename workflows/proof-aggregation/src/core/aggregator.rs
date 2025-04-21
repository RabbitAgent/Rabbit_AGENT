use ark_groth16::{Groth16, Proof};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use rayon::prelude::*;

#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct AggregatedProof {
    pub circuit_id: [u8; 32],
    pub public_inputs: Vec<Vec<u8>>,
    pub batch_proof: Proof<Bls12_381>,
    pub validity_window: u64,
}

impl AggregatedProof {
    pub fn new(circuit_id: [u8; 32], proofs: Vec<Proof<Bls12_381>>) -> Self {
        let public_inputs = proofs.iter()
            .map(|p| p.inputs.clone())
            .collect();
            
        let batch_proof = Groth16::aggregate_proofs::<Bls12_381>(proofs)
            .expect("Valid proof aggregation");

        Self {
            circuit_id,
            public_inputs,
            batch_proof,
            validity_window: Utc::now().timestamp() + 300, // 5-minute window
        }
    }

    pub fn verify_batch(&self, vk: &VerifyingKey<Bls12_381>) -> Result<(), VerificationError> {
        Groth16::verify_aggregate_proof(vk, &self.batch_proof, &self.public_inputs)
    }
}

// GPU-accelerated batch processing
pub fn parallel_aggregation(
    proofs: Vec<Proof<Bls12_381>>,
    circuit_ids: Vec<[u8; 32]>
) -> Vec<AggregatedProof> {
    proofs.par_iter()
        .zip(circuit_ids.par_iter())
        .map(|(proof, cid)| AggregatedProof::new(*cid, vec![proof.clone()]))
        .collect()
}
