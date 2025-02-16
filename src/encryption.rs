use argon2::{
    password_hash::{
        PasswordHasher, SaltString, PasswordVerifier, PasswordHash
    },
    Argon2
};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use colored::Colorize;

pub fn hash_password(password: &str, salt: &SaltString) -> Result<String, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), salt)?.to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

pub fn key_derivation(password: &str, salt: SaltString) -> Result<[u8; 32], argon2::password_hash::Error> {
    let mut output_key_material = [0u8; 32]; 
    Argon2::default().hash_password_into(password.as_bytes(), salt.decode_b64(&mut [0u8; 32]).unwrap(), &mut output_key_material)?;
    Ok(output_key_material)
}

pub fn encrypt_password(key: &[u8; 32], password: &str) -> Result<Vec<u8>, String>{
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, password.as_bytes().as_ref()).map_err(|e| format!("Errore nella crittografia: {}", e.to_string().red().bold()))?;
    let mut combined = Vec::new();
    combined.extend_from_slice(&nonce);
    combined.extend_from_slice(&ciphertext);
    Ok(combined)

}

pub fn decrypt_password(key: &[u8; 32], combined: Vec<u8>) -> Result<String, String> {
    if combined.len() < 12 {
        return Err("Dato crittografato non valido".red().bold().to_string());
    }
    let (nonce, ciphertext) = combined.split_at(12);
    let cipher = Aes256Gcm::new(key.into());
    let plaintext = cipher.decrypt(Nonce::from_slice(nonce), ciphertext).map_err(|e| format!("Errore nella decrittografia: {}", e.to_string().red().bold()))?;
    String::from_utf8(plaintext).map_err(|e| format!("Errore nella conversione: {}", e.to_string().red().bold()))
}