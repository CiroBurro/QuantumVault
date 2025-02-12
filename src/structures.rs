use argon2::password_hash::{SaltString, rand_core::OsRng};
use serde::{Deserialize, Serialize};
use crate::encryption::*;
use std::io;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub nome: String,
    username: String,
    password_hash: String,
}

impl Login {
    pub fn new(nome: String, username: String, hash: String) -> Self {
        Login {
            nome,
            username,
            password_hash: hash,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    Locked,
    Unlocked,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vault {
    pub username: String,
    master_password_hash: String,
    salt: String,
    pub logins: Vec<Login>,
    pub state: State,
}

impl Vault {
    pub fn new() -> Result<Self, String> {
        let mut user = String::new();
        let mut password= String::new();
        let mut check= String::new();
        let salt = SaltString::generate(&mut OsRng);
        let salt_str = salt.as_str();

        println!("Inserisci un nome utente:");
        print!("> ");
        io::stdout().flush().expect("Errore nel flush del buffer");
        io::stdin().read_line(&mut user).expect("errore nella lettura del nome");

        println!("\nCiao {}, crea una password:", &user.trim());
        print!("> ");
        io::stdout().flush().expect("Errore nel flush del buffer");
        io::stdin().read_line(&mut password).expect("errore nella lettura della password");

        println!("\nReinserisci la password:");
        print!("> ");
        io::stdout().flush().expect("Errore nel flush del buffer");
        io::stdin().read_line(&mut check).expect("errore nella lettura della password");

        if password != check {
            return Err("le due password non coincidono".to_string())
        }

        let m_p_h = hash_password(password.trim(), &salt).map_err(|e| e.to_string())?;

        let vault = Self {
            username: user.to_string(),
            master_password_hash: m_p_h,
            salt: salt_str.to_string(),
            logins: Vec::new(),
            state: State::Unlocked,
        };

        Ok(vault)
    }

    pub fn unlock(&mut self, passwd: &str) -> bool {

        let passwd_hash = match hash_password(passwd, &SaltString::from_b64(self.salt.as_str()).unwrap()) {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        if passwd_hash == self.master_password_hash { true } else { false }
    }
}