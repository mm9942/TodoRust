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

3. Build the project:
```bash
cargo build --release
```

### Usage

The primary objects are `Tasks` and `TasksErr` enum which encapsulates task details and possible errors respectively.

Here's how you can create a new task:

```rust
let task_result = Tasks::add("New Task", "This is a new task", None);
```

Set a due date for a task:

```rust
let _ = task.set_due_date("31.12.2023").unwrap();
```

Mark a task as done:

```rust
let _ = task.done();
```

For more examples and usage, please refer to the `main.rs` file.

### Testing

Run the tests using the following command:

```bash
cargo test
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Contact

Feel free to open issues if you find anything wrong or have suggestions.
