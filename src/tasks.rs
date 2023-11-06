pub use chrono::{Local, NaiveDate};
pub use std::error::Error;
pub use std::fmt::{self, Display};

const DATE_FORMATS: [&str; 3] = ["%Y-%m-%d", "%d.%m.%Y", "%d/%m/%Y"];

#[derive(PartialEq, Debug)]
pub enum TasksErr {
    TaskAlreadyDone,
    InvalidDateFormat,
    FailedToAddTask,
    InvalidFormat,
    TaskDateNotValid,
    InvalidTaskId,
}

impl fmt::Display for TasksErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TasksErr::TaskAlreadyDone => write!(f, "\nTask is already marked as done!\n"),
            TasksErr::InvalidDateFormat => write!(f, "\nProvided date format is invalid!\n"),
            TasksErr::FailedToAddTask => write!(f, "\nFailed to add task!\n"),
            TasksErr::InvalidFormat => write!(f, "\nProvided format is invalid!\n"),
            TasksErr::TaskDateNotValid => write!(f, "\nThe task's due date has already passed!\n"),
            TasksErr::InvalidTaskId => write!(f, "\nInvalid task id!\n"),
        }
    }
}

impl Error for TasksErr {}

#[derive(PartialEq, Clone, Debug)]
pub struct Tasks {
    pub id: i32,
    pub task: String,
    pub done: bool,
    pub description: String,
    pub due_date: Option<NaiveDate>,
    pub format: String,
}

impl Tasks {
    pub fn new() -> Self {
        let id : i32 = 0;
        let task = "".to_string(); 
        let done = false;
        let description = "".to_string();
        let due_date = None;
        let format = "%m/%d/%Y".to_string();
        Self {
            id,
            task,
            done,
            description,
            due_date,
            format,
        }
    }
    pub fn task(&mut self, id: i32, task: &str, description: &str, due_date: Option<&str>) -> Result<Self, TasksErr> {
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
            id,
            task: task.to_string(),
            done: false,
            description: description.to_string(),
            due_date: parsed_due_date,
            format: "%m/%d/%Y".to_string(),
        })
    } 
    pub fn add(id: i32, task: &str, description: &str, due_date: Option<&str>) -> Result<Self, TasksErr> {
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
            id,
            task: task.to_string(),
            done: false,
            description: description.to_string(),
            due_date: parsed_due_date,
            format: "%m/%d/%Y".to_string(),
        })
    }

    pub fn set_due_date(&mut self, due_date: &str) -> Result<(), TasksErr> {
        match Self::parse_date(due_date) {
            Ok(date) => {
                self.due_date = Some(date);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn rm_task(&mut self) -> Result<(), TasksErr> {
        if self.done {
            Err(TasksErr::TaskAlreadyDone)
        } else {
            self.task = "".to_string();
            self.description = "".to_string();
            self.due_date = None;
            Ok(())
        }
    }

    pub fn done(&mut self) -> Result<(), TasksErr> {
        if self.done {
            Err(TasksErr::TaskAlreadyDone)
        } else {
            self.done = true;
            Ok(())
        }
    }

    pub fn set_format(&mut self, format: &str) -> Result<(), TasksErr> {
        if !Self::is_valid_format(format) {
            return Err(TasksErr::InvalidFormat);
        }
        self.format = format.to_string();
        Ok(())
    }

    pub fn set_title(&mut self, title: &str) {
        self.task = title.to_string();
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn is_valid_format(format: &str) -> bool {
        // Check if the format string is valid (you can implement your own validation logic here)
        format == "%Y-%m-%d" || format == "%d.%m.%Y" || format == "%d/%m/%Y"
    }

    pub fn check_validation(&self) -> Result<(), TasksErr> {
        self.due_date
            .as_ref()
            .filter(|&&date| date < Local::now().naive_local().date())
            .map_or(Ok(()), |_| Err(TasksErr::TaskDateNotValid))
    }
    pub fn parse_date(date_str: &str) -> Result<NaiveDate, TasksErr> {
        DATE_FORMATS
            .iter()
            .find_map(|&format| NaiveDate::parse_from_str(date_str, format).ok())
            .ok_or(TasksErr::InvalidDateFormat)
    }
    pub fn is_done(&self) -> bool {
        self.done
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_task(&self) -> &str {
        &self.task
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_due_date(&self) -> Option<NaiveDate> {
        self.due_date
    }
    pub fn get_format(&self) -> &str {
        &self.format
    }
}
impl Display for Tasks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let due_date_str = self
            .due_date
            .map(|date| date.format(&self.format).to_string())
            .unwrap_or("No due date".to_string());
        write!(
            f,
            "\tTask:\t\t{}\n\tDone:\t\t{}\n\tDescription:\t{}\n\tDue Date:\t{}\n\tFormat:\t\t{}\n",
            self.task, self.done, self.description, due_date_str, self.format
        )
    }
}
