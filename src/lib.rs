mod tag;
mod tag_parser;
mod selector;
mod html_selector;

pub fn parse(html : &str) -> tag::Tag {
    tag::extract_tag_name(html)   
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