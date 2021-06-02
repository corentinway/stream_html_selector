pub fn extract_element_like(html: &str, start_str: &str, end_str: &str) -> (String, usize) {
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
pub fn extract_multiline_element_like(html: &str, start_str: &str, end_str: &str) -> (String, usize) {
    let start = start_str.len();

    let end = html.find(end_str).unwrap();

    let tag_content = html.get(start..end).unwrap();

    let name: String = tag_content.to_string();

    let length = name.len() + start + end_str.len();
    (name, length)
}

#[cfg(test)]
mod test_utils {

    use super::*;

    #[test]
    fn should_extract_element_like() {
        let content = "</div>";
        let start_str = "</";
        let end_str = ">";
        let (name, length) = extract_element_like(content, start_str, end_str);
        assert_eq!("div".to_string(), name);
        assert_eq!(6, length);
    }

    #[test]
    fn should_extrat_multiline_element() {
        let html = r#"<!--Hello
        World-->"#;

        let (content, length) = extract_multiline_element_like(html, "<!--", "-->");

        let expected_content = r#"Hello
        World"#.to_string();
        assert_eq!(expected_content, content);
        assert_eq!(26, length);

    }
}
