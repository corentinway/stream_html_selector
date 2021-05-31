use super::tag_parser::TagParser;
use std::collections::HashMap;
use std::cmp;

/// hold elements of an HTML tag
#[derive(PartialEq, Debug)]
pub struct Tag {
    /// tag name
    pub name: String,
    /// attributes map : attributes name -> attributes value
    pub attributes: HashMap<String, String>,
    /// length of the whole tag from `<` to `>`
    pub length: usize,
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
pub fn extract_tag_name(html: &str) -> Option<Tag> {
    let start = html.find('<').unwrap();

    let end_autoclosing = html.find("/>");
    let mut is_autoclosing_tag = end_autoclosing.is_some(); // FIXME


    let end_closing = html.find('>');

    let end = match (end_autoclosing, end_closing) {
        (Some(end_autoclosing), Some(end_closing)) => {
            is_autoclosing_tag = end_autoclosing < end_closing;
            cmp::min(end_autoclosing, end_closing)
        },
        (Some(end_autoclosing), None) => end_autoclosing,
        (None, Some(end_closing)) => end_closing,
        _ => return None,
    };



    let tag_content = html.get(start + 1..end).unwrap();

    let tag_content: String = tag_content
        .replace("\n\r", " ")
        .replace("\n", " ")
        .replace("\r", " ");

    let start_attributes_index = tag_content.find(' ');


    let end_attributes_index = tag_content.len();

    let attributes = if let Some(start_attributes_index) = start_attributes_index {
        let attributes_code = tag_content
            .get(start_attributes_index..end_attributes_index)
            .unwrap_or_default();
        let mut tag_parser = TagParser::new();
        tag_parser.parse_attributes(attributes_code)
    } else {
        HashMap::new()
    };

    let offset = if is_autoclosing_tag { 2 } else { 1 };

    let name = tag_content
        .find(' ')
        .and_then(|position| tag_content.get(0..position));

    let name = if let Some(name) = name {
        name.to_string()

    } else {
        tag_content
    };

    Some(Tag {
        name,
        attributes,
        length: end - start + offset,
    })

}

/// Parse an starting HTML tag like `<div id'foo' class="bar" hidden aria-label='baz'>`
pub fn extract_end_tag_name(html: &str) -> (String, usize) {
    extract_element_like(html, "</", ">")
}


fn extract_element_like(html: &str, start_str: &str, end_str: &str) -> (String, usize) {
    let start = start_str.len();

    let end = html.find(end_str).unwrap();

    let tag_content = html.get(start..end).unwrap();

    let name: String = tag_content
        .replace("\n\r", "")
        .replace("\n", "")
        .replace("\r", "");

    let length = name.len() + start + end_str.len();
    (name, length)
}





#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_extract_tag_name_from_tiny_tag() {
        let html = "<div>";
        let tag = extract_tag_name(html).unwrap();
        assert_eq!("div", tag.name);
        assert!(tag.attributes.is_empty());
        assert_eq!(5, tag.length);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_starting_with_blank() {
        let tag = extract_tag_name("   <div>").unwrap();
        assert_eq!("div", tag.name);
        assert_eq!(5, tag.length);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_ending_with_blank() {
        let tag = extract_tag_name("<div>    ").unwrap();
        assert_eq!("div", tag.name);
        assert_eq!(5, tag.length);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_with_blank_after_tag_name() {
        let html = "<div   >";
        let tag = extract_tag_name(html).unwrap();
        assert_eq!("div", tag.name);
        assert_eq!(html.len(), tag.length);

        let html = "<div  id='foo' >";
        let tag = extract_tag_name(html).unwrap();
        assert_eq!("div", tag.name);
        assert_eq!("foo", tag.attributes.get("id").unwrap());
        assert_eq!(html.len(), tag.length);

        let html = "<div  id=\"foo\" >";
        let tag = extract_tag_name(html).unwrap();
        assert_eq!("div", tag.name);
        assert_eq!("foo", tag.attributes.get("id").unwrap());
        assert_eq!(html.len(), tag.length);
    }
    #[test]
    fn should_extract_zero_html_class() {
        let html = "<div >";
        let tag = extract_tag_name(html).unwrap();
        assert!(tag.attributes.get("class").is_none());
        assert_eq!(html.len(), tag.length);
    }
    #[test]
    fn should_extract_one_html_class() {
        let html = "<div  class=\"bar\" >";
        let tag = extract_tag_name(html).unwrap();
        assert!(tag.attributes.get("class").is_some());
        assert_eq!(Some(&String::from("bar")), tag.attributes.get("class"));
        assert_eq!(html.len(), tag.length);
    }
    #[test]
    fn should_extract_many_html_classes() {
        let html = "<div  class='bar   baz   foo mun' >";
        let tag = extract_tag_name(html).unwrap();
        assert_eq!("div", tag.name);
        assert!(tag.attributes.get("class").is_some());
        assert_eq!(
            Some(&String::from("bar   baz   foo mun")),
            tag.attributes.get("class")
        );
        assert_eq!(html.len(), tag.length);
    }

    #[test]
    fn should_extract_consider_id_and_class_as_attributes() {
        let html = "<div id='foo' class='bar'>";
        let tag = extract_tag_name(html).unwrap();
        assert!(!tag.attributes.is_empty());
        assert_eq!(2, tag.attributes.len());
        assert_eq!("foo", tag.attributes.get("id").unwrap());
        assert_eq!("bar", tag.attributes.get("class").unwrap());
        assert_eq!(html.len(), tag.length);
    }

    #[test]
    fn should_extract_ignore_id_and_class_as_attributes_and_read_one() {
        let html = "<input id='foo' class='bar' type='password'>";
        let tag = extract_tag_name(html).unwrap();
        assert!(!tag.attributes.is_empty());
        println!("attributes {:?}", tag.attributes);
        assert_eq!(3, tag.attributes.len());
        assert_eq!(html.len(), tag.length);
    }

    #[test]
    fn should_extract_tag_name_from_tag_into_multi_line() {
        let html = r#"<div
            id='foo'
            class='bar'
            about
        >"#;
        let tag = extract_tag_name(html).unwrap();
        assert_eq!("div", tag.name);
        assert_eq!(html.len(), tag.length);
    }
    #[test]
    fn should_extract_auto_closing_tag() {
        let tag = extract_tag_name("<br/>").unwrap();
        assert_eq!("br", tag.name);
        assert_eq!(5, tag.length);
    }
    #[test]
    fn should_extract_tag_name_that_does_not_close() {
        let tag = extract_tag_name("<br");
        assert_eq!(None, tag);
    }
}
