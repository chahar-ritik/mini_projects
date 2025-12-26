use std::fs;
use std::error::Error;
use std::env;
pub struct Config {

   pub search_string : String,
   pub file_name : String,
   pub ignore_case : bool

}

 
impl Config{
    // in place of new we make build method because new are not expected to fail
   pub fn build(args : &[String]) -> Result<Config,&'static str>{
      if args.len() < 3{
          return Err("not enough arguments")
      } 
      let search = args[1].clone();
       let file  = args[2].clone();
       //IGNORE_CASE=1 cargo run -- to myfile.txt
       //Here we are using IGNOR_CASE to set variable value true by 1 or empty
       let ignore_case = env::var("IGNORE_CASE").is_ok();
      Ok(Config{search_string: search , file_name: file , ignore_case})

    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
     
      let content = fs::read_to_string(&config.file_name)?;
  
 

      let result = if config.ignore_case {   
      search_case_insensitive(&config.search_string, &content)
     
      }
   else { 
        search(&config.search_string, &content)
       
    };
   
      for i in result{
        println!("{i}");
      }
        Ok(())

}

fn search<'a>(search_string : &str , content :  & 'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    
    for line in content.lines(){
      if line.contains(search_string){
         results.push(line);
      }
    }
    results
}


fn search_case_insensitive<'a>(search_string : &str , content :  & 'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
   let search_string = search_string.to_lowercase(); 
    
    for line in content.lines(){
      if line.to_lowercase().contains(&search_string){
         results.push(line);
      }
    }
    results
}

#[cfg(test)]
mod tests{
  use super::*;


  #[test]
  fn case_sensitive(){
    let search_string = "duct";
    let content  = "\
    Rust:
Safe,fast, productive ,Duct";

    assert_eq!(vec!["Safe,fast, productive"],search(search_string , content))

  }

  #[test]
  fn case_insensitive(){
 let search_string = "duct";
    let content  = "\
    Rust:
Safe,fast, productive
Trust";

    assert_eq!(vec!["Rust:","Trust"],search_case_insensitive(search_string , content))

  }
}