use crate::structures::{Azione, Vault};
use colored::Colorize;
use std::{io::{stdin, stdout, Write}, fs};
use directories::BaseDirs;

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}

pub fn opzioni() -> Result<Azione, String> {
    println!(
        "\n{}",
        "A) Aggiungi password  B) Visualizza password  C) Rimuovi password  D) Esci"
            .green()
            .bold()
    );
    let mut input = String::new();
    print!("Scegli un'opzione (A-B-C-D) > ");
    stdout()
        .flush()
        .expect("Errore nel flush del buffer".red().to_string().as_str());
    stdin().read_line(&mut input).expect(
        "errore nella lettura della password"
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

pub fn opzioni_2() -> Result<Azione, String> {
    println!(
        "\n{}",
        "A) Modifica password  B) Copia password  C) Torna al menu  D) Esci"
            .green()
            .bold()
    );
    let mut input = String::new();
    print!("Scegli un'opzione (A-B-C-D) > ");
    stdout()
        .flush()
        .expect("Errore nel flush del buffer".red().to_string().as_str());
    stdin().read_line(&mut input).expect(
        "errore nella lettura della password"
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

pub fn salva_vault(vault: &Vault) -> Result<(), String> {
    let base_dirs = BaseDirs::new().unwrap();
    let path = base_dirs.data_local_dir().join(".password_manager");
    let serialized = serde_json::to_string(&vault).map_err(|e| e.to_string())?;
    fs::write(path, serialized).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn cariva_vault() -> Result<Vault, String> {
    let base_dirs = BaseDirs::new().unwrap();
    let path = base_dirs.data_local_dir().join(".password_manager");
    let serialized = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let vault: Vault = serde_json::from_str(&serialized).map_err(|e| e.to_string())?;
    Ok(vault)
}
