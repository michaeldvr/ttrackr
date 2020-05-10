pub mod models;
pub mod ops;
pub mod schema;
pub mod utils;

use crate::config::Config;
use crate::utils::BoxError;

use diesel::prelude::*;

embed_migrations!();

pub fn get_connection(config: &Config) -> Result<SqliteConnection, BoxError> {
    establish_connection(config.database.get("path").unwrap())
}

pub fn establish_connection(dbpath: &str) -> Result<SqliteConnection, BoxError> {
    let conn = SqliteConnection::establish(dbpath)
        .unwrap_or_else(|_| panic!("Error connecting to {}", dbpath));

    // This will run the necessary migrations.
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())?;

    Ok(conn)
}
