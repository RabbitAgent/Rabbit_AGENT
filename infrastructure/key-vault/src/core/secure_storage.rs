use hkdf::Hkdf;
use secrecy::{Secret, ExposeSecret};
use sha2::Sha512;
use zeroize::Zeroizing;

pub struct HardwareBackedKeystore {
    master_key: Secret<[u8; 32]>,
    derivation_path: String,
    hsm_connector: HsmClient,
}

impl HardwareBackedKeystore {
    pub fn initialize(hsm_config: &HsmConfig) -> Result<Self, VaultError> {
        let mut hsm = HsmClient::connect(hsm_config)?;
        let master_key = hsm.generate_secure_key(KeyGenOptions {
            key_type: KeyType::AES256,
            persistent: true,
            exportable: false,
        })?;
        
        Ok(Self {
            master_key: Secret::new(master_key),
            derivation_path: "m/44'/626'/0'/0".into(),
            hsm_connector: hsm,
        })
    }

    pub fn derive_key(&self, path_suffix: &str) -> Result<Zeroizing<[u8; 32]>, VaultError> {
        let hk = Hkdf::<Sha512>::new(None, self.master_key.expose_secret());
        let mut derived = Zeroizing::new([0u8; 32]);
        
        hk.derive(
            format!("{}/{}", self.derivation_path, path_suffix).as_bytes(),
            &mut *derived
        )?;

        Ok(derived)
    }

    pub fn secure_store(&self, key_id: &str, data: &[u8]) -> Result<StoredKeyMeta, VaultError> {
        let enc_key = self.derive_key(key_id)?;
        let nonce = rand::thread_rng().gen::<[u8; 12]>();
        let ciphertext = aes_gcm::encrypt(&enc_key, &nonce, data)?;
        
        self.hsm_connector.store_sealed(
            key_id,
            &ciphertext,
            KeyPolicy {
                decryption_approvers: 3,
                validity_window: 3600,
            }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyPolicy {
    pub decryption_approvers: u8,
    pub validity_window: u64,
}
