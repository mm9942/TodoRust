use sqlite3::{Connection, State, Statement, Error};
use std::result::Result;
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

    pub fn select_all(&mut self) -> Result<(), Error> {
        self.conn.iterate("SELECT * FROM users WHERE age > 50", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
                true
            })
            .unwrap();
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

    pub fn select_by_id(&mut self, id: i32) -> Result<Statement,Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks WHERE id = ?")?;
        stmt.bind(1, id as i64)?;  // Convert id to i64
        Ok(stmt)
    }

    pub fn select_by_task(&mut self, task: &str) -> Result<Statement,Error> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks WHERE task = ?")?;
        stmt.bind(1, task)?;
        Ok(stmt)
    }

    pub fn mark_done(&mut self, id: i32) -> Result<Statement, Error> {
        let mut stmt = self.conn.prepare("UPDATE tasks SET done = 1 WHERE id = ?")?;
        stmt.bind(1, id as i64)?;  // Convert id to i64
        Ok(stmt)
    }
}
