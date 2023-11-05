pub use clap::{
    Arg, 
    Command, arg, Parser, command
};
use std::io::{stdin, stdout, Write};


use crate::db::DB;
use crate::{tasks::Tasks, todo::Todo};


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

pub fn args_parse() {
    let mut db = DB::new("tasks.db".to_string());
    let _ = db.connect();

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
        list => {
            match db.select_all() {
                Ok(mut stmt) => {
                    loop {
                        match stmt.next() {
                            Ok(sqlite3::State::Row) => {
                                // Assume the task title is in the first column, and description is in the second column.
                                // Adjust column indices as necessary based on your database schema.
                                let title: String = stmt.read::<String>(0).unwrap();
                                let description: String = stmt.read::<String>(1).unwrap();
                                println!("Title: {}, Description: {}", title, description);
                            }
                            Ok(sqlite3::State::Done) => break,
                            Err(e) => {
                                println!("Failed to read row: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => println!("Failed to execute statement: {}", e),
            }
        },
        ref set_due_date => {},
        ref finished => {},
        ref check => {},
    }
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