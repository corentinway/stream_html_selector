use crate::elements::Element;

#[derive(PartialEq, Debug)]
pub struct TextElement {
    pub content: String,
    pub length: usize,
}

impl Element<TextElement> for TextElement {
    fn extract(html: &str) -> Option<TextElement> {
        if is_text_element(html) {
            let (content, length) = extract_text_element(html);
            Some(TextElement { content, length })
        } else {
            None
        }
    }
}

fn extract_text_element(html: &str) -> (String, usize) {
    let end = html.find("<").unwrap_or_else(|| html.len());
    let text = html.get(..end).unwrap();
    (text.to_string(), end)
}

fn is_text_element(html: &str) -> bool {
    if html.len() > 0 {
        let first_char = html.get(0..1);
        match first_char {
            Some(char_str) => char_str != "<",
            None => false,
        }
    } else {
        false
    }
}
