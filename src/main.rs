pub mod cli;
pub mod encryption;
pub mod structures;
pub mod utils;

use cli::Comando;
use colored::Colorize;
use std::{process, thread, time};
use structopt::StructOpt;
use structures::*;
use directories::BaseDirs;

fn menu(vault: &mut Vault) -> Result<(), String> {
    utils::clear_screen();
    println!(
        "{} {}\n",
        "Benvenuto".blue().bold(),
        vault.username.blue().bold()
    );
    vault.lista();
    let azione = utils::opzioni()?;
    match azione {
        Azione::AggiungiLogin => vault.aggiungi_login(),
        Azione::VisualizzaLogin => vault.visualizza_login(),
        Azione::RimuoviLogin => vault.rimuovi_login(),
        Azione::Esci => {
            vault.state = State::Locked;
            utils::salva_vault(&vault)?;
            process::exit(0);
        },
        _ => Err("Scelta non valida".to_string()),
    }
}


fn main() {
    let base_dirs = BaseDirs::new().unwrap();
    let path = base_dirs.data_local_dir().join(".password_manager");
    utils::clear_screen();
    let comando = Comando::from_args();
    let mut vault = match comando {
        Comando::NuovoVault => {
            if path.exists() {
                println!("{}", "Il vault esiste già".red().bold());
                thread::sleep(time::Duration::from_secs(1));
                process::exit(1);
            }
            Vault::new().unwrap_or_else(|e| {
            println!("{}\n", e);
            Vault::new().unwrap()
        })},
        Comando::Login => {
            let vault = match utils::cariva_vault() {
                Ok(v) => v,
                Err(_) => {
                    println!("{}", "Nessun vault trovato - Esegui 'quantumvault nuovo-vault'".red().bold());
                    thread::sleep(time::Duration::from_secs(1));
                    process::exit(1);
                },
            };
            vault
        }
        Comando::EliminaVault => {
            if !path.exists() {
                println!("{}", "Il vault non esiste".red().bold());
                thread::sleep(time::Duration::from_secs(1));
                process::exit(1);
            }
            std::fs::remove_file(path).unwrap();
            println!("{}", "Vault eliminato".green().bold());
            process::exit(0);
        }
    };
    vault.state = State::Locked;

    loop {
        match vault.state {
            State::Locked => vault.unlock(),
            State::Unlocked => match menu(&mut vault) {
                Ok(()) => (),
                Err(e) => {
                    println!("{}", e);
                    thread::sleep(time::Duration::from_secs(2));
                    continue;
                }
            },
        }
    }
}
