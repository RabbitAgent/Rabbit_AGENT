use bellman::groth16;
use rand::rngs::OsRng;

pub struct ZkProver {
    params: groth16::Parameters<Bls12>,
}

impl ZkProver {
    pub fn new(params: groth16::Parameters<Bls12>) -> Self {
        Self { params }
    }

    pub fn generate_proof<C: Circuit<Fr>>(
        &self,
        circuit: C,
        public_inputs: &[Fr]
    ) -> Result<(groth16::Proof<Bls12>, Vec<Fr>), ProverError> {
        let rng = &mut OsRng;
        groth16::create_random_proof(circuit, &self.params, rng)
            .map(|p| (p, public_inputs.to_vec()))
            .map_err(Into::into)
    }

    pub fn generate_proof_batch<C: Circuit<Fr> + Clone>(
        &self,
        circuits: Vec<C>,
        public_inputs: Vec<Vec<Fr>>
    ) -> Result<Vec<(groth16::Proof<Bls12>, Vec<Fr>)>, ProverError> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()?;

        pool.install(|| {
            circuits.par_iter()
                .zip(public_inputs.par_iter())
                .map(|(circuit, inputs)| {
                    self.generate_proof(circuit.clone(), inputs)
                })
                .collect()
        })
    }
}
