use crate::elements::start_element::Tag;
use crate::tag_iterator::Elements;
use crate::tag_iterator::TagIterator;

use crate::tag_path::match_tag_path;
use crate::tag_path::TagPathItem;


pub struct TagPathHtmlSelector {
    path: Vec<Box<TagPathItem>>,
}

impl TagPathHtmlSelector {
    fn new() -> Self {
        TagPathHtmlSelector { path: Vec::new() }
    }

    fn count(&mut self, html: &str, matchers: &Vec<&Vec<Box<dyn Fn(&TagPathItem) -> bool>>>) -> Vec<usize> {
        let mut counts = vec![0; matchers.len()];

        let tag_iterator = TagIterator::new(html);
        tag_iterator.for_each(|element| match element {
            Elements::Start(tag, _begin, _end) => {
                self.path.push(Box::new(TagPathItem {
                    tag: Box::new(tag),
                    nth_child: 0,
                }));

                self.update_counts_if_matching(&mut counts, &matchers);
            }
            Elements::End(_tag_name, _begin, _end) => {
                self.path.pop();
            }
            _ => {}
        });

        counts
    }

    fn update_counts_if_matching(
        &self,
        counts: &mut Vec<usize>,
        matchers: &Vec<&Vec<Box<dyn Fn(&TagPathItem) -> bool>>>,
    ) {
        self.check_any_matching(&matchers)
            .into_iter()
            .enumerate()
            .for_each(|(index, does_match)| {
                if does_match {
                    if let Some(value) = counts.get_mut(index) {
                        *value = *value + 1;
                    }
                }
            });
    }

    fn check_any_matching(&self, matchers: &Vec<&Vec<Box<dyn Fn(&TagPathItem) -> bool>>>) -> Vec<bool> {
        matchers
            .into_iter()
            .map(|matcher| self.check_matching(&matcher))
            .collect()
    }

    fn check_matching(&self, first_matcher: &Vec<Box<dyn Fn(&TagPathItem) -> bool>>) -> bool {
        let path: Vec<&TagPathItem> = self
            .path
            .iter()
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

    use std::{fs, path::is_separator};

    fn get_amazon_email_html() -> String {
        let filename = "./amazon_command.html";
        fs::read_to_string(filename).unwrap()
    }
    fn get_simple_email_html() -> String {
        let filename = "./simple_table.html";
        fs::read_to_string(filename).unwrap()
    }

    #[test]
    fn should_count_row_and_cells() {
        let html = get_simple_email_html();

        let path_matcher1 = vec![
            css_selector!(table),
            css_selector!(tbody),
            css_selector!(tr),
        ];
        let path_matcher2 = vec![
            css_selector!(table),
            css_selector!(tbody),
            css_selector!(tr),
            css_selector!(td),
        ];

        let paths_matcher = vec![&path_matcher1, &path_matcher2];

        let mut html_selector = TagPathHtmlSelector::new();

        let counts = html_selector.count(&html, &paths_matcher);

        assert_eq!(vec![4, 12], counts);
    }
    #[test]
    fn should_get_total() {
        // #costBreakdown > tbody > tr:nth-child(9) > td:nth-child(2) > strong

        let html = get_amazon_email_html();

        let path_matcher1 = vec![
            css_selector!(#costBreakdown),
            css_selector!(tbody),
        ];

        let paths_matcher = vec![&path_matcher1];

        let mut html_selector = TagPathHtmlSelector::new();

        let counts = html_selector.count(&html, &paths_matcher);

        assert_eq!(vec![1], counts);
    }
}
