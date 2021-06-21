use crate::elements::start_element::Tag;

pub struct TagPathItem {
    pub tag: Box<Tag>,
    pub nth_child: usize,
}

pub struct TagPath {
    path: Vec<Box<TagPathItem>>,
}


impl TagPath {
    pub fn new() -> Self {
        TagPath {
            path: Vec::new()
        }
    }
    pub fn add(&mut self, tag: Tag) {
        self.path.push(Box::new(TagPathItem {
            tag: Box::new(tag),
            nth_child: 0,
        }));
    }
    pub fn reduce(&mut self) {
        self.path.pop();
    }

    pub fn get_matching_path(&self) -> Vec<&TagPathItem> {
        self
            .path
            .iter()
            .map(|boxed_tag| boxed_tag.as_ref())
            .collect()
    }
}



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

    fn build_tag_with_attribute(name: &str, attribute_key: &str, attribute_value: &str) -> TagPathItem {
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
            nth_child:0,
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
            nth_child:0,
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
        let body_tag = &build_tag("body");
        let table_tag = &build_tag("table");
        let tbody_tag = &build_tag("tbody");
        let tag_path = vec![html_tag, body_tag, table_tag, tbody_tag];

        let css_selector = vec![css_selector!(table), css_selector!(tbody)];

        let does_match = match_tag_path(tag_path, &css_selector);

        assert!(does_match)
    }
}
