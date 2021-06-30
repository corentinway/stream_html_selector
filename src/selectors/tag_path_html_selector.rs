use crate::tag_iterator::Elements;
use crate::tag_iterator::TagIterator;

use crate::tag_path::match_tag_path;
use crate::tag_path::TagPath;
use crate::tag_path::TagPathItem;

pub struct TagPathHtmlSelector {
    path: TagPath,
}

impl TagPathHtmlSelector {
    pub fn new() -> Self {
        TagPathHtmlSelector {
            path: TagPath::new(),
        }
    }

    pub fn count(
        &mut self,
        html: &str,
        matchers: &Vec<&Vec<Box<dyn Fn(&TagPathItem) -> bool>>>,
    ) -> Vec<usize> {
        let mut counts = vec![0; matchers.len()];

        let tag_iterator = TagIterator::new(html);
        tag_iterator.for_each(|element| match element {
            Elements::Start(tag, _begin, _end) => {
                let is_autoclosing_tag = tag.is_autoclosing;
                self.path.add(tag);

                self.update_counts_if_matching(&mut counts, &matchers);

                if is_autoclosing_tag {
                    self.path.reduce()
                }
            }
            Elements::End(_tag_name, _begin, _end) => {
                self.path.reduce();
            }
            _ => {}
        });

        counts
    }

    pub fn find_first(
        &mut self,
        html: &str,
        matchers: &Vec<&Vec<Box<dyn Fn(&TagPathItem) -> bool>>>,
    ) -> Vec<String> {
        let mut founds = vec![String::new(); matchers.len()];
        let mut reading_positions = vec![None; matchers.len()];
        let tag_iterator = TagIterator::new(html);
        tag_iterator.for_each(|element| match element {
            Elements::Start(tag, _begin, end) => {
                let is_autoclosing_tag = tag.is_autoclosing;
                self.path.add(tag);
                self.check_any_matching(&matchers)
                    .into_iter()
                    .enumerate()
                    .for_each(|(index, does_match)| {
                        if does_match {
                            if let Some(position) = reading_positions.get_mut(index) {
                                *position = Some(end);
                            }
                        }
                    });
                if is_autoclosing_tag {
                    self.path.reduce();
                }
            }
            Elements::Text(content) => {
                #[cfg(test)]
                println!("\t\t CONTENT : {:?}", content);
            }
            Elements::End(_tag_name, begin, _end) => {
                self.path.reduce();
                for position in reading_positions.iter().enumerate() {
                    if let (index, Some(start_text)) = position {
                        let content = html.get(*start_text..begin);
                        if let Some(content) = content {
                            if let Some(value) = founds.get_mut(index) {
                                // fill the content only if it was not filled before
                                if value.is_empty() {
                                    value.push_str(content);
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        });

        founds
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

    fn check_any_matching(
        &self,
        matchers: &Vec<&Vec<Box<dyn Fn(&TagPathItem) -> bool>>>,
    ) -> Vec<bool> {
        matchers
            .into_iter()
            .map(|matcher| self.check_matching(&matcher))
            .collect()
    }

    fn check_matching(&self, first_matcher: &Vec<Box<dyn Fn(&TagPathItem) -> bool>>) -> bool {
        match_tag_path(self.path.get_matching_path(), first_matcher)
    }
}

#[macro_use]
#[cfg(test)]
mod test_tag_path_html_selector {

    use crate::css_selector;

    use super::*;

    use std::fs;

    fn get_amazon_email_html() -> String {
        let filename = "./amazon_command.html";
        fs::read_to_string(filename).unwrap()
    }
    fn get_amazon_header_email_html() -> String {
        let filename = "./amazon_command_header.html";
        fs::read_to_string(filename).unwrap()
    }
    fn get_simple_email_html() -> String {
        let filename = "./simple_table.html";
        fs::read_to_string(filename).unwrap()
    }

    #[test]
    fn should_count_row_and_cells() {
        // GIVEN
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
        // WHEN
        let mut html_selector = TagPathHtmlSelector::new();
        let counts = html_selector.count(&html, &paths_matcher);
        // THEN
        assert_eq!(vec![4, 12], counts);
    }

    #[test]
    fn should_get_label_in_deep_dom_tree() {
        // GIVEN
        let html = get_simple_email_html();
        let total_label_matcher = vec![
            css_selector!(table),
            css_selector!(tbody),
            css_selector!(tr:nth-child(4)),
            css_selector!(td:nth-child(1)),
        ];
        let total_amount_matcher = vec![
            css_selector!(table),
            css_selector!(tbody),
            css_selector!(tr:nth-child(4)),
            css_selector!(td:nth-child(3)),
        ];
        let paths_matcher = vec![&total_label_matcher, &total_amount_matcher];
        // WHEN
        let mut html_selector = TagPathHtmlSelector::new();
        let founds = html_selector.find_first(&html, &paths_matcher);
        // THEN
        assert_eq!(vec!["TOTAL".to_string(), "125 €".to_string()], founds);
    }

    #[test]
    fn should_get_total() {
        // "Chrome Dev Tools > Inspect > Copy > Copy Selector"
        //      nth-child start at 1 index
        // #costBreakdown > tbody > tr:nth-child(9) > td:nth-child(2) > strong

        // GIVEN
        let html = get_amazon_email_html();

        let path_matcher1 = vec![
            css_selector!(#costBreakdown),
            css_selector!(tbody),
            css_selector!(tr:nth-child(9)),
            css_selector!(td:nth-child(2)),
            css_selector!(strong),
        ];
        let paths_matcher = vec![&path_matcher1];
        // WHEN
        let mut html_selector = TagPathHtmlSelector::new();
        let counts = html_selector.count(&html, &paths_matcher);
        // THEN
        assert_eq!(vec![1], counts);

        // WHEN
        let founds = html_selector.find_first(&html, &paths_matcher);
        // THEN
        assert_eq!(vec!["EUR 61,90".to_string()], founds);
    }

    #[test]
    fn should_get_command_number() {
        // GIVEN
        let html = get_amazon_header_email_html();

        // #header > tbody > tr:nth-child(2) > td > a
        let command_number_matcher = vec![
            css_selector!(#header),
            css_selector!(tbody),
            css_selector!(tr:nth-child(2)),
            css_selector!(td),
            css_selector!(a),
        ];
        let paths_matcher = vec![&command_number_matcher];
        // WHEN
        let mut html_selector = TagPathHtmlSelector::new();
        let counts = html_selector.count(&html, &paths_matcher);
        // THEN
        assert_eq!(vec![1], counts);

        // WHEN
        let founds = html_selector.find_first(&html, &paths_matcher);
        // THEN
        println!("Founds {:?}", founds);
        assert!(founds.len() == 1);
        assert_eq!(vec!["405-5855855-9921124".to_string()], founds);
    }
}
