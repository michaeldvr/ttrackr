use super::{get_connection, models, schema, BoxError, Config};
use diesel::prelude::*;
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
        Ok(val) => {
            println!("result: {}", val);
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
    let taskid = get_task_id(config, name)?;
    let updatetask = models::UpdateTask {
        id: taskid,
        notes: notes.map(String::from),
        allocated: allocated.unwrap_or(0),
        duedate: duedate.map(String::from),
        done,
    };
    let conn = get_connection(config)?;
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

fn get_task_id(config: &Config, name: &str) -> Result<i32, BoxError> {
    use schema::task::dsl::*;
    let conn = get_connection(config)?;
    let taskid = task
        .select(id)
        .filter(taskname.eq(name))
        .first::<i32>(&conn);
    match taskid {
        Ok(val) => Ok(val),
        Err(err) => Err(err.into()),
    }
}
