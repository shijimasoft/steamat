extern crate ureq;
extern crate scraper;
extern crate term_table;
extern crate clearscreen;

#[path = "./utils.rs"]
mod utils;

use scraper::{Html, Selector};
use colored::Colorize;
use term_table::{Table, TableStyle, row::Row, table_cell::{Alignment, TableCell}};

pub fn run(link: String, title: String) {
    let domain = format!("https://steamcharts.com{}", link);
    let body = ureq::get(&domain)
        .call()
        .into_string()
        .unwrap();
    let historypage = Html::parse_document(&body);

    /* History data */

    // Avg. Players
    let mut select = Selector::parse(r#"td[class="right num-f"]"#).unwrap();
    let avg: Vec<String> = historypage.select(&select).map(|avg| avg.inner_html()).collect();

    // Gain
    select = Selector::parse(r#"td[class="right num-p gainorloss"]"#).unwrap();
    let gain: Vec<String> = historypage.select(&select).map(|gain| gain.inner_html()).collect();

    // % Gain
    select = Selector::parse(r#"td[class="right gainorloss"]"#).unwrap();
    let pctgain: Vec<String> = historypage.select(&select).map(|pctgain| pctgain.inner_html()).collect();
    
    // Peak Players
    select = Selector::parse(r#"td[class="right num"]"#).unwrap();
    let peaks: Vec<String> = historypage.select(&select).map(|peak| peak.inner_html()).collect();
    
    // Months
    select = Selector::parse(r#"td[class="month-cell left"]"#).unwrap();
    let months: Vec<String> = historypage.select(&select).map(|month| month.inner_html()).collect();

    /* History Table */
    let mut historytab = Table::new();
    historytab.style = TableStyle::simple();

    // Title head
    historytab.add_row(Row::new(vec![TableCell::new_with_alignment(title.bold(), 5, Alignment::Center)]));

    // Properties
    historytab.add_row(Row::new(vec![
            TableCell::new("Month".bold()),
            TableCell::new("Avg. Players".bold()),
            TableCell::new("Gain".bold()),
            TableCell::new("% Gain".bold()),
            TableCell::new("Peak Players".bold())
        ]));
    
    let mut index: usize = 0;

    // Data to Table
    for _ in 0..months.len() {
        let gain_color = utils::to_color(gain.get(index).unwrap().clone());
        let pctgain_color = utils::to_color(pctgain.get(index).unwrap().clone());
        historytab.add_row(Row::new(vec![
            TableCell::new(months.get(index).unwrap().trim()),
            TableCell::new(avg.get(index).unwrap()),
            TableCell::new(gain_color),
            TableCell::new(pctgain_color),
            TableCell::new(peaks.get(index).unwrap())
        ]));
        index += 1;
    }

    // Show Table
    clearscreen::clear().unwrap();
    println!("{}", historytab.render());
}