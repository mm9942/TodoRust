pub use clap::{
    Arg, ArgGroup, ArgMatches, 
    Command, arg, Subcommand, Parser, Error, command , ValueEnum
};
use std::io::{stdin, stdout, Write};
use std::result::Result;

use crate::{tasks::Tasks, todo::Todo};


#[derive(Parser, Debug)]
#[command(name = "tasks", version = "0.1.0", long_about = "A task manager for better management of a time")]
pub struct Args {
    #[arg(short='n', long="new", help="Create a new task")]
    new: bool,

    #[arg(short='d', long="date", help="Define a date for the new task or to set due date")]
    date: Option<String>,

    #[arg(short='t', long="task", help="Define wich task to address")]
    task: i16,
    
    #[arg(long="title", help="Define a title for the new task")]
    title: String,

    #[arg(long="description", help="Define a description for the new task")]
    description: String,

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
    let cli = cmd().get_matches();
    match cli.args_present() {
        new => {
            println!("new");
        }
        due_date => {
            println!("due_date");
        }
        task => {
            println!("task");
        }
        remove => {
            println!("remove");
        }
        list => {
            println!("list");
        }
        set_due_date => {
            println!("set_due_date");
        }
        finished => {
            println!("finished");
        }
        check => {
            println!("check");
        }
    }

    let args = Args::parse();
    let mut new_task = Tasks::new();
    let mut task_id = args.task;
    match args {
        ref new => {
            if args.date != None && !args.title.is_empty() && !args.description.is_empty() {
                let date = args.date.unwrap();
                let _ = new_task.task(&args.title, &args.description, Some(date.as_str())).unwrap();
            } else if !args.title.is_empty() && !args.description.is_empty() {
                let _ = new_task.task(&args.title, &args.description, None).unwrap();
            } else if !args.title.is_empty() {
                print!("Enter a description for the task: ");
                let _ = stdout().flush();
                let mut description = String::new();
                stdin().read_line(&mut description).unwrap();
                let _ = new_task.task(&args.title, &description, None).unwrap();
            } else if !args.description.is_empty() {
                print!("Enter a title for the task: ");
                let _ = stdout().flush();
                let mut title = String::new();
                stdin().read_line(&mut title).unwrap();
                let _ = new_task.task(&title, &args.description, None).unwrap();
            }
        },
        ref due_date => {
            let mut tdv = Vec::new();
            let mut td = Todo::new(tdv);
        },
        ref task => {
            if args.task <= 0 {
                print!("Enter a task id: ");
                let _ = stdout().flush();
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                task_id = input.parse::<i16>().unwrap();
            }
        },
        remove => {},
        list => {},
        set_due_date => {},
        finished => {},
        check => {},
    }
    // Continued program logic goes here...
}
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
