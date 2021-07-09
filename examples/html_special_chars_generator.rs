extern crate stream_html_selector;

use std::vec;

use stream_html_selector::elements::start_element::Tag;
use stream_html_selector::tag_iterator::Elements;
use stream_html_selector::tag_iterator::TagIterator;

use std::collections::HashMap;

struct Parser<'a> {
    map: HashMap<&'a str, &'a str>,
    character_start_position: Option<usize>,
    tmp_char: &'a str,

    code_start_position: Option<usize>,
    tmp_codes: Vec<&'a str>,
}

fn main() {
    let path = "./examples/Character Entity Reference Chart.html";
    let html = std::fs::read_to_string(path).unwrap();
    let html = html.as_str();

    let has_class = |tag: &Tag, expected_class_name: String| {
        if let Some(actual_class_name) = tag.classes() {
            return actual_class_name.contains(expected_class_name.as_str());
        }
        false
    };

    let iterator = TagIterator::new(html);

    let mut init = Parser {
        map: HashMap::new(),
        character_start_position: None,
        tmp_char: "",
        code_start_position: None,
        tmp_codes: vec![],
    };

    iterator
        .fold(&mut init, |parser, element| {
            match element {
                Elements::Start(tag, _begin, end) => {
                    if tag.name == String::from("td") && has_class(&tag, String::from("character"))
                    {
                        parser.character_start_position = Some(end);
                    }
                    if tag.name == String::from("code") {
                        parser.code_start_position = Some(end);
                    }
                }
                Elements::End(name, begin, _end) => {
                    if name == String::from("td") && parser.character_start_position.is_some() {
                        let character = html.get(parser.character_start_position.unwrap()..begin);
                        //println!("- {:?}", character);
                        parser.character_start_position = None;
                        parser.tmp_char = character.unwrap();
                    }
                    if name == String::from("code") && parser.code_start_position.is_some() {
                        let code_found = html.get(parser.code_start_position.unwrap()..begin);
                        parser.code_start_position = None;
                        parser.tmp_codes.push(code_found.unwrap());
                    }
                    if name == String::from("tr") {
                        for code in &parser.tmp_codes {
                            parser.map.insert(parser.tmp_char, code);
                        }
                        parser.tmp_char = "";
                        parser.tmp_codes.clear();
                        parser.character_start_position = None;
                        parser.code_start_position = None;
                    }
                }
                _ => {}
            }

            parser
        })
        .map
        .iter()
        .map(|(key, val)| (key, val.replace("&amp;", "&")))
        .for_each(|(key, val)| {
            println!("m.insert({:?}, {:?});", key, val);
        });
}
