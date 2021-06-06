use crate::elements::utils::extract_element_like;

/// Parse an starting HTML tag like `<div id'foo' class="bar" hidden aria-label='baz'>`
fn extract_doctype_content(html: &str) -> (String, usize) {
    extract_element_like(html, "<!doctype", ">")
}

/// return true if the element starts with `</` and a letter
fn is_doctype_element(html: &str) -> bool {
    // min length 4, like `<!doctype html>`
    is_element_like(html, "<!doctype", 15)
}

fn is_element_like(html: &str, start: &str, expected_smallest_length: usize) -> bool {
    let has_smallest_length_possible = html.len() >= expected_smallest_length;

    let actual_start = html.get(0..start.len());

    let is_start = actual_start.is_some() && actual_start.unwrap().eq_ignore_ascii_case(start);

    has_smallest_length_possible && is_start
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn shohuld_test_end_element() {
        assert_eq!(true, is_doctype_element("<!doctype html>"));
        assert_eq!(true, is_doctype_element("<!DOCTYPE html>"));
        assert_eq!(true, is_doctype_element("<!doctype html>"));
        assert_eq!(true, is_doctype_element("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">"));

        assert_eq!(false, is_doctype_element("<!--"));
        assert_eq!(false, is_doctype_element("hello"));
        assert_eq!(false, is_doctype_element("</a foo -->"));
    }
}

use crate::elements::Element;

#[derive(PartialEq, Debug)]
pub struct DoctypeElement {
    pub content: String,
    pub length: usize,
}

impl Element<DoctypeElement> for DoctypeElement {
    fn extract(html: &str) -> Option<DoctypeElement> {
        if is_doctype_element(html) {
            let (content, length) = extract_doctype_content(html);
            Some(DoctypeElement { content, length })
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
        let html = "<div>";
        let end_element = DoctypeElement::extract(html);
        assert_eq!(None, end_element);
    }
    #[test]
    fn should_return_some_in_case_of_valid_end_element() {
        let html = "<!doctype html>";
        let end_element = DoctypeElement::extract(html);
        assert_eq!(
            Some(DoctypeElement {
                content: " html".to_string(),
                length: 15
            }),
            end_element
        );
    }
}
