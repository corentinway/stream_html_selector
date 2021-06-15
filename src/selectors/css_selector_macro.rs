macro_rules! assert_is_dollar  {
    ( $ ) => ();
}

#[macro_export]
macro_rules! css_selector {
    ($tag_name: tt) => {
        crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!($tag_name)))
    };
    // ID Selectors
    (# $id:tt) => {
        crate::selectors::selector_predicates::id_predicate(String::from(stringify!($id)))
    };
    ($tag_name:tt # $id:tt) => {
        crate::selectors::selector_predicates::and_predicate(vec![
            crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!($tag_name))),
            crate::selectors::selector_predicates::id_predicate(String::from(stringify!($id))),
        ])
    };

    // CLASS Selectors
    (. $class:tt) => {
        crate::selectors::selector_predicates::class_predicate(String::from(stringify!($class)))
    };
    ($tag_name:tt . $class:tt) => {
        crate::selectors::selector_predicates::and_predicate(vec![
            crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!($tag_name))),
            crate::selectors::selector_predicates::class_predicate(String::from(stringify!($class))),
        ])
    };

    // ATTRIBUTE Selectors
    ($tag_name:tt [$attribute_name:tt]) => {
        crate::selectors::selector_predicates::and_predicate(vec![
            crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!($tag_name))),
            crate::selectors::selector_predicates::has_attribute_predicate(String::from(stringify!($attribute_name))),
        ])
    };
    ($tag_name:tt [$attribute_name:tt = $attribute_value:literal ]) => {
        crate::selectors::selector_predicates::and_predicate(vec![
            crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!($tag_name))),
            crate::selectors::selector_predicates::attribute_equals_predicate(
                String::from(stringify!($attribute_name)), 
                String::from($attribute_value)
            ),
        ])
    };
    ($tag_name:tt [$attribute_name:tt ^= $attribute_value:literal ]) => {
        crate::selectors::selector_predicates::and_predicate(vec![
            crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!($tag_name))),
            crate::selectors::selector_predicates::attribute_starts_with_predicate(
                String::from(stringify!($attribute_name)), 
                String::from($attribute_value)
            ),
        ])
    };
    ($tag_name:tt [$attribute_name:tt $dollar:tt = $attribute_value:literal ]) => {
        //assert_is_dollar!( $dollar );
        crate::selectors::selector_predicates::and_predicate(vec![
            crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!($tag_name))),
            crate::selectors::selector_predicates::attribute_ends_with_predicate(
                String::from(stringify!($attribute_name)), 
                String::from($attribute_value)
            ),
        ])
    };
}




#[cfg(test)]
mod test_css_selector_macro {
    use crate::elements::start_element::Tag;
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
    fn should_test_macro_given_only_id() {
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

        let matcher = css_selector!(#foo);

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
    fn should_test_macro_given_only_class() {
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

        let matcher = css_selector!(.foo);

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
    #[test]
    fn should_test_macro_given_tag_name_and_attribute_equals_vallue() {
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

        let matcher = css_selector!(div[class="foo"]);

        assert!(matcher(&matched_tag));
        assert!(!matcher(&unmatched_tag));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_attribute_starts_with_vallue() {
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

        let matcher = css_selector!(div[class^="foo"]);

        assert!(matcher(&matched_tag));
        assert!(!matcher(&unmatched_tag));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_attribute_ends_with_vallue() {
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

        let matcher = css_selector!(div[class$="foo"]);

        assert!(matcher(&matched_tag));
        assert!(!matcher(&unmatched_tag));
    }
    

}
