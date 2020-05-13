use super::schema::*;

#[derive(Identifiable, Debug, Queryable)]
#[table_name = "task"]
pub struct Task {
    pub id: i32,
    pub created: String,
    pub taskname: String,
    pub notes: Option<String>,
    pub allocated: i32,
    pub duedate: Option<String>,
    pub done: bool,
}

impl Task {
    pub fn create_changeset(&self) -> UpdateTask {
        let duedate = match &self.duedate {
            Some(v) => Some(v.to_owned()),
            None => None,
        };
        let notes = match &self.notes {
            Some(v) => Some(v.to_owned()),
            None => None,
        };
        UpdateTask {
            id: self.id,
            notes,
            allocated: self.allocated,
            duedate,
            done: Some(self.done),
        }
    }
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

#[derive(Identifiable, Associations, Debug, Queryable)]
#[belongs_to(Task)]
#[table_name = "worklog"]
pub struct Worklog {
    pub id: i32,
    pub task_id: i32,
    pub started: String,
    pub stopped: Option<String>,
    pub duration: i32,
    pub ignored: bool,
}

#[derive(Debug, Insertable, Default)]
#[table_name = "worklog"]
pub struct NewWorklog {
    pub task_id: i32,
}
