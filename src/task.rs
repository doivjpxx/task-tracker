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
    fn update(&mut self, description: String);
    fn mark_in_progress(&mut self);
    fn mark_done(&mut self);
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

    fn update(&mut self, description: String) {
        self.description = description;
        self.updated_at = chrono::Local::now().naive_local();
    }

    fn mark_in_progress(&mut self) {
        self.status = Status::InProgress;
        self.updated_at = chrono::Local::now().naive_local();
    }

    fn mark_done(&mut self) {
        self.status = Status::Done;
        self.updated_at = chrono::Local::now().naive_local();
    }
}
