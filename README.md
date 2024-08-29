## Backend Projects by [roadmap.sh](https://roadmap.sh)

This repository contains a list of project solutions for the [Backend Developer](https://roadmap.sh/backend) roadmap. The projects are divided into three categories: beginner, intermediate, and advanced.

Solution for each project is in the respective folder. The folder name is the same as the project name. Each project folder contains a `readme.md` file that describes the project and the solution.

### Beginner Projects

1. [Task Tracker](https://roadmap.sh/projects/task-tracker)
2. [GitHub User Activity](https://roadmap.sh/projects/github-user-activity)
3. [Expense Tracker](https://roadmap.sh/projects/expense-tracker)

You can find the list of beginner projects [here](https://roadmap.sh/backend/projects?difficulty=beginner).

---
# Task Tracker CLI Application

This is a command-line interface (CLI) application for tracking tasks. It allows users to add, update, delete, and mark tasks as in progress.

## Installation

To install the application, you need to have Rust installed on your machine. If you don't have Rust installed, you can get it from [here](https://www.rust-lang.org/tools/install).

Clone the repository and build the application:

```sh
git clone <repository-url>
cd <repository-directory>
cargo build --release
```

## Usage
Run the application using the following command:

```sh
cargo run -- <command> [options]
```

### Commands

- `add`: Add a new task.
```sh
cargo run -- add "Task description"
```
- `update`: Update an existing task.
```sh
cargo run -- update <task-id> "New task description"
```
- `delete`: Delete a task.
```sh
cargo run -- delete <task-id>
```
- `list [status]`: List tasks based on status.
```sh
cargo run -- list "todo"
cargo run -- list "in-progress"
cargo run -- list "done"
```
- `in-progress`: Mark a task as in progress.
```sh
cargo run -- in-progress <task-id>
```
- `done`: Mark a task as done.
```sh
cargo run -- done <task-id>
```

## EXAMPLES

- Add a new task:
```sh
cargo run -- add "Implement user authentication"
```

Thank you for reading! 🚀
