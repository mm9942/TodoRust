mod tasks;
use crate::tasks::{Tasks, TasksErr};
use std::fmt::Display;
use std::io::{self, stdin, stdout, Write};
use std::result::Result;
enum Command {
    new,
    done,
    rm,
    set_due_date,
    set_format,
    check_validation,
    list,
}
#[derive(Debug, Clone, PartialEq)]
struct Todo {
    tasks: Vec<Tasks>,
    //command: Command,
}
impl Todo {
    fn new(tasks: Vec<Tasks>) -> Self {
        Todo { tasks }
    }
    fn interactive_mode(mut self) -> Result<(), TasksErr> {
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
                "new" => {
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
                    let task_result = Tasks::add(task, description, None);
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
                    let mut task = self.get_task().unwrap();
                    let done_result = task.done();
                    match done_result {
                        Ok(_) => println!("Task marked as done successfully:\n{}", task.clone()),
                        Err(e) => eprintln!("Failed to mark task as done: {}", e),
                    }
                }
                "rm" => {
                    if let len = input_vec.len() < 2 {
                        let mut task = self.get_task().unwrap();
                        let rm_result = task.rm_task();
                        match rm_result {
                            Ok(_) => println!("Task removed successfully"),
                            Err(e) => eprintln!("Failed to remove task: {}", e),
                        }
                    } else {
                        let mut task = self.get_task().unwrap();
                        let rm_result = task.rm_task();
                        match rm_result {
                            Ok(_) => println!("Task removed successfully"),
                            Err(e) => eprintln!("Failed to remove task: {}", e),
                        }
                    }
                }
                "set_due_date" => {
                    let mut task = self.get_task().unwrap();
                    if input_vec.len() < 2 {
                        eprintln!("Invalid command entered. Usage: set_due_date <due_date>");
                        continue;
                    }
                    let set_due_date_result = task.set_due_date(input_vec[1]);
                    match set_due_date_result {
                        Ok(_) => println!("Due date set successfully:\n{}", task),
                        Err(e) => eprintln!("Failed to set due date for task: {}", e),
                    }
                }
                "set_format" => {
                    let mut task = self.get_task().unwrap();
                    if input_vec.len() < 2 {
                        eprintln!("Invalid command entered. Usage: set_format <format>");
                        continue;
                    }
                    let set_format_result = task.set_format(input_vec[1]);
                    match set_format_result {
                        Ok(_) => println!("Date format set successfully:\n{}", task),
                        Err(e) => eprintln!("Failed to set date format for task: {}", e),
                    }
                }
                "check_validation" => {
                    let mut task = self.get_task().unwrap();
                    let check_validation_result = task.check_validation();
                    match check_validation_result {
                        Ok(_) => println!("Task date is valid"),
                        Err(e) => eprintln!("Task date validation failed: {}", e),
                    }
                }
                "list" => match self.list() {
                    Ok(tasks) => {
                        println!(
                            "{}\nPlease enter the task number you want to manipulate: ",
                            tasks
                        );
                    }
                    Err(e) => {
                        eprintln!("Failed to list tasks: {}", e);
                        continue;
                    }
                },
                "clear" => {
                    self.clear();
                }
                "help" | "h" => {
                    self.help();
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
    fn add_task(&mut self, task: &Tasks) {
        self.tasks.push(task.to_owned());
    }
    fn help(&self) {
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
    fn list(&self) -> Result<String, TasksErr> {
        let mut result_finished = String::new();
        let mut resuresult_unfinished = String::new();
        for (index, task) in self.tasks.iter().enumerate() {
            if task.done {
                result_finished.push_str(&format!("\nTask {}:\n{}\n", index + 1, task));
            } else {
                resuresult_unfinished.push_str(&format!("\nTask {}:\n\n{}\n", index + 1, task));
            }
        }
        let result = format!(
            "Finished tasks:\n{}\nUnfinished tasks:\n{}",
            result_finished, resuresult_unfinished
        );
        Ok(result)
    }
    fn get_task(&self) -> Result<Tasks, TasksErr> {
        loop {
            let mut input_task_id = String::new();
            stdin().read_line(&mut input_task_id).unwrap();
            let input_task_id = input_task_id.trim().parse::<usize>().unwrap();
            if input_task_id >= self.tasks.len() {
                eprintln!("Invalid task number entered");
                continue;
            } else {
                let mut task = self.tasks[input_task_id - 1].clone();
                return Ok(task);
            }
        }
    }
    fn get_task_by_id(&mut self, task_id: usize) -> Result<Tasks, TasksErr> {
        if task_id >= self.tasks.len() {
            eprintln!("Invalid task number entered");
            return Err(TasksErr::InvalidTaskId);
        } else {
            let mut task = self.tasks[task_id - 1].clone();
            return Ok(task);
        }
    }
    fn clear(&self) {
        print!("{}[2J", 27 as char);
    }
}
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
        }
        Err(e) => eprintln!("Failed to create task: {}", e),
    }

    let new_task_result = Tasks::add("urgent", "needs to be done", Some("2021-11-05"));
    match new_task_result {
        Ok(mut new_task) => {
            let _ = new_task.set_format("%d.%m.%Y").unwrap();
            tasks_vec.push(new_task);
        }
        Err(e) => eprintln!("Failed to create task: {}", e),
    }

    let todo = Todo::new(tasks_vec); // Just pass tasks_vec directly
    todo.interactive_mode().unwrap();
}
