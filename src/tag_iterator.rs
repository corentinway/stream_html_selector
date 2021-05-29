use crate::tag::extract_tag_name;
use crate::tag::extract_end_tag_name;
use crate::tag::Tag;

#[derive(PartialEq, Debug)]
enum Elements {
    StartElement(Tag),
    EndElements(String),
}

struct TagIterator<'a> {
    html: &'a str,
}

impl<'a> TagIterator<'a> {
    fn new(html: &'a str) -> Self {
        TagIterator { html }
    }
}

impl Iterator for TagIterator<'_> {
    type Item = Elements;

    fn next(&mut self) -> Option<Self::Item> {
        if self.html.is_empty() {
            None
        } else if is_start_element(self.html) {
            let tag = extract_tag_name(self.html);
            let reduced_html = self.html.get(tag.length..);
            if reduced_html.is_some() {
                self.html = reduced_html.unwrap();
            } else {
                self.html = "";
            }
            Some(Elements::StartElement(tag))
        } else if is_end_element(self.html) {
            let (name, length) = extract_end_tag_name(self.html);
            let reduced_html = self.html.get(length..);
            if reduced_html.is_some() {
                self.html = reduced_html.unwrap();
            } else {
                self.html = "";
            }
            Some(Elements::EndElements(name))
        } else {
            None
        }
    }
}

/// return true if the element starts with `<` and a letter
fn is_start_element(html: &str) -> bool {
    // min length 3, like `<a>`
    let has_smallest_length_possible = html.len() >= 3;
    let is_start = html.get(0..1) == Some("<");

    let first_letter_tag_name = html.get(1..2);
    if first_letter_tag_name.is_some() {
        let is_alphabetic = first_letter_tag_name
            .unwrap()
            .chars()
            .next()
            .unwrap()
            .is_alphabetic();
        has_smallest_length_possible && is_start && is_alphabetic
    } else {
        false
    }
}
/// return true if the element starts with `</` and a letter
fn is_end_element(html: &str) -> bool {
    // min length 4, like `</a>`
    let has_smallest_length_possible = html.len() >= 4;
    let is_start = html.get(0..2) == Some("</");

    let first_letter_tag_name = html.get(2..3);
    if first_letter_tag_name.is_some() {
        let is_alphabetic = first_letter_tag_name
            .unwrap()
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
mod test_utils {
    use super::*;
    #[test]
    fn should_test_start_element() {
        assert_eq!(true, is_start_element("<a>"));
        assert_eq!(true, is_start_element("<div>"));
        assert_eq!(true, is_start_element("<br>"));
        assert_eq!(true, is_start_element("<br/>"));

        assert_eq!(false, is_start_element("hello"));
        assert_eq!(false, is_start_element("<123"));
        assert_eq!(false, is_start_element("</p>"));
    }
    #[test]
    fn shohuld_test_end_element() {
        assert_eq!(true, is_end_element("</p>"));
        assert_eq!(true, is_end_element("</div>"));

        assert_eq!(false, is_end_element("<div>"));
        assert_eq!(false, is_end_element("hello"));
        assert_eq!(false, is_end_element("<!-- foo -->"));
    }
}

#[cfg(test)]
mod tag_iterator_tests {

    use Elements::StartElement;

    use super::*;
    use std::collections::HashMap;

    fn get_simple_div() -> Elements {
        StartElement(Tag {
            name: String::from("div"),
            attributes: HashMap::new(),
            length: 5,
        })
    }

    #[test]
    fn should_create_tag_iterator() {
        let html = "";
        let mut tag_iterator = TagIterator::new(html);
        assert_eq!(None, tag_iterator.next());
    }
    #[test]
    fn should_read_the_next_simple_tag() {
        let html = "<div>";
        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div()), tag_iterator.next());
        assert_eq!(None, tag_iterator.next());
    }
    #[test]
    fn should_read_2_following_simple_tag() {
        let html = "<div><div>";
        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div()), tag_iterator.next());
        assert_eq!(Some(get_simple_div()), tag_iterator.next());
        assert_eq!(None, tag_iterator.next());
    }

    #[test]
    fn should_return_start_elements_and_end_elements() {
        let html = "<div></div>";
        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div()), tag_iterator.next());
        assert_eq!(
            Some(Elements::EndElements(String::from("div"))),
            tag_iterator.next()
        );
        assert_eq!(None, tag_iterator.next());
    }

    /*
    TODO
     test autoclosing tag
     add `is_autoclosing`to struct Tag
    */
}
