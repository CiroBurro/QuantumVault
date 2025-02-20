use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "QuantumVault", about = "CLI Password Manager Quantico")]
pub enum Comando {
    #[structopt(about = "Accedi a un vault creato in precedenza")]
    Login,

    #[structopt(about = "Crea un nuovo vault")]
    NuovoVault,

    #[structopt(about = "Elimina il vault")]
    EliminaVault,
}
