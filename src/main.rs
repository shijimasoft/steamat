extern crate clearscreen;

mod search;
mod statistic;
mod last_month;
mod utils;

use colored::Colorize;

fn main() {
    let ver = "v1.1";
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        utils::abort(r#"
        Usage: steamat [option] "game title"
        
        OPTIONS:
            
            -lm,  --last-month   Last month statistics
            -v,   --version      Show steamat version
        "#);
    }
    match args[1].clone().as_ref() {
        // Version
        "-v" | "--version" => println!("STEAMAT version: {}", ver),

        // Last month stats
        "-lm" | "--last-month" => {
            if args.len() < 3 { utils::abort("Please also enter the game title") }
            let searched = search::run(args[2].clone());
            let mut last_month = last_month::run(searched[1].to_string());
            if last_month[1] == "" { last_month[1] = "N/A".to_string()}
            if last_month[2] == "" { last_month[2] = "N/A".to_string()}
            clearscreen::clear().unwrap();
            println!("
    > GAME: {}

    [Last 30 Days]
      
      {} = {}
      {}       = {}
      {}     = {}
      {}    = {}
            ", searched[0].bold(), "Average players".bold(), last_month[0], "Gain-Loss".bold(), last_month[1], "% Gain-Loss".bold(), last_month[2], "Peak players".bold(), last_month[3]);
        }

        // Current stats
        _ => {
            let searched = search::run(args[1].clone());
            let stats = statistic::run(searched[1].to_string());
            clearscreen::clear().unwrap();
            println!("
    > GAME: {}
            
      {} players are online
              
      [Peak]
      {} = {} players
      {} = {} players\n", searched[0].bold(), stats[0].green(), "24-hour".bold(), stats[1], "all-time".bold(), stats[2]);
        }
    }
}