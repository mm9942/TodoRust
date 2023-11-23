pub use crate::tasks::{Tasks, TasksErr};
pub use std::io::{stdin, stdout, Write};
pub use std::result::Result;
use crate::db::{remove, done, update, set_format, DB, set_due_date};

use chrono::{NaiveDate, Local};
#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub tasks: Vec<Tasks>,
    //command: Command,
}
impl Todo {
    pub fn new(tasks: Vec<Tasks>) -> Self {
        Todo { tasks }
    }
    pub fn interactive_mode(mut self) -> Result<(), TasksErr> {
        let mut db = DB::new("tasks.db".to_string());
        self.clear();
        self.help();
        loop {
            match self.list() {
                Ok(tasks) => {
                    println!("{}\n", tasks);

                    print!("\n\nConsole (h for Help): ");
                }
                Err(e) => {
                    eprintln!("Failed to list tasks: {}", e);
                    continue;
                }
            }
            let _ = stdout().flush();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let input_vec: Vec<&str> = input.split_whitespace().collect();

            match input_vec[0].to_lowercase().as_str() {
                "new" | "add" | "create" => {
                    if input_vec.len() < 3 {
                        eprintln!(
                            "Invalid command entered. Usage: new <task> <description> [due_date]"
                        );
                        continue;
                    }
                    let mut title = String::new();
                    let mut i = 1;
                    while i < input_vec.len() && !input_vec[i].starts_with('"') {
                        title.push(' ');
                        title.push_str(input_vec[i]);
                        i += 1;
                    }
                    if i < input_vec.len() {
                        title.push_str(&input_vec[i][1..]);
                        i += 1;
                        while i < input_vec.len() && !input_vec[i].ends_with('"') {
                            title.push(' ');
                            title.push_str(input_vec[i]);
                            i += 1;
                        }
                        if i < input_vec.len() {
                            title.push(' ');
                            title.push_str(&input_vec[i][..input_vec[i].len() - 1]);
                        }
                    }
                    let desc_start: usize = i + 1;
                    let mut description = String::new();
                    let mut y = desc_start;
                    while y < input_vec.len() && !input_vec[y].starts_with('"') {
                        description.push(' ');
                        description.push_str(input_vec[y]);
                        y += 1;
                    }
                    if y < input_vec.len() {
                        description.push_str(&input_vec[y][1..]);
                        y += 1;
                        while y < input_vec.len() && !input_vec[y].ends_with('"') {
                            description.push(' ');
                            description.push_str(input_vec[y]);
                            y += 1;
                        }
                        if y < input_vec.len() {
                            description.push(' ');
                            description.push_str(&input_vec[y][..input_vec[y].len() - 1]);
                        }
                    }
                    let task = title.as_str();
                    let description = description.as_str();
                    let task_result = Tasks::add(1,task, description, None);
                    let _ = db.insert(task, description, false, None, "%m/%d/%Y");
                    match task_result {
                        Ok(new_task) => {
                            let task = new_task;
                            self.add_task(&task);
                            println!("Task created successfully:\n{}", task);
                        }
                        Err(e) => eprintln!("Failed to create task: {}", e),
                    }
                }
                "done" => {
                    if let Ok(i) = input_vec[1].parse::<usize>() {
                        if i == 0 || i > self.tasks.len() {
                            eprintln!("Invalid task number entered");
                            continue;
                        }
                        let i = i - 1;
                        let _ = done(self.tasks[i].get_id() as usize);
                        let done_result = self.tasks[i].done();
                        match done_result {
                            Ok(_) => println!("Task marked as done successfully:\n{}", self.tasks[i].clone()),
                            Err(e) => eprintln!("Failed to mark task as done: {}", e),
                        }
                    } else {
                        eprintln!("Invalid task number entered");
                    }
                    
                }
                "rm" | "remove" | "purge" | "delete" => {
                    if input_vec.len() < 2 {
                        eprintln!("Usage: rm <task_number>");
                        continue;
                    }
                    if let Ok(i) = input_vec[1].parse::<usize>() {
                        if i == 0 || i > self.tasks.len() {
                            eprintln!("Invalid task number entered");
                            continue;
                        }
                        let i = i - 1;
                        let _ = remove(self.tasks[i].get_id() as usize);
                        self.tasks.remove(i);
                    } else {
                        eprintln!("Invalid task number entered");
                    }
                }
                "set_due_date" | "date" => {
                    if input_vec.len() < 3 {
                        eprintln!("Usage: set_due_date <task_number> <due_date>");
                        continue;
                    }
                    if let Ok(i) = input_vec[1].parse::<usize>() {
                        if i == 0 || i > self.tasks.len() {
                            eprintln!("Invalid task number entered. Usage: set_due_date <task_number> <due_date>");
                            continue;
                        }
                        let i = i - 1;  // Adjust for 1-based indexing
                        let mut t = self.tasks[i].clone();
                        let _ = self.tasks[i].set_due_date(input_vec[2]);
                        let _ = t.set_due_date(input_vec[2]);
                        let date_str = format!("{}", t.get_due_date().unwrap());
                        let _ = update("due_date", &date_str, self.tasks[i].get_id() as usize);
                        match self.tasks[i].set_due_date(input_vec[2]) {
                            Ok(_) => {
                                match set_due_date(input_vec[2].to_string(), self.tasks[i].get_id() as usize) {
                                    Ok(_) => (),
                                    Err(e) => eprintln!("Failed to update due_date: {}", e),
                                }
                            },
                            Err(e) => eprintln!("Failed to set due_date: {}", e),
                        }
                    } else {
                        eprintln!("Invalid task number entered. Usage: set_due_date <task_number> <due_date>");
                    }
                }

                "set_format" | "format"=> {
                    if input_vec.len() < 3 {
                        eprintln!("Usage: set_format <task_number> <format>");
                        continue;
                    }
                    if let Ok(i) = input_vec[1].parse::<usize>() {
                        if i == 0 || i > self.tasks.len() {
                            eprintln!("Invalid command entered. Usage: set_format <task_number> <format>");
                            continue;
                        }
                        let i = i - 1;
                        let format = input_vec[2];
                        let _ = self.tasks[i].set_format(format);
                        let _ = update("format", format, self.tasks[i].get_id() as usize);
                        match self.tasks[i].set_format(format) {
                            Ok(_) => {
                                match set_format(format, self.tasks[i].get_id() as usize) {
                                    Ok(_) => (),
                                    Err(e) => eprintln!("Failed to update format: {}", e),
                                }
                            },
                            Err(e) => eprintln!("Failed to set format: {}", e),
                        }
                    } else {
                        eprintln!("Invalid task number entered");
                    }
                }
                "check_validation" | "validate" => {
                    let task = self.get_task().unwrap();
                    let check_validation_result = task.check_validation();
                    match check_validation_result {
                        Ok(_) => println!("Task date is valid"),
                        Err(e) => eprintln!("Task date validation failed: {}", e),
                    }
                }
                "list" => {
                    let _ = self.list();
                },
                "clear" => {
                    let _ = self.clear();
                }
                "help" | "h" => {
                    let _ = self.help();
                }
                "exit" => {
                    return Ok(());
                }
                _ => {
                    eprintln!("Invalid command entered");
                }
            }
        }
    }
    pub fn add_task(&mut self, task: &Tasks) {
        self.tasks.push(task.to_owned());
    }
    pub fn help(&self) {
        let commands = vec![
            ("new <task> <description> [due_date]", "Create a new task"),
            ("done", "Mark a task as done"),
            ("rm", "Remove a task"),
            ("set_due_date <due_date>", "Set a due date for a task"),
            ("set_format <format>", "Set a date format for a task"),
            ("check_validation", "Check if a task's date is valid"),
            ("list", "List all tasks"),
            ("help", "Show this help message"),
            ("exit", "Exit the program"),
        ];

        let max_command_len = commands.iter().map(|(cmd, _)| cmd.len()).max().unwrap_or(0);
        let padding = 4;

        for (cmd, desc) in commands {
            let spaces = " ".repeat(max_command_len - cmd.len() + padding);
            println!("{}{}- {}", cmd, spaces, desc);
        }
    }
    pub fn list(&self) -> Result<String, TasksErr> {
        let mut result_finished = String::new();
        let mut result_unfinished = String::new();

        for (index, task) in self.tasks.iter().enumerate() {
            if task.done {
                result_finished.push_str(&format!("\nTask {}:\n{}\n", index + 1, task));
            } else {
                result_unfinished.push_str(&format!("\nTask {}:\n{}\n", index + 1, task));
            }
        }

        let mut result = String::new();

        if !result_finished.is_empty() {
            result.push_str(&format!("\nFinished tasks:\n{}", result_finished));
        }
        if !result_unfinished.is_empty() {
            if !result.is_empty() {
                result.push_str("\n");
            }
            result.push_str(&format!("\nUnfinished tasks:\n{}", result_unfinished));
        }
        Ok(result)
    }
    pub fn get_task(&self) -> Result<Tasks, TasksErr> {
        loop {
            let mut input_task_id = String::new();
            stdin().read_line(&mut input_task_id).unwrap();
            let input_task_id = input_task_id.trim().parse::<usize>().unwrap();
            if input_task_id >= self.tasks.len() {
                eprintln!("Invalid task number entered");
                continue;
            } else {
                let task = self.tasks[input_task_id - 1].clone();
                return Ok(task);
            }
        }
    }
    pub fn check(&self, task_id: usize) -> Result<String, TasksErr> {
        let mut current_date = Local::now().naive_local().date();
        let task = self.tasks[task_id - 1].clone();
        let date: NaiveDate = task.get_due_date().unwrap();
        if let equal = date == current_date {
            let task_str = format!("Task: {} needs to be finished today!", task.get_id());
            return Ok(task_str);
        } else {
            if let passed = date < current_date {
                let task_str = format!("Task: {} the date has already passed and lays in the past!", task.get_id());
                return Ok(task_str);
            } else {
                let task_str = format!("Task: {} should be finished until: {}!", task.get_id(), date);
                return Ok(task_str);
            }
        }
    }
    pub fn get_task_by_id(&mut self, task_id: usize) -> Result<Tasks, TasksErr> {
        if task_id >= self.tasks.len() {
            eprintln!("Invalid task number entered");
            return Err(TasksErr::InvalidTaskId);
        } else {
            let task = self.tasks[task_id - 1].clone();
            return Ok(task);
        }
    }
    pub fn clear(&self) {
        print!("{}[2J", 27 as char);
    }
    pub fn get_todo() -> Todo {
        let connection = sqlite3::open("tasks.db").unwrap();
        let mut tasks = Vec::new();

        connection.iterate("SELECT * FROM tasks", |pairs| {
            let mut task = Tasks::new();
            for &(column, value) in pairs.iter() {
                match column {
                    "id" => task.id = value.unwrap_or_default().parse().unwrap_or_default(),
                    "task" => task.task = value.unwrap_or_default().to_string(),
                    "description" => task.description = value.unwrap_or_default().to_string(),
                    "due_date" => {
                        //println!("{}", value.unwrap_or_default().to_string());
                        if let Some(date) = value {
                            task.due_date = NaiveDate::parse_from_str(date, "%d/%m/%y")
                                .or_else(|_| NaiveDate::parse_from_str(date, "%d/%m/%Y"))
                                .or_else(|_| NaiveDate::parse_from_str(date, "%Y-%m-%d"))
                                .ok();
                        }
                    },
                    "done" => task.done = {
                        match value.unwrap_or_default().parse().unwrap_or_default() {
                            0 => false,
                            1 => true,
                            _ => false,
                        }
                    },
                    _ => (),
                }
            }
            tasks.push(task);
            true
        }).unwrap();

        let todo = Todo::new(tasks);
        todo
    }
    fn get_tasks(&self) -> Vec<Tasks> {
        self.tasks.clone()
    }
}