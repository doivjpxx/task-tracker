use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskList {
    pub tasks: HashMap<usize, Task>,
    pub next_id: usize,
}

pub trait TaskListImpl {
    fn new() -> Self;
    fn load() -> Self;
    fn save(&self);
}

impl TaskListImpl for TaskList {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn load() -> Self {
        let path = PathBuf::from("tasks.json");
        if !path.exists() {
            return TaskList::new();
        }

        let data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&data).unwrap()
    }

    fn save(&self) {
        let data = serde_json::to_string_pretty(&self).unwrap();
        fs::write("tasks.json", data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::task::TaskImpl;

    use super::*;

    #[test]
    fn test_new_task_list() {
        let task_list = TaskList::new();
        assert_eq!(task_list.tasks.len(), 0);
        assert_eq!(task_list.next_id, 1);
    }

    #[test]
    fn test_load_task_list() {
        // At the moment the file does not exist
        let task_list = TaskList::load();
        assert_eq!(task_list.tasks.len(), 0);
        assert_eq!(task_list.next_id, 1);
    }

    #[test]
    fn test_save_task_list() {
        let mut task_list = TaskList::new();
        task_list
            .tasks
            .insert(1, Task::new(1, "Task 1".to_string()));
        task_list.next_id += 1;
        task_list.save();

        let loaded_task_list = TaskList::load();
        assert_eq!(loaded_task_list.tasks.len(), 1);
        assert_eq!(loaded_task_list.next_id, 2);
    }
}
