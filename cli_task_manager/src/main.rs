mod cli;
mod storage;

use storage::Task;
use clap::Parser;
use cli::Cli;
fn main() {
  let cli = Cli::parse();
  let mut tasks = storage::load_task();
      match cli.command{
        cli::Commands::Add { task, is_done } => {
         let status =  if is_done {"Done"} else {"Undone"};
          tasks.push(Task{
            task : task,
            is_done : status.to_string()
          });
          storage::save_task(&tasks);
          for c in &tasks{
          println!("Task : {} , Status : {}" ,c.task,c.is_done);
         } },
          }

}
