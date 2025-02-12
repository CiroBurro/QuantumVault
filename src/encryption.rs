use argon2::{
    password_hash::{
        PasswordHasher, SaltString, PasswordVerifier, PasswordHash
    },
    Argon2
};

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