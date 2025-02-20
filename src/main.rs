// Moduli del progetto
pub mod cli;
pub mod encryption;
pub mod structures;
pub mod utils;

// Importazioni necessarie
use cli::Comando;
use colored::Colorize;
use std::{process, thread, time};
use structopt::StructOpt;
use structures::*;
use directories::BaseDirs;

// Funzione per gestire il menu principale del vault
// Mostra le opzioni disponibili e gestisce le azioni dell'utente
fn menu(vault: &mut Vault) -> Result<(), String> {
    // Pulisce lo schermo
    utils::clear_screen();
    
    // Stampa il messaggio di benvenuto
    println!(
        "{} {}\n",
        "Benvenuto".blue().bold(),
        vault.username.blue().bold()
    );
    
    // Mostra la lista dei login salvati
    vault.lista();
    
    // Ottiene l'azione scelta dall'utente
    let azione = utils::opzioni()?;
    
    // Gestisce l'azione scelta dall'utente
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

// Funzione principale del programma
// Gestisce l'inizializzazione del vault e il ciclo principale del programma
fn main() {
    // Ottiene la directory base per i dati locali
    let base_dirs = BaseDirs::new().unwrap();
    let path = base_dirs.data_local_dir().join(".password_manager");
    
    // Pulisce lo schermo
    utils::clear_screen();
    
    // Ottiene il comando passato dall'utente
    let comando = Comando::from_args();
    
    // Inizializza il vault in base al comando
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
            })
        },
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
    
    // Imposta lo stato del vault come bloccato
    vault.state = State::Locked;

    // Ciclo principale del programma
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
