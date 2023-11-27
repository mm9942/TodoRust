# TodoRust

A Rust-based Todo List Manager to keep track of your daily tasks, with features such as adding tasks, updating due dates, marking tasks as done, and validating task dates.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust compiler and Cargo (Rust's build system and package manager)

### Dependencies

- `chrono`: A date and time library for Rust, to handle date parsing and formatting.

### Installation

1. Clone the repository:
```bash
git clone https://github.com/mm9942/TodoRust.git
```

2. Navigate to the project directory:
```bash
cd TodoRust
```

3.1. Direct installation through setup.py:
```bash
python3 setup.py
```

3.2. Alternativly build and install the project manually:
```bash
cargo build --release
cargo install --path .
```
##### when installing manually you also need to build the Sqlite3 database manually, use the code in the setup.py to see how to design the database table and the names

### Usage

The primary objects are `Tasks` and `TasksErr` enum which encapsulates task details and possible errors respectively.

Here's how you can create a new task:

```bash
todo new -t "new title" -d "new description" --date 22/12/24
```

Set or change a date for a task:

```bash
todo task -t --date 22/12/24 -i 2
```

Mark a task as done:

```bash
todo --done 2
```

For more examples and usage, please refer to the `main.rs` file.


## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Contact

Feel free to open issues if you find anything wrong or have suggestions.
