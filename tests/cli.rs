use assert_cmd::prelude::*;
use predicates::prelude::*;
// use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

type BoxError = Box<dyn std::error::Error>;

#[test]
fn create_new_config_file() -> Result<(), BoxError> {
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

#[cfg(target_os = "linux")]
#[test]
fn shouldnt_create_config_file_on_root() -> Result<(), BoxError> {
    // TODO remove this :D
    let mut rootfile = PathBuf::new();
    rootfile.push("/root");
    rootfile.push(".ttrackrrc.test");

    let mut cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&rootfile)
        .arg("create")
        .arg("task1")
        .assert()
        .failure();

    Ok(())
}
