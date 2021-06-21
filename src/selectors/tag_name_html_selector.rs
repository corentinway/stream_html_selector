use crate::elements::start_element::Tag;
use crate::selectors::format_css_request;
use crate::tag_iterator::{Elements, TagIterator};

use crate::selectors::HtmlSelectorCounter;
use crate::selectors::HtmlSelectorFindFirst;


pub struct TagNameHtmlSelector {
    tag_name_path: Vec<String>,
    tag_name_path_string: String,
}

impl TagNameHtmlSelector {
    pub fn new() -> Self {
        TagNameHtmlSelector {
            tag_name_path: Vec::new(),
            tag_name_path_string: String::new(),
        }
    }

    fn search_for_css(&self, css_requests: &Vec<String>, counts: &mut Vec<usize>) {
        css_requests
            .iter()
            .enumerate()
            .for_each(|(index, request)| {
                if self.match_request(request) {
                    counts[index] += 1;
                }
            });
    }

    fn does_match_css_request(&self, css_requests: &Vec<String>) -> Option<usize> {
        let a = css_requests
            .iter()
            .enumerate()
            .find(|(_, request)| self.match_request(request));
        a.map(|(index, _)| index)
    }

    fn match_request(&self, request: &str) -> bool {
        self.tag_name_path_string.ends_with(request)
    }

    fn reduce_path(&mut self) {
        self.tag_name_path.pop();
        self.tag_name_path_string = self.tag_name_path.join(" ");
    }

    fn increase_path(&mut self, tag: Box<Tag>) {
        self.tag_name_path.push(tag.name.clone()); // FIXME
        self.tag_name_path_string = self.tag_name_path.join(" ");
    }
}

impl HtmlSelectorCounter<&str> for TagNameHtmlSelector {
    fn count(&mut self, html: &str, css_requests: &[&str]) -> Vec<usize> {
        let css_requests = format_css_request(css_requests);
        let mut counts = vec![0; css_requests.len()];

        let tag_iterator = TagIterator::new(html);
        tag_iterator.for_each(|element| {
            //println!("PATH : {:?}", self.tag_path);

            match element {
                Elements::Start(tag, _begin, _end) => {
                    let is_autoclosing = tag.is_autoclosing;
                    self.increase_path(Box::new(tag));
                    self.search_for_css(&css_requests, &mut counts);
                    if is_autoclosing {
                        self.reduce_path();
                    }
                }
                Elements::End(_, _, _) => {
                    self.reduce_path();
                }
                //Elements::Comment(tag) => {},
                //Elements::Text(tag) => {},
                _ => {}
            }
        });

        counts
    }
}

impl HtmlSelectorFindFirst<&str> for TagNameHtmlSelector {
    fn find_first(&mut self, html: &str, css_requests: &[&str]) -> String {
        let css_requests = format_css_request(css_requests);
        let mut text = String::new();
        let mut find_first_position = None;

        let tag_iterator = TagIterator::new(html);

        for element in tag_iterator {
            match element {
                Elements::Start(tag, begin, end) => {
                    self.increase_path(Box::new(tag));
                    if let Some(index) = self.does_match_css_request(&css_requests) {
                        // get begin and end position of the tag in the
                        // then, if the next decrease the path with the ending tag,
                        // so we have all tag position
                        find_first_position = Some(end);
                    }
                }
                Elements::End(_, begin, _end) => {
                    if let Some(position) = find_first_position {
                        let content = html.get(position..begin);
                        if let Some(content) = content {
                            text.push_str(content);
                            break;
                        }
                    }
                    self.reduce_path();
                }
                //Elements::Comment(tag) => {},
                Elements::Text(tag) => {}
                _ => {}
            }
        }

        text
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

        let css_request = vec![" body"];

        let mut html_selector = TagNameHtmlSelector::new();

        let count = html_selector.count(&html, &css_request);

        assert_eq!(count, vec![1]);
    }
    #[test]
    fn should_count_all_td_tag() {
        let html = get_html();

        let css_request = vec![" td"];

        let mut html_selector = TagNameHtmlSelector::new();

        let now = Instant::now();
        let count = html_selector.count(&html, &css_request);

        println!("Parsing execution time: {} ms", now.elapsed().as_millis());

        assert_eq!(count, vec![69]);
    }

    #[test]
    fn should_match_many_tag_request() {
        let html = get_html();
        let css_requests = vec![" body", " td"];

        let mut html_selector = TagNameHtmlSelector::new();

        let counts = html_selector.count(&html, &css_requests);

        assert_eq!(counts, vec![1, 69]);
    }

    #[test]
    fn should_match_child_tag_request() {
        let html = String::from(
            r#"
            <html>
            <body>
                <div>
                <p>foo</p>
                <p>bar</p>
                <p>baz</p>
                </div>
            </body>
            </html>
        "#,
        );
        // /!\
        let css_requests = vec![" body", " div p"];

        let mut html_selector = TagNameHtmlSelector::new();

        let counts = html_selector.count(&html, &css_requests);

        assert_eq!(counts, vec![1, 3]);
    }

    #[test]
    fn should_find_firstpattern_and_return_text() {
        let html = String::from(
            r#"
            <html>
            <body>
                <div>
                <p>foo</p>
                <p>bar</p>
                <p>baz</p>
                </div>
            </body>
            </html>
        "#,
        );
        let css_requests = vec![" body div p"];

        let mut html_selector = TagNameHtmlSelector::new();

        let texts = html_selector.find_first(&html, &css_requests);

        assert_eq!(texts, "foo".to_string());
    }
}
