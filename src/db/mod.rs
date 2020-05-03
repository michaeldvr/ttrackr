pub mod models;
pub mod ops;
pub mod schema;

// use crate::utils::BoxError;
use diesel::prelude::*;

pub fn get_connection(dbpath: &str) -> SqliteConnection {
    SqliteConnection::establish(dbpath).unwrap_or_else(|_| panic!("Error connecting to {}", dbpath))
}
