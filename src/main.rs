pub mod cli;
pub mod encryption;
pub mod structures;
pub mod utils;

use cli::Comando;
use colored::Colorize;
use std::{process, thread, time};
use structopt::StructOpt;
use structures::*;

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
        Azione::Esci => process::exit(0),
        _ => Err("Scelta non valida".to_string()),
    }
}

fn main() {
    utils::clear_screen();
    let comando = Comando::from_args();
    let mut vault = match comando {
        Comando::NuovoVault => Vault::new().unwrap_or_else(|e| {
            println!("{}\n", e);
            Vault::new().unwrap()
        }),
        Comando::Login => {
            let _vault: Vault = todo!();
        }
    };

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
