pub mod cli;
pub mod structures;
pub mod encryption;
pub mod utils;

use cli::Comando;
use structures::*;
use structopt::StructOpt;
use std::{thread, time, process};
use colored::Colorize;

fn menu(vault: &mut Vault) -> Result<(), String> {
    utils::clear_screen();
    println!("{} {}\n", "Benvenuto".blue().bold(), vault.username.blue().bold());
    vault.lista();
    let azione = utils::opzioni()?;
    match azione {
        Azione::AggiungiLogin => todo!(),
        Azione::VisualizzaLogin => todo!(),
        Azione::RimuoviLogin => todo!(),
        Azione::Esci => process::exit(0)
    }
    Ok(())
}

fn main() {
    utils::clear_screen();
    let comando = Comando::from_args(); 
    let mut vault = match comando {
        Comando::NuovoVault => {
            Vault::new().unwrap_or_else(|e| {
                println!("{}\n", e);
                Vault::new().unwrap()
            })
        }
        Comando::Login => {
            let _vault: Vault = todo!();
        }
    };

    loop {
        match vault.state {
            State::Locked => vault.unlock(),
            State::Unlocked => {
                match menu(&mut vault) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("{}", e);
                        thread::sleep(time::Duration::from_secs(1));
                        continue;
                    }
                }
            },
        }
    }

}
