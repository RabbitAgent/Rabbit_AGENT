use reed_solomon_erasure::galois_16::ReedSolomon;

pub struct ErasureCoder {
    rs: ReedSolomon,
}

impl ErasureCoder {
    pub fn new(data_shards: usize, parity_shards: usize) -> Self {
        Self {
            rs: ReedSolomon::new(data_shards, parity_shards).unwrap(),
        }
    }

    pub fn encode(&self, data: &[Vec<u8>]) -> Vec<Vec<u8>> {
        let mut shards = data.to_vec();
        self.rs.encode(&mut shards).unwrap();
        shards
    }

    pub fn reconstruct(&self, shards: &mut [Option<Vec<u8>>]) -> Result<()> {
        self.rs.reconstruct(shards).map_err(|e| e.into())
    }
}
