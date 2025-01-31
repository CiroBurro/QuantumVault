pub mod cli;
pub mod structures;
pub mod encryption;

use directories::BaseDirs;
use surrealdb::{
    Surreal,
    engine::local::RocksDb,
};

#[tokio::main]
async fn main() -> Result<(), surrealdb::Error> {
    let base_dirs = BaseDirs::new().unwrap();
    let data = base_dirs.data_local_dir();

    let db_path = data.join("surreal.db").to_str();
    //let db = Surreal::new::<RocksDb>(db_path).await?;
    
    Ok(())
}
