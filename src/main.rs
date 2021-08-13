extern crate clearscreen;

mod search;
mod statistic;

use colored::Colorize;
use std::io::Write;

fn main() {
    let game_name: Vec<String> = std::env::args().collect();
    if game_name.len() < 2 {
        println!("Please enter the name of the game.");
    } 
    else 
    {
        let searched = search::run(game_name[1].to_string());
        let stats = statistic::run(searched[1].to_string());
        clearscreen::clear().unwrap();
        std::io::stdout().flush().unwrap();
        println!("
    > GAME: {}
    
      {} players are online
      
      [Peak]
      {} = {} players
      {} = {} players\n", searched[0].bold(), stats[0].green(), "24-hour".bold(), stats[1], "all-time".bold(), stats[2]);
    }
}