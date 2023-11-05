use sqlite3::{Connection, Result, Statement};

pub struct DB {
    path: String,
    conn: Connection,
}

impl DB {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            conn: Connection::open(&path).expect("Failed to open the database"),
        }
    }

    pub fn connect(&mut self) -> Result<Statement> {
        let stmt = self.conn.prepare("SELECT * FROM tasks")?;
        Ok(stmt)
    }
    pub fn select_all(&mut self) -> Result<Statement> {
        let stmt = self.conn.prepare("SELECT * FROM tasks")?;
        Ok(stmt)
    }

    pub fn select_by_id(&mut self, id: i32) -> Result<Statement> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks WHERE id = ?")?;
        stmt.bind(1, id as i64)?;  // Convert id to i64
        Ok(stmt)
    }

    pub fn select_by_task(&mut self, task: &str) -> Result<Statement> {
        let mut stmt = self.conn.prepare("SELECT * FROM tasks WHERE task = ?")?;
        stmt.bind(1, task)?;
        Ok(stmt)
    }

    pub fn mark_done(&mut self, id: i32) -> Result<Statement> {
        let mut stmt = self.conn.prepare("UPDATE tasks SET done = 1 WHERE id = ?")?;
        stmt.bind(1, id as i64)?;  // Convert id to i64
        Ok(stmt)
    }
}
