use stream_html_selector::by_tag_path::find_first;

use std::fs;
use std::time::Instant;
use stream_html_selector::css_selector;

fn main() {
    let filename = "./amazon_command.html";
    let html = fs::read_to_string(filename).unwrap();

    let now = Instant::now();

    // #header > tbody > tr:nth-child(2) > td > a
    let command_number_matcher = vec![
        css_selector!(#header),
        css_selector!(tbody),
        css_selector!(tr: nth - child(2)),
        css_selector!(td),
        css_selector!(a),
    ];

    let total_matcher = vec![
        css_selector!(#costBreakdown),
        css_selector!(tbody),
        css_selector!(tr: nth - child(9)),
        css_selector!(td: nth - child(2)),
        css_selector!(strong),
    ];

    let matchers = vec![&command_number_matcher, &total_matcher];

    let founds = find_first(html.as_str(), &matchers);

    println!("Parsing execution time: {} ms", now.elapsed().as_millis());
    println!(
        "Command number {:?} for a total of {:?}",
        founds.get(0),
        founds.get(1)
    );
}
