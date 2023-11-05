use rusqlite::{params, Connection, Result};
use std::error::Error;
pub use chrono::{NaiveDate};

pub struct DB {
    path: String,
    con: Connection,
}

impl DB {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            con: Connection::open_in_memory().unwrap(),
        }
    }

    pub fn select_all(&self) -> Result<(), rusqlite::Error> {
        let mut stmt = self.con.prepare("SELECT * FROM users WHERE age > 50")?;
        let user_iter = stmt.query_map([], |row| {
            let name: String = row.get("name")?;
            let age: i32 = row.get("age")?;
            println!("{} is {} years old", name, age);
            Ok(())
        })?;
    
        for user in user_iter {
            user?;  // Unwrap the Result<()> from each iteration
        }
    
        Ok(())
    }

    pub fn insert(&mut self, task: &str, description: &str, done: bool, due_date: Option<NaiveDate>, format: &str) -> Result<(), Box<dyn Error>> {
        let due_date_str = match &due_date {
            Some(due_date) => due_date.format("%Y-%m-%d").to_string(),
            None => "".to_string(),
        };
        
        self.con.execute(
            "INSERT INTO tasks (task, description, done, due_date, format) VALUES (?, ?, ?, ?, ?)",
            params![task, description, if done { 1i64 } else { 0i64 }, &*due_date_str, format],
        )?;
        
        Ok(())
    }

    pub fn remove(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        self.con.execute(
            "DELETE FROM tasks WHERE id = ?",
            params![id],
        )?;
        
        Ok(())
    }

    pub fn update(&mut self, column: &str, value: &str, id: i32) -> Result<(), Box<dyn Error>> {
        let query = format!("UPDATE tasks SET {} = ? WHERE id = ?", column);
        self.con.execute(&query, params![value, id])?;
        
        Ok(())
    }
}
