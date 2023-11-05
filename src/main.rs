mod tasks;
mod todo;
mod db;
//mod clap;
mod command;
use std::env::args;
use crate::{
    tasks::{Tasks, TasksErr},
    todo::Todo,
    db::DB,
};
pub use clap::{
    Arg, 
    Command, arg, Parser, command
};
use std::io::{self, Write, stdout, stdin};
use std::fmt::Display;
//use std::result::Result;

use std::collections::HashMap;

use chrono::NaiveDate;
use sqlite3::{Result, State, *};
use sqlite3;

#[derive(Parser, Debug)]
#[command(name = "tasks", version = "0.1.0", long_about = "A task manager for better management of a time")]
pub struct Args {
    #[arg(short='n', long="new", help="Create a new task")]
    new: bool,

    #[arg(short='d', long="date", help="Define a date for the new task or to set due date")]
    date: Option<String>,

    #[arg(short='t', long="task", help="Define wich task to address")]
    task: Option<i16>,
    
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

fn main() {

    let mut pairs: HashMap<String, String> = HashMap::new();
    let connection = sqlite3::open("tasks.db").unwrap();
    connection.iterate("SELECT * FROM tasks", |rows| {
        for &(column, value) in rows.iter() {
            if let Some(val) = value {
                pairs.insert(column.to_string(), val.to_string());
            }
        }
        true
    })
    .unwrap();
    
    let mut task = Tasks::new();
    let mut Taskss = Vec::new();
    let no_description = "No description found".to_string();
    let default_format = "%d.%m.%Y".to_string();
    for (column, value) in &pairs {
        println!("{}-{}", column, value);
        let description = pairs.get("description").unwrap_or(&no_description);
        let format = pairs.get("format").unwrap_or(&default_format);  
        let due_date = match pairs.get("due_date") {
            Some(date_str) => {
                // If you have a due_date key, you can convert the date string to the desired format here
                // assuming the date string is in the format specified by the 'format' value
                let formatted_date = date_str;  // replace this with actual formatting code
                Some(formatted_date)
            },
            None => None,
        };
        let _ = task.task(column, description, None);
        let _ = Taskss.push(task.to_owned());
    }
    let todo = Todo::new(Taskss);
    let args = Args::parse();
    let mut new_task = Tasks::new();
    let mut task_id = args.task;
    match args {
        list => {
        },
        ref new => {
            if args.date != None && args.title != None && args.description != None {
                let date = args.date.unwrap();
                let _ = new_task.task(&args.title.unwrap(), &args.description.unwrap(), Some(date.as_str())).unwrap();
            } else if args.title != None && args.description != None {
                let _ = new_task.task(&args.title.unwrap(), &args.description.unwrap(), None).unwrap();
            } else if args.title != None {
                print!("Enter a description for the task: ");
                let _ = stdout().flush();
                let mut description = String::new();
                stdin().read_line(&mut description).unwrap();
                let _ = new_task.task(&args.title.unwrap(), &description, None).unwrap();
            } else if args.description != None {
                print!("Enter a title for the task: ");
                let _ = stdout().flush();
                let mut title = String::new();
                stdin().read_line(&mut title).unwrap();
                let _ = new_task.task(&title, &args.description.unwrap(), None).unwrap();
            }
        },
        ref due_date => {
            let tdv = Vec::new();
            let _td = Todo::new(tdv);
        },
        ref task => {
            if args.task <= Some(0)  {
                print!("Enter a task id: ");
                let _ = stdout().flush();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                task_id = Some(input.parse::<i16>().unwrap());
            }
        },
        ref remove => {},
        ref set_due_date => {},
        ref finished => {},
        ref check => {},
    }
    let _ = todo.clone().interactive_mode().unwrap();
}

/*
fn main() {
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
        },
        Err(e) => eprintln!("Failed to create task: {}", e),
    }

    let new_task_result = Tasks::add("urgent", "needs to be done", Some("2021-11-05"));
    match new_task_result {
        Ok(mut new_task) => {
            let _ = new_task.set_format("%d.%m.%Y").unwrap();
            tasks_vec.push(new_task);
        },
        Err(e) => eprintln!("Failed to create task: {}", e),
    }

    let todo = Todo::new(tasks_vec);  // Just pass tasks_vec directly
}*/