use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task-cli")]
#[command(version = "1.0")]
#[command(about = "A CLI task tracker application")]
pub struct CLI {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Add { description: String },
    Update { id: usize, description: String },
    Delete { id: usize },
    MarkInProgress { id: usize },
    MarkDone { id: usize },
    List { status: Option<String> },
}

