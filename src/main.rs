mod tasks;
mod clap;
mod command;
mod todo;
use crate::todo::Todo;
use crate::clap::{cmd, args_parse};
pub use crate::command::Command;

pub use crate::tasks::{Tasks, TasksErr};
pub use std::io::{stdin, stdout, Write};
pub use std::result::Result;

fn main() {
    let _command = cmd();
    let _arg = args_parse();
    let mut tasks_vec = Vec::new();
    let task_result = Tasks::add("new", "test", None);
    match task_result {
        Ok(mut task) => {
            let _ = task.set_due_date("31.12.2023").unwrap();
            let _ = task.set_format("%d.%m.%Y");
            let _ = task.done();
            tasks_vec.push(task);
        }
        Err(e) => eprintln!("Failed to create task: {}", e),
    }
    let task_with_due_date_result = Tasks::add("new", "test", Some("24.12.2023"));
    match task_with_due_date_result {
        Ok(mut task_with_due_date) => {
            let _ = task_with_due_date.set_format("%d.%m.%Y").unwrap();
            tasks_vec.push(task_with_due_date);
        }
        Err(e) => eprintln!("Failed to create task: {}", e),
    }

    let new_task_result = Tasks::add("urgent", "needs to be done", Some("2021-11-05"));
    match new_task_result {
        Ok(mut new_task) => {
            let _ = new_task.set_format("%d.%m.%Y").unwrap();
            tasks_vec.push(new_task);
        }
        Err(e) => eprintln!("Failed to create task: {}", e),
    }

    let todo = Todo::new(tasks_vec); // Just pass tasks_vec directly
    todo.interactive_mode().unwrap();
}
