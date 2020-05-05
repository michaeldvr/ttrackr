use assert_cmd::prelude::*;
use predicates::prelude::*;
// use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

mod utils;

#[test]
fn create_new_config_file() -> Result<(), utils::BoxError> {
    let tempdir = TempDir::new()?;
    let mut configfile = PathBuf::new();
    configfile.push(tempdir.path());
    configfile.push(".ttrackrrc.test");

    // check wheter config file has yet to exist
    assert!(!configfile.exists());

    let mut cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&configfile)
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created config file"));

    // check whether config file created
    assert!(configfile.exists());

    Ok(())
}

#[test]
fn create_tasks() -> Result<(), utils::BoxError> {
    let (_tempdir, configpath, dbpath) = utils::setup()?;

    let mut cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&configpath)
        .arg("--dbfile")
        .arg(&dbpath)
        .arg("create")
        .arg("test-task")
        .assert()
        .success();

    cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&configpath)
        .arg("--dbfile")
        .arg(&dbpath)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-task"));

    Ok(())
}
