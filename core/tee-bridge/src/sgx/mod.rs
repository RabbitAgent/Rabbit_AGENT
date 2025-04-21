use sgx_urts::SgxEnclave;
use sgx_types::{sgx_enclave_id_t, sgx_status_t};
use serde_bincode::to_vec;

mod attestation;
mod secure_channel;

#[derive(Clone)]
pub struct EnclaveBridge {
    enclave: Arc<SgxEnclave>,
    report_verifier: Arc<dyn AttestationVerifier>,
}

impl EnclaveBridge {
    pub fn init(enclave_path: &str, spid: &[u8]) -> Result<Self, TeeError> {
        let mut launch_token: sgx_launch_token_t = [0; 1024];
        let mut updated = 0;
        
        let enclave = SgxEnclave::create(
            enclave_path,
            true, // debug
            &mut launch_token,
            &mut updated,
            |_| Ok(()),
        )?;

        let report = attestation::generate_enclave_report(enclave.geteid())?;
        let verifier = RemoteAttestationVerifier::new(spid)?;
        verifier.verify_report(&report)?;

        Ok(Self {
            enclave: Arc::new(enclave),
            report_verifier: Arc::new(verifier),
        })
    }

    pub async fn secure_call<T: Serialize, R: DeserializeOwned>(
        &self,
        command: u32,
        payload: T,
    ) -> Result<R, TeeError> {
        let encrypted_payload = secure_channel::encrypt_payload(&payload)?;
        let mut retval = sgx_status_t::SGX_SUCCESS;
        let mut output = vec![0u8; 4096];
        
        let status = unsafe {
            ecall_secure_command(
                self.enclave.geteid(),
                &mut retval,
                command,
                encrypted_payload.as_ptr(),
                encrypted_payload.len(),
                output.as_mut_ptr(),
                output.len(),
            )
        };

        if status != sgx_status_t::SGX_SUCCESS || retval != sgx_status_t::SGX_SUCCESS {
            return Err(TeeError::ExecutionFailed);
        }

        let decrypted = secure_channel::decrypt_response(&output)?;
        Ok(bincode::deserialize(&decrypted)?)
    }
}

extern "C" {
    fn ecall_secure_command(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        cmd: u32,
        input: *const u8,
        input_len: usize,
        output: *mut u8,
        output_len: usize,
    ) -> sgx_status_t;
}
