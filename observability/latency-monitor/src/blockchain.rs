use ethers::{
    core::types::U256,
    prelude::*,
};
use serde::{Serialize, Serializer};

abigen!(LatencyOracleContract, "artifacts/LatencyOracle.json");

#[derive(Serialize)]
pub struct LatencyProof {
    #[serde(serialize_with = "serialize_duration")]
    pub p99: Duration,
    #[serde(serialize_with = "serialize_duration")]
    pub max: Duration,
    pub signature: Vec<u8>,
    pub prediction_model_hash: [u8; 32],
}

fn serialize_duration<S>(dur: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(dur.as_millis() as u64)
}

impl LatencyProof {
    pub async fn submit_to_chain(
        &self,
        contract: &LatencyOracleContract<Provider<Http>>,
    ) -> Result<TransactionReceipt> {
        let tx = contract.submit_latency_metrics(
            U256::from(self.p99.as_millis()),
            U256::from(self.max.as_millis()),
            self.prediction_model_hash.into(),
            self.signature.clone(),
        );
        
        tx.send().await?.await?.ok_or_else(|| eyre::eyre!("Tx failed"))
    }
}
