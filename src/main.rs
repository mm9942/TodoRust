mod tasks;
use crate::tasks::{Tasks, TasksErr};

enum todo {
    new,
    done,
    rm,
    set_due_date,
    set_format,
    check_validation,
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
    use crate::tasks::{Tasks, TasksErr};

    #[test]
    fn test_task_creation() {
        let task = Tasks::add("new", "test", None).unwrap();
        assert_eq!(
            format!("{}", task),
            "Task:\t\tnew\nDone:\t\tfalse\nDescription:\ttest\nDue Date:\tNo due date\n"
        );
    }

    #[test]
    fn test_task_due_date_update_and_completion() {
        let mut task = Tasks::add("new", "test", None).unwrap();
        let _ = task.set_due_date("31.12.2023").unwrap();
        let _ = task.set_format("%d.%m.%Y");
        let _ = task.done().unwrap();
        assert_eq!(
            format!("{}", task),
            "Task:\t\tnew\nDone:\t\ttrue\nDescription:\ttest\nDue Date:\t31.12.2023\n"
        );
    }

    #[test]
    fn test_task_with_due_date_creation_and_completion() {
        let mut task_with_due_date = Tasks::add("new", "test", Some("24.12.2023")).unwrap();
        let _ = task_with_due_date.set_format("%d.%m.%Y");
        assert_eq!(
            format!("{}", task_with_due_date),
            "Task:\t\tnew\nDone:\t\tfalse\nDescription:\ttest\nDue Date:\t24.12.2023\n"
        );

        let _ = task_with_due_date.done().unwrap();
        assert_eq!(
            format!("{}", task_with_due_date),
            "Task:\t\tnew\nDone:\t\ttrue\nDescription:\ttest\nDue Date:\t24.12.2023\n"
        );
    }

    #[test]
    #[test]
    fn test_task_removal() {
        let mut task = Tasks::add("new", "test", None).unwrap();
        task.rm_task();
        assert_eq!(
            format!("{}", task),
            "Task:\t\t\nDone:\t\tfalse\nDescription:\t\nDue Date:\tNo due date\n"
        );
    }

    #[test]
    fn test_task_date_validation() {
        let mut task = Tasks::add("urgent", "needs to be done", Some("2021-11-05")).unwrap();
        assert_eq!(task.check_validation().is_err(), true);
    }
}
