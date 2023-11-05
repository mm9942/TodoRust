mod tasks;
mod todo;
mod db;
mod clap;
mod command;
use crate::{
    tasks::{Tasks, TasksErr},
    todo::Todo,
    db::DB,
    clap::{Args,args_parse},
};
use std::io::{self, Write, stdout, stdin};
use std::fmt::Display;
//use std::result::Result;
use clap::Parser;

use chrono::NaiveDate;
use sqlite3::{Result, State, *};
use sqlite3;
fn main() {
    match Args::parse() {
        list => {
            let connection = sqlite3::open(":tasks:").unwrap();
            connection.iterate("SELECT * FROM users WHERE age > 50", |pairs| {
                for &(column, value) in pairs.iter() {
                    println!("{} = {}", column, value.unwrap());
                }
                true
            })
            .unwrap();
        }
    }
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