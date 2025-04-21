impl CircuitCache {
    pub fn precompile(&mut self, circuit: Box<dyn Circuit>) -> CircuitHandle {
        let key = self.calculate_circuit_hash(&circuit);
        if !self.cache.contains_key(&key) {
            let compiled = circuit.compile();
            self.cache.insert(key, compiled);
        }
        key
    }
}
