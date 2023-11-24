mod todo;
mod db;
mod command;
mod tasks;

use crate::{
    db::{DB, set_description, set_due_date, set_format, remove, done},
    tasks::{Tasks, parse_date},
    todo::Todo,
};
use chrono::{format, NaiveDate, Local};
pub use clap::{
    self,
    Arg, 
    Command,
    arg, 
    Parser, 
    command,
    builder::OsStr,
    ArgAction
};
use std::{
    io::{Write, stdout, stdin},
    error::Error,
    fmt::{self, Display},
    result::Result,
};
use std::convert::From;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Done,
    Remove,
}

#[derive(Debug)]
enum ArgErr {
    InvalidDate,
    InvalidTaskId,
    InvalidFormat,
    InvalidTask,
    InvalidDescription,
    InvalidTitle,
    InvalidDueDate,
    InvalidDone,
    InvalidCheck,
    InvalidRemove,
    InvalidList,
    InvalidSetDueDate,
    InvalidNew,
    InvalidDateDatatype,
}

impl Display for ArgErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgErr::InvalidDate => write!(f, "Invalid date format. Please use YYYY-MM-DD format."),
            ArgErr::InvalidTaskId => write!(f, "Invalid task ID. A positive integer is required."),
            ArgErr::InvalidFormat => write!(f, "Invalid format. Please provide a correct format string."),
            ArgErr::InvalidTask => write!(f, "Invalid task. Please specify a valid task."),
            ArgErr::InvalidDescription => write!(f, "Invalid description. A non-empty string is required."),
            ArgErr::InvalidTitle => write!(f, "Invalid title. A non-empty string is required."),
            ArgErr::InvalidDueDate => write!(f, "Invalid due date. Please provide a valid due date."),
            ArgErr::InvalidDone => write!(f, "Invalid done status. Please specify true or false."),
            ArgErr::InvalidCheck => write!(f, "Invalid check. An issue occurred during checking."),
            ArgErr::InvalidRemove => write!(f, "Invalid remove operation. A task ID is required to remove a task."),
            ArgErr::InvalidList => write!(f, "Invalid list operation. An issue occurred during listing tasks."),
            ArgErr::InvalidSetDueDate => write!(f, "Invalid set due date operation. A valid task ID and due date are required."),
            ArgErr::InvalidNew => write!(f, "Invalid new task creation. Required parameters for creating a task are missing."),
            ArgErr::InvalidDateDatatype => write!(f, "Invalid datatype of date. Please provide a valid date."),
        }
    }
}
impl std::error::Error for ArgErr {}

async fn cli() -> Command {
    Command::new("todo_rust")
        .about("Todo Manager in Rust")
        .long_about(
            "This is a Todo manager application developed using the Rust programming language. It leverages SQLite for database management, ensuring efficient and reliable data storage."
        )
        .author("mm29942, mm29942@pm.me")
        .display_name("RustTodo")
        .arg(arg!(--check "Check if the finishing date is today.").action(ArgAction::SetTrue).required(false))
        .arg(arg!(-l --list "List all available tasks.").action(ArgAction::SetTrue).required(false))
        .arg(arg!(-c --cli "Start interactive prompt controler.").action(ArgAction::SetTrue).required(false))
        .arg(arg!(-d --done <DONE> "Mark task as completed").required(false))
        .arg(arg!(-r --remove <REMOVE> "Remove the selected task").required(false))
        .subcommand(
            Command::new("task")
                .about("Edit an already existing task")
                .arg(arg!(-i --task_id <TASK_ID> "Set the task's id").default_value("0").required(true))
                .arg(arg!(--date <DATE> "Set a new or change the due date value for when the selected task should be completed.").required(false))
                .arg(arg!(-f --format <FORMAT> "Set a new format to display the date in.").default_value("%m/%d/%Y").required(false))
                .arg(arg!(-d --description <DESCRIPTION> "Change description of the selected task").required(false))
        )
        .subcommand(
            Command::new("new")
                .about("Create a new task")
                .arg(arg!(-t --title <TITLE> "Set the title of the new task").required(true))
                .arg(arg!(-d --description  <DESCRIPTION> "Describe the new task").required(true))
                .arg(arg!(--date <DATE> "Set the date at wich the task should be completed.").required(false))
                .arg(arg!(-f --format <FORMAT> "Set the format the date will be displayed.").default_value("%m/%d/%Y").required(false))
        )
}

async fn check() {
    let mut task = Tasks::new();
    let matches = cli().await.get_matches();

    let mut todo = Todo::get_todo();

    let mut title = String::new();
    let mut description = String::new();
    let mut date = String::new();
    let mut format = String::new();
    let mut done_bool = false;
    let mut remove_bool = false;

    let mut db = DB::new("tasks.db".to_string());
    let mut task_id: i32 = 0;

    if let Some(sub_matches) = matches.subcommand_matches("task") {
        let id_str = sub_matches.get_one::<String>("task_id");
        let id: usize = id_str.expect("REASON").parse().unwrap();
        let mut task_id: usize = (id - 1).try_into().unwrap();
        let format_str = sub_matches.get_one::<String>("format");
        let format = if let Some(format_str) = format_str {
            set_format(format_str, todo.tasks[task_id].get_id().try_into().unwrap())
        } else {
            Ok(())
        };

        let description_str = sub_matches.get_one::<String>("description");
        let description = if let Some(description_str) = description_str {
            set_description(description_str, todo.tasks[task_id].get_id().try_into().unwrap())
        } else {
            Ok(())
        };

        let date_str = sub_matches.get_one::<String>("date");
        if let Some(date_str) = date_str {
            let formatted_date_str = if date_str.len() == 8 && date_str.chars().nth(6) == Some('/') {
                format!("20{}", date_str)
            } else {
                date_str.to_string()
            };

            match Tasks::parse_date(&formatted_date_str) {
                Ok(date) => {
                    set_due_date(date_str.to_string(), todo.tasks[task_id].get_id().try_into().unwrap()).unwrap();
                },
                Err(_) => {
                    eprintln!("Invalid date format provided.");
                }
            };
        }
    } 
    if let Some(sub_matches) = matches.subcommand_matches("new") {
        let format_str = sub_matches.get_one::<String>("format").expect("Not a string").as_str();
        let format = format_str;

        let title_str = sub_matches.get_one::<String>("title");
        let title = title_str.unwrap().as_str();

        let description_str = sub_matches.get_one::<String>("description");
        let description = description_str.unwrap().as_str();

        let date_str = sub_matches.get_one::<String>("date");
        if let Some(date_str) = date_str {
            let formatted_date_str = if date_str.len() == 8 && date_str.chars().nth(2) == Some('/') && date_str.chars().nth(5) == Some('/') {
                let parts: Vec<&str> = date_str.split('/').collect();
                if parts.len() == 3 && parts[2].len() == 2 {
                    format!("{}-{}-20{}", parts[1], parts[0], parts[2])
                } else {
                    eprintln!("Invalid date format provided.");
                    return;
                }
            } else {
                eprintln!("Invalid date format provided.");
                return;
            };

            match NaiveDate::parse_from_str(&formatted_date_str, "%m-%d-%Y") {
                Ok(parsed_date) => {
                    db.insert(title, description, false, Some(parsed_date), format).unwrap();
                },
                Err(_) => {
                    eprintln!("Error parsing date string.");
                    return;
                }
            }
        } else {
            db.insert(title, description, false, None, format).unwrap();
        }
    }

    let mut todo = Todo::get_todo();

    if matches.get_flag("list") {
        let list = todo.list();
        println!("{}", list.unwrap())
    }

    if matches.get_flag("cli") {
        let _ = todo.clone().interactive_mode();
    }

    if matches.get_flag("check") {
        let _ = todo.check_all();
    }

    if let Some(done_str) = matches.get_one::<String>("done") {
        if let Ok(done_id) = done_str.parse::<usize>() {
            let _ = done(todo.tasks[done_id - 1].get_id().try_into().unwrap());
        } else {
            eprintln!("Invalid task ID provided for done.");
        }
    }

    if let Some(remove_str) = matches.get_one::<String>("remove") {
        if let Ok(remove_id) = remove_str.parse::<usize>() {
            let _ = remove(todo.tasks[remove_id - 1].get_id().try_into().unwrap()).expect("Error processing remove operation");
        } else {
            eprintln!("Invalid task ID provided for remove.");
        }
    }

    
}

#[tokio::main]
async fn main() {
    let db = DB::new("tasks.db".to_string());
    let _ = check().await;
}

async fn change_done_or_delete(task: Operation, task_id: usize) -> Result<(), Box<dyn Error>> {
    if task == Operation::Done {
        done(task_id.try_into().unwrap())?;
    } else if task == Operation::Remove {
        remove(task_id.try_into().unwrap())?;
    }
    Ok(())
}