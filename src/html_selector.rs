use crate::tag_iterator::{Elements, TagIterator};

#[derive(PartialEq)]
enum PathUpdateState {
    TagAdded,
    TagDeleted,
    NewTagToDelete,
}

use PathUpdateState::*;

#[derive(PartialEq)]
enum State {
    Start,
    StartElement,
    EndElement,
}

pub struct HtmlSelector {
    tag_path: Vec<String>,
    tag_path_string: String,
    state: State,
}

impl HtmlSelector {
    pub fn new() -> Self {
        HtmlSelector {
            tag_path: Vec::new(),
            tag_path_string: String::new(),
            state: State::Start,
        }
    }

    fn search_for_css(&self, css_requests: &[&str], counts: &mut Vec<usize>) {
        css_requests
            .iter()
            .enumerate()
            .for_each(|(index, request)| {
                if self.match_request(request) {
                    counts[index] += 1;
                }
            });
    }

    pub fn count(&mut self, html: &str, css_requests: &[&str]) -> Vec<usize> {
        let mut counts = vec![0; css_requests.len()];

        let tag_iterator = TagIterator::new(html);
        tag_iterator.for_each(|element| {
            match element {
                Elements::Start(tag) => {
                    self.increase_path(tag.name);
                    self.search_for_css(css_requests, &mut counts);
                    if tag.is_autoclosing {
                        self.reduce_path();
                    }
                }
                Elements::End(_) => {
                    self.reduce_path();
                }
                //Elements::Comment(tag) => {},
                //Elements::Text(tag) => {},
                _ => {}
            }
        });

        counts
    }

    fn match_request(&self, request: &str) -> bool {
        if self.tag_path.len() == 1 {
            self.tag_path_string.ends_with(request)
        } else {
            self.tag_path_string
                .ends_with(format!(" {}", request).as_str())
        }
    }

    fn reduce_path(&mut self) {
        self.tag_path.pop();
        self.tag_path_string = self.tag_path.join(" ");
    }

    fn increase_path(&mut self, tag_name: String) {
        self.tag_path.push(tag_name);
        self.tag_path_string = self.tag_path.join(" ");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::time::Instant;

    fn get_html() -> String {
        let filename = "./amazon_command.html";
        fs::read_to_string(filename).unwrap()
    }

    #[test]
    fn should_count_all_body_tag() {
        let html = get_html();

        let css_request = vec!["body"];

        let mut html_selector = HtmlSelector::new();

        let count = html_selector.count(&html, &css_request);

        assert_eq!(count, vec![1]);
    }
    #[test]
    fn should_count_all_td_tag() {
        let html = get_html();

        let css_request = vec!["td"];

        let mut html_selector = HtmlSelector::new();

        let now = Instant::now();
        let count = html_selector.count(&html, &css_request);

        println!("Parsing execution time: {} ms", now.elapsed().as_millis());

        assert_eq!(count, vec![69]);
    }

    #[test]
    fn should_match_many_tag_request() {
        let html = get_html();
        let css_requests = vec!["body", "td"];

        let mut html_selector = HtmlSelector::new();

        let counts = html_selector.count(&html, &css_requests);

        assert_eq!(counts, vec![1, 69]);
    }

    #[test]
    fn should_match_child_tag_request() {
        let html = String::from(
            r#"
            <body>
                <div>
                <p>foo</p>
                <p>bar</p>
                <p>baz</p>
                </div>
            </body>
        "#,
        );
        let css_requests = vec!["body", "div p"];

        let mut html_selector = HtmlSelector::new();

        let counts = html_selector.count(&html, &css_requests);

        assert_eq!(counts, vec![1, 3]);
    }
}
