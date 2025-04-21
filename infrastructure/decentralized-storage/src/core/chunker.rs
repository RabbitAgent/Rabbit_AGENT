use blake3::Hasher;
use rayon::prelude::*;

pub struct AdaptiveChunker {
    min_chunk_size: usize,
    max_chunk_size: usize,
    target_chunks: usize,
}

impl AdaptiveChunker {
    pub fn new(data: &[u8]) -> Vec<Chunk> {
        let avg_size = data.len() / self.target_chunks;
        let mut chunks = Vec::new();
        let mut hasher = Hasher::new();
        
        data.par_chunks(avg_size)
            .with_min_len(self.min_chunk_size)
            .with_max_len(self.max_chunk_size)
            .for_each(|c| {
                let mut h = hasher.reset();
                h.update(c);
                let digest = h.finalize();
                
                chunks.push(Chunk {
                    data: c.to_vec(),
                    digest: digest.into(),
                    proof: None,
                });
            });
        
        chunks
    }
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub data: Vec<u8>,
    pub digest: [u8; 32],
    pub proof: Option<StorageProof>,
}
