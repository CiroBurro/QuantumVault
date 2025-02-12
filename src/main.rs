pub mod cli;
pub mod structures;
pub mod encryption;

use cli::Comando;
use structures::*;
use structopt::StructOpt;


fn menu(_vault: Vault) {
    todo!()
}

fn main() {
    let comando = Comando::from_args(); 
    let vault = match comando {
        Comando::NuovoVault => {
            Vault::new().unwrap_or_else(|e| {
                println!("{}\n", e);
                Vault::new().unwrap()
            })
        }
        Comando::Login => {
            let vault: Vault = todo!();
            let passwd = todo!();
            vault.unlock(passwd);
            vault
        }
    };

    menu(vault);

}
