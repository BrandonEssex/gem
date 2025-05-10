use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand_core::{RngCore, OsRng};

pub fn encrypt(plain: &[u8], key: &[u8; 32]) -> Option<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).ok()?;
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let encrypted = cipher.encrypt(nonce, plain).ok()?;
    let mut output = b"GEMX".to_vec();
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&encrypted);
    Some(output)
}

pub fn decrypt(ciphertext: &[u8], key: &[u8; 32]) -> Option<Vec<u8>> {
    if !ciphertext.starts_with(b"GEMX") {
        return None;
    }
    let nonce_bytes = &ciphertext[4..16];
    let data = &ciphertext[16..];
    let cipher = Aes256Gcm::new_from_slice(key).ok()?;
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.decrypt(nonce, data).ok()
}
