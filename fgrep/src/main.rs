use std::env;
use std::fs;
fn main(){
    let args: Vec<String> = env::args().collect();
     

     let search = &args[1];
     let file_name = &args[2];

      println!("Searching for {search}");
    println!("In file {file_name}");

      let content = fs::read_to_string(file_name).expect("something");

      println!("With text:\n{content}");


}