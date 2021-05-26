use super::tag_parser::TagParser;
use std::collections::HashMap;

/// hold elements of an HTML tag
pub struct Tag {
    /// tag name
    pub name: String,
    /// attributes map : attributes name -> attributes value
    pub attributes: HashMap<String, String>,
}

impl Tag {
    /// Get the tag id from the attributes
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }
    /// Get the tag class from the attributes. It is the class bulk value : a
    /// string where each classes are separated by spaces, like in HTML code.
    pub fn classes(&self) -> Option<&String> {
        self.attributes.get("class")
    }
}
/// Parse an starting HTML tag like `<div id'foo' class="bar" hidden aria-label='baz'>`
pub fn extract_tag_name(html: &str) -> Tag {
    let start = html.find('<').unwrap();
    let mut tag_name = html.get(start + 1..).unwrap();

    let mut end = tag_name
        .find("/>")
        .unwrap_or_else(|| tag_name.find('>').unwrap_or_else(|| tag_name.len()));

    tag_name = tag_name.get(0..end).unwrap();

    let mut tag_name: String = tag_name
        .replace("\n\r", " ")
        .replace("\n", " ")
        .replace("\r", " ");

    let start_attributes_index = tag_name.find(' ').unwrap_or(1);
    let end_attributes_index = tag_name.len();

    let attributes_code = tag_name
        .get(start_attributes_index..end_attributes_index)
        .unwrap_or_default();
    let mut tag_parser = TagParser::new();
    let attributes = tag_parser.parse_attributes(attributes_code);

    // remove spaces
    end = tag_name.find(' ').unwrap_or_else(|| tag_name.len());

    tag_name = tag_name.get(0..end).unwrap().to_string();

    Tag {
        name: tag_name,
        attributes,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_extract_tag_name_from_tiny_tag() {
        assert_eq!("div", extract_tag_name("<div>").name);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_starting_with_blank() {
        assert_eq!("div", extract_tag_name("   <div>").name);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_ending_with_blank() {
        assert_eq!("div", extract_tag_name("<div>    ").name);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_with_blank_after_tag_name() {
        let html = "<div   >";
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);

        let html = "<div  id='foo' >";
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);
        assert_eq!("foo", tag.attributes.get("id").unwrap());

        let html = "<div  id=\"foo\" >";
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);
        assert_eq!("foo", tag.attributes.get("id").unwrap());
    }
    #[test]
    fn should_extract_zero_html_class() {
        let html = "<div >";
        let tag = extract_tag_name(html);
        assert!(tag.attributes.get("class").is_none());
    }
    #[test]
    fn should_extract_one_html_class() {
        let html = "<div  class=\"bar\" >";
        let tag = extract_tag_name(html);
        assert!(tag.attributes.get("class").is_some());
        assert_eq!(Some(&String::from("bar")), tag.attributes.get("class"));
    }
    #[test]
    fn should_extract_many_html_classes() {
        let html = "<div  class='bar   baz   foo mun' >";
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);
        assert!(tag.attributes.get("class").is_some());
        assert_eq!(
            Some(&String::from("bar   baz   foo mun")),
            tag.attributes.get("class")
        );
    }

    #[test]
    fn should_extract_consider_id_and_class_as_attributes() {
        let html = "<div id='foo' class='bar'>";
        let tag = extract_tag_name(html);
        assert!(!tag.attributes.is_empty());
        assert_eq!(2, tag.attributes.len());
        assert_eq!("foo", tag.attributes.get("id").unwrap());
        assert_eq!("bar", tag.attributes.get("class").unwrap());
    }

    #[test]
    fn should_extract_ignore_id_and_class_as_attributes_and_read_one() {
        let html = "<input id='foo' class='bar' type='password'>";
        let tag = extract_tag_name(html);
        assert!(!tag.attributes.is_empty());
        println!("attributes {:?}", tag.attributes);
        assert_eq!(3, tag.attributes.len());
    }

    #[test]
    fn should_extract_tag_name_from_tag_into_multi_line() {
        let html = r#"<div
            id='foo'
            class='bar'
            about
        >"#;

        assert_eq!("div", extract_tag_name(html).name);
    }
    #[test]
    fn should_extract_auto_closing_tag() {
        assert_eq!("br", extract_tag_name("<br/>").name);
    }
    #[test]
    fn should_extract_tag_name_that_does_not_close() {
        assert_eq!("br", extract_tag_name("<br").name);
    }
}
