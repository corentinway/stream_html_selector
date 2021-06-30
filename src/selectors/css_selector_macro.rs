//! We define a macro that return a predicate that match a CSS selector

#[macro_export]
macro_rules! assert_is_dollar {
    ( $ ) => {};
}

/// Macro that return a predicate that match a CSS selector
#[macro_export]
macro_rules! css_selector {
    ($tag_name: tt) => {
        $crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!(
            $tag_name
        )))
    };
    // ID Selectors
    (# $id:tt) => {
        $crate::selectors::selector_predicates::id_predicate(String::from(stringify!($id)))
    };
    ($tag_name:tt # $id:tt) => {
        $crate::selectors::selector_predicates::and_predicate(vec![
            $crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!(
                $tag_name
            ))),
            $crate::selectors::selector_predicates::id_predicate(String::from(stringify!($id))),
        ])
    };

    // CLASS Selectors
    (. $class:tt) => {
        $crate::selectors::selector_predicates::class_predicate(String::from(stringify!($class)))
    };
    ($tag_name:tt . $class:tt) => {
        $crate::selectors::selector_predicates::and_predicate(vec![
            $crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!(
                $tag_name
            ))),
            $crate::selectors::selector_predicates::class_predicate(String::from(stringify!(
                $class
            ))),
        ])
    };

    // ATTRIBUTE Selectors
    ($tag_name:tt [$attribute_name:tt]) => {
        $crate::selectors::selector_predicates::and_predicate(vec![
            $crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!(
                $tag_name
            ))),
            $crate::selectors::selector_predicates::has_attribute_predicate(String::from(
                stringify!($attribute_name),
            )),
        ])
    };
    ($tag_name:tt [$attribute_name:tt = $attribute_value:literal ]) => {
        $crate::selectors::selector_predicates::and_predicate(vec![
            $crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!(
                $tag_name
            ))),
            $crate::selectors::selector_predicates::attribute_equals_predicate(
                String::from(stringify!($attribute_name)),
                String::from($attribute_value),
            ),
        ])
    };
    ($tag_name:tt [$attribute_name:tt ^= $attribute_value:literal ]) => {
        $crate::selectors::selector_predicates::and_predicate(vec![
            $crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!(
                $tag_name
            ))),
            $crate::selectors::selector_predicates::attribute_starts_with_predicate(
                String::from(stringify!($attribute_name)),
                String::from($attribute_value),
            ),
        ])
    };
    ($tag_name:tt [$attribute_name:tt $dollar:tt = $attribute_value:literal ]) => {
        //assert_is_dollar!( $dollar );
        $crate::selectors::selector_predicates::and_predicate(vec![
            $crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!(
                $tag_name
            ))),
            $crate::selectors::selector_predicates::attribute_ends_with_predicate(
                String::from(stringify!($attribute_name)),
                String::from($attribute_value),
            ),
        ])
    };
    (:nth-child ( $n:literal ) ) => {
        $crate::selectors::selector_predicates::nth_child_predicate($n)
    };
    ($tag_name:tt :nth-child ( $n:literal ) ) => {
        $crate::selectors::selector_predicates::and_predicate(vec![
            $crate::selectors::selector_predicates::tag_name_predicate(String::from(stringify!(
                $tag_name
            ))),
            $crate::selectors::selector_predicates::nth_child_predicate($n),
        ])
    };
}

#[cfg(test)]
mod test_css_selector_macro {
    use crate::elements::{start_element::Tag, Element};
    use crate::tag_path::TagPathItem;

    fn create_tag(html: &str) -> TagPathItem {
        let tag = Tag::extract(html).expect("invalid HTML code to create tag in the tests");
        TagPathItem {
            tag: Box::new(tag),
            nth_child: 1,
        }
    }

    #[test]
    fn should_test_macro_given_tag_name() {
        let matched_tag_path_item = create_tag("<div>");
        let unmatched_tag_path_item = create_tag("<h1>");

        let matcher = css_selector!(div);

        assert!(matcher(&matched_tag_path_item));
        assert!(!matcher(&unmatched_tag_path_item));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_id() {
        let matched_tag_path_item = create_tag("<div id='foo'>");
        let unmatched_tag_path_item = create_tag("<h1>");

        let matcher = css_selector!(div#foo);

        assert!(matcher(&matched_tag_path_item));
        assert!(!matcher(&unmatched_tag_path_item));
    }
    #[test]
    fn should_test_macro_given_only_id() {
        let matched_tag_path_item = create_tag("<div id='foo'>");
        let unmatched_tag_path_item = create_tag("<h1>");

        let matcher = css_selector!(#foo);

        assert!(matcher(&matched_tag_path_item));
        assert!(!matcher(&unmatched_tag_path_item));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_class() {
        let matched_tag_path_item = create_tag("<div class='foo'>");
        let unmatched_tag_path_item = create_tag("<h1>");

        let matcher = css_selector!(div.foo);

        assert!(matcher(&matched_tag_path_item));
        assert!(!matcher(&unmatched_tag_path_item));
    }
    #[test]
    fn should_test_macro_given_only_class() {
        let matched_tag_path_item = create_tag("<div class='foo'>");
        let unmatched_tag_path_item = create_tag("<h1>");

        let matcher = css_selector!(.foo);

        assert!(matcher(&matched_tag_path_item));
        assert!(!matcher(&unmatched_tag_path_item));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_attrbute() {
        let matched_tag_path_item = create_tag("<div class='foo'>");
        let unmatched_tag_path_item = create_tag("<h1>");

        let matcher = css_selector!(div[class]);

        assert!(matcher(&matched_tag_path_item));
        assert!(!matcher(&unmatched_tag_path_item));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_attribute_equals_vallue() {
        let matched_tag_path_item = create_tag("<div class='foo'>");
        let unmatched_tag_path_item = create_tag("<h1>");

        let matcher = css_selector!(div[class = "foo"]);

        assert!(matcher(&matched_tag_path_item));
        assert!(!matcher(&unmatched_tag_path_item));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_attribute_starts_with_vallue() {
        let matched_tag_path_item = create_tag("<div class='foo'>");
        let unmatched_tag_path_item = create_tag("<h1>");

        let matcher = css_selector!(div[class ^= "foo"]);

        assert!(matcher(&matched_tag_path_item));
        assert!(!matcher(&unmatched_tag_path_item));
    }
    #[test]
    fn should_test_macro_given_tag_name_and_attribute_ends_with_vallue() {
        let matched_tag_path_item = create_tag("<div class='foo'>");
        let unmatched_tag_path_item = create_tag("<h1>");

        let matcher = css_selector!(div[class$="foo"]);

        assert!(matcher(&matched_tag_path_item));
        assert!(!matcher(&unmatched_tag_path_item));
    }
    #[test]
    fn should_match_nth_child_only() {
        let mut tag_path_item = create_tag("<div>");
        tag_path_item.nth_child = 2;

        let matcher = css_selector!(:nth-child(2));

        assert!(matcher(&tag_path_item));
    }
    #[test]
    fn should_match_nth_child_and_tag_name() {
        let mut tag_path_item = create_tag("<div>");
        tag_path_item.nth_child = 2;

        let matcher = css_selector!(div:nth-child(2));

        assert!(matcher(&tag_path_item));
    }
}
