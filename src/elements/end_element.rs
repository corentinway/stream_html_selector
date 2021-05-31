use crate::elements::is_element_like;
use crate::elements::utils::extract_element_like;

/// Parse an starting HTML tag like `<div id'foo' class="bar" hidden aria-label='baz'>`
fn extract_end_tag_name(html: &str) -> (String, usize) {
    extract_element_like(html, "</", ">")
}

/// return true if the element starts with `</` and a letter
fn is_end_element(html: &str) -> bool {
    // min length 4, like `</a>`
    is_element_like(html, "</", 4)
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn shohuld_test_end_element() {
        assert_eq!(true, is_end_element("</p>"));
        assert_eq!(true, is_end_element("</div>"));

        assert_eq!(false, is_end_element("<div>"));
        assert_eq!(false, is_end_element("hello"));
        assert_eq!(false, is_end_element("<!-- foo -->"));
    }
}

use crate::elements::Element;

#[derive(PartialEq,Debug)]
pub struct EndElement {
    pub name: String,
    pub length: usize,
}

impl Element<EndElement> for EndElement {

    fn extract(html: &str) -> Option<EndElement> {
        if is_end_element(html) {
            let (name, length) = extract_end_tag_name(html);
            Some(EndElement { name, length })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_end_elements {
    use super::*;

    #[test] 
    fn should_return_none_in_case_of_invalid_end_element() {
        let html ="<div>";
        let end_element = EndElement::extract(html);
        assert_eq!(None, end_element);
    }
    #[test] 
    fn should_return_some_in_case_of_valid_end_element() {
        let html ="</div>";
        let end_element = EndElement::extract(html);
        assert_eq!(Some(EndElement {
            name: "div".to_string(),
            length: 6
        }), end_element);
    }
}
