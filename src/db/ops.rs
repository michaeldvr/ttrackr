use super::{get_connection, models, schema};
use diesel;
use diesel::prelude::*;
// use std::io::{stdin, Read};

pub fn create_task(taskname: &str, notes: Option<&str>, duration: Option<i32>) -> usize {
    let conn = get_connection("/home/michael/Documents/Projects/ttrackr/ttrackr.db");
    /*let mut taskname = String::new();
    println!("Task name:");
    stdin().read_line(&mut taskname).unwrap();
    let taskname = &taskname[..(taskname.len()-1)];
    let notes = None;
    let duration = 0;*/
    let new_task = models::NewTask {
        taskname: taskname,
        notes: notes,
        duration: duration,
    };

    diesel::insert_into(schema::task::table)
        .values(&new_task)
        .execute(&conn)
        .expect("Error saving new task")
}
