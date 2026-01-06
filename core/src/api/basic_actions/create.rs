use core::fmt;

use crate::{
    enums::TaskStatus,
    structs::{done::Done, pending::Pending},
};

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

impl fmt::Display for ItemTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Pending(pending) => write!(f, "Pending: {}", pending.super_struct.title),
            Self::Done(done) => write!(f, "Done: {}", done.super_struct.title),
        }
    }
}

pub fn create(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
    }
}
