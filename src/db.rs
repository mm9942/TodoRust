use sqlite3::{Connection, State, Statement, Bindable};
use std::result::Result;
use std::error::Error;
pub use chrono::{Local, NaiveDate};

pub struct DB {
    path: String,
    conn: Connection,
}

impl DB {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            conn: sqlite3::open(&path).expect("Failed to open the database"),
        }
    }

    pub fn select_all(&mut self) -> Result<(), Box<dyn Error>> {
        self.conn.iterate("SELECT * FROM users WHERE age > 50", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        }).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
    pub fn insert(&mut self, task: &str, description: &str, done: bool, due_date: Option<NaiveDate>, format: &str) -> Result<(), Box<dyn Error>> {
        let mut stmt = self.conn.prepare("INSERT INTO tasks (task, description, done, due_date, format) VALUES (?, ?, ?, ?, ?)")?;
        
        let mut due_date_str = String::new();
        match due_date {
            Some(due_date) => {
                due_date_str = due_date.format("%Y-%m-%d").to_string();
            },
            None => {
                // Handle null value for due_date if necessary
                // Your database should be set up to handle NULL values for this column
            }
        }
    
        stmt.bind(1, task)?;
        stmt.bind(2, description)?;
        stmt.bind(3, if done { 1i64 } else { 0i64 })?;
        stmt.bind(4, &*due_date_str)?;
        stmt.bind(5, format)?;
    
        let mut cursor = stmt.cursor();
        cursor.next()?;
        Ok(())
    }
    pub fn remove(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        let id = id as i64;
        let stmt: String = format!("DELETE FROM tasks WHERE id = {}", id);
        self.conn.execute(stmt)
            .unwrap();
        Ok(())
    }
    pub fn update(&mut self, column:&str, value:&str, id: i32) -> Result<(), Box<dyn Error>> {
        let id = id as i64;
        let mut stmt = self.conn.prepare(&format!("UPDATE tasks SET {} = ? WHERE id = ?", column))?;
        stmt.bind(1, value)?;
        stmt.bind(2, id)?;
        let mut cursor = stmt.cursor();
        cursor.next()?;
        Ok(())
    }
    
    
    /*pub fn select_all(&mut self) -> Result<Vec<State::Row>> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks")?;
        let mut rows = Vec::new();
        while let State::Row = stmt.step()? {
            let row = stmt.get_row(0)?;
            rows.push(row);
        }
        Ok(rows)
    }*/

    pub fn select_by_id(&mut self, id: i32) -> Result<Statement,Box<dyn Error>> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks WHERE id = ?")?;
        stmt.bind(1, id as i64)?;  // Convert id to i64
        Ok(stmt)
    }

    pub fn select_by_task(&mut self, task: &str) -> Result<Statement,Box<dyn Error>> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks WHERE task = ?")?;
        stmt.bind(1, task)?;
        Ok(stmt)
    }

    pub fn mark_done(&mut self, id: i32) -> Result<Statement, Box<dyn Error>> {
        let mut stmt = self.conn.prepare("UPDATE tasks SET done = 1 WHERE id = ?")?;
        stmt.bind(1, id as i64)?;  // Convert id to i64
        Ok(stmt)
    }
}
