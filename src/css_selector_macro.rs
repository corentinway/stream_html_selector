/*

- `css_selector!( div )` will lead to generate

```rust
tag_name_predicate("div".to_string())


*/
#[macro_export]
macro_rules! css_selector {
    ($tag_name: tt) => {
        crate::selector::tag_name_predicate(String::from(stringify!($tag_name)))
    };
    ($tag_name:tt # $id:tt) => {
        crate::selector::and_predicate(vec![
            crate::selector::tag_name_predicate(String::from(stringify!($tag_name))),
            crate::selector::id_predicate(String::from(stringify!($id))),
        ])
    };
    ($tag_name:tt . $class:tt) => {
        crate::selector::and_predicate(vec![
            crate::selector::tag_name_predicate(String::from(stringify!($tag_name))),
            crate::selector::class_predicate(String::from(stringify!($class))),
        ])
    };
    ($tag_name:tt [$attribute_name:tt]) => {
        crate::selector::and_predicate(vec![
            crate::selector::tag_name_predicate(String::from(stringify!($tag_name))),
            crate::selector::has_attribute_predicate(String::from(stringify!($attribute_name))),
        ])
    };
}

#[cfg(test)]
mod test_css_selector_macro {
    use super::super::elements::start_element::Tag;
    use std::collections::HashMap;

    #[test]
    fn should_test_macro_given_tag_name() {
        let matched_tag = Tag {
            name: "div".to_string(),
            attributes: HashMap::new(),
            is_autoclosing: false,
            length: 0, // FAKE
        };
        let unmatched_tag = Tag {
            name: "h1".to_string(),
            attributes: HashMap::new(),
            is_autoclosing: false,
            length: 0, // FAKE
        };

        let matcher = css_selector!(div);

        assert!(matcher(&matched_tag));
        assert!(!matcher(&unmatched_tag));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_id() {
        let mut map = HashMap::new();
        map.insert(String::from("id"), String::from("foo"));
        let matched_tag = Tag {
            name: "div".to_string(),
            attributes: map,
            is_autoclosing: false,
            length: 0, // FAKE
        };
        let unmatched_tag = Tag {
            name: "h1".to_string(),
            attributes: HashMap::new(),
            is_autoclosing: false,
            length: 0, // FAKE
        };

        let matcher = css_selector!(div#foo);

        assert!(matcher(&matched_tag));
        assert!(!matcher(&unmatched_tag));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_class() {
        let mut map = HashMap::new();
        map.insert(String::from("class"), String::from("foo"));
        let matched_tag = Tag {
            name: "div".to_string(),
            attributes: map,
            is_autoclosing: false,
            length: 0, // FAKE
        };
        let unmatched_tag = Tag {
            name: "h1".to_string(),
            attributes: HashMap::new(),
            is_autoclosing: false,
            length: 0, // FAKE
        };

        let matcher = css_selector!(div.foo);

        assert!(matcher(&matched_tag));
        assert!(!matcher(&unmatched_tag));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_attrbute() {
        let mut map = HashMap::new();
        map.insert(String::from("class"), String::from("foo"));
        let matched_tag = Tag {
            name: "div".to_string(),
            attributes: map,
            is_autoclosing: false,
            length: 0, // FAKE
        };
        let unmatched_tag = Tag {
            name: "h1".to_string(),
            attributes: HashMap::new(),
            is_autoclosing: false,
            length: 0, // FAKE
        };

        let matcher = css_selector!(div[class]);

        assert!(matcher(&matched_tag));
        assert!(!matcher(&unmatched_tag));
    }
}