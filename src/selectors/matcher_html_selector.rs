use crate::selectors::HtmlSelectorCounter;
use crate::selectors::HtmlSelectorFindFirst;
use crate::tag_iterator::Elements;
use crate::tag_iterator::TagIterator;
use crate::tag_path::TagPathItem;

pub struct MatcherHtmlSelector {}
impl MatcherHtmlSelector {
    fn new() -> Self {
        MatcherHtmlSelector {}
    }
}
impl Default for MatcherHtmlSelector {
    fn default() -> Self {
        MatcherHtmlSelector::new()
    }
}

/// HTML matcher that only use 1 predicate for the last tag path item.
/// It returns the count of element that match
/// **nth-child predicate is not supported**
impl<F> HtmlSelectorCounter<F> for MatcherHtmlSelector
where
    F: Fn(&TagPathItem) -> bool,
{
    fn count(&mut self, html: &str, matchers: &[F]) -> Vec<usize> {
        let mut count = 0;
        let matcher = &matchers[0];

        let tag_iterator = TagIterator::new(html);
        tag_iterator.for_each(|element| 
            if let Elements::Start(tag, _begin, _end) = element {
                let tag_path_item = TagPathItem {
                    tag: Box::new(tag),
                    nth_child: 0, //FIXME
                };
                if matcher(&tag_path_item) {
                    count += 1;
                }
            }
        );

        vec![count]
    }
}

/// HTML matcher that only use 1 predicate for the last tag path item.
/// It returns the 1st text of element that match
/// **nth-child predicate is not supported**
impl<F> HtmlSelectorFindFirst<F> for MatcherHtmlSelector
where
    F: Fn(&TagPathItem) -> bool,
{
    fn find_first(&mut self, html: &str, matchers: &[F]) -> Vec<String> {
        
        let mut founds = vec![String::new(); matchers.len()];
        let mut text_store = super::FindFirstTextStore::new(matchers.len());

        let tag_iterator = TagIterator::new(html);
        for element in tag_iterator {
            match element {
                Elements::Start(tag, _begin, end) => {
                    let tag_path_item = TagPathItem {
                        tag: Box::new(tag),
                        nth_child: 0, // FIXME don't need.
                    };
                    matchers.iter()
                        .enumerate()
                        .for_each(|(index, predicate)| {
                            if predicate(&tag_path_item) {
                                text_store.store_starting_position(index, end);
                            }
                        })
                }
                Elements::End(_name, begin, _end) => {
                    text_store.update_content(&mut founds, begin, html);
                }
                _ => {}
            }
        }

        founds
    }
}

#[macro_use]
#[cfg(test)]
mod test_matcher_selector {

    use crate::css_selector;

    use super::*;

    use std::fs;

    fn get_html() -> String {
        let filename = "./amazon_command.html";
        fs::read_to_string(filename).unwrap()
    }

    // #costBreakdown > tbody > tr:nth-child(9) > td:nth-child(2) > strong
    #[test]
    fn should_select_by_id() {
        let html = get_html();

        let id_matcher = css_selector!(#costBreakdown);

        let mut html_selector = MatcherHtmlSelector::new();

        let count = html_selector.count(&html, &[id_matcher]);

        assert_eq!(vec![1], count);
    }

    #[test]
    fn should_find_first_simple_content() {
        let html = r#"
        <html>
            <body>
                <div>
                    <p id="head">foo</p>
                    <p>bar</p>
                    <p>baz</p>
                </div>
            </body>
        </html>
        "#;
        let id_matcher = css_selector!(#head);
        let mut html_selector = MatcherHtmlSelector::new();

        let founds = html_selector.find_first(html, &[id_matcher]);

        assert_eq!(vec!["foo".to_string()], founds);
    }
}
