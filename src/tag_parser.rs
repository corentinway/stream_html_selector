use std::collections::HashMap;
use std::cmp::PartialEq;

#[derive(Debug)]
struct TagParser {
    attribute_name_tmp: String,
    attribute_value_tmp: String,
    state: ReadingState,
}

#[derive(PartialEq, Debug)]
enum ReadingState {
    START,
    READING_ATTRIBUTE_NAME,
    WAIT_FOR_EQUALS,
    WAIT_FOR_ATTRIBUTE_VALUE,
    READING_ATTRIBUTE_VALUE,
}

impl TagParser {

    pub fn new() -> Self {
        TagParser{
            attribute_name_tmp: String::new(),
            attribute_value_tmp: String::new(),
            state: ReadingState::START,
        }
    }

    // TODO clean tmp values

    pub fn parse_attributes(&mut self, html : &str) -> HashMap<String, String> {
        let mut attributes = HashMap::new();

        use ReadingState::*;

        html.chars()
            .for_each(|c| {

                println!("c = {:?}, parser = {:?}", c, self);

                if is_attribute_name(&c) && (self.state == START || self.state == READING_ATTRIBUTE_NAME) {
                    self.attribute_name_tmp.push(c);
                    self.state = READING_ATTRIBUTE_NAME;
                } else if is_attribute_name(&c) && self.state == READING_ATTRIBUTE_NAME {
                    self.attribute_name_tmp.push(c);
                } else if c == '=' && (self.state == READING_ATTRIBUTE_NAME || self.state == WAIT_FOR_EQUALS )  {
                    self.state = WAIT_FOR_ATTRIBUTE_VALUE;
                } else if is_quote(&c) && self.state == WAIT_FOR_ATTRIBUTE_VALUE {
                    // next is the value
                    self.state = READING_ATTRIBUTE_VALUE;
                } else if !is_quote(&c) && self.state == READING_ATTRIBUTE_VALUE {
                    self.attribute_value_tmp.push(c);
                } else if self.is_last_quote_ending_reading_attribute_value(&c) {
                    // end of attributes reading
                    // 1. storing
                    attributes.insert(self.attribute_name_tmp.clone(), self.attribute_value_tmp.clone());
                    // 2. cleaning
                    self.attribute_name_tmp = String::new();
                    self.attribute_value_tmp = String::new();
                    // 3. init state
                    self.state = START;
                } else if c.is_whitespace() && self.state == READING_ATTRIBUTE_NAME {
                    self.state = WAIT_FOR_EQUALS
                } else if is_attribute_name(&c) && self.state == WAIT_FOR_EQUALS {
                    // end of attributes reading
                    // 1. storing
                    attributes.insert(self.attribute_name_tmp.clone(), String::from("true"));
                    // 2. cleaning
                    self.attribute_name_tmp = String::new();
                    self.attribute_value_tmp = String::new();
                    // 3. init state
                    self.state = START;
                    self.attribute_name_tmp.push(c);
                }

                
            });

            // if it has attribute name without a value, then store the attribute with a true value
            if self.attribute_name_tmp.len() > 0 && self.attribute_value_tmp.is_empty() 
                && (self.state == READING_ATTRIBUTE_NAME || self.state == WAIT_FOR_EQUALS) {
                // 1. storing 
                attributes.insert(self.attribute_name_tmp.clone(), String::from("true"));
                // 2. cleaning
                self.attribute_name_tmp = String::new();
                self.attribute_value_tmp = String::new();
            }

        attributes
    }

    fn is_last_quote_ending_reading_attribute_value(&self, c : &char ) -> bool {
        is_quote(&c) && self.state == ReadingState::READING_ATTRIBUTE_VALUE
    }
}

fn is_attribute_name(c :&char) -> bool {
    c.is_alphanumeric() || *c == '-'
}
fn is_quote(c :&char) -> bool {
    *c == '"' || *c == '\''
}

#[cfg(test)]
mod tag_parser_test {

    use super::*;

    #[test] 
    fn should_return_empty_attributes_map_given_empty_str() {
        let html = "";
        let mut parser = TagParser::new();
        let attributes = parser.parse_attributes(html);
        assert!(attributes.is_empty());
    }

    #[test]
    fn should_return_find_one_attribute_given_one_tag_attribute() {
        let html = " foo='bar' hidden = 'true' ";
        println!("Parsing : {:?}", html);
        let mut parser = TagParser::new();
        let attributes = parser.parse_attributes(html);
        assert!(!attributes.is_empty());
        assert_eq!(Some(&String::from("bar")), attributes.get("foo"));
        assert_eq!(Some(&String::from("true")), attributes.get("hidden"));
    }

    #[test]
    fn should_read_an_attribute_without_value_at_last_position_of_the_html_code() {
        let html = " foo='bar' hidden ";
        let mut parser = TagParser::new();
        let attributes = parser.parse_attributes(html);
        assert!(!attributes.is_empty());
        assert_eq!(Some(&String::from("bar")), attributes.get("foo"));
        assert_eq!(Some(&String::from("true")), attributes.get("hidden"));
    }

    #[test]
    fn should_read_an_attribute_without_value_in_the_middle_position_of_the_html_code() {
        let html = " foo='bar' hidden class='title'";
        let mut parser = TagParser::new();
        let attributes = parser.parse_attributes(html);
        assert!(!attributes.is_empty());
        assert_eq!(Some(&String::from("bar")), attributes.get("foo"));
        assert_eq!(Some(&String::from("true")), attributes.get("hidden"));
        assert_eq!(Some(&String::from("title")), attributes.get("class"));
    }
}
