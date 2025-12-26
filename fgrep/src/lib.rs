use std::fs;
use std::error::Error;
pub struct Config {

   pub search_string : String,
   pub file_name : String,

}


impl Config{
    // in place of new we make build method because new are not expected to fail
   pub fn build(args : &[String]) -> Result<Config,&'static str>{
      if args.len() < 3{
          return Err("not enough arguments")
      } 
      let search = args[1].clone();
       let file  = args[2].clone();

      Ok(Config{search_string: search , file_name: file})

    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
   
      let content = fs::read_to_string(config.file_name)?;
      
    for line in search(&config.search_string, &content) {
        println!("{line}");
    }

        Ok(())

}

fn search<'a>(search_string : &str , file_name :  & 'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    
    for line in file_name.lines(){
      if line.contains(search_string){
         results.push(line);
      }
    }
    results
}


#[cfg(test)]
mod tests{
  use super::*;


  #[test]
  fn res(){
    let search_string = "duct";
    let file_name  = "\
    Rust:
Safe,fast, productive";

    assert_eq!(vec!["Safe,fast, productive"],search(search_string , file_name))

  }
}