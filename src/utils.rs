use crate::structures::{Azione, Vault};
use colored::Colorize;
use std::{io::{stdin, stdout, Write}, fs};
use directories::BaseDirs;

// Funzione per pulire lo schermo
pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

// Funzione per ottenere l'azione scelta dall'utente nel menu principale
pub fn opzioni() -> Result<Azione, String> {
    println!(
        "\n{}",
        "A) Aggiungi login  B) Visualizza login  C) Rimuovi login  D) Esci"
            .green()
            .bold()
    );
    let mut input = String::new();
    print!("Scegli un'opzione (A-B-C-D) > ");
    stdout()
        .flush()
        .expect("Errore nel flush del buffer".red().to_string().as_str());
    stdin().read_line(&mut input).expect(
        "errore nella lettura dell'input"
            .red()
            .to_string()
            .as_str(),
    );

    match input.trim().to_lowercase().as_str() {
        "a" => Ok(Azione::AggiungiLogin),
        "b" => Ok(Azione::VisualizzaLogin),
        "c" => Ok(Azione::RimuoviLogin),
        "d" => Ok(Azione::Esci),
        _ => Err("Scelta non valida".red().bold().to_string()),
    }
}

// Funzione per ottenere l'azione scelta dall'utente nel menu secondario
pub fn opzioni_2() -> Result<Azione, String> {
    println!(
        "\n{}",
        "A) Modifica login  B) Copia password  C) Torna al menu  D) Esci"
            .green()
            .bold()
    );
    let mut input = String::new();
    print!("Scegli un'opzione (A-B-C-D) > ");
    stdout()
        .flush()
        .expect("Errore nel flush del buffer".red().to_string().as_str());
    stdin().read_line(&mut input).expect(
        "errore nella lettura dell'input"
            .red()
            .to_string()
            .as_str(),
    );

    match input.trim().to_lowercase().as_str() {
        "a" => Ok(Azione::ModificaLogin),
        "b" => Ok(Azione::CopiaPassword),
        "c" => Ok(Azione::TornaMenu),
        "d" => Ok(Azione::Esci),
        _ => Err("Scelta non valida".red().bold().to_string()),
    }
}

// Funzione per salvare il vault su disco
pub fn salva_vault(vault: &Vault) -> Result<(), String> {
    let base_dirs = BaseDirs::new().unwrap();
    let path = base_dirs.data_local_dir().join(".password_manager");
    let serialized = serde_json::to_string(&vault).map_err(|e| e.to_string())?;
    fs::write(path, serialized).map_err(|e| e.to_string())?;
    Ok(())
}

// Funzione per caricare il vault da disco
pub fn cariva_vault() -> Result<Vault, String> {
    let base_dirs = BaseDirs::new().unwrap();
    let path = base_dirs.data_local_dir().join(".password_manager");
    let serialized = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let vault: Vault = serde_json::from_str(&serialized).map_err(|e| e.to_string())?;
    Ok(vault)
}
