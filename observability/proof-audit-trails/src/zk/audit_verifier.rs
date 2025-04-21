use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, SynthesisError,
};

#[derive(Clone)]
struct AuditCircuit {
    // Public inputs
    root_hash: Option<Fr>,
    leaf_hash: Option<Fr>,
    
    // Private inputs
    path_elements: Vec<Option<Fr>>,
    path_indices: Vec<Option<Fr>>,
}

impl ConstraintSynthesizer<Fr> for AuditCircuit {
    fn generate_constraints(
        self,
        cs: &mut ConstraintSystem<Fr>,
    ) -> Result<(), SynthesisError> {
        let root = RootVar::new_input(cs.ns(|| "root"), || {
            self.root_hash.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        let leaf = LeafVar::new_input(cs.ns(|| "leaf"), || {
            self.leaf_hash.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        let path = PathVar::new_witness(cs.ns(|| "path"), || {
            Ok((self.path_elements, self.path_indices))
        })?;
        
        path.verify_membership(cs.ns(|| "verify"), &root, &leaf)?;
        
        Ok(())
    }
}

pub fn generate_audit_proof(
    pk: &ProvingKey<Bls12_381>,
    root_hash: [u8; 32],
    leaf_hash: [u8; 32],
    path: &MerkleProof,
) -> Result<Proof<Bls12_381>, ZkError> {
    let circuit = AuditCircuit {
        root_hash: Some(Fr::from_le_bytes_mod_order(&root_hash)),
        leaf_hash: Some(Fr::from_le_bytes_mod_order(&leaf_hash)),
        path_elements: path.path.iter()
            .map(|(hash, _)| Some(Fr::from_le_bytes_mod_order(hash)))
            .collect(),
        path_indices: path.path.iter()
            .map(|(_, is_right)| Some(Fr::from(*is_right as u8)))
            .collect(),
    };
    
    Groth16::<Bls12_381>::prove(pk, circuit, &mut rand::thread_rng())
}

pub fn verify_audit_proof(
    vk: &VerifyingKey<Bls12_381>,
    proof: &Proof<Bls12_381>,
    root_hash: [u8; 32],
) -> Result<bool, ZkError> {
    let public_inputs = [Fr::from_le_bytes_mod_order(&root_hash)];
    Ok(Groth16::verify(vk, &public_inputs, proof)?)
}
