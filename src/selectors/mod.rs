pub mod css_selector_macro;
pub mod matcher_html_selector;
pub mod selector_predicates;
pub mod tag_name_html_selector;
pub mod tag_path_html_selector;

pub trait HtmlSelectorCounter<T> {
    fn count(&mut self, html: &str, css_requests: &[T]) -> Vec<usize>;
}
pub trait HtmlSelectorFindFirst<T> {
    fn find_first(&mut self, html: &str, css_requests: &[T]) -> Vec<String>;
}

pub fn format_css_request(css_requests: &[&str]) -> Vec<String> {
    css_requests
        .iter()
        .map(|css_request| {
            if !css_request.starts_with(' ') {
                format!(" {}", css_request)
            } else {
                css_request.to_string()
            }
        })
        .collect()
}

pub struct FindFirstTextStore {
    reading_positions: Vec<Option<usize>>,
}
impl FindFirstTextStore {
    // FIXME test it
    fn new(capacity: usize) -> Self {
        FindFirstTextStore {
            reading_positions: vec![None; capacity],
        }
    }
    pub fn store_starting_position(&mut self, matcher_index: usize, content_start_index: usize) {
        if let Some(position) = self.reading_positions.get_mut(matcher_index) {
            *position = Some(content_start_index);
        }
    }

    pub fn update_content(&self, founds: &mut Vec<String>, content_end_index: usize, html: &str) {
        for position in self.reading_positions.iter().enumerate() {
            if let (index, Some(start_text)) = position {
                let content = html.get(*start_text..content_end_index);
                if let Some(content) = content {
                    if let Some(value) = founds.get_mut(index) {
                        // fill the content only if it was not filled before
                        if value.is_empty() {
                            value.push_str(content.to_string().replace("\n", " ").as_str());
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test_selectors {
    use super::*;

    #[test]
    fn should_format_css_request() {
        let css_requests = vec!["body", "div p"];

        let actual_css_requests = format_css_request(&css_requests);

        assert_eq!(vec![" body", " div p"], actual_css_requests);
    }
    #[test]
    fn should_notformat_css_request_valid() {
        let css_requests = vec![" body", " div p"];

        let actual_css_requests = format_css_request(&css_requests);

        assert_eq!(vec![" body", " div p"], actual_css_requests);
    }
}
