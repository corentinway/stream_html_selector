use crate::elements::utils::extract_element_like;

/// Parse an starting HTML tag like `<div id'foo' class="bar" hidden aria-label='baz'>`
fn extract_comment_tag_name(html: &str) -> (String, usize) {
    extract_element_like(html, "<!--", "-->")
}   

/// return true if the element starts with `</` and a letter
fn is_comment_element(html: &str) -> bool {
    // min length 4, like `<!-- -->`
    is_element_like(html, "<!--", 8)
}

fn is_element_like(html: &str, start: &str, expected_smallest_length: usize) -> bool {
    let has_smallest_length_possible = html.len() >= expected_smallest_length;
    let is_start = html.get(0..start.len()) == Some(start);

    has_smallest_length_possible && is_start
}



#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn shohuld_test_end_element() {
        assert_eq!(true, is_comment_element("<!-- -->"));
        assert_eq!(true, is_comment_element("<!-- hello -->"));

        assert_eq!(false, is_comment_element("<!--"));
        assert_eq!(false, is_comment_element("hello"));
        assert_eq!(false, is_comment_element("</a foo -->"));
    }
}

use crate::elements::Element;

#[derive(PartialEq,Debug)]
pub struct CommentElement {
    pub content: String,
    pub length: usize,
}

impl Element<CommentElement> for CommentElement {

    fn extract(html: &str) -> Option<CommentElement> {
        if is_comment_element(html) {
            let (content, length) = extract_comment_tag_name(html);
            Some(CommentElement { content, length })
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
        let end_element = CommentElement::extract(html);
        assert_eq!(None, end_element);
    }
    #[test] 
    fn should_return_some_in_case_of_valid_end_element() {
        let html ="<!--div-->";
        let end_element = CommentElement::extract(html);
        assert_eq!(Some(CommentElement {
            content: "div".to_string(),
            length: 10
        }), end_element);
    }
}
