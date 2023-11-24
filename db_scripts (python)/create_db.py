import sqlite3

# Connect to SQLite database (or create it if it doesn't exist)
conn = sqlite3.connect('tasks.db')
c = conn.cursor()

# Create table
c.execute('''
          CREATE TABLE IF NOT EXISTS tasks
          (id INTEGER PRIMARY KEY AUTOINCREMENT,
          task TEXT,
          done BOOLEAN,
          description TEXT,
          due_date DATE,
          format TEXT)
          ''')


# Commit changes and close connection
conn.commit()
conn.close()