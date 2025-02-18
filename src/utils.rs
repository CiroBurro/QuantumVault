use crate::structures::Azione;
use colored::Colorize;
use std::io::{stdin, stdout, Write};

pub fn clear_screen() {
    println!("\x1B[2J");
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
