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

# Insert record
c.execute('''
          INSERT INTO tasks
          (task, done, description, format)
          VALUES ('Projekt Tasks Beenden', 0, 'Dont Forget: Github Commit', '%d.%m.%Y')
          ''')

# Insert record
c.execute('''
          INSERT INTO tasks
          (task, done, description, format)
          VALUES ('Check my mails', 0, 'Relogin into Outlook with personal domain''s mail', '%d.%m.%Y')
          ''')

# Insert record
c.execute('''
          INSERT INTO tasks
          (task, done, description, format)
          VALUES ('Buy a new CPU', 0, 'At market X are currently good discounts', '%d.%m.%Y')
          ''')

# Commit changes and close connection
conn.commit()
conn.close()