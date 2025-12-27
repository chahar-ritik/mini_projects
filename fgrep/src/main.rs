use fgrep::Config;
use std::env;
use std::process;

fn main() {
   // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        /* eprintln! this is a standard error that not like normal print as it standard output
         if we run command cargo run > output.txt that redirect output to this file instead of screen
         and this command has error as no argument supplied but standart output will not show it on screen */
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.search_string);
    println!("In file {}", config.file_name);

    if let Err(e) = fgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
