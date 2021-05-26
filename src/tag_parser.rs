use std::cmp::PartialEq;
use std::collections::HashMap;

/// represent the state when parsing elements inside an HTML tag
#[derive(Debug)]
pub struct TagParser {
    /// temporary content for the attribute name
    attribute_name_tmp: String,
    /// temporary content for the attribute value
    attribute_value_tmp: String,
    /// state when reading the content of an HTML tag
    state: ReadingState,
}

/// state of the parser when reading inside an HTML tag
#[derive(PartialEq, Debug)]
enum ReadingState {
    /// First state and initial state when before reading a (new) attribute.
    Start,
    /// The parser is reading the attribute name.
    ReadingAttributeName,
    /// The parser had finished to read the attribute name and is expecting equals characters.
    /// This state is useful when the `=` character has white spaces before
    WaitForEquals,
    /// The parser found the `=` character and expect to read the attribute value.
    WaitForAttributeValue,
    /// The parser is reading the attribute value.
    ReadingAttributeValue,
}

/// implementation of a tag parser
impl TagParser {
    /// initialize the parser
    pub fn new() -> Self {
        TagParser {
            attribute_name_tmp: String::new(),
            attribute_value_tmp: String::new(),
            state: ReadingState::Start,
        }
    }

    /// Parse all attributes after the tag name. It read attributes character by character.
    pub fn parse_attributes(&mut self, html: &str) -> HashMap<String, String> {
        let mut attributes = HashMap::new();

        use ReadingState::*;

        html.chars().for_each(|c| {
            //println!("c = {:?}, parser = {:?}", c, self);

            if is_attribute_name(&c) && (self.state == Start || self.state == ReadingAttributeName)
            {
                self.attribute_name_tmp.push(c);
                self.state = ReadingAttributeName;
            } else if is_attribute_name(&c) && self.state == ReadingAttributeName {
                self.attribute_name_tmp.push(c);
            } else if c == '='
                && (self.state == ReadingAttributeName || self.state == WaitForEquals)
            {
                self.state = WaitForAttributeValue;
            } else if is_quote(&c) && self.state == WaitForAttributeValue {
                // next is the value
                self.state = ReadingAttributeValue;
            } else if !is_quote(&c) && self.state == ReadingAttributeValue {
                self.attribute_value_tmp.push(c);
            } else if self.is_last_quote_ending_reading_attribute_value(&c) {
                // end of attributes reading
                // 1. storing
                attributes.insert(
                    self.attribute_name_tmp.clone(),
                    self.attribute_value_tmp.clone(),
                );
                // 2. cleaning
                self.re_initialise_state();
            } else if c.is_whitespace() && self.state == ReadingAttributeName {
                self.state = WaitForEquals
            } else if is_attribute_name(&c) && self.state == WaitForEquals {
                // end of attributes reading
                // 1. storing
                attributes.insert(self.attribute_name_tmp.clone(), String::from("true"));
                // 2. cleaning
                self.re_initialise_state();
                // 3. init state
                self.attribute_name_tmp.push(c);
            }
        });

        // if it has attribute name without a value, then store the attribute with a true value
        if !self.attribute_name_tmp.is_empty()
            && self.attribute_value_tmp.is_empty()
            && (self.state == ReadingAttributeName || self.state == WaitForEquals)
        {
            // 1. storing
            attributes.insert(self.attribute_name_tmp.clone(), String::from("true"));
            // 2. cleaning
            self.re_initialise_state();
        }

        attributes
    }

    /// Initialise the parser in order to read the next attribute.
    /// It cleans tmp variables and re-initialize the parser state
    fn re_initialise_state(&mut self) {
        self.attribute_name_tmp = String::new();
        self.attribute_value_tmp = String::new();
        self.state = ReadingState::Start;
    }

    fn is_last_quote_ending_reading_attribute_value(&self, c: &char) -> bool {
        is_quote(&c) && self.state == ReadingState::ReadingAttributeValue
    }
}

/// Returns true in the case the letter belongs to the allowed letter for attribute names.
fn is_attribute_name(c: &char) -> bool {
    c.is_alphanumeric() || *c == '-'
}
/// Returns true if the character is `"` or a `'`.
fn is_quote(c: &char) -> bool {
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
