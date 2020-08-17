use std::collections::HashMap;

pub struct Tag {
    pub name: String,
    id: Option<String>,
    classes: Option<Vec<String>>,
    attributes: Option<HashMap<String, String>>,
}



pub fn extract_tag_name(html: &str) -> Tag {
    // remote '<' and '>'
    let start = html.find("<").unwrap();
    let mut tag_name = html.get(start + 1..).unwrap();

    let mut end = tag_name
        .find("/>")
        .unwrap_or(tag_name.find(">").unwrap_or(tag_name.len()));

    tag_name = tag_name.get(0..end).unwrap();

    let mut tag_name: String = tag_name
        .replace("\n\r", " ")
        .replace("\n", " ")
        .replace("\r", "");

    let id_str = find_id(tag_name.as_str());
    let classes = find_classes(tag_name.as_str());

    // remove spaces
    end = tag_name.find(" ").unwrap_or(tag_name.len());

    tag_name = tag_name.get(0..end).unwrap().to_string();

    Tag {
        name : tag_name,
        id: id_str,
        classes: classes,
        attributes: None,
    }
}

fn find_id(tag : &str) -> Option<String> {

    let start = tag.find("id=\"").or_else( || tag.find("id='"));

    if let Some(start) = start {
        let id = tag.get(start + 4..).unwrap();
        let end = id.find("\"").or_else(|| id.find("'")).unwrap();
        return Some(id.get(0..end).unwrap().to_string())
    }

    None
}

fn find_classes(tag : &str) -> Option<Vec<String>> {

    let start = tag.find("class=\"").or_else( || tag.find("class='"));

    if let Some(start) = start {
        let id = tag.get(start + 7..).unwrap();
        let end = id.find("\"").or_else(|| id.find("'")).unwrap();
        let class_str = id.get(0..end).unwrap();
        let classes : Vec<String> = class_str.split(" ")
            .filter( |class| !class.is_empty())
            .map( |class| class.to_string())
            .collect();
        return Some(classes)
    }

    None
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
        assert_eq!("div", extract_tag_name("<div   >").name);
        assert_eq!("div", extract_tag_name("<div  id='foo' >").name);
        assert_eq!("foo", extract_tag_name("<div  id='foo' >").id.unwrap());
        assert_eq!("bar", extract_tag_name("<div  id=\"bar\" >").id.unwrap());
    }
    #[test]
    fn should_extract_zero_html_class() {
        let html = "<div >";
        let tag = extract_tag_name(html);
        assert!(tag.classes.is_none());
    }
    #[test]
    fn should_extract_one_html_class() {
        let html = "<div  class=\"bar\" >";
        let tag = extract_tag_name(html);
        assert!(tag.classes.is_some());
        assert_eq!(Some(vec!["bar".to_string()]), tag.classes);
    }
    #[test]
    fn should_extract_many_html_classes() {
        let html = "<div  class='bar   baz   foo mun' >";
        let tag = extract_tag_name(html);
        assert!(tag.classes.is_some());
        assert_eq!(4, tag.classes.as_ref().unwrap().len());
        assert_eq!(Some(
            vec![
                "bar".to_string(),
                "baz".to_string(),
                "foo".to_string(),
                "mun".to_string()
                ]), 
            tag.classes);
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
