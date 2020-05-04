pub mod models;
pub mod ops;
pub mod schema;

use crate::config::Config;
use crate::utils::BoxError;

use diesel::prelude::*;

pub fn get_connection(config: &Config) -> SqliteConnection {
    establish_connection(config.database.get("path").unwrap())
}

pub fn establish_connection(dbpath: &str) -> SqliteConnection {
    SqliteConnection::establish(dbpath).unwrap_or_else(|_| panic!("Error connecting to {}", dbpath))
}
