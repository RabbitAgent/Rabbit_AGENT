pub trait ProofSystemBackend {
    fn setup(&self, circuit: &dyn Circuit) -> Result<Parameters>;
    fn prove(&self, params: &Parameters, circuit: &dyn Circuit) -> Result<Proof>;
    fn verify(&self, vk: &VerifyingKey, proof: &Proof, inputs: &[Fr]) -> Result<bool>;
}

pub struct Groth16Backend;
pub struct PlonkBackend;
pub struct Halo2Backend;
