pub use clap::{
    Arg, 
    Command, arg, Parser, command
};
use std::io::{stdin, stdout, Write};
use std::vec;
use crate::db::DB;
use crate::{tasks::Tasks, todo::Todo};

use std::collections::HashMap;

use chrono::NaiveDate;
use sqlite3::{State, *};
use sqlite3;
std::result::Result;


pub fn args_parse() {
    let mut db = DB::new("tasks.db".to_string());
    let _ = db.select_all();

    let args = Args::parse();
    let mut new_task = Tasks::new();
    let mut task_id = args.task;
    match args {
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
        ref list => {
            fn get_list();
        },
        ref set_due_date => {},
        ref finished => {},
        ref check => {},
    }
}
pub fn get_list() -> Result<Vec<Tasks>, Error> {
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
                vec![Taskss.push(task)];
            }
            Ok(Taskss);
}
/*
pub fn cmd() -> Command {

    Command::new("TaskManager")
    .version("0.1.0")
    .about("A task manager for better management of time")
    .subcommand(
        Command::new("new")
            .about("Create a new task")
            .arg(Arg::new("title").help("Title of the task").required(true))
            .arg(Arg::new("description").help("Description of the task").required(true))
            .arg(Arg::new("due_date").help("Due date for the task").required(false)),
    )
    .subcommand(
        Command::new("done")
            .about("Mark a task as done")
            .arg(Arg::new("task_id").help("ID of the task to mark as done").required(true)),
    )
    .subcommand(
        Command::new("rm")
            .about("Remove a task")
            .arg(Arg::new("task_id").help("ID of the task to remove").required(true)),
    )
    .subcommand(
        Command::new("set_due_date")
            .about("Set a due date for a task")
            .arg(Arg::new("task_id").help("ID of the task to set due date for").required(true))
            .arg(Arg::new("due_date").help("New due date for the task").required(true)),
    )
    .subcommand(
        Command::new("list")
            .about("List all tasks")
            .arg(Arg::new("all").help("List all tasks, including done tasks").short('a').long("all")),
    )
    .subcommand(
        Command::new("check")
            .about("Check if a task is overdue")
            .arg(Arg::new("task_id").help("ID of the task to check").required(true)),
    )
    .subcommand(
        Command::new("version")
            .about("Prints version information"),
    )

}
*/