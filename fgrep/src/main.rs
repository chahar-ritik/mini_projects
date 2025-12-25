use std::env;
use std::fs;
use std::error::Error;
use std::process;
fn main(){
    let args: Vec<String> = env::args().collect();
     
       let config = Config::build(&args).unwrap_or_else(|err|{
         println!("Problem parsing arguments: {err}");
        process::exit(1);
       });

      println!("Searching for {}",config.search_string);
    println!("In file {}",config.file_name);

 if let Err(e) = run(config){
    println!("Application error: {e}");
     process::exit(1);// if code O success , 1 failure
 };

}

struct Config {
    search_string : String,
    file_name : String,

}


impl Config{
    // in place of new we make build method because new are not expected to fail
    fn build(args : &[String]) -> Result<Config,&'static str>{
      if args.len() < 3{
          return Err("not enough arguments")
      } 
      let search = args[1].clone();
       let file  = args[2].clone();

      Ok(Config{search_string: search , file_name: file})

    }
}

fn run(config:Config) -> Result<(),Box<dyn Error>>{
   
      let content = fs::read_to_string(config.file_name)?;
      
      println!("With text:\n{content}");

        Ok(())

}