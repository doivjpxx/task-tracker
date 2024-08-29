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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let task = Task::new(1, "Task 1".to_string());
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Task 1");
        assert_eq!(task.status, Status::Todo);
    }

    #[test]
    fn test_update_task() {
        let mut task = Task::new(1, "Task 1".to_string());
        task.update("Updated Task 1".to_string());
        assert_eq!(task.description, "Updated Task 1");
    }

    #[test]
    fn test_mark_in_progress() {
        let mut task = Task::new(1, "Task 1".to_string());
        task.mark_in_progress();
        assert_eq!(task.status, Status::InProgress);
    }

    #[test]
    fn test_mark_done() {
        let mut task = Task::new(1, "Task 1".to_string());
        task.mark_done();
        assert_eq!(task.status, Status::Done);
    }
}
