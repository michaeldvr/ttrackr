use super::schema::*;

#[derive(Debug, Queryable)]
pub struct Task {
    pub id: i32,
    pub created: String,
    pub taskname: String,
    pub notes: Option<String>,
    pub allocated: i32,
    pub duedate: Option<String>,
    pub done: bool,
}

#[derive(Debug, AsChangeset, Identifiable)]
#[table_name = "task"]
pub struct UpdateTask {
    pub id: i32,
    pub notes: Option<String>,
    pub allocated: i32,
    pub duedate: Option<String>,
    pub done: Option<bool>,
}

#[derive(Debug, Insertable, Default)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub taskname: &'a str,
    pub notes: Option<&'a str>,
    pub allocated: Option<i32>,
    pub duedate: Option<&'a str>,
}
