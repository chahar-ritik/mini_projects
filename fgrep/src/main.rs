use std::env;
use std::process;
use fgrep::Config;

fn main(){
    let args: Vec<String> = env::args().collect();
     
       let config = Config::build(&args).unwrap_or_else(|err|{
         println!("Problem parsing arguments: {err}");
        process::exit(1);
       });

      println!("Searching for {}",config.search_string);
    println!("In file {}",config.file_name);

 if let Err(e) =  fgrep::run(config){
    println!("Application error: {e}");
     process::exit(1);// if code O success , 1 failure
 };

}
