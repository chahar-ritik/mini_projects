use clap::{Parser, Subcommand, command};

#[derive(Parser)]
#[command(name = "Task Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    Add {
        task: String,
        #[arg(short, long)] // -i or --is-done
        is_done: bool,
    },
    Complete {
        task: String,
    },
    Delete {
        task: String,
    },
    List,
    Filter {
        status: String,
    },
}
