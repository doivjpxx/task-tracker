use task::{Task, TaskImpl};
use task_list::{TaskList, TaskListImpl};

mod task;
mod task_list;

fn main() {
    println!("Task tracker CLI application");
    let task_list = TaskList::new();

    let task = Task::new(1, "Learn Rust".to_string());

    println!("{:?}", task);
    println!("{:?}", task_list);
}
