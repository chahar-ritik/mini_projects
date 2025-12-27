use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub search_string: String,
    pub file_name: String,
    pub ignore_case: bool,
}

impl Config {
    // in place of new we make build method because new are not expected to fail
    pub fn build(mut args: impl Iterator < Item = String>) -> Result<Config, &'static str> {
         args.next();

        let search = match args.next() {
             Some(x)=> x,
             None => return Err("MISSING SEARCH STRING")
        };
        let file = match args.next() {
             Some(x)=> x,
             None => return Err("MISSING FILE PATH")
        };

        //IGNORE_CASE=1 cargo run -- to myfile.txt
        //Here we are using IGNOR_CASE to set variable value true by 1 or empty
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            search_string: search,
            file_name: file,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_name)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.search_string, &content)
    } else {
        search(&config.search_string, &content)
    };

    for i in result {
        println!("{i}");
    }
    Ok(())
}

fn search<'a>(search_string: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(search_string) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(search_string: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let search_string = search_string.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&search_string) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let search_string = "duct";
        let content = "\
    Rust:
Safe,fast, productive ,Duct";

        assert_eq!(
            vec!["Safe,fast, productive"],
            search(search_string, content)
        )
    }

    #[test]
    fn case_insensitive() {
        let search_string = "duct";
        let content = "\
    Rust:
Safe,fast, productive
Trust";

        assert_eq!(
            vec!["Rust:", "Trust"],
            search_case_insensitive(search_string, content)
        )
    }
}
