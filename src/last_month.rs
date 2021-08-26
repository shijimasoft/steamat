extern crate ureq;
extern crate scraper;

#[path = "./utils.rs"]
mod utils;

use scraper::{Html, Selector};

pub fn run(link: String) -> Vec<String> {
    let mut last_month: Vec<String> = Vec::new();
    let domain = format!("https://steamcharts.com{}", link);
    let body = ureq::get(&domain)
        .call()
        .into_string()
        .unwrap();
    
    let stats = Html::parse_document(&body);

    // Avg. Players
    let mut class = Selector::parse(r#"td[class="right num-f italic"]"#).unwrap();
    last_month.push(stats.select(&class).next().unwrap().inner_html());

    // Gain
    class = Selector::parse(r#"td[class="right num-p gainorloss italic"]"#).unwrap();
    last_month.push(stats.select(&class).next().unwrap().inner_html());
    last_month[1] = utils::to_color(last_month[1].clone());

    // % Gain
    class = Selector::parse(r#"td[class="right gainorloss italic"]"#).unwrap();
    last_month.push(stats.select(&class).next().unwrap().inner_html());
    last_month[2] = utils::to_color(last_month[2].clone());

    // Peak Players
    class = Selector::parse(r#"td[class="right num italic"]"#).unwrap();
    last_month.push(stats.select(&class).next().unwrap().inner_html());

    return last_month;
}