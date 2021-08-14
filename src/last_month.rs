extern crate ureq;
extern crate scraper;

use scraper::{Html, Selector};
use colored::Colorize;

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
    if last_month[1].contains("+") { last_month[1] = last_month[1].green().to_string() }
    else if last_month[1].contains("-") { last_month[1] = last_month[1].red().to_string() }

    // % Gain
    class = Selector::parse(r#"td[class="right gainorloss italic"]"#).unwrap();
    last_month.push(stats.select(&class).next().unwrap().inner_html());
    if last_month[2].contains("+") { last_month[2] = last_month[2].green().to_string() }
    else if last_month[2].contains("-") { last_month[2] = last_month[2].red().to_string() }

    // Peak Players
    class = Selector::parse(r#"td[class="right num italic"]"#).unwrap();
    last_month.push(stats.select(&class).next().unwrap().inner_html());

    return last_month;
}