use super::{get_connection, models, schema, BoxError, Config};
use diesel;
use diesel::prelude::*;
// use std::io::{stdin, Read};

pub fn create_task(
    config: &Config,
    taskname: &str,
    notes: Option<&str>,
    duration: Option<i32>,
) -> Result<(), BoxError> {
    let conn = get_connection(config.database.get("path").unwrap());
    let new_task = models::NewTask {
        taskname: taskname,
        notes: notes,
        duration: duration,
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
