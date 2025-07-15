use colored::*;
use std::io;
use std::io::Write;

#[derive(Debug)]
enum Priority {
    High,
    Medium,
    Low,
    None,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::High => write!(f, "🔴 High"),
            Priority::Medium => write!(f, "🟡 Medium"),
            Priority::Low => write!(f, "🟢 Low"),
            Priority::None => write!(f, "⚪ None"),
        }
    }
}

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    priority: Priority,
    completed: bool,
}

impl Task {
    fn new(id: usize, description: String, priority: Priority) -> Self {
        Task {
            id,
            description,
            priority,
            completed: false,
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut task_id_counter = 1;

    println!("🛠️ Welcome to CLI To-Do List !");
    println!("=====================================");

    loop {
        println!("{}", "== To-Do List Manager ==".bold().underline().blue());
        println!("{}", "1. ✅ Add Task".cyan());
        println!("{}", "2. 📋 List Tasks".cyan());
        println!("{}", "3. ✔️ Mark Task as Completed".cyan());
        println!("{}", "4. ❌ Delete Task".cyan());
        println!("{}", "5. 👋 Exit".cyan());

        let choice = read_input("Enter your choice (1-5): ");

        match choice.trim() {
            "1" => add_task(&mut tasks, &mut task_id_counter),
            "2" => list_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                println!("👋 Thank you for using To-Do List Manager!");
                break;
            }
            _ => {
                println!("❌ Invalid choice. Please enter 1, 2, 3, 4, or 5.");
            }
        }
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn add_task(tasks: &mut Vec<Task>, task_id_counter: &mut usize) {
    println!("\n--- Adding New Task ---");

    let description = read_input("Enter task description: ");
    let description = description.trim();

    if description.is_empty() {
        println!("❌ Task description cannot be empty!");
        return;
    }

    let priority_input = read_input("Enter priority (High/Medium/Low) or press Enter for None: ");
    let priority = match priority_input.trim().to_lowercase().as_str() {
        "high" | "h" => Priority::High,
        "medium" | "m" => Priority::Medium,
        "low" | "l" => Priority::Low,
        _ => Priority::None,
    };

    let task = Task::new(*task_id_counter, description.to_string(), priority);
    tasks.push(task);

    println!("✅ Task added successfully with ID {}!", *task_id_counter);
    *task_id_counter += 1;
}

fn list_tasks(tasks: &Vec<Task>) {
    println!("\n--- Your Tasks ---");

    if tasks.is_empty() {
        println!("📭 No tasks yet. Add some tasks to get started!");
        return;
    }

    println!("Total tasks: {}", tasks.len());
    let completed_count = tasks.iter().filter(|t| t.completed).count();
    println!(
        "Completed: {} | Pending: {}",
        completed_count,
        tasks.len() - completed_count
    );
    println!("---");

    for task in tasks {
        let status = if task.completed {
            "✅ Done"
        } else {
            "❌ Pending"
        };
        println!(
            "[{}] {} | Priority: {} | Status: {}",
            task.id, task.description, task.priority, status
        );
    }
}

fn mark_task_completed(tasks: &mut Vec<Task>) {
    println!("\n--- Mark Task as Completed ---");

    if tasks.is_empty() {
        println!("📭 No tasks to mark as completed!");
        return;
    }

    // Show pending tasks only
    let pending_tasks: Vec<&Task> = tasks.iter().filter(|t| !t.completed).collect();
    if pending_tasks.is_empty() {
        println!("🎉 All tasks are already completed!");
        return;
    }

    println!("Pending tasks:");
    for task in &pending_tasks {
        println!(
            "[{}] {} (Priority: {})",
            task.id, task.description, task.priority
        );
    }

    let id_input = read_input("Enter Task ID to mark as completed: ");

    match id_input.trim().parse::<usize>() {
        Ok(id) => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                if task.completed {
                    println!("⚠️ Task {} is already completed!", id);
                } else {
                    task.completed = true;
                    println!("✅ Task {} '{}' marked as completed!", id, task.description);
                }
            } else {
                println!("⚠️ Task with ID {} not found.", id);
            }
        }
        Err(_) => {
            println!("❌ Invalid task ID. Please enter a valid number.");
        }
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("\n--- Delete Task ---");

    if tasks.is_empty() {
        println!("📭 No tasks to delete!");
        return;
    }

    // Show all tasks
    println!("All tasks:");
    for task in tasks.iter() {
        let status = if task.completed {
            "✅ Done"
        } else {
            "❌ Pending"
        };
        println!(
            "[{}] {} | Priority: {} | Status: {}",
            task.id, task.description, task.priority, status
        );
    }

    let id_input = read_input("Enter Task ID to delete: ");

    match id_input.trim().parse::<usize>() {
        Ok(id) => {
            if let Some(index) = tasks.iter().position(|t| t.id == id) {
                let deleted_task = tasks.remove(index);
                println!(
                    "🗑️ Task {} '{}' deleted successfully!",
                    id, deleted_task.description
                );
            } else {
                println!("⚠️ Task with ID {} not found.", id);
            }
        }
        Err(_) => {
            println!("❌ Invalid task ID. Please enter a valid number.");
        }
    }
}
