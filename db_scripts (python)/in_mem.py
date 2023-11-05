import sqlite3
from io import StringIO
    
def init_sqlite_db(app):
    # Read database to tempfile
    con = sqlite3.connect("tasks.db")
    tempfile = StringIO()
    for line in con.iterdump():
        tempfile.write('%s\n' % line)
    con.close()
    tempfile.seek(0)

    # Create a database in memory and import from tempfile
    app.sqlite = sqlite3.connect(":memory:")
    app.sqlite.cursor().executescript(tempfile.read())
    app.sqlite.commit()
    app.sqlite.row_factory = sqlite3.Row