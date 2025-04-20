use sgx_isa::{Report, TargetInfo};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct EdgeAttestation {
    device_id: [u8; 32],
    chain_client: BlockchainClient,
}

impl EdgeAttestation {
    pub async fn generate_remote_attestation(
        &self,
        challenge: &[u8],
    ) -> Result<AttestationReport> {
        let report_data = self.create_report_data(challenge).await?;
        let hardware_report = self.get_hw_quote(&report_data)?;
        
        let sig = self.sign_report(&hardware_report)?;
        Ok(AttestationReport {
            device_id: self.device_id,
            hardware_report,
            signature: sig,
        })
    }

    async fn create_report_data(&self, challenge: &[u8]) -> Result<[u8; 64]> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.device_id)?;
        mac.update(challenge);
        Ok(mac.finalize().into_bytes().into())
    }

    fn sign_report(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.chain_client
            .sign_with_device_key(data)
            .await
            .map_err(|e| Error::CryptoError(e))
    }
}
