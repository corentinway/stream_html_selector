/*

- `css_selector!( div )` will lead to generate 

```rust 
tag_name_predicate("div".to_string())


*/
#[macro_export]
macro_rules!  css_selector {
    ($tag_name: expr) => {
        crate::selector::tag_name_predicate(String::from(stringify!($tag_name)))
    };
}

#[cfg(test)]
mod test_css_selector_macro {
    use super::super::elements::start_element::Tag;

     #[test]
     fn should_test_macro() {
        let tag = Tag {
            name: "div".to_string(),
            attributes: std::collections::HashMap::new(),
            is_autoclosing: false,
            length: 0, // FAKE
        };

        let matcher = css_selector!(div);

        let does_match = matcher(&tag);

        assert!(does_match);

     }
}