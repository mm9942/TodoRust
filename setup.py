import subprocess
import os
import sqlite3
import sys

def is_cargo_installed():
    """Check if Cargo is installed."""
    try:
        subprocess.run(["cargo", "--version"], check=True)
        return True
    except subprocess.CalledProcessError:
        return False

def install_cargo():
    """Install Cargo from the official Rust website."""
    if sys.platform == "win32":
        raise RuntimeError("Cargo needs to be installed manually on Windows. Please visit https://www.rust-lang.org/tools/install")
    print("Installing Cargo...")
    subprocess.run(["curl", "--proto", "=https", "--tlsv1.2", "-sSf", "https://sh.rustup.rs", "|", "sh"], check=True)

def build_and_install_package():
    """Build and install the Rust package."""
    print("Building the Rust package...")
    subprocess.run(["cargo", "build"], check=True)
    print("Installing the Rust package...")
    subprocess.run(["cargo", "install", "--path", "."], check=True)

def setup_database():
    """Create an SQLite3 database and tasks table if they don't exist."""
    conn = sqlite3.connect("tasks.db")
    c = conn.cursor()
    c.execute('''
              CREATE TABLE IF NOT EXISTS tasks
              (id INTEGER PRIMARY KEY AUTOINCREMENT,
              task TEXT,
              done BOOLEAN,
              description TEXT,
              due_date DATE,
              format TEXT)
              ''')
    conn.commit()
    conn.close()

def main():
    if not is_cargo_installed():
        install_cargo()
    build_and_install_package()
    setup_database()

if __name__ == "__main__":
    main()
