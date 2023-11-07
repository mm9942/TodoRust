mod tasks;
mod todo;
mod db;
mod command;

use crate::{
    db::DB,
    tasks::{Tasks},
    todo::Todo,
};
use chrono::{format, NaiveDate, Local};
pub use clap::{
    self,
    Arg, 
    Command,
    arg, 
    Parser, 
    command
};
use std::{
    io::{Write, stdout, stdin},
    error::Error,
    fmt::{self, Display},
    result::Result,
};

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

#[derive(Parser, Debug, PartialEq)]
#[command(name = "tasks", version = "0.1.0", long_about = "A task manager for better management of a time")]
pub struct Args {
    #[arg(short='n', long="new", help="Create a new task")]
    new: bool,

    #[arg(short='d', long="date", help="Define a date for the new task or to set due date")]
    date: Option<String>,

    #[arg(short='t', long="task", help="Define wich task to address")]
    task_id: Option<i16>,
    
    #[arg(long="title", help="Define a title for the new task")]
    title: Option<String>,

    #[arg(long="description", help="Define a description for the new task")]
    description: Option<String>,

    #[arg(short = 'r', long="rm", help="Remove a task")]
    remove: bool,

    #[arg(short = 'l', long="list", help="List all tasks")]
    list: bool,

    #[arg(long="set_due_date", help="Set due date")]
    set_due_date: bool,

    #[arg(long="done", help="Set task as finished")]
    done: bool,

    #[arg(short = 'c', long="check", help="Check if date already has passed")]
    check: bool,

    #[arg(short = 'f', long="format", help="Define a format for the new task")]
    format: Option<String>,
}
impl Args {
    pub fn new() -> Self {
        Self {
            new: false,
            date: None,
            task_id: None,
            title: None,
            description: None,
            remove: false,
            list: false,
            set_due_date: false,
            done: false,
            check: false,
            format: None,
        }
    }
    pub fn parse(&mut self) -> Self {
        let mut args: Args = Args::new();
        let args = args.parse();
        args
    }
    pub fn check(&mut self, db: DB) -> Result<(), ArgErr> {
        let mut required_id = false;
        let mut required_for_new = false;
        let mut required_date = false;

        let required_id = self.check_required_id().unwrap();
        let required_for_new = self.check_required_for_new().unwrap();
        let required_date = self.check_required_date().unwrap();

        
        let title = self.get_title()?;
        let date = self.date()?;
        let description = self.get_description()?;
        let task_id = self.get_task_id()?;
        let format = self.get_format()?;

        let mut date_str = "";
        let todo = Todo::get_todo();
        let Args = Args::new();
    
        let args = self.parse();
        let mut new_task = Tasks::new();
        let mut db = DB::new("tasks.db".to_string());
        
        let args = self.parse();
        let mut val_args = self.parse();

        let mut task = Tasks::new();
        let todo = Todo::get_todo();

         // Handling boolean flags with 'if'
        if self.new {
            // Check required arguments for creating a new task
            if self.title.is_none() || self.description.is_none() {
                return Err(ArgErr::InvalidNew);
            }

            // Parse the date or provide a default error message
            let due_date = match self.date { 
                Some(ref date_str) => NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap(),
                _ => Local::now().date().naive_local(),
            };

            // Insert the new task into the database, including the format
            db.insert(
                &self.title.as_ref().unwrap(),
                &self.description.as_ref().unwrap(),
                self.done,
                Some(due_date),
                "%Y-%m-%d" // Assuming a default format here, adjust as necessary
            ).unwrap();
            println!("New task created successfully.");
        }

        if self.list {
            todo.list();
        }

        if self.remove {
        }

        if self.set_due_date {
            let required_date = self.check_required_date().unwrap();
        }

        if self.done {
        }

        if self.check {
            let required_date = self.check_required_date().unwrap();
        }
        
        Ok(())
    }

    pub fn get_new_task(&mut self, title: String, description: String, date: String, format: String) {
        let mut task = Tasks::new();
        let _ = task.task(0, &title, &description, Some(&date), Some(&format));
    }

    fn check_required_for_new(&self) -> Result<bool, ArgErr> {
        match self.description.is_none() || self.title.is_none() {
            false => Ok(true),
            true => Ok(false),
            _ => Err(ArgErr::InvalidNew),
        }
    }

    fn check_required_date(&self) -> Result<bool, ArgErr> {
        match self.date.is_none() {
            false => Ok(true),
            true => Ok(false),
            _ => Err(ArgErr::InvalidDate),
        }
    }

    fn check_required_id(&self) -> Result<bool, ArgErr> {
        match self.task_id.is_none() {
            false => Ok(true),
            true => Ok(false),
            _ => Err(ArgErr::InvalidTaskId),
        }
    }
    fn date(&self) -> Result<NaiveDate, ArgErr> {
        self.date
            .as_ref()
            .ok_or(ArgErr::InvalidDate)
            .and_then(|date_str| {
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| ArgErr::InvalidDate)
            })
    }
    fn get_date(&self) -> String {
        if let Some(date) = &self.date {
            date.clone()
        } else if self.check_required_date().unwrap() {
            println!("Enter a date: ");
            stdout().flush().expect("Failed to flush stdout.");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line.");
            input.trim().to_string()
        } else {
            String::new()
        }
    }

    // Getters for title, description, and format have been adjusted to not use ?
    fn get_title(&self) -> Result<String, ArgErr> {
        self.title.clone().ok_or(ArgErr::InvalidTitle)
    }

    fn get_description(&self) -> Result<String, ArgErr> {
        self.description.clone().ok_or(ArgErr::InvalidDescription)
    }

    fn get_format(&self) -> Result<String, ArgErr> {
        self.format.clone().ok_or(ArgErr::InvalidFormat)
    }

    // Corrected get_task_id to return Result
    fn get_task_id(&self) -> Result<i16, ArgErr> {
        self.task_id.ok_or(ArgErr::InvalidTaskId)
    }

}
fn main() {
    let todo = Todo::get_todo();
    let mut db = DB::new("tasks.db".to_string());
    let mut Args = Args::new();
    let _ = Args.parse();
    if Args == Args::new() {
        let _ = todo.clone().interactive_mode().unwrap();
    } else {
        let _ = Args.check(db);
    }
   
}