use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Copy)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub status: Status,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait TaskImpl {
    fn new(id: usize, description: String) -> Self;
}

impl TaskImpl for Task {
    fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            status: Status::Todo,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}
