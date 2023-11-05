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
    let args = Args::parse();

    let connection = sqlite3::open("tasks.db").unwrap();
    let mut tasks = Vec::new();

    // Query the database and fill the tasks vector
    connection.iterate("SELECT * FROM tasks", |pairs| {
        let mut task = Tasks::new();
        for &(column, value) in pairs.iter() {
            match column {
                "task" => task.task = value.unwrap_or_default().to_string(),
                "description" => task.description = value.unwrap_or_default().to_string(),
                "due_date" => task.due_date = value.and_then(|date| NaiveDate::parse_from_str(date, &task.format).ok()),
                "done" => task.done = value.unwrap_or_default().parse().unwrap_or_default(),
                _ => (),
            }
        }
        tasks.push(task);
        true
    }).unwrap();

    let todo = Todo::new(tasks);
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