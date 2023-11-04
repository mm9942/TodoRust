use std::fmt::{self, Display};
use std::error::Error;
use chrono::{Local, NaiveDate};

#[derive(PartialEq, Debug)]
enum TasksErr {
    TaskAlreadyDone,
    InvalidDateFormat,
    FailedToAddTask,
    InvalidFormat,
    TaskDateNotValid,
}

impl fmt::Display for TasksErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TasksErr::TaskAlreadyDone => write!(f, "\nTask is already marked as done!\n"),
            TasksErr::InvalidDateFormat => write!(f, "\nProvided date format is invalid!\n"),
            TasksErr::FailedToAddTask => write!(f, "\nFailed to add task!\n"),
            TasksErr::InvalidFormat => write!(f, "\nProvided format is invalid!\n"),
            TasksErr::TaskDateNotValid => write!(f, "\nThe task's due date has already passed!\n"),

        }
    }
}

impl Error for TasksErr {}

#[derive(PartialEq, Debug)]
struct Tasks {
    task: String,
    done: bool,
    description: String,
    due_date: Option<NaiveDate>,
    format: String
}


impl Tasks {
    fn add(task: &str, description: &str, due_date: Option<&str>) -> Result<Self, TasksErr> {
        let parsed_due_date = match due_date {
            Some(date_str) => match Self::parse_date(date_str) {
                Ok(date) => Some(date),
                Err(_) => return Err(TasksErr::InvalidDateFormat),
            },
            None => None,
        };

        if task.is_empty() || description.is_empty() {
            return Err(TasksErr::FailedToAddTask);
        }

        Ok(Self {
            task: task.to_string(),
            done: false,
            description: description.to_string(),
            due_date: parsed_due_date,
            format: "%m/%d/%Y".to_string()
        })
    }

    fn set_due_date(&mut self, due_date: &str) -> Result<(), TasksErr> {
        match Self::parse_date(due_date) {
            Ok(date) => {
                self.due_date = Some(date);
                Ok(())
            },
            Err(e) => Err(e),
        }       
    }

    fn rm_task(&mut self) {
        self.task.clear();
        self.done = false;
        self.description.clear();
        self.due_date = None;
    }

    fn done(&mut self) -> Result<(), TasksErr> {
        if self.done {
            Err(TasksErr::TaskAlreadyDone)
        } else {
            self.done = true;
            Ok(())
        }
    }

    fn parse_date(date_str: &str) -> Result<NaiveDate, TasksErr> {
        let formats = ["%Y-%m-%d", "%d.%m.%Y", "%d/%m/%Y"];
        for format in &formats {
            if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
                return Ok(date);
            }
        }
        Err(TasksErr::InvalidDateFormat)
    }

    fn set_format(&mut self, format: &str) -> Result<(), TasksErr> {
        if !Self::is_valid_format(format) {
            return Err(TasksErr::InvalidFormat);
        }
        self.format = format.to_string();
        Ok(())
    }

    fn is_valid_format(format: &str) -> bool {
        // Check if the format string is valid (you can implement your own validation logic here)
        format == "%Y-%m-%d" || format == "%d.%m.%Y" || format == "%d/%m/%Y"
    }

    fn check_validation(&self) -> Result<(), TasksErr> {
        match &self.due_date {
            Some(date) if date < &Local::now().naive_local().date() => Err(TasksErr::TaskDateNotValid),
            _ => Ok(())
        }
    }
}

impl Display for Tasks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let due_date_str = self.due_date.map(|date| date.format(&self.format).to_string()).unwrap_or("No due date".to_string());
        write!(f, "Task:\t\t{}\nDone:\t\t{}\nDescription:\t{}\nDue Date:\t{}\n", self.task, self.done, self.description, due_date_str)
    }
}

fn main() {
    let task_result = Tasks::add("new", "test", None);
    match task_result {
        Ok(mut task) => {
            println!("\n{}", task);
            let _ = task.set_due_date("31.12.2023").unwrap();
            let _ = task.set_format("%d.%m.%Y");
            let _ = task.done();
            println!("{}", task);
        }
        Err(e) => eprintln!("Failed to create task: {}", e),
    }

    let task_with_due_date_result = Tasks::add("new", "test", Some("24.12.2023"));
    match task_with_due_date_result {
        Ok(mut task_with_due_date) => {
            let _ = task_with_due_date.set_format("%d.%m.%Y");
            println!("{}", task_with_due_date);
            let _ = task_with_due_date.done();
            println!("{}", task_with_due_date);
        }
        Err(e) => eprintln!("Failed to create task: {}", e),
    }

    let new_task_result = Tasks::add("urgent", "needs to be done", Some("2021-11-05"));
    match new_task_result {
        Ok(mut new_task) => {
            let _ = new_task.set_format("%d.%m.%Y");
            println!("{}", new_task);
            match new_task.check_validation() {
                Ok(_) => println!("Task date is valid"),
                Err(e) => eprintln!("Task date validation failed: {}", e),
            }

            let _ = new_task.rm_task();
            println!("{}", new_task);
        }
        Err(e) => eprintln!("Failed to create task: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::Tasks;

    #[test]
    fn test_task_creation() {
        let task = Tasks::add("new", "test", None).unwrap();
        assert_eq!(format!("{}", task), "Task: new, Done: false, Description: test, Due Date: No due date");
    }

    #[test]
    fn test_task_due_date_update_and_completion() {
        let mut task = Tasks::add("new", "test", None).unwrap();
        let _ = task.set_due_date("31.12.2023").unwrap();
        let _ = task.done().unwrap();
        assert_eq!(format!("{}", task), "Task: new, Done: true, Description: test, Due Date: 12/31/2023");
    }

    #[test]
    fn test_task_with_due_date_creation_and_completion() {
        let mut task_with_due_date = Tasks::add("new", "test", Some("24.12.2023")).unwrap();
        assert_eq!(format!("{}", task_with_due_date), "Task: new, Done: false, Description: test, Due Date: 12/24/2023");

        let _ = task_with_due_date.done().unwrap();
        assert_eq!(format!("{}", task_with_due_date), "Task: new, Done: true, Description: test, Due Date: 12/24/2023");
    }

    #[test]
    fn test_task_removal() {
        let mut task = Tasks::add("new", "test", None).unwrap();
        task.rm_task();
        assert_eq!(format!("{}", task), "Task: , Done: false, Description: , Due Date: No due date");
    }

    #[test]
    fn test_task_date_validation() {
        let mut task = Tasks::add("urgent", "needs to be done", Some("2021-11-05")).unwrap();
        assert_eq!(task.check_validation().is_err(), true);
    }
}