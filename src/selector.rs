use crate::elements::start_element::Tag;

fn tag_name_predicate(name: String) -> impl Fn(&Tag) -> bool {
    move |tag: &Tag| tag.name == name
}

fn id_predicate(id: String) -> impl Fn(&Tag) -> bool {
    move |tag: &Tag| {
        if let Some(actual_id) = tag.id() {
            return *actual_id == id;
        }
        false
    }
}

fn class_predicate(class: String) -> impl Fn(&Tag) -> bool {
    move |tag: &Tag| {
        if let Some(actual_classes) = tag.classes() {
            return actual_classes.contains(class.as_str());
        }
        false
    }
}

#[cfg(test)]
mod test_selectors {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn should_match_with_tag_name() {
        let tag = Tag {
            name: String::from("div"),
            attributes: HashMap::new(),
            length: 5,
            is_autoclosing: false,
        };

        let predicate = tag_name_predicate(String::from("div"));

        let does_match = predicate(&tag);

        assert!(does_match);
    }
    #[test]
    fn should_match_with_id() {
        let mut map = HashMap::new();
        map.insert("id".to_string(), "foo".to_string());
        let tag = Tag {
            name: String::from("div"),
            attributes: map,
            length: 5, // FAKE
            is_autoclosing: false,
        };

        let predicate = id_predicate(String::from("foo"));

        let does_match = predicate(&tag);

        assert!(does_match);
    }
    #[test]
    fn should_match_with_class() {
        let mut map = HashMap::new();
        map.insert("class".to_string(), "foo bar baz".to_string());
        let tag = Tag {
            name: String::from("div"),
            attributes: map,
            length: 5, // FAKE
            is_autoclosing: false,
        };

        let predicate = class_predicate(String::from("bar"));

        let does_match = predicate(&tag);

        assert!(does_match);
    }

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
}
fn match_tag_path<F>(tag_path: Vec<Tag>, css_selector: Vec<F>) -> bool
where
    F: Fn(&Tag) -> bool,
{
    /*
        TODO
        - tag_path empty
    */

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
