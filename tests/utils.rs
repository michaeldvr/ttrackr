use assert_cmd::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

pub type BoxError = Box<dyn std::error::Error>;

pub fn setup() -> Result<(TempDir, PathBuf, PathBuf), BoxError> {
    // create temp dir for config file and db file
    let tempdir = TempDir::new()?;

    let mut confpath = PathBuf::new();
    confpath.push(tempdir.path());
    confpath.push(".ttrackrrc");

    let mut dbpath = PathBuf::new();
    dbpath.push(tempdir.path());
    dbpath.push(".ttrackr.db");

    Ok((tempdir, confpath, dbpath))
}
