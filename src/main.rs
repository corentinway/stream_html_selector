use stream_html_selector::count;

use std::fs;
use std::time::Instant;

fn main() {
    let filename = "./amazon_command.html";
    let html = fs::read_to_string(filename).unwrap();

    let css_request = vec!["td"];

    let now = Instant::now();
    let count = count(html.as_str(), &css_request);

    println!("Parsing execution time: {} ms", now.elapsed().as_millis());

    assert_eq!(count, vec![69]);
}
