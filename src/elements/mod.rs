pub mod comment_element;
pub mod doctype_element;
pub mod end_element;
pub mod start_element;
pub mod text_element;
pub mod tag_parser;
mod utils;

pub trait Element<T> {
    fn extract(html: &str) -> Option<T>;
}

pub fn is_element_like(html: &str, start: &str, expected_smallest_length: usize) -> bool {
    let has_smallest_length_possible = html.len() >= expected_smallest_length;
    let is_start = html.get(0..start.len()) == Some(start);

    let first_letter_tag_name = html.get(start.len()..start.len() + 1);
    if let Some(first_letter_tag_name) = first_letter_tag_name {
        let is_alphabetic = first_letter_tag_name
            .chars()
            .next()
            .unwrap()
            .is_alphabetic();
        has_smallest_length_possible && is_start && is_alphabetic
    } else {
        false
    }
}

#[cfg(test)]
mod test_elements {

    use super::*;

    #[test]
    fn should_detect_element_like_starting_tag() {
        let html = "<div id='ee'>";
        let start = "<";
        let smallest_length = 3; // like <a>

        let is_valid = is_element_like(html, start, smallest_length);

        assert!(is_valid);
    }
    #[test]
    fn should_detect_invalid_element_like_starting_tag() {
        let html = "<1div id='ee'>";
        let start = "<";
        let smallest_length = 3; // like <a>

        let is_valid = is_element_like(html, start, smallest_length);

        assert!(!is_valid);
    }
    #[test]
    fn should_detect_invalid_element_like_starting_tag_too_small() {
        let html = "<a>";
        let start = "<";
        let smallest_length = 4; // like <a>

        let is_valid = is_element_like(html, start, smallest_length);

        assert!(!is_valid);
    }
    #[test]
    fn should_detect_invalid_element_like_starting_tag_bad_starting() {
        let html = "|a>";
        let start = "<";
        let smallest_length = 4; // like <a>

        let is_valid = is_element_like(html, start, smallest_length);

        assert!(!is_valid);
    }
}
