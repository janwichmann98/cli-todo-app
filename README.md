# CLI ToDo App

A simple command-line application written in Rust to manage your to-do lists. This project demonstrates the basics of Rust syntax, error handling, and building CLI applications, while persisting data in a JSON file.

## Features

- **Add a Task:** Add a new task with a description.
- **Remove a Task:** Delete a task by its unique ID.
- **List Tasks:** Display all currently stored tasks.
- **JSON Persistence:** Tasks are stored in a `todo.json` file.

## Getting Started

Follow these instructions to set up and run the project on your local machine.

### Prerequisites

- **Rust and Cargo:** Make sure you have Rust installed. Cargo, the Rust package manager, is included with Rust. You can install Rust via [rustup](https://rustup.rs/).

### Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/janwichmann98/cli-todo-app.git
   cd cli-todo-app

2. **Clone the repository:**

   cargo build

### Usage
Run the application using Cargo with the following commands:

- **Add a Task:**
  '''bash
  cargo run -- add "Buy groceries"
  
- **List Tasks:**
  ```Bash
  cargo run -- list
  
- **Remove a Task:**
  ```Bash
  cargo run -- remove 1

Tasks are stored in a file named todo.json in the project root. The file is created automatically when you add your first task.

### Project Structure
```
cli-todo-app/
├── Cargo.toml        # Project metadata and dependencies
└── src/
    └── main.rs       # Main application code
```

### Contributing
Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch (git checkout -b feature/your-feature).
3. Commit your changes (git commit -m 'Add some feature').
4. Push to the branch (git push origin feature/your-feature).
5. Open a pull request.

### License
This project is licensed under the MIT License. See the [MIT LICENSE](LICENSE) file for details.
