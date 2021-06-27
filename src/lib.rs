pub mod selectors;
mod tag_iterator;
mod tag_path;

mod elements;


/// this will search into html based on tag name only.
// a query is a Vec<&str> where each element is a tag name to match.
// the query does not have its length equals to the actual depth of the element.
/// HTML code is not validated before. It has to be valid HTML
pub mod by_name {

    use super::selectors::tag_name_html_selector::TagNameHtmlSelector;
    use super::selectors::HtmlSelectorCounter;
    use super::selectors::HtmlSelectorFindFirst;

    /// Count the matching of each element in the request Vector.
    /// - `html` is an HTML code
    /// - `css_requests` contains a list of CSS selectors. **One CSS Selector** is a list of tag name within a `Vec<&str>` struct.
    pub fn count(html: &str, css_requests: &[&str]) -> Vec<usize> {
        let mut html_selector = TagNameHtmlSelector::new();
        html_selector.count(html, css_requests)
    }

    /// Returns the 1st string that match the 1st CSS requests.
    /// TODO : MUST return a Vec<String>
    /// - `html` is an HTML code
    /// - `css_requests` contains a list of CSS selectors. **One CSS Selector** is a list of tag name within a `Vec<&str>` struct.
    pub fn find_first(html: &str, css_requests: &[&str]) -> String {
        let mut html_selector = TagNameHtmlSelector::new();
        html_selector.find_first(html, css_requests)
    }
}

pub mod with_matcher {
    use super::selectors::matcher_html_selector::MatcherHtmlSelector;
    use super::selectors::HtmlSelectorCounter;
    use super::selectors::HtmlSelectorFindFirst;
    use super::tag_path::TagPathItem;

    /// Count the matching of each element in the request Vector.
    /// - `html` is an HTML code
    /// - `css_requests` contains a list of CSS selectors. **One CSS Selector** is a list of tag name within a `Vec<&str>` struct.
    pub fn count<F>(html: &str, css_requests: &[F]) -> Vec<usize>
    where
        F: Fn(&TagPathItem) -> bool,
    {
        let mut html_selector = MatcherHtmlSelector::new();
        html_selector.count(html, css_requests)
    }

    /// Returns the 1st string that match the 1st CSS requests.
    /// TODO : MUST return a Vec<String>
    /// - `html` is an HTML code
    /// - `css_requests` contains a list of CSS selectors. **One CSS Selector** is a list of tag name within a `Vec<&str>` struct.
    pub fn find_first<F>(html: &str, css_requests: &[F]) -> String
    where
        F: Fn(&TagPathItem) -> bool,
    {
        let mut html_selector = MatcherHtmlSelector::new();
        html_selector.find_first(html, css_requests)
    }
}

pub mod by_tag_path {
    pub use crate::tag_path::TagPathItem;
    use super::selectors::tag_path_html_selector::TagPathHtmlSelector;

    pub fn count(
        html: &str,
        matchers: &Vec<&Vec<Box<dyn Fn(&TagPathItem) -> bool>>>,
    ) -> Vec<usize> {
        let mut html_selector = TagPathHtmlSelector::new();
        html_selector.count(html, matchers)
    }
    pub fn find_first(
        html: &str,
        matchers: &Vec<&Vec<Box<dyn Fn(&TagPathItem) -> bool>>>,
    ) -> Vec<String> {
        let mut html_selector = TagPathHtmlSelector::new();
        html_selector.find_first(html, matchers)
    }
}
