pub async fn generate_state_proof(
    &self,
    block_number: u64,
    storage_key: &[u8],
) -> Result<MerkleProof> {
    let header = self.client.header(block_number).await?;
    let proof = self.client.prove_storage(storage_key, block_number).await?;
    
    Ok(MerkleProof {
        header,
        proof,
    })
}
