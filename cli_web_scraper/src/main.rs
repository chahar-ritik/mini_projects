use reqwest::Client;
use scraper::{Html, Selector};
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run <url1> <url2>");
        return;
    }

    let pg1 = page_title(&args[1]);
    let pg2 = page_title(&args[2]);

    tokio::select! {
        res = pg1 => {
            println!("First finished {}", res.0);

            match res.1 {
                Some(title) => println!("Title: {}", title),
                None => println!("Title not found"),
            }
        },

        res = pg2 => {
            println!("First finished {}", res.0);

            match res.1 {
                Some(title) => println!("Title: {}", title),
                None => println!("Title not found"),
            }
        },
    }
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let page = reqwest::get(url).await.unwrap().text().await.unwrap();
    let doc = Html::parse_document(&page);
    let selector = Selector::parse("title").unwrap();

    let tittle = doc.select(&selector).next().map(|t| t.inner_html());
    (url, tittle)
}

/*
//This web scraper is made using trpl crat that is only for learning purpose. In actuall developing we will not use trpl crate

use trpl::{Html,Either};
use std::env;
// async main needs runtime nut main can initialize a runtime but it is not a runtime itself
//instead we can use trpl::run and async fn as an argument
// and if we want async main we need runtime one runtime example tokio
async fn page_title(url: &str) ->(&str, Option<String>) {
  let response = trpl::get(url).await;
  let text = response.text().await;
  let maybe_title = Html::parse(&text).select_first("title").map(|title_element| title_element.inner_html());
  (url,maybe_title)
}


fn main(){
    let  args :Vec<String> = env::args().collect();

  trpl::run(async{

    let pg1 = page_title(&args[1]);
    let pg2= page_title(&args[2]);

    let (url,maybe_title) =
    match trpl::race(pg1,pg2).await{
        Either::Left(left) => left,
        Either::Right(right) => right
    };

    println!("{url} faster one");

    match maybe_title {
       Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
    }

})
    }

*/
