use crate::tag::Tag;
use crate::tag::extract_tag_name;
struct TagIterator<'a> {
    html: &'a str,
}

impl<'a> TagIterator<'a> {
    fn new(html: &'a str) -> Self {
        TagIterator { html }
    }
}

impl Iterator for TagIterator<'_> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        if self.html.is_empty() {
            None
        } else {
            let tag = extract_tag_name(self.html);
            let reduced_html = self.html.get(tag.length..);
            if reduced_html.is_some() {
                self.html = reduced_html.unwrap();
            } else {
                self.html = "";
            }
            Some(tag)
        }
    }
    
        
    
}

#[cfg(test)]
mod tag_iterator_tests {

    use super::*;
    use std::collections::HashMap;

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
        
        assert_eq!( Some(Tag{
            name: String::from("div"),
            attributes: HashMap::new(),
            length: 5
        }), tag_iterator.next());

        assert_eq!(None, tag_iterator.next());
    }
}
