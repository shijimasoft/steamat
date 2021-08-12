extern crate ureq;
extern crate scraper;

use scraper::{Html, Selector};
use std::io::{self, Write};

pub fn run(game: String) -> [String; 2] {
    let url = format!("https://steamcharts.com/search/?q={}", game.replace(" ", "+")); // Search URL
    let body = ureq::get(&url) // Get HTML code
        .call()
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
    for part in query.select(&a) {
        parts.push(part.value().attr("href").unwrap());
    }
    // Removal of unnecessary parts
    parts.truncate(parts.len()-5); // "https://twitter.com/steamcharts" "/about" "/privacy" "/ads" "http://steampowered.com"
    parts.remove(0); // "/"
    let mut o = 1; parts.retain(|_| { o +=1; o % 2 == 0 }); // Remove doubled values
    
    let mut choice = String::from("");
    print!(">> ");
    io::stdout().flush().expect("stdout: error in flush");
    io::stdin().read_line(&mut choice).expect("stdin: error in read_line");
    let select: usize = choice.trim_end().parse::<usize>().expect("Please enter a number");

    if select > parts.len()-1 {
        println!("Please enter a correct option");
        return ["".to_string(), "".to_string()];
    } else {
        return [names[select].to_string(), parts[select].to_string()];
    }
}