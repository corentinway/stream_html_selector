use crate::elements::{
    comment_element::CommentElement, end_element::EndElement, start_element::Tag,
    text_element::TextElement, Element,
};

#[derive(PartialEq, Debug)]
pub enum Elements {
    Start(Tag, usize, usize),
    End(String, usize, usize),
    Comment(String),
    Text(String),
}

pub struct TagIterator<'a> {
    html: &'a str,
    reading_position: usize,
}

impl<'a> TagIterator<'a> {
    pub fn new(html: &'a str) -> Self {
        TagIterator {
            html,
            reading_position: 0,
        }
    }
    fn reduce_html(&mut self, element_length: usize) -> usize {
        self.reading_position += element_length;
        let reduced_html = self.html.get(element_length..);
        if let Some(html) = reduced_html {
            self.html = html;
        } else {
            self.html = "";
        }
        self.reading_position
    }
}

impl Iterator for TagIterator<'_> {
    type Item = Elements;

    fn next(&mut self) -> Option<Self::Item> {
        if self.html.is_empty() {
            None
        } else if let Some(start_element) = Tag::extract(self.html) {
            let begin = self.reading_position;
            let end = self.reduce_html(start_element.length);
            Some(Elements::Start(start_element, begin, end))
        } else if let Some(end_element) = EndElement::extract(self.html) {
            let begin = self.reading_position;
            let end = self.reduce_html(end_element.length);
            Some(Elements::End(end_element.name, begin, end))
        } else if let Some(comment_element) = CommentElement::extract(self.html) {
            self.reduce_html(comment_element.length);
            Some(Elements::Comment(comment_element.content))
        } else if let Some(text_element) = TextElement::extract(self.html) {
            self.reduce_html(text_element.length);
            Some(Elements::Text(text_element.content))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tag_iterator_tests {

    use Elements::Start;

    use super::*;
    use std::collections::HashMap;

    fn get_simple_div(begin: usize, end: usize) -> Elements {
        Start(
            Tag {
                name: String::from("div"),
                attributes: HashMap::new(),
                length: 5,
                is_autoclosing: false,
            },
            begin,
            end,
        )
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

        assert_eq!(Some(get_simple_div(0, 5)), tag_iterator.next());
        assert_eq!(None, tag_iterator.next());
    }
    #[test]
    fn should_read_2_following_simple_tag() {
        let html = "<div><div>";
        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div(0, 5)), tag_iterator.next());
        assert_eq!(Some(get_simple_div(5, 10)), tag_iterator.next());
        assert_eq!(None, tag_iterator.next());
    }

    #[test]
    fn should_return_start_elements_and_end_elements() {
        let html = "<div></div>";
        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div(0, 5)), tag_iterator.next());
        assert_eq!(
            Some(Elements::End(String::from("div"), 5, 11)),
            tag_iterator.next()
        );
        assert_eq!(None, tag_iterator.next());
    }

    #[test]
    fn should_return_an_autoclosinng_tag() {
        let html = "<div><input type='password' name='password' hidden /></div>";
        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div(0, 5)), tag_iterator.next());

        let mut expected_attributes = HashMap::new();
        expected_attributes.insert("type".to_string(), "password".to_string());
        expected_attributes.insert("name".to_string(), "password".to_string());
        expected_attributes.insert("hidden".to_string(), "true".to_string());
        assert_eq!(
            Some(Elements::Start(
                Tag {
                    name: "input".to_string(),
                    attributes: expected_attributes,
                    length: 48,
                    is_autoclosing: true,
                },
                5,
                53
            )),
            tag_iterator.next()
        );
        assert_eq!(
            Some(Elements::End(String::from("div"), 53, 59)),
            tag_iterator.next()
        );
        assert_eq!(None, tag_iterator.next());
    }

    #[test]
    fn should_return_start_element_comment_element_end_elements() {
        let html = "<div><!-- hello world --></div>";
        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div(0, 5)), tag_iterator.next());

        assert_eq!(
            Some(Elements::Comment(String::from(" hello world "))),
            tag_iterator.next()
        );

        assert_eq!(
            Some(Elements::End(String::from("div"), 25, 31)),
            tag_iterator.next()
        );

        assert_eq!(None, tag_iterator.next());
    }
    #[test]
    fn should_return_start_element_multiline_comment_element_end_elements() {
        let html = r#"<div><!-- hello 
        world --></div>"#;

        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div(0, 5)), tag_iterator.next());

        let expected_comment_content = r#" hello 
        world "#
            .to_string();
        assert_eq!(
            Some(Elements::Comment(expected_comment_content)),
            tag_iterator.next()
        );

        assert_eq!(
            Some(Elements::End(String::from("div"), 34, 40)),
            tag_iterator.next()
        );

        assert_eq!(None, tag_iterator.next());
    }

    #[test]
    fn should_return_start_element_text_element_end_elements() {
        let html = "<div>hello world</div>";
        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div(0, 5)), tag_iterator.next());

        assert_eq!(
            Some(Elements::Text(String::from("hello world"))),
            tag_iterator.next()
        );

        assert_eq!(
            Some(Elements::End(String::from("div"), 16, 22)),
            tag_iterator.next()
        );

        assert_eq!(None, tag_iterator.next());
    }
    #[test]
    fn should_return_start_element_multiline_text_element_end_elements() {
        let html = r#"<div>hello 
        world</div>"#;
        let mut tag_iterator = TagIterator::new(html);

        assert_eq!(Some(get_simple_div(0, 5)), tag_iterator.next());

        let exptected_text = r#"hello 
        world"#
            .to_string();

        assert_eq!(Some(Elements::Text(exptected_text)), tag_iterator.next());

        assert_eq!(
            Some(Elements::End(String::from("div"), 25, 31)),
            tag_iterator.next()
        );

        assert_eq!(None, tag_iterator.next());
    }
}
