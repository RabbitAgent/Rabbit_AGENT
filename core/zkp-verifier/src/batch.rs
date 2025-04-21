impl ParallelProver {
    pub fn parallel_prove(
        &self, 
        circuits: Vec<Box<dyn Circuit>>,
        params: &Parameters
    ) -> Vec<Proof> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus::get())
            .build()
            .unwrap();

        pool.install(|| {
            circuits.into_par_iter()
                .map(|circuit| {
                    let prover = Prover::new(params.clone());
                    prover.generate_proof(*circuit)
                })
                .collect()
        })
    }
}
