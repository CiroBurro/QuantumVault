use serde::{Deserialize, Serialize};

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
    pub master_password_hash: String,
    pub logins: Vec<Login>,
    pub state: State,
}