// config file parser
use crate::utils::BoxError;
use log::debug;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub autodone: bool,
    pub database: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            autodone: false,
            database: HashMap::<String, String>::new(),
        }
    }

    pub fn save(&self, filepath: Option<&PathBuf>) -> Result<(), BoxError> {
        let cfgpath = get_config_path(filepath);
        let mut file = File::create(&cfgpath)?;
        self.save_to(&mut file)
    }

    pub fn save_to(&self, file: &mut impl std::io::Write) -> Result<(), BoxError> {
        let resultstr = toml::to_string(self)?;
        file.write_all(resultstr.as_bytes())?;
        Ok(())
    }

    pub fn load(filepath: Option<&PathBuf>) -> Result<Self, BoxError> {
        let cfgpath = get_config_path(filepath);
        let content = read_to_string(&cfgpath)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

pub fn get_config_path(filepath: Option<&PathBuf>) -> PathBuf {
    match filepath {
        None => {
            let mut tmp_path: PathBuf = dirs::home_dir().unwrap();
            tmp_path.push(".ttrackrrc");
            tmp_path
        }
        Some(filepath) => PathBuf::from(filepath),
    }
}

pub fn create_config(
    filepath: Option<&PathBuf>,
    dbfile: Option<&PathBuf>,
) -> Result<(bool, PathBuf), BoxError> {
    let cfgpath = get_config_path(filepath);
    debug!("using cfgpath: {:?} [exists:{}]", cfgpath, cfgpath.exists());
    if cfgpath.exists() {
        return Ok((false, cfgpath));
    }

    let mut dbpath: PathBuf;
    if let Some(path) = dbfile {
        dbpath = path.to_path_buf();
    } else {
        dbpath = dirs::home_dir().unwrap();
        dbpath.push(".ttrackr.db");
    }

    let mut config = Config::new();
    config
        .database
        .insert("path".to_owned(), dbpath.to_string_lossy().to_string());

    match config.save(filepath) {
        Ok(()) => Ok((true, cfgpath)),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn load_config_from_file() -> Result<(), BoxError> {
        let mut conf = Config::new();
        conf.database
            .insert("path".to_owned(), "/tmp/testfile".to_owned());
        let mut file = NamedTempFile::new()?;
        conf.save_to(&mut file)?;
        let check = Config::load(Some(&file.path().to_path_buf()))?;
        assert_eq!(conf.autodone, check.autodone);
        assert_eq!(conf.database, conf.database);
        Ok(())
    }
}
