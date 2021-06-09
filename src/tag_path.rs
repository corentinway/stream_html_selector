use crate::elements::start_element::Tag;

pub struct TagPathItem {
    pub tag: Box<Tag>,
    pub nth_child: usize,
}

fn match_tag_path<F>(tag_path: Vec<Tag>, css_selector: Vec<F>) -> bool
where
    F: Fn(&Tag) -> bool,
{
    if tag_path.is_empty() || tag_path.len() < css_selector.len() {
        return false;
    }

    let tag_index = tag_path.len() - 1;
    let selector_index = css_selector.len() - 1;

    match_tag_path_index(tag_path, tag_index, css_selector, selector_index)
}

fn match_tag_path_index<F>(
    tag_path: Vec<Tag>,
    tag_index: usize,
    css_selector: Vec<F>,
    selector_index: usize,
) -> bool
where
    F: Fn(&Tag) -> bool,
{
    // FIXME unwrap
    let tag = tag_path.get(tag_index).unwrap();
    let selector = css_selector.get(selector_index).unwrap();
    if selector(tag) {
        if tag_index == 0 && selector_index == 0 {
            true
        } else {
            match_tag_path_index(tag_path, tag_index - 1, css_selector, selector_index - 1)
        }
    } else {
        false
    }
}

#[cfg(test)]
mod test_tag_path {

    use super::*;
    use crate::selector_predicates::*;
    use std::collections::HashMap;

    fn build_tag_with_attribute(name: &str, attribute_key: &str, attribute_value: &str) -> Tag {
        let mut map = HashMap::new();
        map.insert(attribute_key.to_string(), attribute_value.to_string());
        Tag {
            name: name.to_string(),
            attributes: map,
            length: 0,
            is_autoclosing: false,
        }
    }

    fn build_tag(name: &str) -> Tag {
        Tag {
            name: name.to_string(),
            attributes: HashMap::new(),
            length: 0,
            is_autoclosing: false,
        }
    }

    #[test]
    fn should_match_a_tag_path() {
        let tag_path = vec![
            build_tag_with_attribute("div", "id", "foo"),
            build_tag("div"),
        ];

        let css_selector = vec![
            tag_name_predicate("div".to_string()),
            tag_name_predicate("div".to_string()),
        ];

        let does_match = match_tag_path(tag_path, css_selector);

        assert!(does_match)
    }
    #[test]
    fn should_not_match_given_tag_path_smaller_than_css_selector_vec() {
        let tag_path = vec![
            build_tag_with_attribute("div", "id", "foo"),
            build_tag("div"),
        ];

        let css_selector = vec![
            tag_name_predicate("div".to_string()),
            tag_name_predicate("div".to_string()),
            tag_name_predicate("div".to_string()),
            tag_name_predicate("div".to_string()),
            tag_name_predicate("div".to_string()),
        ];

        let does_match = match_tag_path(tag_path, css_selector);

        assert!(!does_match)
    }
    #[test]
    fn should_not_match_given_empty_tag_path() {
        let tag_path = vec![];

        let css_selector = vec![
            tag_name_predicate("div".to_string()),
            tag_name_predicate("div".to_string()),
        ];

        let does_match = match_tag_path(tag_path, css_selector);

        assert!(!does_match);
    }
}
