use crate::css_selector;
use crate::elements::start_element::Tag;
use crate::tag_iterator::Elements;
use crate::tag_iterator::TagIterator;

use crate::tag_path::match_tag_path;

pub struct TagPathHtmlSelector {
    path: Vec<Box<Tag>>,
}

impl TagPathHtmlSelector {
    fn new() -> Self {
        TagPathHtmlSelector { path: Vec::new() }
    }

    fn count(&mut self, html: &str, matcher: &Vec<Box<dyn Fn(&Tag) -> bool>>) -> usize {
        let mut count = 0;

        let tag_iterator = TagIterator::new(html);
        tag_iterator.for_each(|element| {
            println!("path: {:?}", self.path);
            match element {
            Elements::Start(tag, _begin, _end) => {
                self.path.push(Box::new(tag));

                if self.check_matching(matcher) {
                    count += 1;
                }

            },
            Elements::End(_tag_name, _begin, _end) => {
                self.path.pop();
            }
            _ => {}
        }});

        count
    }
    fn check_matching(&mut self, first_matcher: &Vec<Box<dyn Fn(&Tag) -> bool>> ) -> bool {
        let path : Vec<&Tag> = self.path.iter()
            .map(|boxed_tag| boxed_tag.as_ref())
            .collect();


        match_tag_path(path, first_matcher)
    }
}

#[macro_use]
#[cfg(test)]
mod test_tag_path_html_selector {

    use crate::css_selector;

    use super::*;

    use std::fs;

    fn get_html() -> String {
        let filename = "./amazon_command.html";
        fs::read_to_string(filename).unwrap()
    }

    // #costBreakdown > tbody > tr:nth-child(9) > td:nth-child(2) > strong
    #[test]
    fn should_select_by_id() {
        let html = get_html();

        let path_matcher = vec![css_selector!(#costBreakdown), css_selector!(tbody)];

        let mut html_selector = TagPathHtmlSelector::new();

        let count = html_selector.count(&html, &path_matcher);

        assert_eq!(1, count);
    }
}
