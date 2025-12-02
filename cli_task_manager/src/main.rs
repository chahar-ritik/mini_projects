mod cli;


use clap::Parser;
use cli::Cli;
fn main() {
  let cli = Cli::parse();
      match cli.command{
        cli::Commands::Add { task, is_done } => {
         let status =  if is_done {"Done"} else {"Undone"};
          println!("Task : {} , Status : {}" ,task,status);
         },
          }

}
