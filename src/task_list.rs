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
