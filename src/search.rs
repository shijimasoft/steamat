extern crate ureq;
extern crate scraper;

#[path = "./utils.rs"]
mod utils;

use scraper::{Html, Selector};
use std::io::{self, Write};

pub fn run(game: String) -> [String; 2] {
    let url = format!("https://steamcharts.com/search/?q={}", game.replace(" ", "+")); // Search URL
    let body = ureq::get(&url) // Get HTML code
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    
    let img = Selector::parse("img").unwrap();
    let a = Selector::parse("a").unwrap();
    let query = Html::parse_document(&body);
    let mut idx: u16 = 0;
    let mut names: Vec<&str> = Vec::new();
    let mut parts: Vec<&str> = Vec::new();
    
    for alt in query.select(&img) {
        println!("|{}| {}", idx, alt.value().attr("alt").unwrap()); // Print all occurences with index
        names.push(alt.value().attr("alt").unwrap());
        idx += 1;
    }
    if names.len() == 0 { utils::abort("No matching games found.") }

    for part in query.select(&a) {
        parts.push(part.value().attr("href").unwrap());
    }
    // Removal of unnecessary parts
    parts.truncate(parts.len()-5); // "https://twitter.com/steamcharts" "/about" "/privacy" "/ads" "http://steampowered.com"
    parts.remove(0); // "/"
    let mut o = 1; parts.retain(|_| { o +=1; o % 2 == 0 }); // Remove doubled values

    if names.len() == 1 { return [names[0].to_string(), parts[0].to_string()] }

    let mut choice = String::from("");
    print!(">> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut choice).unwrap();
    let select: usize = choice.trim_end().parse::<usize>().expect("Please enter a number");
    if select > parts.len()-1 { utils::abort("Please enter a correct option") }

    return [names[select].to_string(), parts[select].to_string()]
}