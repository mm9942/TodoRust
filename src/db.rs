use sqlite3::{Connection, Statement, Bindable, State};
use std::result::Result;
use std::error::Error;
pub use chrono::{NaiveDate};

pub struct DB {
    path: String,
}

impl DB {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone()
        }
    }

    pub fn select_all(&mut self) -> Result<(), Box<dyn Error>> {
        let mut conn: Connection = sqlite3::open(&self.path).expect("Failed to open the database");
        conn.iterate("SELECT * FROM users WHERE age > 50", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        }).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
    pub fn insert(&mut self, task: &str, description: &str, done: bool, due_date: Option<NaiveDate>, format: &str) -> Result<(), Box<dyn Error>> {
        let mut conn: Connection = sqlite3::open(&self.path).expect("Failed to open the database");
        let mut stmt = conn.prepare("INSERT INTO tasks (task, description, done, due_date, format) VALUES (?, ?, ?, ?, ?)")?;
        
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
        cursor.next().unwrap();
        Ok(())
    }
    

    pub fn remove(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        let conn: Connection = sqlite3::open(&self.path).expect("Failed to open the database");
        let id = id as i64;
        let stmt: String = format!("DELETE FROM tasks WHERE id = {}", id);
        conn.execute(stmt)
            .unwrap();
        conn.execute("COMMIT;")?;
        Ok(())
    }

    pub fn update(&mut self, column: &str, value: &str, id: i32) -> Result<(), Box<dyn Error>> {
        let conn: Connection = sqlite3::open(&self.path).expect("Failed to open the database");
        let id = id as i64;
        let mut stmt = conn.prepare(&format!("UPDATE tasks SET {} = ? WHERE id = ?", column))?;
        stmt.bind(1, value)?;
        stmt.bind(2, id)?;
        let mut cursor = stmt.cursor()
            .next()
            .unwrap();
        conn.execute("COMMIT;")?;
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
}
pub fn remove(id: usize) -> Result<(), Box<dyn Error>> {
    let conn: Connection = sqlite3::open("tasks.db").expect("Failed to open the database");
    let stmt: String = format!("DELETE FROM tasks WHERE id = {}", id);
    conn.execute(stmt)
        .unwrap();
    conn.execute("COMMIT;")?;
    Ok(())
}

pub fn done(id: usize) -> Result<(), Box<dyn Error>> {
    let conn: Connection = sqlite3::open("tasks.db").expect("Failed to open the database");
    let mut stmt = format!("UPDATE tasks SET Done = true WHERE id = {};", id);
    conn.execute(
        stmt
    )
    .unwrap();
    Ok(())
}

pub fn done_new(id: usize) -> Result<(), Box<dyn Error>> {
    let conn: Connection = sqlite3::open("tasks.db").expect("Failed to open the database");
    let mut stmt = conn.prepare("UPDATE tasks SET Done = ? WHERE id = ?")?;
    stmt.bind(1, "true")?;
    stmt.bind(2, id as i64)?;
    stmt.next().unwrap();
    conn.execute("COMMIT;")?;
    Ok(())
}
pub fn update(column: &str, value: &str, id: usize) -> Result<(), Box<dyn Error>> {
    let conn: Connection = sqlite3::open("tasks.db").expect("Failed to open the database");
    let mut stmt = format!("UPDATE tasks SET {} = '{}' where id = {};", column , value, id);
    conn.execute(
        stmt
    )
    .unwrap();
    conn.execute("COMMIT;")?;
    Ok(())
}
/*pub fn set_due_date(value: String, id: usize) -> Result<(), Box<dyn Error>> {
    let conn: Connection = sqlite3::open("tasks.db").expect("Failed to open the database");
    let mut stmt = format!("UPDATE tasks SET due_date = '{}' where id = {};", value , id);
    conn.execute(
        stmt
    )
    .unwrap();
    conn.execute("COMMIT;")?;
    Ok(())
}*/
pub fn set_due_date(value: String, id: usize) -> Result<(), Box<dyn Error>> {
    let conn: Connection = sqlite3::open("tasks.db").expect("Failed to open the database");
    let mut stmt = conn.prepare("UPDATE tasks SET due_date = '?' WHERE id = ?")?;
    stmt.bind(1, value.as_str())?;
    stmt.bind(2, id as i64)?;
    stmt.next().unwrap();
    conn.execute("COMMIT;")?;
    Ok(())
}
pub fn set_format(value: &str, id: usize) -> Result<(), Box<dyn Error>> {
    let conn: Connection = sqlite3::open("tasks.db").expect("Failed to open the database");
    let mut stmt = conn.prepare("UPDATE tasks SET format = ? WHERE id = ?")?;
    stmt.bind(1, value)?;
    stmt.bind(2, id as i64)?;
    stmt.next().unwrap();
    conn.execute("COMMIT;")?;
    Ok(())
}
pub fn set_description(value: &str, id: usize) -> Result<(), Box<dyn Error>> {
    let conn: Connection = sqlite3::open("tasks.db").expect("Failed to open the database");
    let mut stmt = conn.prepare("UPDATE tasks SET description = ? WHERE id = ?")?;
    stmt.bind(1, value)?;
    stmt.bind(2, id as i64)?;
    stmt.next().unwrap();
    conn.execute("COMMIT;")?;
    Ok(())
}