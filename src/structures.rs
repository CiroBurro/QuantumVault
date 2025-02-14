use argon2::password_hash::{SaltString, rand_core::OsRng};
use serde::{Deserialize, Serialize};
use crate::{
    encryption::*,
    utils::clear_screen
};
use std::{io, thread, time};
use std::io::Write;
use colored::Colorize;

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
    logins: Vec<Login>,
    pub state: State,
}

impl Vault {
    pub fn new() -> Result<Self, String> {
        let mut user = String::new();
        let mut password= String::new();
        let mut check= String::new();
        let salt = SaltString::generate(&mut OsRng);
        let salt_str = salt.as_str();

        println!("{}", "Inserisci un nome utente:".blue().bold());
        print!("{}", "> ".green());
        io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
        io::stdin().read_line(&mut user).expect("errore nella lettura del nome".red().to_string().as_str());

        println!("\n{} {}{}", "Ciao".blue().bold(), &user.trim().blue().bold(), ", crea una password:".blue().bold());
        print!("{}", "> ".green());
        io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
        io::stdin().read_line(&mut password).expect("errore nella lettura della password".red().to_string().as_str());

        println!("\n{}", "Reinserisci la password".blue().bold());
        print!("{}", "> ".green());
        io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
        io::stdin().read_line(&mut check).expect("errore nella lettura della password".red().to_string().as_str());

        if password != check {
            return Err("Le due password non coincidono".red().to_string())
        }

        let m_p_h = hash_password(password.trim(), &salt).map_err(|e| e.to_string())?;

        let vault = Self {
            username: user.trim().to_string(),
            master_password_hash: m_p_h,
            salt: salt_str.to_string(),
            logins: Vec::new(),
            state: State::Locked,
        };

        Ok(vault)
    }

    pub fn unlock(&mut self){
        for i in 0..5 {
            clear_screen();
            let mut user = String::new();
            let mut passwd = String::new();
            println!("{}",  "Sblocca il tuo vault".yellow().bold());

            println!("{}", "Inserisci un nome utente:".blue().bold());
            print!("{}", "> ".green());
            io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
            io::stdin().read_line(&mut user).expect("errore nella lettura del nome".red().to_string().as_str());

            println!("\n{}", "inserisci la tua password:".blue().bold());
            print!("{}", "> ".green());
            io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
            io::stdin().read_line(&mut passwd).expect("errore nella lettura della password".red().to_string().as_str());

            let passwd_hash = match hash_password(passwd.trim(), &SaltString::from_b64(self.salt.as_str()).unwrap()) {
                Ok(hash) => hash,
                Err(_) => {
                    println!("{}", "Errore nel calcolo dell'hash".red().bold());
                    thread::sleep(time::Duration::from_secs(1));
                    continue;
                },
            };

            

            if passwd_hash == self.master_password_hash && user.trim() == self.username {
                self.state = State::Unlocked;
                break;
            } else {
                println!("Le credenziali non coincidono, hai ancora {} tentativi", 5 - 1 - i);
                thread::sleep(time::Duration::from_secs(5));
                continue;
            }
        }   
    }

    pub fn lista(&self) {
        let logins:Vec<&str> = self.logins.iter().map(|l| l.nome.as_str()).collect();
        if logins.is_empty() {
            println!("{}", "Hai 0 password salvate".red().bold());
        } else {
            println!("{}", "Password salvate:".blue().bold());
            for (i, nome) in logins.iter().enumerate() {
                println!("{}. {}", (i + 1).to_string().blue().bold(), &nome)
            } 
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Azione {
    AggiungiLogin,
    VisualizzaLogin,
    RimuoviLogin,
    Esci,    
}
