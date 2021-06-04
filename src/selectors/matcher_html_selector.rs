
use crate::selectors::HtmlSelectorCounter;
use crate::selectors::HtmlSelectorFindFirst;
use crate::tag_iterator::TagIterator;
use crate::tag_iterator::Elements;
use crate::elements::start_element::Tag;


struct MatcherHtmlSelector {}
impl MatcherHtmlSelector {
    fn new() -> Self {
        MatcherHtmlSelector{}
    }
}


impl<F> HtmlSelectorCounter<F> for MatcherHtmlSelector
    where F: Fn(Tag) -> bool
{


    fn count(&mut self, html: &str, matchers: &[F]) -> Vec<usize>  
    {
        let mut count = 0;
        let f = &matchers[0];

        let tag_iterator = TagIterator::new(html);
        tag_iterator.for_each(|element| {

            match element {
                Elements::Start(tag, _begin, _end) => {
                    if f(tag) {
                        count +=1;
                    }
                    
                }
                _ => {}
            }
        });

        vec![count]
    }
}

#[cfg(test)]
mod test_matcher_selector {

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

        let expected_id = String::from("costBreakdown");
        let id_matcher = |tag: Tag| {
            if let Some(id) = tag.id() {
                return *id == expected_id;
            }
            false
        };

        let mut html_selector = MatcherHtmlSelector::new();

        let count = html_selector.count(&html, &[id_matcher]);

        assert_eq!(vec![1], count);

    }
}