extern crate ureq;
extern crate scraper;

use scraper::{Html, Selector};

#[path = "./utils.rs"]
mod utils;

pub fn run(link: String) -> Vec<String> {
    let mut stats: Vec<String> = Vec::new();
    let domain = format!("https://steamcharts.com{}", link); // Final URL
    let body = ureq::get(&domain)
        .call()
        .into_string()
        .unwrap();
    
    let statpage = Html::parse_document(&body);
    let span = Selector::parse(r#"span[class="num"]"#).unwrap();
    for stat in statpage.select(&span) {
        stats.push(stat.inner_html());
    }
    if stats.len() == 0 { utils::abort("Internal Server Error") }
    return stats;
}