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

    let mut cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("create").arg("test-task").assert().success();

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("list")
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
    let mut cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("edit").arg("test-task2").assert().failure(); // not found

    // edit task
    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("edit")
        .arg("test-task")
        .arg("-n")
        .arg("zxcv")
        .arg("-t")
        .arg("30")
        .arg("-f")
        .assert()
        .success();

    // check changes
    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("list")
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

    let mut cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("task2"))
        .stdout(predicate::str::contains("subtask"))
        .stdout(predicate::str::contains("todo"));

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("list")
        .arg("-f")
        .arg("task1")
        .assert()
        .success()
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("task2").not());

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("list")
        .arg("-f")
        .arg("task2::subtask")
        .assert()
        .success()
        .stdout(predicate::str::contains("todo"))
        .stdout(predicate::str::contains(" task2 ").not());

    Ok(())
}

#[test]
fn delete_task() -> Result<(), utils::BoxError> {
    let (_tempdir, configpath, dbpath) = utils::setup()?;
    helper::create_task(&configpath, &dbpath, "task1", "1", "")?;
    helper::create_task(&configpath, &dbpath, "task2", "1", "")?;

    let mut cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("delete")
        .arg("task1")
        .arg("--noconfirm")
        .assert()
        .success();

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("task2"))
        .stdout(predicate::str::contains("task1").not());

    Ok(())
}

#[test]
fn delete_invalid_task() -> Result<(), utils::BoxError> {
    let (_tempdir, configpath, dbpath) = utils::setup()?;
    let mut cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("delete")
        .arg("task1")
        .arg("--noconfirm")
        .assert()
        .failure();
    Ok(())
}

#[test]
fn start_task() -> Result<(), utils::BoxError> {
    let (_tempdir, configpath, dbpath) = utils::setup()?;
    helper::create_task(&configpath, &dbpath, "task1", "1", "")?;
    helper::create_task(&configpath, &dbpath, "task2", "1", "")?;
    helper::create_task(&configpath, &dbpath, "task3", "1", "")?;

    let mut cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("No running task"));

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("start")
        .arg("task1")
        .arg("task3")
        .assert()
        .success();

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("task2").not());

    Ok(())
}

#[test]
fn stop_task() -> Result<(), utils::BoxError> {
    let (_tempdir, configpath, dbpath) = utils::setup()?;
    helper::create_task(&configpath, &dbpath, "task1", "1", "")?;
    helper::create_task(&configpath, &dbpath, "task2", "1", "")?;
    helper::create_task(&configpath, &dbpath, "task3", "1", "")?;

    let mut cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("start")
        .arg("task1")
        .arg("task3")
        .assert()
        .success();

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("task2").not());

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("stop").arg("task1").arg("task3").assert().success();

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("No running task"));

    Ok(())
}

#[test]
fn stop_not_running_task() -> Result<(), utils::BoxError> {
    let (_tempdir, configpath, dbpath) = utils::setup()?;
    helper::create_task(&configpath, &dbpath, "task1", "1", "")?;
    let mut cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("stop").arg("task1").assert().failure();

    Ok(())
}

#[test]
fn stopall_task() -> Result<(), utils::BoxError> {
    let (_tempdir, configpath, dbpath) = utils::setup()?;
    helper::create_task(&configpath, &dbpath, "task1", "1", "")?;
    helper::create_task(&configpath, &dbpath, "task2", "1", "")?;
    helper::create_task(&configpath, &dbpath, "task3", "1", "")?;

    let mut cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("start")
        .arg("task1")
        .arg("task3")
        .assert()
        .success();

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("task2").not());

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("stopall").assert().success();

    cmd = helper::prepare_cmd(&configpath, &dbpath)?;
    cmd.arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("No running task"));

    Ok(())
}

mod helper {
    use super::*;

    pub fn prepare_cmd(
        config: &PathBuf,
        db: &PathBuf,
    ) -> Result<std::process::Command, utils::BoxError> {
        let mut cmd = Command::cargo_bin("ttrackr")?;
        cmd.arg("--config").arg(&config).arg("--dbfile").arg(&db);
        Ok(cmd)
    }

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
