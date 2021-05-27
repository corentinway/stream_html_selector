use super::tag_parser::TagParser;
use std::collections::HashMap;

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
pub fn extract_tag_name(html: &str) -> Tag {
    let start = html.find('<').unwrap();
    
    let end_opt = html.find("/>");
    let is_autoclosing_tag = end_opt.is_some();

    let end = if end_opt.is_some() {
        end_opt.unwrap()
    } else {
        html.find(">").unwrap_or_else(|| html.len())
    };

    //let mut tag_name = html.get(start + 1..).unwrap();

    let tag_content = html.get(start + 1..end).unwrap();

    let tag_content: String = tag_content
        .replace("\n\r", " ")
        .replace("\n", " ")
        .replace("\r", " ");

    let start_attributes_index = tag_content.find(' ');

    let end_attributes_index = tag_content.len();

    let attributes = if start_attributes_index.is_some() {
        let start_attributes_index = start_attributes_index.unwrap();
        let attributes_code = tag_content
        .get(start_attributes_index..end_attributes_index)
        .unwrap_or_default();
        let mut tag_parser = TagParser::new();
        tag_parser.parse_attributes(attributes_code)
    } else {
        HashMap::new()
    };
    

    let offset = if is_autoclosing_tag {
        2
    } else {
        1
    };

    let name = tag_content
        .find(" ").and_then(|position| tag_content.get(0..position));

    let name = if name.is_some() {
        name.unwrap().to_string()
    } else {
        tag_content
    };

    Tag {
        name: name,
        attributes,
        length: end - start + offset,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_extract_tag_name_from_tiny_tag() {
        let html = "<div>";
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);
        assert!(tag.attributes.is_empty());
        assert_eq!(5, tag.length);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_starting_with_blank() {
        let tag = extract_tag_name("   <div>");
        assert_eq!("div", tag.name);
        assert_eq!(5, tag.length);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_ending_with_blank() {
        let tag = extract_tag_name("<div>    ");
        assert_eq!("div", tag.name);
        assert_eq!(5, tag.length);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_with_blank_after_tag_name() {
        let html = "<div   >";
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);
        assert_eq!(html.len(), tag.length);

        let html = "<div  id='foo' >";
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);
        assert_eq!("foo", tag.attributes.get("id").unwrap());
        assert_eq!(html.len(), tag.length);

        let html = "<div  id=\"foo\" >";
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);
        assert_eq!("foo", tag.attributes.get("id").unwrap());
        assert_eq!(html.len(), tag.length);
    }
    #[test]
    fn should_extract_zero_html_class() {
        let html = "<div >";
        let tag = extract_tag_name(html);
        assert!(tag.attributes.get("class").is_none());
        assert_eq!(html.len(), tag.length);
    }
    #[test]
    fn should_extract_one_html_class() {
        let html = "<div  class=\"bar\" >";
        let tag = extract_tag_name(html);
        assert!(tag.attributes.get("class").is_some());
        assert_eq!(Some(&String::from("bar")), tag.attributes.get("class"));
        assert_eq!(html.len(), tag.length);
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
        assert_eq!(html.len(), tag.length);
    }

    #[test]
    fn should_extract_consider_id_and_class_as_attributes() {
        let html = "<div id='foo' class='bar'>";
        let tag = extract_tag_name(html);
        assert!(!tag.attributes.is_empty());
        assert_eq!(2, tag.attributes.len());
        assert_eq!("foo", tag.attributes.get("id").unwrap());
        assert_eq!("bar", tag.attributes.get("class").unwrap());
        assert_eq!(html.len(), tag.length);
    }

    #[test]
    fn should_extract_ignore_id_and_class_as_attributes_and_read_one() {
        let html = "<input id='foo' class='bar' type='password'>";
        let tag = extract_tag_name(html);
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
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);
        assert_eq!(html.len(), tag.length);
    }
    #[test]
    fn should_extract_auto_closing_tag() {
        let tag = extract_tag_name("<br/>");
        assert_eq!("br", tag.name);
        assert_eq!(5, tag.length);
    }
    #[test]
    fn should_extract_tag_name_that_does_not_close() {
        let tag = extract_tag_name("<br");
        assert_eq!("br", tag.name);
        assert_eq!(4, tag.length);
    }
}
