use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305,
};
use rand_core::OsRng;

pub struct EncryptedShard {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub auth_tag: [u8; 16],
}

pub fn encrypt_shard(key: &[u8; 32], data: &[u8]) -> Result<EncryptedShard> {
    let cipher = ChaCha20Poly1305::new_from_slice(key)?;
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    
    let ciphertext = cipher.encrypt(&nonce, data)?;
    let (ct, tag) = ciphertext.split_at(ciphertext.len() - 16);
    
    Ok(EncryptedShard {
        ciphertext: ct.to_vec(),
        nonce: nonce.into(),
        auth_tag: tag.try_into()?,
    })
}
