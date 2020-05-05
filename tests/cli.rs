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
fn create_task() -> Result<(), utils::BoxError> {
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

#[test]
fn edit_task() -> Result<(), utils::BoxError> {
    let (_tempdir, configpath, dbpath) = utils::setup()?;

    // create new task
    helper::create_task(&configpath, &dbpath, "test-task", "12", "abcdef")?;

    // edit non existing tasks
    let mut cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&configpath)
        .arg("--dbfile")
        .arg(&dbpath)
        .arg("edit")
        .arg("test-task2")
        .assert()
        .failure(); // not found

    // edit task
    cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&configpath)
        .arg("--dbfile")
        .arg(&dbpath)
        .arg("edit")
        .arg("test-task")
        .arg("-n")
        .arg("zxcv")
        .arg("-t")
        .arg("30")
        .arg("-f")
        .assert()
        .success();

    // check changes
    cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&configpath)
        .arg("--dbfile")
        .arg(&dbpath)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("zxcv"))
        .stdout(predicate::str::contains("30 minutes"))
        .stdout(predicate::str::contains("true"));

    Ok(())
}

#[test]
fn list_tasks() -> Result<(), utils::BoxError> {
    let (_tempdir, configpath, dbpath) = utils::setup()?;

    // create new task
    helper::create_task(&configpath, &dbpath, "task1", "1", "")?;
    helper::create_task(&configpath, &dbpath, "task2", "2", "")?;
    helper::create_task(&configpath, &dbpath, "task2::subtask", "3", "")?;
    helper::create_task(&configpath, &dbpath, "task2::subtask::todo", "4", "")?;

    let mut cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&configpath)
        .arg("--dbfile")
        .arg(&dbpath)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("task2"))
        .stdout(predicate::str::contains("subtask"))
        .stdout(predicate::str::contains("todo"));

    cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&configpath)
        .arg("--dbfile")
        .arg(&dbpath)
        .arg("list")
        .arg("-f")
        .arg("task1")
        .assert()
        .success()
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("task2").not());

    cmd = Command::cargo_bin("ttrackr")?;
    cmd.arg("--config")
        .arg(&configpath)
        .arg("--dbfile")
        .arg(&dbpath)
        .arg("list")
        .arg("-f")
        .arg("task2::subtask")
        .assert()
        .success()
        .stdout(predicate::str::contains("todo"))
        .stdout(predicate::str::contains(" task2 ").not());

    Ok(())
}

mod helper {
    use super::*;

    pub fn create_task(
        config: &PathBuf,
        db: &PathBuf,
        taskname: &str,
        alloc: &str,
        note: &str,
    ) -> Result<(), utils::BoxError> {
        let mut cmd = Command::cargo_bin("ttrackr")?;        

        // create new task
        cmd.arg("--config")
            .arg(config)
            .arg("--dbfile")
            .arg(db)
            .arg("create")
            .arg(taskname)
            .arg("-t")
            .arg(alloc)
            .arg("-n")
            .arg(note)
            .assert()
            .success();
        Ok(())
    }
}
