use crate::elements::start_element::Tag;
use crate::tag_path::TagPathItem;

pub fn tag_name_predicate(name: String) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| tag_path_item.tag.name == name)
}

pub fn id_predicate(id: String) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| {
        if let Some(actual_id) = tag_path_item.tag.id() {
            return *actual_id == id;
        }
        false
    })
}

pub fn class_predicate(class: String) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| {
        if let Some(actual_classes) = tag_path_item.tag.classes() {
            return actual_classes.contains(class.as_str());
        }
        false
    })
}

pub fn has_attribute_predicate(attribute_name: String) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| tag_path_item.tag.attributes.get(&attribute_name).is_some())
}

pub fn attribute_equals_predicate(
    attribute_name: String,
    attribute_value: String,
) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| {
        if let Some(actual_attribute_value) = tag_path_item.tag.attributes.get(&attribute_name) {
            return actual_attribute_value.eq(&attribute_value);
        }
        false
    })
}

pub fn attribute_starts_with_predicate(
    attribute_name: String,
    attribute_value: String,
) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| {
        if let Some(actual_attribute_value) = tag_path_item.tag.attributes.get(&attribute_name) {
            return actual_attribute_value.starts_with(&attribute_value);
        }
        false
    })
}

pub fn attribute_ends_with_predicate(
    attribute_name: String,
    attribute_value: String,
) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| {
        if let Some(actual_attribute_value) = tag_path_item.tag.attributes.get(&attribute_name) {
            return actual_attribute_value.ends_with(&attribute_value);
        }
        false
    })
}

pub fn attribute_contains_with_predicate(
    attribute_name: String,
    attribute_value: String,
) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| {
        if let Some(actual_attribute_value) = tag_path_item.tag.attributes.get(&attribute_name) {
            return actual_attribute_value.contains(&attribute_value);
        }
        false
    })
}
pub fn attribute_has_word_predicate(
    attribute_name: String,
    word: String,
) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| {
        if let Some(actual_attribute_value) =tag_path_item.tag.attributes.get(&attribute_name) {
            return actual_attribute_value.starts_with(&format!("{} ", &word))
                || actual_attribute_value.contains(&format!(" {} ", &word))
                || actual_attribute_value.ends_with(&format!(" {}", &word));
        }
        false
    })
}

pub fn and_predicate(predicates: Vec<Box<dyn Fn(&TagPathItem) -> bool>>) -> Box<dyn Fn(&TagPathItem) -> bool> {
    Box::new(move |tag_path_item: &TagPathItem| {
        predicates
            .iter()
            .fold(true, |acc, predicate| acc && predicate(&tag_path_item))
    })
}

#[cfg(test)]
mod test_selectors {

    use super::*;
    use std::collections::HashMap;
    use crate::elements::{Element, start_element::Tag};


    fn create_tag(html: &str) -> TagPathItem {
        let tag = Tag::extract(html).expect("invalid code to create tag for test");
        TagPathItem {
            tag: Box::new(tag),
            nth_child: 0,
        }
    }


    #[test]
    fn should_match_with_tag_name() {
        let tag_path_item =create_tag("<div>");

        let predicate = tag_name_predicate(String::from("div"));

        let does_match = predicate(&tag_path_item);

        assert!(does_match);
    }
    #[test]
    fn should_match_with_id() {
        let tag_path_item =create_tag("<div id='foo'>");

        let predicate = id_predicate(String::from("foo"));

        let does_match = predicate(&tag_path_item);

        assert!(does_match);
    }
    #[test]
    fn should_match_with_class() {
        let tag_path_item =create_tag("<div class='foo bar baz'>");

        let predicate = class_predicate(String::from("bar"));

        let does_match = predicate(&tag_path_item);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_2_predicates() {
        let tag_path_item =create_tag("<div id='foo'>");

        let tag_name_matcher = tag_name_predicate(String::from("div"));
        let id_matcher = id_predicate(String::from("foo"));

        let matcher = and_predicate(vec![tag_name_matcher, id_matcher]);

        let does_match = matcher(&tag_path_item);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute() {
        let tag_path_item =create_tag("<div hidden>");

        let matcher = has_attribute_predicate(String::from("hidden"));

        let does_match = matcher(&tag_path_item);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_equals_its_value() {
        let tag_path_item =create_tag("<div foo='bar'>");

        let matcher = attribute_equals_predicate(String::from("foo"), String::from("bar"));

        let does_match = matcher(&tag_path_item);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_starts_with_its_value() {
        let tag_path_item =create_tag("<div foo='baaaaar'>");

        let matcher = attribute_starts_with_predicate(String::from("foo"), String::from("baaa"));

        let does_match = matcher(&tag_path_item);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_ends_with_its_value() {
        let tag_path_item =create_tag("<div foo='baaaaar'>");

        let matcher = attribute_ends_with_predicate(String::from("foo"), String::from("aar"));

        let does_match = matcher(&tag_path_item);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_contains_with_its_value() {

        let tag_path_item =create_tag("<div foo='baaaaar'>");

        let matcher = attribute_contains_with_predicate(String::from("foo"), String::from("aaa"));

        let does_match = matcher(&tag_path_item);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_has_a_word_with_its_value() {
        let tag_path_item =create_tag("<div data='foo bar baz'>");

        let matcher = attribute_has_word_predicate(String::from("data"), String::from("foo"));
        let does_match = matcher(&tag_path_item);
        assert!(does_match);
        let matcher = attribute_has_word_predicate(String::from("data"), String::from("bar"));
        let does_match = matcher(&tag_path_item);
        assert!(does_match);
        let matcher = attribute_has_word_predicate(String::from("data"), String::from("baz"));
        let does_match = matcher(&tag_path_item);
        assert!(does_match);
    }

    // X:not(selector)
    // X::pseudoElement
    // X:nth-child(n)
    // X:nth-last-child(n)
    // X:nth-of-type(n)
    // X:nth-last-of-type(n)
    // X:first-child
    // X:last-child
    // X:only-child
    // X:only-of-type
    // X:first-of-type
}
