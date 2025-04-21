use openssl::{
    encrypt::Encrypter,
    hash::MessageDigest,
    pkey::PKey,
    rand::rand_bytes,
    rsa::Rsa,
    sign::Signer,
    symm::{Cipher, Crypter, Mode},
};

pub struct SessionCipher {
    session_key: [u8; 32],
    hmac_key: [u8; 32],
}

impl SessionCipher {
    pub fn new_from_attestation(attested_pubkey: &[u8]) -> Result<Self, TeeError> {
        let rsa = Rsa::public_key_from_pem(attested_pubkey)?;
        let key_pair = PKey::from_rsa(rsa)?;
        
        let mut session_key = [0u8; 32];
        let mut hmac_key = [0u8; 32];
        rand_bytes(&mut session_key)?;
        rand_bytes(&mut hmac_key)?;

        let mut encrypter = Encrypter::new(&key_pair)?;
        encrypter.set_rsa_padding(openssl::rsa::Padding::PKCS1_OAEP)?;
        
        let mut encrypted = vec![0; key_pair.size()?];
        let encrypted_len = encrypter.encrypt(&session_key, &mut encrypted)?;
        encrypted.truncate(encrypted_len);
        
        Ok(Self {
            session_key,
            hmac_key,
        })
    }

    pub fn encrypt_payload(&self, data: &[u8]) -> Result<Vec<u8>, TeeError> {
        let cipher = Cipher::aes_256_gcm();
        let mut iv = [0u8; 12];
        rand_bytes(&mut iv)?;
        
        let mut crypter = Crypter::new(cipher, Mode::Encrypt, &self.session_key, Some(&iv))?;
        crypter.set_tag_len(16)?;
        
        let mut encrypted = vec![0; data.len() + cipher.block_size()];
        let count = crypter.update(data, &mut encrypted)?;
        let rest = crypter.finalize(&mut encrypted[count..])?;
        encrypted.truncate(count + rest);
        
        let tag = crypter.get_tag()?;
        Ok([&iv, &encrypted, tag.as_slice()].concat())
    }
}
