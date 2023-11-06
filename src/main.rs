mod tasks;
mod todo;
mod db;
//mod clap;
mod command;

use crate::{
    db::DB,
    tasks::{Tasks},
    todo::Todo,
};
pub use clap::{
    self,
    Arg, 
    Command, arg, Parser, command
};
use std::io::{Write, stdout, stdin};

//use std::result::Result;


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

    #[arg(short = 'f', long="finished", help="Set task as finished")]
    finished: bool,

    #[arg(short = 'c', long="check", help="Check if date already has passed")]
    check: bool,
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
            finished: false,
            check: false,
        }
    }
    pub fn parse(&mut self) -> Self {
        let args = self::clap::Parser::parse();
        args
    }
    pub fn check(&mut self) {
        let mut required_id = false;
        let mut required_for_new = false;
        let mut required_date = false;

        let mut date_str = "";
        let todo = Todo::get_todo();
        let Args = Args::new();
    
        let args = self.parse();
        let mut new_task = Tasks::new();
        let mut db = DB::new("tasks.db".to_string());
        
        let args = self.parse();
        let mut val_args = self.parse();

        let mut tasks = Tasks::new();

         // Handling boolean flags with 'if'
         if self.new && self.task_id == None {
            required_for_new = self.check_required_for_new();
        }

        if self.list {
            let todo = Todo::get_todo();
            todo.list(); // Assuming this is the correct method call
        }

        if self.remove {
        }

        if self.set_due_date {
            required_date = self.check_required_date();
        }

        if self.finished {
        }

        if self.check {
            required_date = self.check_required_date();
        }
        
         // Handling Option types with 'if let'
         if let Some(date) = &self.date {
            tasks.set_due_date(date);
        } else {
            if required_date {
                print!("Enter a date: ");
                let _ = stdout().flush();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                self.date = Some(input.trim().to_string());
                let date_str = self.date.unwrap().as_str();
            } else {
                self.date = Some("".to_string());
            }
        }

        loop {
            if let Some(task_id) = self.task_id {
                if task_id >= 0 {
                    break;
                } else {
                    self.task_id = None;
                }
            } else {
                if required_id {
                    print!("Enter a task id: ");
                    let _ = stdout().flush();
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    self.task_id = Some(input.parse::<i16>().unwrap());
                    break;
                } else {
                    break;
                }
            }
        }

        if let Some(title) = &self.title {
            tasks.set_title(title.unwrap());
        } else {
            if required_for_new {
                print!("Enter a title: ");
                let _ = stdout().flush();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                self.title = Some(input.trim().to_string());
            } else {
                self.title = Some("".to_string());
            }
        }

        if let Some(description) = &self.description {
            tasks.set_description(description)
        } else {
            if required_for_new {
                print!("Enter a description: ");
                let _ = stdout().flush();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                self.description = Some(input.trim().to_string());
            } else {
                self.description = Some("".to_string());
            }
        }
    }

    fn check_required_for_new(&mut self) -> bool {
        if self.description != None && self.title != None {
            false
        } else {
            true
        }
    }
    fn check_required_date(&mut self) -> bool {
        if self.date != None {
            false
        } else {
            true
        }
    }
    fn check_required_id(&mut self) -> bool {
        if self.task_id != None {
            false
        } else {
            true
        }
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
        let _ = Args.check();
    }
   
}