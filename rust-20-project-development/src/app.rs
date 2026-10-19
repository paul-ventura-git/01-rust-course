use std::error::Error;
use std::fmt;

use crate::task_store::TaskStore;

#[derive(Debug)]
pub enum AppError {
    InvalidCommand(String),
    InvalidTaskId(String),
    UnknownTask(usize),
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCommand(command) => write!(formatter, "invalid command: {command}"),
            Self::InvalidTaskId(id) => write!(formatter, "invalid task ID: {id}"),
            Self::UnknownTask(id) => write!(formatter, "no task exists with ID {id}"),
        }
    }
}

impl Error for AppError {}

fn execute(command: &str, store: &mut TaskStore) -> Result<(), AppError> {
    let (name, argument) = command
        .split_once(' ')
        .map_or((command, ""), |(name, argument)| (name, argument.trim()));

    match name {
        "add" if !argument.is_empty() => {
            let id = store.add(argument);
            println!("Added task {id}: {argument}");
        }
        "done" => {
            let id = argument
                .parse::<usize>()
                .map_err(|_| AppError::InvalidTaskId(argument.to_owned()))?;
            if !store.complete(id) {
                return Err(AppError::UnknownTask(id));
            }
            println!("Completed task {id}");
        }
        "list" if argument.is_empty() => {
            for task in store.tasks() {
                let status = if task.completed { "done" } else { "open" };
                println!("{} [{status}] {}", task.id, task.title);
            }
        }
        _ => return Err(AppError::InvalidCommand(command.to_owned())),
    }

    Ok(())
}

pub fn run_demo() -> Result<(), AppError> {
    println!("\n--- A small, modular task tracker ---");

    let mut store = TaskStore::default();
    for command in [
        "add Learn ownership",
        "add Write a Rust program",
        "done 1",
        "list",
    ] {
        execute(command, &mut store)?;
    }
    Ok(())
}
