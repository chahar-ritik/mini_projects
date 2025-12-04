mod cli;
mod storage;

use clap::Parser;
use cli::Cli;
use storage::Task;
fn main() {
    let cli = Cli::parse();
    let mut tasks = storage::load_task();
    match cli.command {
        cli::Commands::Add { task, is_done } => {
            let status = if is_done { "Done" } else { "Undone" };
            tasks.push(Task {
                task: task,
                is_done: status.to_string(),
            });
            storage::save_task(&tasks);
            for c in &tasks {
                println!("Task : {} , Status : {}", c.task, c.is_done);
            }
        }

        cli::Commands::Complete { task } => {
            let mut t = false;
            for c in &mut tasks {
                if task == c.task {
                    c.is_done = "Done".to_string();
                    println!("Task Completed : Task {} , Status {}", c.task, c.is_done);
                    t = true;
                }
            }
            if !t {
                println!("Task doesn't exit")
            }
            storage::save_task(&tasks);
        }
        cli::Commands::Delete { task } => {
            tasks.retain(|c| !(c.task == task));
            storage::save_task(&tasks);
        }
        cli::Commands::List => {
            if tasks.is_empty() {
                println!("There is no task to lisk")
            } else {
                for c in &tasks {
                    println!("Task : {} , Status : {}", c.task, c.is_done);
                }
            }
        }
        cli::Commands::Filter { status } => {
            if status != "Done" && status != "Undone" {
                println!("Invalid! Status can be Done or Undone.");
                return;
            }
            let mut found = false;
            for c in &tasks {
                if c.is_done == status {
                    println!("Task : {} , Status : {}", c.task, c.is_done);
                    found = true;
                }
                if !found {
                    println!("No task to filter")
                }
            }
              if !found {
                    println!("No task to filter")
                }
        }
    }
}
