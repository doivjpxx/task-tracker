use clap::Parser;
use cli::{Command, CLI};
use task::{Task, TaskImpl};
use task_list::{TaskList, TaskListImpl};

mod cli;
mod task;
mod task_list;

fn main() {
    println!("Task tracker CLI application");
    let mut task_list: TaskList = TaskList::load();

    let args = CLI::parse();

    match args.command {
        Command::Add { description } => {
            let task = Task::new(task_list.next_id, description);
            task_list.tasks.insert(task.id, task);
            task_list.next_id += 1;
            task_list.save();
        }
        Command::Update { id, description } => {
            let task = task_list.tasks.get_mut(&id).unwrap();
            task.update(description);
            task_list.save();
        }
        Command::Delete { id } => {
            task_list.tasks.remove(&id);
            task_list.save();
        }
        Command::MarkInProgress { id } => {
            let task = task_list.tasks.get_mut(&id).unwrap();
            task.mark_in_progress();
            task_list.save();
        }
        Command::MarkDone { id } => {
            let task = task_list.tasks.get_mut(&id).unwrap();
            task.mark_done();
            task_list.save();
        }
        Command::List { status } => {
            let tasks = task_list.tasks.values();
            if let Some(status) = status {
                let status = match status.as_str() {
                    "todo" => task::Status::Todo,
                    "in-progress" => task::Status::InProgress,
                    "done" => task::Status::Done,
                    _ => panic!("Invalid status"),
                };

                let tasks = tasks.filter(|task| task.status == status);
                for task in tasks {
                    println!("{:?}", task);
                }
            } else {
                for task in tasks {
                    println!("{:?}", task);
                }
            }
        }
    }
}
