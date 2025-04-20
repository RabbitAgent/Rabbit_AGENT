impl LightClientBridge {
    pub async fn sync_header(&self, chain: ChainType) -> Result<BlockHeader> {
        match chain {
            ChainType::EVM => self.sync_eth_header().await,
            ChainType::Move => self.sync_diem_header().await,
            ChainType::Substrate => self.sync_substrate_header().await,
        }
    }
}
