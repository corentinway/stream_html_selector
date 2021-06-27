use std::{borrow::Borrow, fmt::Write};

use crate::elements::start_element::Tag;

pub struct TagPathItem {
    pub tag: Box<Tag>,
    pub nth_child: usize,
}

impl std::fmt::Debug for TagPathItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nth_child = /*if self.nth_child > 1 {*/
            format!(":nth-child({})", self.nth_child);
        /*} else {
            format!("")
        };*/
        let id = if self.tag.id().is_none() {
            format!("")
        } else {
            format!("#{}", self.tag.id().unwrap())
        };

        f.write_fmt(format_args!("{}{}{}", self.tag.name, id, nth_child))
    }
}

#[derive(Debug)]
pub struct TagPath {
    path: Vec<Box<TagPathItem>>,
    last_popped_tag: Option<(Box<TagPathItem>, usize)>, // FIXME : should we have a vector of last popped tag ?
}

impl TagPath {
    pub fn new() -> Self {
        TagPath {
            path: Vec::new(),
            last_popped_tag: None,
        }
    }
    pub fn add(&mut self, tag: Tag) {
        //println!("ADD - {:?} - new tag {:?} - last popped {:?}", self.path, tag, self.last_popped_tag);

        let next_nth_child = match self.last_popped_tag.borrow() {
            Some((last_tag_path_item, depth)) => {
                // if the depth of the last popped tag is lower than the actual path length,
                // then the last popped tag was not meaningfull
                if *depth < self.path.len() {
                    1
                } else {
                    let last_tag_name = &last_tag_path_item.as_ref().tag.name;
                    if *last_tag_name == tag.name {
                        // index +1
                        last_tag_path_item.as_ref().nth_child + 1
                    } else {
                        1
                    }
                }
            }
            None => 1,
        };

        self.path.push(Box::new(TagPathItem {
            tag: Box::new(tag),
            nth_child: next_nth_child,
        }));
    }
    pub fn reduce(&mut self) {
        let len = self.path.len();
        self.last_popped_tag = self.path.pop().map(|t| (t, len));
        //println!("REDUCE - {:?} - new tag {:?}", self.path, self.last_popped_tag);
    }

    pub fn get_matching_path(&self) -> Vec<&TagPathItem> {
        self.path
            .iter()
            .map(|boxed_tag| boxed_tag.as_ref())
            .collect()
    }
}

/// match a `tag_path` as read by the HTML stream reader with a CSS selector `css_selector`.
/// - tag_path is a vector where each element match an HTML tag. Each element indexed N has its parent at index N-1
/// - css_selector is a vector of predicate. Each element of the vector will match one tag at a given index of the tag_path.
/// This is a recursive algorithm where it tries to match the last element of the tag_path and go backwards to its parent.
pub fn match_tag_path<F>(tag_path: Vec<&TagPathItem>, css_selector: &Vec<F>) -> bool
where
    F: Fn(&TagPathItem) -> bool,
{
    if tag_path.is_empty() || tag_path.len() < css_selector.len() {
        return false;
    }

    let tag_index = tag_path.len() - 1;
    let selector_index = css_selector.len() - 1;

    match_tag_path_index(tag_path, tag_index, css_selector, selector_index)
}

fn match_tag_path_index<F>(
    tag_path: Vec<&TagPathItem>,
    tag_index: usize,
    css_selector: &Vec<F>,
    selector_index: usize,
) -> bool
where
    F: Fn(&TagPathItem) -> bool,
{
    // FIXME unwrap
    let tag = tag_path.get(tag_index).unwrap();
    let selector = css_selector.get(selector_index).unwrap();
    if selector(tag) {
        if tag_index == 0 && selector_index == 0 {
            true
        } else if selector_index == 0 {
            true
        } else {
            match_tag_path_index(tag_path, tag_index - 1, css_selector, selector_index - 1)
        }
    } else {
        false
    }
}

#[macro_use]
#[cfg(test)]
mod test_tag_path {

    use super::*;
    use crate::css_selector;
    use std::collections::HashMap;

    fn build_tag_with_attribute(
        name: &str,
        attribute_key: &str,
        attribute_value: &str,
    ) -> TagPathItem {
        let mut map = HashMap::new();
        map.insert(attribute_key.to_string(), attribute_value.to_string());
        let tag = Tag {
            name: name.to_string(),
            attributes: map,
            length: 0,
            is_autoclosing: false,
        };
        TagPathItem {
            tag: Box::new(tag),
            nth_child: 1,
        }
    }

    fn build_tag(name: &str) -> TagPathItem {
        let tag = Tag {
            name: name.to_string(),
            attributes: HashMap::new(),
            length: 0,
            is_autoclosing: false,
        };
        TagPathItem {
            tag: Box::new(tag),
            nth_child: 1,
        }
    }

    #[test]
    fn should_match_a_tag_path() {
        let tag1 = build_tag_with_attribute("div", "id", "foo");
        let tag2 = &build_tag("div");
        let tag_path = vec![&tag1, &tag2];

        let css_selector = vec![css_selector!(div), css_selector!(div)];

        let does_match = match_tag_path(tag_path, &css_selector);

        assert!(does_match)
    }
    #[test]
    fn should_not_match_given_tag_path_smaller_than_css_selector_vec() {
        let tag1 = build_tag_with_attribute("div", "id", "foo");
        let tag2 = &build_tag("div");
        let tag_path = vec![&tag1, &tag2];

        let css_selector = vec![
            css_selector!(div),
            css_selector!(div),
            css_selector!(div),
            css_selector!(div),
            css_selector!(div),
        ];

        let does_match = match_tag_path(tag_path, &css_selector);

        assert!(!does_match)
    }
    #[test]
    fn should_not_match_given_empty_tag_path() {
        let tag_path = vec![];

        let css_selector = vec![css_selector!(div), css_selector!(div)];

        let does_match = match_tag_path(tag_path, &css_selector);

        assert!(!does_match);
    }
    #[test]
    fn should_match_given_a_css_selector_sorter_than_the_tag_path() {
        let html_tag = &build_tag("html");
        let body_tag = &build_tag("body:nth-child(1)");
        let table_tag = &build_tag("table");
        let tbody_tag = &build_tag("tbody");
        let tag_path = vec![html_tag, body_tag, table_tag, tbody_tag];

        let css_selector = vec![css_selector!(table), css_selector!(tbody)];

        let does_match = match_tag_path(tag_path, &css_selector);

        assert!(does_match)
    }
}

#[cfg(test)]
mod test_tag_path_nth {

    use super::*;
    use crate::elements::{start_element::Tag, Element};

    fn create_tag(html: &str) -> Tag {
        Tag::extract(html).expect("invalid HTML tag for testing")
    }

    #[test]
    fn should_have_index_1_given_a_first_new_tag_is_added() {
        // GIVEN
        let body = create_tag("<body>");
        // WHEN
        let mut tag_path = TagPath::new();
        tag_path.add(body);
        // THEN
        assert_nth_child_at(&tag_path, 0, 1);
    }
    #[test]
    fn should_have_index_2_given_a_first_new_tag_is_added() {
        // GIVEN
        let body = create_tag("<body>");
        let div1 = create_tag("<div>");
        let div2 = create_tag("<div>");
        // WHEN
        let mut tag_path = TagPath::new();
        tag_path.add(body);
        // body:nth-child(1)
        // THEN
        assert_nth_child_at(&tag_path, 0, 1);
        // WHEN
        tag_path.add(div1);
        // body:nth-child(1) > div:nth-child(1)
        // THEN
        assert_nth_child_at(&tag_path, 1, 1);

        // WHEN
        tag_path.reduce();
        // THEN
        assert_nth_child_at(&tag_path, 0, 1);

        tag_path.add(div2);
        // body:nth-child(1) > div:nth-child(2)
        // THEN
        assert_nth_child_at(&tag_path, 1, 2);

        // WHEN
        tag_path.reduce();
        // THEN
        assert_nth_child_at(&tag_path, 0, 1);
    }

    fn assert_nth_child_at(tag_path: &TagPath, index: usize, expected_nth_child: usize) {
        let tag_path_item = tag_path
            .path
            .get(index)
            .expect("invalid position of the tag for test")
            .as_ref();
        assert_eq!(expected_nth_child, tag_path_item.nth_child);
    }
}

#[cfg(test)]
mod test_tag_path_nth_child {
    use super::*;
    use crate::tag_iterator::{Elements, TagIterator};

    #[test]
    fn should_match_all_tag_nth_child() {
        let html = std::fs::read_to_string("tables.html").unwrap();

        let mut tag_path = TagPath::new();

        let expected_paths = vec![
            "body:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(1) td:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(1) td:nth-child(2)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(2)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(2) td:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(2) td:nth-child(2)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(3)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(3) td:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(3) td:nth-child(2)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4) td:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4) td:nth-child(1) table:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4) td:nth-child(1) table:nth-child(1) tr:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4) td:nth-child(1) table:nth-child(1) tr:nth-child(1) td:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4) td:nth-child(1) table:nth-child(1) tr:nth-child(1) td:nth-child(2)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4) td:nth-child(1) table:nth-child(1) tr:nth-child(2)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4) td:nth-child(1) table:nth-child(1) tr:nth-child(2) td:nth-child(1)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4) td:nth-child(1) table:nth-child(1) tr:nth-child(2) td:nth-child(2)".to_string(),
            "body:nth-child(1) table:nth-child(1) tr:nth-child(4) td:nth-child(2)".to_string(),
        ];

        let mut start_element_index = 0;

        let tag_iterator = TagIterator::new(&html);
        tag_iterator.enumerate().for_each(|(_index, element)| {
            match element {
                Elements::Start(tag, _begin, _end) => {
                    tag_path.add(tag);
                    assert_tag_path(
                        &tag_path,
                        expected_paths
                            .get(start_element_index)
                            .expect("expected value required for test"),
                    );
                    start_element_index = start_element_index + 1;
                }
                Elements::End(_, _, _) => {
                    tag_path.reduce();
                }
                //Elements::Comment(tag) => {},
                //Elements::Text(tag) => {},
                _ => {}
            }
        });
    }

    fn assert_tag_path(tag_path: &TagPath, expected_path: &String) {
        let names: Vec<String> = tag_path
            .path
            .iter()
            .map(|boxed_tag_path_item| boxed_tag_path_item.as_ref())
            .map(|tpi| {
                let name = tpi.tag.as_ref().name.as_str();
                let nth_child = tpi.nth_child;
                format!("{}:nth-child({})", name, nth_child)
            })
            .collect();

        let actual_path = names.join(" ");
        println!("Actual Path:   {:?}", actual_path);
        println!("Expected Path: {:?}", expected_path);

        assert_eq!(*expected_path, actual_path);
    }
}
