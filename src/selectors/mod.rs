pub mod css_selector_macro;
pub mod matcher_html_selector;
pub mod selector_predicates;
pub mod tag_name_html_selector;
pub mod tag_path_html_selector;

pub trait HtmlSelectorCounter<T> {
    fn count(&mut self, html: &str, css_requests: &[T]) -> Vec<usize>;
}
pub trait HtmlSelectorFindFirst<T> {
    fn find_first(&mut self, html: &str, css_requests: &[T]) -> String;
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
