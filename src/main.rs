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

//use std::result::Result;
#[derive(Debug)] // This is necessary to comply with the Error trait
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

fn cli() -> Command {
    Command::new("todo_rust")
        .about("Todo Manager in Rust")
        .long_about(
            "This is a Todo manager application developed using the Rust programming language. It leverages SQLite for database management, ensuring efficient and reliable data storage."
        )
        .author("mm29942, mm29942@pm.me")
        .display_name("RustTodo")
        .arg(arg!(-c --check "Check if the finishing date is today.").action(ArgAction::SetTrue))
        .arg(arg!(-l --list "List all available tasks.").action(ArgAction::SetTrue))
        .subcommand(
            Command::new("task")
                .about("Edit an already existing task")
                .arg(arg!(-i --task_id <TASK_ID> "").default_value("0").required(true))
                .arg(arg!(-c --completed "").action(ArgAction::SetTrue).required(false))
                .arg(arg!(-r --remove "").action(ArgAction::SetTrue).required(false))
                .arg(arg!(--date <DATE> "Set a new or change the due date value for when the selected task should be completed.").required(false))
                .arg(arg!(-f --format <FORMAT> "Set a new format to display the date in.").default_value("%m/%d/%Y").required(false))
                .arg(arg!(-d --description  <DESCRIPTION> "Change description of the selected task").required(false))
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

fn check() {
    let mut task = Tasks::new();
    let matches = cli().get_matches();

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
        let format = format_str.unwrap().as_str();
        
        let description_str = sub_matches.get_one::<String>("description");
        let description = description_str.unwrap().as_str();

        let date_str = sub_matches.get_one::<String>("date");
        if let Some(date_str) = date_str {
            match Tasks::parse_date(date_str) {
                Ok(date) => {
                    let _ = task.task(task_id as i32, &title, &description, Some(&date.format("%Y-%m-%d").to_string()), Some(&format));
                    //let _ = task.task(task_id as i32, &title, &description, Some(&date), Some(&format));
                },
                Err(_) => {
                    eprintln!("Invalid date format provided.");
                    return; // Or handle the error appropriately
                }
                //_ => task.task(task_id as i32, &title, &description, None, Some(&format)),
            }
        }
        //let _ = task.task(task_id as i32, &title, &description, Some(&date), Some(&format));
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
            match Tasks::parse_date(date_str) {
                Ok(date) => {
                    let _ = task.task(task_id as i32, &title, &description, Some(&date.format("%Y-%m-%d").to_string()), Some(&format));
                },
                Err(_) => {
                    eprintln!("Invalid date format provided.");
                    return; // Or handle the error appropriately
                }
                //_ => task.task(task_id as i32, &title, &description, None, Some(&format)),
            }
        }
        db.insert(title, description, false, task.get_due_date(), format).unwrap();
    }

    if task.get_id() != 0 && description != "" {
        let _ = set_description(&description, todo.tasks[task_id as usize].get_id().try_into().unwrap());
    } 
    
    if task.get_id() != 0 && date != "" {
        let _ = set_due_date(&date, todo.tasks[task_id as usize].get_id().try_into().unwrap());
    }

    if task.get_id() != 0 && format != "" {
        let _ = set_format(&format, todo.tasks[task_id as usize].get_id().try_into().unwrap());
    }

    if task.get_id() != 0 && matches.get_flag("completed") {
        let _ = done(todo.tasks[task_id as usize].get_id().try_into().unwrap());
    }

    if task.get_id() != 0 && matches.get_flag("remove") {
        println!("Removing task with ID: {}", task_id);
        let _ = remove(todo.tasks[task_id as usize].get_id().try_into().unwrap());
    }

    let mut todo = Todo::get_todo();

    if matches.get_flag("list") {
        let list = todo.list();
        println!("{}", list.unwrap())
    }
    if matches.get_flag("check") {
        let _ = todo.check(todo.tasks[task_id as usize].get_id().try_into().unwrap());    
    }
}

fn main() {
    // Create a DB instance
    let db = DB::new("tasks.db".to_string());
    let _ = check();
}