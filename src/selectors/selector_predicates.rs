use crate::elements::start_element::Tag;

pub fn tag_name_predicate(name: String) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| tag.name == name)
}

pub fn id_predicate(id: String) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| {
        if let Some(actual_id) = tag.id() {
            return *actual_id == id;
        }
        false
    })
}

pub fn class_predicate(class: String) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| {
        if let Some(actual_classes) = tag.classes() {
            return actual_classes.contains(class.as_str());
        }
        false
    })
}

pub fn has_attribute_predicate(attribute_name: String) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| tag.attributes.get(&attribute_name).is_some())
}

pub fn attribute_equals_predicate(
    attribute_name: String,
    attribute_value: String,
) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| {
        if let Some(actual_attribute_value) = tag.attributes.get(&attribute_name) {
            return actual_attribute_value.eq(&attribute_value);
        }
        false
    })
}

pub fn attribute_starts_with_predicate(
    attribute_name: String,
    attribute_value: String,
) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| {
        if let Some(actual_attribute_value) = tag.attributes.get(&attribute_name) {
            return actual_attribute_value.starts_with(&attribute_value);
        }
        false
    })
}

pub fn attribute_ends_with_predicate(
    attribute_name: String,
    attribute_value: String,
) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| {
        if let Some(actual_attribute_value) = tag.attributes.get(&attribute_name) {
            return actual_attribute_value.ends_with(&attribute_value);
        }
        false
    })
}

pub fn attribute_contains_with_predicate(
    attribute_name: String,
    attribute_value: String,
) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| {
        if let Some(actual_attribute_value) = tag.attributes.get(&attribute_name) {
            return actual_attribute_value.contains(&attribute_value);
        }
        false
    })
}
pub fn attribute_has_word_predicate(
    attribute_name: String,
    word: String,
) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| {
        if let Some(actual_attribute_value) = tag.attributes.get(&attribute_name) {
            return actual_attribute_value.starts_with(&format!("{} ", &word))
                || actual_attribute_value.contains(&format!(" {} ", &word))
                || actual_attribute_value.ends_with(&format!(" {}", &word));
        }
        false
    })
}

pub fn and_predicate(predicates: Vec<Box<dyn Fn(&Tag) -> bool>>) -> Box<dyn Fn(&Tag) -> bool> {
    Box::new(move |tag: &Tag| {
        predicates
            .iter()
            .fold(true, |acc, predicate| acc && predicate(&tag))
    })
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
    #[test]
    fn should_match_a_tag_with_2_predicates() {
        let mut map = HashMap::new();
        map.insert("id".to_string(), "foo".to_string());
        let tag = Tag {
            name: String::from("div"),
            attributes: map,
            length: 5, // FAKE
            is_autoclosing: false,
        };

        let tag_name_matcher = tag_name_predicate(String::from("div"));
        let id_matcher = id_predicate(String::from("foo"));

        let matcher = and_predicate(vec![tag_name_matcher, id_matcher]);

        let does_match = matcher(&tag);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute() {
        let mut map = HashMap::new();
        map.insert("hidden".to_string(), "true".to_string());
        let tag = Tag {
            name: String::from("div"),
            attributes: map,
            length: 5, // FAKE
            is_autoclosing: false,
        };

        let matcher = has_attribute_predicate(String::from("hidden"));

        let does_match = matcher(&tag);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_equals_its_value() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "bar".to_string());
        let tag = Tag {
            name: String::from("div"),
            attributes: map,
            length: 5, // FAKE
            is_autoclosing: false,
        };

        let matcher = attribute_equals_predicate(String::from("foo"), String::from("bar"));

        let does_match = matcher(&tag);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_starts_with_its_value() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "baaaaar".to_string());
        let tag = Tag {
            name: String::from("div"),
            attributes: map,
            length: 5, // FAKE
            is_autoclosing: false,
        };

        let matcher = attribute_starts_with_predicate(String::from("foo"), String::from("baaa"));

        let does_match = matcher(&tag);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_ends_with_its_value() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "baaaaar".to_string());
        let tag = Tag {
            name: String::from("div"),
            attributes: map,
            length: 5, // FAKE
            is_autoclosing: false,
        };

        let matcher = attribute_ends_with_predicate(String::from("foo"), String::from("aar"));

        let does_match = matcher(&tag);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_contains_with_its_value() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "baaaaar".to_string());
        let tag = Tag {
            name: String::from("div"),
            attributes: map,
            length: 5, // FAKE
            is_autoclosing: false,
        };

        let matcher = attribute_contains_with_predicate(String::from("foo"), String::from("aaa"));

        let does_match = matcher(&tag);

        assert!(does_match);
    }
    #[test]
    fn should_match_a_tag_with_attribute_has_a_word_with_its_value() {
        let mut map = HashMap::new();
        map.insert("data".to_string(), "foo bar baz".to_string());
        let tag = Tag {
            name: String::from("div"),
            attributes: map,
            length: 5, // FAKE
            is_autoclosing: false,
        };

        let matcher = attribute_has_word_predicate(String::from("data"), String::from("foo"));
        let does_match = matcher(&tag);
        assert!(does_match);
        let matcher = attribute_has_word_predicate(String::from("data"), String::from("bar"));
        let does_match = matcher(&tag);
        assert!(does_match);
        let matcher = attribute_has_word_predicate(String::from("data"), String::from("baz"));
        let does_match = matcher(&tag);
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
