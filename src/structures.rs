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
    nome: String,
    username: String,
    encrypted_password: Vec<u8>,
}

impl Login {
    pub fn new(nome: String, username: String, passwd: Vec<u8>) -> Self {
        Login {
            nome,
            username,
            encrypted_password: passwd,
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
    key: [u8; 32],
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
            key: key_derivation(password.trim(), salt).unwrap(),
            logins: Vec::new(),
            state: State::Locked,
        };

        Ok(vault)
    }

    pub fn unlock(&mut self){
        for _ in 0..5 {
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
                println!("{}", "Le credenziali non coincidono".red().bold());
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
        }   
    }

    pub fn lista(&self) {
        let logins:Vec<&str> = self.logins.iter().map(|l| l.nome.as_str()).collect();
        if logins.is_empty() {
            println!("{}", "Hai 0 password salvate".red().bold());
        } else {
            println!("{}", "Password salvate:".yellow().bold());
            for (i, nome) in logins.iter().enumerate() {
                println!("{}{} {}", (i + 1).to_string().blue().bold(), ".".blue().bold(), &nome)
            } 
        }
    }

    pub fn aggiungi_login(&mut self) -> Result<(), String> {
        clear_screen();
        let mut nome = String::new();
        let mut user: String = String::new();
        let mut passwd: String = String::new();

        println!("{}",  "Sblocca il tuo vault".yellow().bold());

        println!("{}", "Inserisci il nome del servizio di cui vuoi salvare la password:".blue().bold());
        print!("{}", "> ".green());
        io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
        io::stdin().read_line(&mut nome).expect("errore nella lettura del nome".red().to_string().as_str());

        println!("{} {}", "Inserisci il nome utente per il servizio:".blue().bold(), &nome.blue().bold());
        print!("{}", "> ".green());
        io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
        io::stdin().read_line(&mut user).expect("errore nella lettura del nome".red().to_string().as_str());

        println!("{} {}", "Inserisci la password associata al nome utente:".blue().bold(), &user.blue().bold());
        print!("{}", "> ".green());
        io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
        io::stdin().read_line(&mut passwd).expect("errore nella lettura del nome".red().to_string().as_str());

        let password = encrypt_password(&self.key, passwd.trim())?;
        let login = Login::new(nome.trim().to_string(), user.trim().to_string(), password);
        Ok(self.logins.push(login))
    }

    pub fn rimuovi_login(&mut self) -> Result<(), String>{
        clear_screen();
        self.lista();
        let mut input = String::new();
        println!("{}", "Inserisci il numero corrispondente al nome del servizio da rimuovere:".blue().bold());
        print!("{}", "> ".green());
        io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
        io::stdin().read_line(&mut input).expect("errore nella lettura del nome".red().to_string().as_str());

        let index: usize = match input.trim().parse::<usize>() {
            Ok(n) => n - 1,
            Err(_) => return Err("Input non valido".red().to_string()),
        };

        input.clear();

        println!("{} {}{}", "Sei sicuro di voler rimuovere:".blue().bold(), self.logins[index].nome.blue().bold(), "?(s/n)".blue().bold());
        print!("{}", "> ".green());
        io::stdout().flush().expect("Errore nel flush del buffer".red().to_string().as_str());
        io::stdin().read_line(&mut input).expect("errore nella lettura del nome".red().to_string().as_str());

        match input.trim().to_lowercase().as_str() {
            "s" => {
                self.state = State::Locked;
                self.unlock();
                self.logins.remove(index);
                println!("{}", "Password eliminata con successo".green().bold());
                thread::sleep(time::Duration::from_secs(2));
            }
            "n" => {
                println!("{}", "Annullamento operazione in corso".red().bold());
                thread::sleep(time::Duration::from_secs(2));
            } 
            _ => return Err("Input non valido".red().to_string())
        }

        Ok(())
    }
}

pub enum Azione {
    AggiungiLogin,
    VisualizzaLogin,
    RimuoviLogin,
    Esci,    
}
