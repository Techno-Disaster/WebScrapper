extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

fn main() {
    println!("This year in the Indian Premier League: ");
    scrape_team_data("https://www.iplt20.com/teams/mumbai-indians/results");
}

fn scrape_team_data(url: &str) {
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());
    let doc_body = Html::parse_document(&resp.text().unwrap());

    let record = Selector::parse(".result__outcome").unwrap();

    for record in doc_body.select(&record) {
        let records = record.text().collect::<Vec<_>>();
        println!(" {}",records[0]);
    }
}