extern crate ureq;
extern crate scraper;

use scraper::{Html, Selector};

pub fn run(link: String) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let domain = format!("https://steamcharts.com{}", link); // Final URL
    let body = ureq::get(&domain)
        .call()
        .into_string()
        .unwrap();
    
    let statpage = Html::parse_document(&body);
    let span = Selector::parse(r#"span[class="num"]"#).unwrap();
    for stat in statpage.select(&span) {
        data.push(stat.inner_html());
    }
    if data.len() == 0 {
        println!("Internal Server Error");
        std::process::exit(0);
    }
    return data;
}