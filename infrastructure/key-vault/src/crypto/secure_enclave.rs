impl EnclaveKeyManager {
    pub fn generate_attested_key(&self, report_data: &[u8]) -> Result<KeyMaterial> {
        let sealed_key = unsafe {
            ecall_generate_key(
                self.eid,
                report_data.as_ptr(),
                report_data.len()
            )
        }?;
        
        Ok(KeyMaterial {
            ciphertext: sealed_key,
            attestation: self.get_remote_attestation()?,
        })
    }
}
