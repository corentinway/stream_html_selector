mod elements;
mod html_selector;
mod tag_iterator;
mod tag_parser;

use crate::html_selector::HtmlSelector;
use std::fs;
use std::time::Instant;

fn main() {
    let filename = "./amazon_command.html";
    let html = fs::read_to_string(filename).unwrap();

    let css_request = vec!["td"];

    let mut html_selector = HtmlSelector::new();

    let now = Instant::now();
    let count = html_selector.count(&html, &css_request);

    println!("Parsing execution time: {} ms", now.elapsed().as_millis());

    assert_eq!(count, vec![69]);
}
