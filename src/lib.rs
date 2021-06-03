mod html_selector;
mod selector;
mod tag_iterator;
mod tag_parser;

mod elements;

use elements::start_element::extract_tag_name;
use elements::start_element::Tag;

/// tmp implementation in order to reduce dead code warning messages
pub fn parse(html: &str) -> Tag {
    extract_tag_name(html).unwrap()
}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn should_parse_a_simple_html_tag() {
        let html = "<div id='foo' class='chef bob' aria-label='Hello World'>";

        let tag = parse(html);

        assert_eq!("div", tag.name.as_str());
        assert_eq!("foo", tag.id().unwrap());
        assert_eq!("chef bob", tag.classes().unwrap());
        assert_eq!("Hello World", tag.attributes.get("aria-label").unwrap());
    }
}
