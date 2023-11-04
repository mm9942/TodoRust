use sqlite3::{Connection, Cursor, Result};

struct db {
    path: Connection::path,
    conn: Connection,
    cursor: Cursor,
}
impl db {
    pub fn new(path: Connection::path) -> Self {
        Self {
            path,
            conn: Connection::open(path),
            cursor: Cursor::new(),
        }
    }
    fn connect(&mut self) -> Result<()> {
        self.conn = Connection::open(self.path)?;
        self.cursor = self.conn.prepare("SELECT * FROM tasks")?;
        Ok(())
    }
    fn disconnect(&mut self) -> Result<()> {
        self.cursor = Cursor::new();
        self.conn.close()?;
        Ok(())
    }
    fn select_all(&mut self) -> Result<()> {
        self.cursor = self.conn.prepare("SELECT * FROM tasks")?;
        Ok(())
    }
    fn select_by_id(&mut self, id: i32) -> Result<()> {
        self.cursor = self.conn.prepare("SELECT * FROM tasks WHERE id = ?")?;
        self.cursor.bind(1, id)?;
        Ok(())
    }
    fn select_by_task(&mut self, task: &str) -> Result<()> {
        self.cursor = self.conn.prepare("SELECT * FROM tasks WHERE task = ?")?;
        self.cursor.bind(1, task)?;
        Ok(())
    }
    fn mark_done(&mut self, id: i32) -> Result<()> {
        self.cursor = self.conn.prepare("UPDATE tasks SET done = 1 WHERE id = ?")?;
        self.cursor.bind(1, id)?;
        Ok(())
    }
}