use crate::elements::start_element::Tag;

pub fn tag_name_predicate(name: String) -> impl Fn(&Tag) -> bool {
    move |tag: &Tag| tag.name == name
}

pub fn id_predicate(id: String) -> impl Fn(&Tag) -> bool {
    move |tag: &Tag| {
        if let Some(actual_id) = tag.id() {
            return *actual_id == id;
        }
        false
    }
}

pub fn class_predicate(class: String) -> impl Fn(&Tag) -> bool {
    move |tag: &Tag| {
        if let Some(actual_classes) = tag.classes() {
            return actual_classes.contains(class.as_str());
        }
        false
    }
}

pub fn and_predicate(predicates: Vec<impl Fn(&Tag) -> bool> ) -> impl Fn(&Tag) -> bool {
    move |tag: &Tag| {
        predicates
            .iter()
            .fold( true, |acc, predicate| acc && predicate(&tag))
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
  
}

