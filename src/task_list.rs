use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskList {
    pub tasks: HashMap<uuid::Uuid, Task>,
    pub next_id: usize,
}

pub trait TaskListImpl {
    fn new() -> Self;
}

impl TaskListImpl for TaskList {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }
}
