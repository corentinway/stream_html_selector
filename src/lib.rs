mod tag;
mod selector;

#[derive(PartialEq)]
enum PathUpdateState {
    TagAdded,
    TagDeleted,
    NewTagToDelete,
}

use PathUpdateState::*;

#[derive(PartialEq)]
enum State {
    Start,
    StartElement,
    EndElement,
}

struct HtmlSelector {
    tag_path: Vec<String>,
    tag_path_string: String,
    state: State,
}

impl HtmlSelector {
    fn new() -> Self {
        HtmlSelector {
            tag_path: Vec::new(),
            tag_path_string: String::new(),
            state: State::Start,
        }
    }

    pub fn count(&mut self, html: &String, css_requests: &Vec<&str>) -> Vec<usize> {
        let mut counts = vec![0; css_requests.len()];

        let mut tag = String::new();

        html.chars()
            // filter HTML tags
            .for_each(|c| {
                if c == '<' {
                    self.state = State::StartElement
                }
                if self.state == State::StartElement {
                    tag.push(c)
                }
                if c == '>' {
                    self.state = State::EndElement;

                    let updated = self.update_tag_path(&tag);

                    if updated == TagAdded || updated == NewTagToDelete {
                        css_requests
                            .into_iter()
                            .enumerate()
                            .for_each(|(index, request)| {
                                //println!("word : {:?} request {:?}", word, request);

                                if self.match_request(request) {
                                    counts[index] = counts[index] + 1;
                                }
                            });
                        if updated == NewTagToDelete {
                            self.reduce_path();
                        }

                    }
                    tag = String::new();
                }
            });

        counts
    }

    fn match_request(&self, request: &str) -> bool {
        if self.tag_path.len() == 1 {
            self.tag_path_string.ends_with(request)
        } else {
            self.tag_path_string.ends_with(format!(" {}", request).as_str())
        }
        
    }

    fn reduce_path(&mut self) {
        self.tag_path.pop();
        self.tag_path_string = self.tag_path.join(" ");
    }

    fn increase_path(&mut self, tag_name : String ) {
        self.tag_path.push(tag_name);
        self.tag_path_string = self.tag_path.join(" ");
    }

    fn update_tag_path(&mut self, word: &String) -> PathUpdateState {
        let mut updated = TagDeleted;
        println!("   WORD = {:?}", word);

        if word.get(0..2) == Some("</") {
            // end of tag
            self.reduce_path();
        } else if word.ends_with("/>") {
            // put the tag, 
            let tag = tag::extract_tag_name(word);
            self.increase_path(tag.name);
            // search for css request
            // auto delete the tag imediatelly
            updated = NewTagToDelete;
        } else if word.get(0..1) == Some("<") {
            // start of tag
            let tag = tag::extract_tag_name(word);
            self.increase_path(tag.name);
            updated = TagAdded;
        } else {
            println!("no mathcing");
        }

        println!("tag path {:?}\n", self.tag_path_string);

        updated
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    fn get_html() -> String {
        let filename = "./amazon_command.html";
        fs::read_to_string(filename).unwrap()
    }

    #[test]
    fn should_return_all_body_tag() {
        let html = get_html();

        let css_request = vec!["body"];

        let mut html_selector = HtmlSelector::new();

        let count = html_selector.count(&html, &css_request);

        assert_eq!(count, vec![1]);
    }

    #[test]
    fn should_match_many_tag_request() {
        let html = get_html();
        let css_requests = vec!["body", "td"];

        let mut html_selector = HtmlSelector::new();

        let counts = html_selector.count(&html, &css_requests);

        assert_eq!(counts, vec![1, 69]);
    }

    #[test]
    fn should_match_child_tag_request() {
        let html = String::from(
            r#"
            <body>
                <div>
                <p>foo</p>
                <p>bar</p>
                <p>baz</p>
                </div>
            </body>
        "#,
        );
        let css_requests = vec!["body", "div p"];

        let mut html_selector = HtmlSelector::new();

        let counts = html_selector.count(&html, &css_requests);

        assert_eq!(counts, vec![1, 3]);
    }
}
