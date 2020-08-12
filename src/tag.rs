pub fn extract_tag_name(html: &str) -> String {
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

    // remove spaces
    end = tag_name.find(" ").unwrap_or(tag_name.len());

    tag_name.get(0..end).unwrap().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_extract_tag_name_from_tiny_tag() {
        assert_eq!("div", extract_tag_name("<div>"));
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_starting_with_blank() {
        assert_eq!("div", extract_tag_name("   <div>"));
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_ending_with_blank() {
        assert_eq!("div", extract_tag_name("<div>    "));
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_with_blank_after_tag_name() {
        assert_eq!("div", extract_tag_name("<div   >"));
        assert_eq!("div", extract_tag_name("<div  id='foo' >"));
    }
    #[test]
    fn should_extract_tag_name_from_tag_into_multi_line() {
        let html = r#"<div
            id='foo'
            class='bar'
            about
        >"#;

        assert_eq!("div", extract_tag_name(html));
    }
    #[test]
    fn should_extract_auto_closing_tag() {
        assert_eq!("br", extract_tag_name("<br/>"));
    }
    #[test]
    fn should_extract_tag_name_that_does_not_close() {
        assert_eq!("br", extract_tag_name("<br"));
    }
}
