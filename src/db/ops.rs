use super::{get_connection, models, schema, BoxError, Config};
use crate::db::utils;
use ansi_term::Style;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use log::debug;
use std::convert::TryFrom;
// use std::io::{stdin, Read};

pub fn create_task(
    config: &Config,
    taskname: &str,
    notes: Option<&str>,
    allocated: Option<i32>,
    duedate: Option<&str>,
) -> Result<(), BoxError> {
    let conn = get_connection(config)?;
    let new_task = models::NewTask {
        taskname,
        notes,
        allocated,
        duedate,
    };

    let result = diesel::insert_into(schema::task::table)
        .values(&new_task)
        .execute(&conn);
    match result {
        Ok(_val) => {
            // println!("result: {}", val);
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

pub fn list_tasks(
    config: &Config,
    taskfilter: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<models::Task>, BoxError> {
    use schema::task::dsl::*;
    let conn = get_connection(config)?;
    // let query = task.load::<models::Task>(&conn);
    let mut query = task.into_boxed();
    if let Some(taskfilter) = taskfilter {
        let mut parent = String::from(taskfilter);
        parent.push_str("::%");
        query = query.filter(taskname.like(taskfilter).or(taskname.like(parent)));
    }
    if let Some(status) = status {
        match status {
            "done" => {
                query = query.filter(done.eq(true));
            }
            "incomplete" => {
                query = query.filter(done.eq(false));
            }
            _ => (),
        };
    }
    let data = query.load::<models::Task>(&conn);
    match data {
        Ok(val) => Ok(val),
        Err(err) => Err(err.into()),
    }
}

pub fn update_tasks(
    config: &Config,
    name: &str,
    notes: Option<&str>,
    allocated: Option<i32>,
    duedate: Option<&str>,
    done: Option<bool>,
) -> Result<(), BoxError> {
    let conn = get_connection(config)?;
    let taskid = helper::get_task_id(&conn, name)?;
    let updatetask = models::UpdateTask {
        id: taskid,
        notes: notes.map(String::from),
        allocated: allocated.unwrap_or(0),
        duedate: duedate.map(String::from),
        done,
    };
    match diesel::update(&updatetask).set(&updatetask).execute(&conn) {
        Ok(_val) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub fn delete_task(config: &Config, name: &str) -> Result<(), BoxError> {
    use schema::task::dsl::*;
    let conn = get_connection(config)?;
    diesel::delete(task.filter(taskname.eq(name))).execute(&conn)?;
    Ok(())
}

pub fn check_task_exists(config: &Config, name: &str) -> Result<bool, BoxError> {
    use diesel::dsl::{exists, select};
    use schema::task::dsl::*;
    let conn = get_connection(config)?;
    match select(exists(task.filter(taskname.eq(name)))).get_result(&conn) {
        Ok(val) => Ok(val),
        Err(err) => Err(err.into()),
    }
}

pub fn start_worklog(config: &Config, name: &str) -> Result<(), BoxError> {
    let conn = get_connection(config)?;
    let current_task = helper::get_task(&conn, name)?;
    if helper::check_task_is_running(&conn, &current_task)? {
        helper::ignore_invalid_worklogs(&conn, &current_task)?;
        eprintln!(
            "Attempting to start an already running task. Any subsequent tasks won't be started."
        );
        return Err(utils::TaskIsAlreadyRunning {
            taskname: current_task.taskname,
        }
        .into());
    }
    let res = helper::create_worklog(&conn, current_task.id);
    println!(
        "{} started at {}.",
        Style::new().bold().paint(current_task.taskname),
        helper::get_timestamp()
    );
    res
}

/// Stop multiple tasks
pub fn stop_worklogs(config: &Config, names: &Vec<String>) -> Result<(), BoxError> {
    let conn = get_connection(config)?;
    for name in names.iter() {
        let current_task = helper::get_task(&conn, &name)?;
        if !helper::check_task_is_running(&conn, &current_task)? {
            // no running worklog
            eprintln!(
                "Attempting to stop a non-running task. Any subsequent tasks won't be stopped."
            );
            return Err(utils::TaskIsNotRunning {
                taskname: current_task.taskname,
            }
            .into());
        }
        helper::ignore_invalid_worklogs(&conn, &current_task)?;
        helper::stop_worklog(&conn, &current_task)?;
        println!(
            "{} stopped at {}.",
            Style::new().bold().paint(current_task.taskname),
            helper::get_timestamp()
        );
    }
    Ok(())
}

pub fn get_total_spent(config: &Config, name: &str) -> Result<i32, BoxError> {
    let conn = get_connection(config)?;
    let taskobj = helper::get_task(&conn, name)?;
    use schema::worklog::dsl::*;
    let spents = models::Worklog::belonging_to(&taskobj)
        .filter(stopped.is_not_null())
        .filter(ignored.eq(false))
        .select(duration)
        .load::<i32>(&conn)?;

    let mut current_spent = 0; // running spent
    if helper::check_task_is_running(&conn, &taskobj)? {
        let running = models::Worklog::belonging_to(&taskobj)
            .filter(stopped.is_null())
            .filter(ignored.eq(false))
            .select(started)
            .first::<String>(&conn)?;
        let stop_timestamp = Utc::now().naive_local();
        let start_timestamp = NaiveDateTime::parse_from_str(&running, "%Y-%m-%d %H:%M:%S")?;
        current_spent = i32::try_from(
            stop_timestamp
                .signed_duration_since(start_timestamp)
                .num_seconds(),
        )?;
    }
    // manual addition
    Ok(spents.iter().sum::<i32>() + current_spent)
}

mod helper {
    use super::*;
    use diesel::SqliteConnection;

    pub fn get_task_id(conn: &SqliteConnection, name: &str) -> Result<i32, BoxError> {
        use schema::task::dsl::*;
        let taskid = task.select(id).filter(taskname.eq(name)).first::<i32>(conn);
        match taskid {
            Ok(val) => Ok(val),
            Err(err) => Err(err.into()),
        }
    }

    pub fn get_task(conn: &SqliteConnection, name: &str) -> Result<models::Task, BoxError> {
        use schema::task::dsl::*;
        let found_task = task.filter(taskname.eq(name)).first::<models::Task>(conn)?;
        Ok(found_task)
    }

    pub fn check_task_is_running(
        conn: &SqliteConnection,
        task: &models::Task,
    ) -> Result<bool, BoxError> {
        use schema::worklog::dsl::*;
        let worklogs: Vec<i32> = models::Worklog::belonging_to(task)
            .filter(stopped.is_null())
            .filter(ignored.eq(false))
            .select(id)
            .load::<i32>(conn)?;
        debug!("worklogs: {:?}", worklogs);
        Ok(!worklogs.is_empty())
    }

    /// Ignore multiple running worklogs of a same task.
    ///
    /// If there are multiple running worklogs (`stopped` field is `None`)
    /// belong to given `task`, then set `ignored` field to `true`
    /// for all of those worklogs but whose the latest `duration`.
    pub fn ignore_invalid_worklogs(
        conn: &SqliteConnection,
        task: &models::Task,
    ) -> Result<(), BoxError> {
        use schema::worklog::dsl::*;
        let worklogs: Vec<models::Worklog> = models::Worklog::belonging_to(task)
            .filter(stopped.is_null())
            .filter(ignored.eq(false))
            .order(started.desc())
            .load::<models::Worklog>(conn)?;
        if worklogs.len() > 1 {
            for (i, row) in worklogs.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                diesel::update(row).set(ignored.eq(true)).execute(conn)?;
            }
        }
        Ok(())
    }

    /// Insert new worklog for given `task_id`.
    ///
    /// This function **does not** check for duplicate running worklog entries.
    pub fn create_worklog(conn: &SqliteConnection, taskid: i32) -> Result<(), BoxError> {
        let new_worklog = models::NewWorklog { task_id: taskid };
        let result = diesel::insert_into(schema::worklog::table)
            .values(&new_worklog)
            .execute(conn);
        match result {
            Ok(_val) => {
                // println!("result: {}", val);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Update `stopped` and `duration` field for running worklog for given `taskobj`.
    ///
    /// This function **doues not** check for duplicate running worklog entries.
    pub fn stop_worklog(conn: &SqliteConnection, taskobj: &models::Task) -> Result<(), BoxError> {
        use schema::worklog::dsl::*;
        let worklog_obj: models::Worklog = models::Worklog::belonging_to(taskobj)
            .filter(stopped.is_null())
            .filter(ignored.eq(false))
            .first::<models::Worklog>(conn)?;
        let stop_timestamp = Utc::now().naive_local();
        let start_timestamp =
            NaiveDateTime::parse_from_str(&worklog_obj.started, "%Y-%m-%d %H:%M:%S")?;
        let spent_seconds: i64 = stop_timestamp
            .signed_duration_since(start_timestamp)
            .num_seconds();

        let seconds = i32::try_from(spent_seconds)?;
        diesel::update(&worklog_obj)
            .set((
                stopped.eq(stop_timestamp.format("%Y-%m-%d %H:%M:%S").to_string()),
                duration.eq(seconds),
            ))
            .execute(conn)?;
        Ok(())
    }

    pub fn get_timestamp() -> String {
        let nowstamp = Utc::now().naive_local();
        nowstamp.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::establish_connection;
    use std::path::PathBuf;
    use std::{thread, time};
    use tempfile::TempDir;

    fn setup() -> Result<(TempDir, PathBuf), BoxError> {
        let tempdir = TempDir::new()?;
        let mut dbpath = PathBuf::new();
        dbpath.push(tempdir.path());
        dbpath.push(".ttrackr.db");
        Ok((tempdir, dbpath))
    }

    fn create_task(
        conn: &SqliteConnection,
        taskname: &str,
        notes: Option<&str>,
        allocated: Option<i32>,
        duedate: Option<&str>,
    ) -> Result<(), BoxError> {
        let new_task = models::NewTask {
            taskname,
            notes,
            allocated,
            duedate,
        };

        let result = diesel::insert_into(schema::task::table)
            .values(&new_task)
            .execute(conn);
        match result {
            Ok(_val) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    fn pause(millis: u64) {
        let ten_millis = time::Duration::from_millis(millis);
        thread::sleep(ten_millis);
    }

    #[test]
    fn create_worklog() -> Result<(), BoxError> {
        let (_tempdir, dbpath) = setup()?;
        let conn_str = dbpath.to_string_lossy().to_string();
        let conn = establish_connection(&conn_str)?;
        self::create_task(&conn, "task1", None, None, None)?;
        let taskobj = helper::get_task(&conn, "task1")?;

        assert_eq!(helper::check_task_is_running(&conn, &taskobj)?, false);
        helper::create_worklog(&conn, taskobj.id)?;
        assert_eq!(helper::check_task_is_running(&conn, &taskobj)?, true);

        Ok(())
    }

    #[test]
    fn ignore_multiple_worklogs() -> Result<(), BoxError> {
        let (_tempdir, dbpath) = setup()?;
        let conn_str = dbpath.to_string_lossy().to_string();
        let conn = establish_connection(&conn_str)?;

        self::create_task(&conn, "task1", None, None, None)?;
        self::create_task(&conn, "task2", None, None, None)?;

        let taskobj = helper::get_task(&conn, "task1")?;

        helper::create_worklog(&conn, taskobj.id)?;
        self::pause(1100);
        helper::create_worklog(&conn, taskobj.id)?;

        use schema::worklog::dsl::*;
        let worklogs: Vec<models::Worklog> = models::Worklog::belonging_to(&taskobj)
            .filter(stopped.is_null())
            .filter(ignored.eq(false))
            .order(started.desc())
            .load::<models::Worklog>(&conn)?;

        debug!("{:?}", taskobj);
        debug!("{}", helper::check_task_is_running(&conn, &taskobj)?);
        debug!("{:?}", worklogs);

        assert_eq!(worklogs.len(), 2);

        helper::ignore_invalid_worklogs(&conn, &taskobj)?;

        let worklogs = models::Worklog::belonging_to(&taskobj)
            .filter(stopped.is_null())
            .filter(ignored.eq(false))
            .order(started.desc())
            .load::<models::Worklog>(&conn)?;

        assert_eq!(worklogs.len(), 1);

        Ok(())
    }

    #[test]
    fn stop_worklog() -> Result<(), BoxError> {
        let (_tempdir, dbpath) = setup()?;
        let conn_str = dbpath.to_string_lossy().to_string();
        let conn = establish_connection(&conn_str)?;

        self::create_task(&conn, "task1", None, None, None)?;
        self::create_task(&conn, "task2", None, None, None)?;

        let taskobj = helper::get_task(&conn, "task1")?;

        helper::create_worklog(&conn, taskobj.id)?;

        use schema::worklog::dsl::*;
        let worklogs = models::Worklog::belonging_to(&taskobj)
            .filter(stopped.is_null())
            .filter(ignored.eq(false))
            .order(started.desc())
            .load::<models::Worklog>(&conn)?;
        assert_eq!(worklogs.len(), 1);

        helper::stop_worklog(&conn, &taskobj)?;
        let worklogs = models::Worklog::belonging_to(&taskobj)
            .filter(stopped.is_null())
            .filter(ignored.eq(false))
            .order(started.desc())
            .load::<models::Worklog>(&conn)?;
        assert_eq!(worklogs.len(), 0);

        Ok(())
    }
}
