use sgx_quote::SgxQuote;
use sgx_dcap::QuoteVerifier;

pub trait AttestationVerifier: Send + Sync {
    fn verify_report(&self, report: &[u8]) -> Result<(), TeeError>;
}

pub struct RemoteAttestationVerifier {
    spid: Vec<u8>,
    root_ca: X509,
}

impl RemoteAttestationVerifier {
    pub fn new(spid: &[u8]) -> Result<Self, TeeError> {
        let root_ca = load_root_certificate()?;
        Ok(Self {
            spid: spid.to_vec(),
            root_ca,
        })
    }
}

impl AttestationVerifier for RemoteAttestationVerifier {
    fn verify_report(&self, report: &[u8]) -> Result<(), TeeError> {
        let quote = SgxQuote::parse(report)?;
        let verification = QuoteVerifier::new()
            .spid(&self.spid)
            .root_ca_cert(&self.root_ca)
            .verify(&quote)?;
        
        if !verification.is_ok() {
            return Err(TeeError::AttestationFailed);
        }
        
        Ok(())
    }
}
