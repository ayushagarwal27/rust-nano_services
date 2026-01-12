use crate::enums::TaskStatus;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

impl fmt::Display for ToDoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            TaskStatus::PENDING => write!(f, "Pending: {}", self.title),
            TaskStatus::DONE => write!(f, "Done: {}", self.title),
        }
    }
}
