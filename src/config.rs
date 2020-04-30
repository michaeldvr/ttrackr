// config file parser
use crate::utils::BoxError;
use dirs;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub autodone: bool,
    pub database: HashMap<String, String>,
}

impl Config {
    fn save(&self, filepath: Option<PathBuf>) -> Result<(), BoxError> {
        let cfgpath = get_config_path(filepath);
        let resultstr = toml::to_string(self)?;
        let mut file = File::create(&cfgpath)?;
        file.write_all(resultstr.as_bytes())?;
        Ok(())
    }
}

pub fn get_config_path(filepath: Option<PathBuf>) -> PathBuf {
    match filepath {
        None => {
            let mut tmp_path: PathBuf = dirs::home_dir().unwrap();
            tmp_path.push(".ttrackrrc");
            tmp_path
        }
        Some(filepath) => filepath,
    }
}

pub fn create_config(filepath: Option<PathBuf>) -> Result<(bool, PathBuf), BoxError> {
    let cfgpath = get_config_path(filepath);

    if cfgpath.exists() {
        return Ok((false, cfgpath));
    }

    let mut dbpath = dirs::home_dir().unwrap();
    dbpath.push(".ttrackr.db");

    let mut configdb = HashMap::<String, String>::new();
    configdb.insert(String::from("path"), dbpath.to_string_lossy().to_string());

    let config = Config {
        autodone: false,
        database: configdb,
    };
    println!("{:?}", &config);
    // println!("created config file at {:?}", cfgpath);

    match config.save(None) {
        Ok(()) => Ok((true, cfgpath)),
        Err(err) => Err(err),
    }
}
