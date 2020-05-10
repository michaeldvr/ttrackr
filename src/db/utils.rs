use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct TaskNotFound;

impl Display for TaskNotFound {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Task not found")
    }
}

impl std::error::Error for TaskNotFound {
    fn description(&self) -> &str {
        "Task not found"
    }

    fn cause(&self) -> Option<&(dyn std::error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct TaskIsAlreadyRunning {
    pub taskname: String,
}

impl Display for TaskIsAlreadyRunning {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", format!("{} is already running", self.taskname))
    }
}

impl std::error::Error for TaskIsAlreadyRunning {
    fn description(&self) -> &str {
        "Task is already running"
    }

    fn cause(&self) -> Option<&(dyn std::error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

#[derive(Debug, Clone)]
pub struct TaskIsNotRunning {
    pub taskname: String,
}

impl Display for TaskIsNotRunning {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", format!("{} is not running", self.taskname))
    }
}

impl std::error::Error for TaskIsNotRunning {
    fn description(&self) -> &str {
        "Task is not running"
    }

    fn cause(&self) -> Option<&(dyn std::error::Error)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
