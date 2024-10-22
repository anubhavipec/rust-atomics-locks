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


/*
import time
import requests
from bs4 import BeautifulSoup

# Start the timer
start_time = time.time()

# Fetch the large HTML document from BBC World News
url = "https://en.wikipedia.org/wiki/World_War_II"
response = requests.get(url)
body = response.text

# Parse the HTML document with BeautifulSoup and lxml parser
soup = BeautifulSoup(body, "html.parser")

# Extract headings (h1), paragraphs (p), and links (a)
headings = [heading.text for heading in soup.find_all('h1')]
paragraphs = [paragraph.text for paragraph in soup.find_all('p')]
links = [(link.text, link.get('href', '#')) for link in soup.find_all('a')]

# End the timer
end_time = time.time()
execution_time_python = end_time - start_time

print(f"Headings: {len(headings)}, Paragraphs: {len(paragraphs)}, Links: {len(links)}")
print(f"Execution time (Python - BeautifulSoup): {execution_time_python} seconds")



*/
