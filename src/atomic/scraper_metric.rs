use reqwest;
use scraper::{Html, Selector};
use std::time::Instant;

pub async  fn scraping() {
    

    // Fetch a large HTML document
    let url = "https://en.wikipedia.org/wiki/World_War_II";
    let response = reqwest::get(url).await.expect("Failed to fetch the page");

    let body = response.text().await.expect("Failed to read the response body");
    let start = Instant::now();
    // Parse the HTML document
    let document = Html::parse_document(&body);

    // Select all <h1>, <p>, and <a> tags
    let heading_selector = Selector::parse("h1").unwrap();
    let paragraph_selector = Selector::parse("p").unwrap();
    let link_selector = Selector::parse("a").unwrap();

    // Extract headings, paragraphs, and links
    let headings: Vec<_> = document.select(&heading_selector).collect();
    let paragraphs: Vec<_> = document.select(&paragraph_selector).collect();
    let links: Vec<_> = document.select(&link_selector).collect();

    let duration = start.elapsed();
    println!("Headings: {}, Paragraphs: {}, Links: {}", headings.len(), paragraphs.len(), links.len());
    println!("Execution time (Rust - Scraper): {:?}", duration);
}
