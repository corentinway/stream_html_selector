extern crate stream_html_selector;

use stream_html_selector::css_selector;

#[test]
fn should_do_foo() {
    
    let matcher = vec![
        css_selector!(#costBreakdown)
    ];

    let matchers = vec![&matcher];
    
    let path = "./amazon_command.html";
    let html = std::fs::read_to_string(path).unwrap();

    let counts = stream_html_selector::by_tag_path::count(html.as_str(), &matchers);

    assert_eq!(vec![1], counts);

}
