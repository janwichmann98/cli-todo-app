use std::env;
use std::fs;
use std::io;

use serde::{Deserialize, Serialize};

/// Represents a single ToDo task.
/// Each task has a unique identifier and a description.
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
}

/// Loads tasks from the JSON file ("todo.json").
/// 
/// If the file does not exist or if its contents are invalid, an empty vector is returned.
///
/// # Returns
/// A vector of `Task` instances.
fn load_tasks() -> Vec<Task> {
    let path = "todo.json";
    // Attempt to read the file; if it fails, use an empty JSON array.
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    // Deserialize the JSON data into a vector of tasks.
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

/// Saves the given tasks to the JSON file ("todo.json").
///
/// # Arguments
/// * `tasks` - A reference to the vector of tasks to be saved.
///
/// # Returns
/// An `io::Result<()>` which is `Ok(())` if saving succeeds or an error if it fails.
fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    // Serialize the tasks vector to a pretty-printed JSON string.
    let data = serde_json::to_string_pretty(tasks)
        .expect("Error serializing tasks");
    // Write the JSON string to the file.
    fs::write("todo.json", data)
}

fn main() {
    // Collect all command line arguments into a vector.
    let args: Vec<String> = env::args().collect();

    // Ensure that at least one command is provided.
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [arguments]", args[0]);
        eprintln!("Commands:");
        eprintln!("  add <task description>   - Add a new task");
        eprintln!("  remove <task id>         - Remove an existing task");
        eprintln!("  list                     - List all tasks");
        return;
    }

    // The first argument is the command.
    let command = args[1].as_str();
    match command {
        "add" => {
            // Ensure that a task description is provided.
            if args.len() < 3 {
                eprintln!("Usage: {} add <task description>", args[0]);
                return;
            }
            // Combine all subsequent arguments into a single task description.
            let task_desc = args[2..].join(" ");
            // Load existing tasks.
            let mut tasks = load_tasks();

            // Generate a new unique task ID:
            // If no tasks exist, start at 1; otherwise, increment the maximum ID.
            let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let new_task = Task {
                id: new_id,
                description: task_desc,
            };
            tasks.push(new_task);

            // Save the updated task list and handle any errors.
            if let Err(e) = save_tasks(&tasks) {
                eprintln!("Error saving tasks: {}", e);
            } else {
                println!("Task added successfully.");
            }
        }
        "remove" => {
            // Ensure that exactly one task ID is provided.
            if args.len() != 3 {
                eprintln!("Usage: {} remove <task id>", args[0]);
                return;
            }
            // Attempt to parse the provided task ID.
            let task_id: u32 = args[2].parse().unwrap_or_else(|_| {
                eprintln!("Invalid task id: {}", args[2]);
                std::process::exit(1);
            });
            // Load existing tasks.
            let mut tasks = load_tasks();
            let initial_len = tasks.len();

            // Remove the task with the specified ID.
            tasks.retain(|task| task.id != task_id);

            // Check if any task was removed.
            if tasks.len() == initial_len {
                eprintln!("Task with id {} not found.", task_id);
            } else if let Err(e) = save_tasks(&tasks) {
                eprintln!("Error saving tasks: {}", e);
            } else {
                println!("Task removed successfully.");
            }
        }
        "list" => {
            // Load tasks from storage.
            let tasks = load_tasks();
            // Check if the task list is empty.
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                println!("Current tasks:");
                for task in tasks {
                    println!("{}: {}", task.id, task.description);
                }
            }
        }
        _ => {
            // Handle an unknown command.
            eprintln!("Unknown command: {}", command);
            eprintln!("Available commands: add, remove, list");
        }
    }
}
