use std::collections::HashMap;

pub struct Tag {
    pub name: String,
    id: Option<String>,
    classes: Option<String>,
    attributes: HashMap<String, Option<String>>,
}



pub fn extract_tag_name(html: &str) -> Tag {
    // remote '<' and '>'
    let start = html.find("<").unwrap();
    let mut tag_name = html.get(start + 1..).unwrap();

    let mut end = tag_name
        .find("/>")
        .unwrap_or(tag_name.find(">").unwrap_or(tag_name.len()));

    tag_name = tag_name.get(0..end).unwrap();

    let mut tag_name: String = tag_name
        .replace("\n\r", " ")
        .replace("\n", " ")
        .replace("\r", " ");

    let mut attributes = find_attributes(tag_name.as_str());

    let mut id_str = attributes.get("id").unwrap();
    let mut classes = attributes.get("class").unwrap();


    // remove spaces
    end = tag_name.find(" ").unwrap_or(tag_name.len());

    tag_name = tag_name.get(0..end).unwrap().to_string();

    Tag {
        name : tag_name,
        id: *id_str.clone(),
        classes: *classes.clone(),
        attributes: attributes,
    }
}

#[derive(PartialEq)]
enum ReadingAttrbuteState {
    READING_NAME,
    READING_VALUE,
    READING_EQUAL,
    READING_SPACE,
    STORE_VALUE,
}

struct Parser {
    attribute_name: String,
    attribute_value: String,
    reading_state : ReadingAttrbuteState,
    attributes: HashMap<String, Option<String>>,
}


fn find_attributes(tag: &str) -> HashMap<String, Option<String>> {

    //let mut map = HashMap::new();

    let mut attribute_value_starter : Option<char> = None;

    let init_parser = Parser {
        attribute_name: String::new(),
        attribute_value: String::new(),
        reading_state: ReadingAttrbuteState::READING_SPACE,
        attributes: HashMap::new(),
    };


    let parsed = 
    tag.chars()
        .skip_while( |c| *c == ' ')
        .fold( init_parser, |mut parser, c| {

            let mut id = Some(String::new());
            let mut class = Some(String::new());

            // TODO handle the case where the attribute value is bound between 'value'

            if (c == '"' || c == '\'') && parser.reading_state == ReadingAttrbuteState::READING_VALUE {
                // end of reading value
                parser.reading_state = ReadingAttrbuteState::STORE_VALUE;
            }
            if (c == '"' || c == '\'') && (parser.reading_state == ReadingAttrbuteState::READING_EQUAL || parser.reading_state == ReadingAttrbuteState::READING_SPACE) {
                // we start reading value
                parser.reading_state == ReadingAttrbuteState::READING_VALUE;
                attribute_value_starter = Some('"');
            }
            else if c == '"' && parser.reading_state == ReadingAttrbuteState::READING_VALUE {
                // end of reading value
                // TODO FIXME
            }
            else if  c == '=' {
                parser.reading_state = ReadingAttrbuteState::READING_EQUAL;
            } else if c == ' ' || parser.reading_state == ReadingAttrbuteState::STORE_VALUE {
                parser.reading_state = ReadingAttrbuteState::READING_SPACE;
                // assigning name & values
                parser.attributes.insert(parser.attribute_name.clone(), Some(parser.attribute_value.clone()));
                parser.attribute_name = String::new();
                parser.attribute_value = String::new();

            } else if parser.reading_state == ReadingAttrbuteState::READING_SPACE {
                parser.attribute_name.push(c);
            } else if parser.reading_state == ReadingAttrbuteState::READING_EQUAL {
                parser.reading_state = ReadingAttrbuteState::READING_VALUE;
            } else if parser.reading_state == ReadingAttrbuteState::READING_VALUE {
                parser.attribute_value.push(c);
            }

            parser

        });


/*

    tag.split(" ")
        .skip(1)
        .for_each( |parts| {
            let mut iter = parts.split("=");
            let name = iter.next().unwrap();
            let value = iter.next()
                .map(|val| val.get(1..val.len()-1).unwrap().to_string());
            if name == "id" {
                id = value;
            } else if name == "class" {
                class = value;
            } else {
                map.insert( name.to_string(), value);
            }
        });
*/
    /* if !map.is_empty() {
        acc.2 = Some(map);
    } */

    parsed.attributes

}
 #[cfg(test)]
mod test_read_attributes {

    use super::*;

    #[test]
    fn should_parse_attributes() {
        let html = "div";
        let attributes = find_attributes(html);
        let id = attributes.get("id").unwrap();
        let class = attributes.get("class").unwrap();
        assert!(id.is_none());
        assert!(class.is_none());
    }

    #[test]
    fn should_parse_id_attributes() {
        let html = " id='bar'";
        let attributes = find_attributes(html);
        let id = attributes.get("id").unwrap();
        let class = attributes.get("class").unwrap();
        assert_eq!(Some(String::from("bar")), *id );
        assert!(class.is_none());
    }


}

fn find_id(tag : &str) -> Option<String> {

    let start = tag.find("id=\"").or_else( || tag.find("id='"));

    if let Some(start) = start {
        let id = tag.get(start + 4..).unwrap();
        let end = id.find("\"").or_else(|| id.find("'")).unwrap();
        return Some(id.get(0..end).unwrap().to_string())
    }

    None
}

fn find_classes(tag : &str) -> Option<Vec<String>> {

    let start = tag.find("class=\"").or_else( || tag.find("class='"));

    if let Some(start) = start {
        let id = tag.get(start + 7..).unwrap();
        let end = id.find("\"").or_else(|| id.find("'")).unwrap();
        let class_str = id.get(0..end).unwrap();
        let classes : Vec<String> = class_str.split(" ")
            .filter( |class| !class.is_empty())
            .map( |class| class.to_string())
            .collect();
        return Some(classes)
    }

    None
}

fn parse_classes(classes_str: &str) -> Option<Vec<String>> {
    let classes : Vec<String> = classes_str.split(" ")
        .filter( |class| !class.is_empty())
        .map( |class| class.to_string())
        .collect();
    Some(classes)
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_extract_tag_name_from_tiny_tag() {
        assert_eq!("div", extract_tag_name("<div>").name);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_starting_with_blank() {
        assert_eq!("div", extract_tag_name("   <div>").name);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_ending_with_blank() {
        assert_eq!("div", extract_tag_name("<div>    ").name);
    }
    #[test]
    fn should_extract_tag_name_from_tiny_tag_with_blank_after_tag_name() {
        assert_eq!("div", extract_tag_name("<div   >").name);
        assert_eq!("div", extract_tag_name("<div  id='foo' >").name);
        assert_eq!("foo", extract_tag_name("<div  id='foo' >").id.unwrap());
        assert_eq!("bar", extract_tag_name("<div  id=\"bar\" >").id.unwrap());
    }
    #[test]
    fn should_extract_zero_html_class() {
        let html = "<div >";
        let tag = extract_tag_name(html);
        assert!(tag.classes.is_none());
    }
    #[test]
    fn should_extract_one_html_class() {
        let html = "<div  class=\"bar\" >";
        let tag = extract_tag_name(html);
        assert!(tag.classes.is_some());
        assert_eq!(Some(vec!["bar".to_string()]), tag.classes);
    }
    #[test]
    fn should_extract_many_html_classes() {
        let html = "<div  class='bar   baz   foo mun' >";
        let tag = extract_tag_name(html);
        assert_eq!("div", tag.name);
        assert!(tag.classes.is_some());
        println!("=====> classes {:?}", tag.classes);
        assert_eq!(4, tag.classes.as_ref().unwrap().len());
        assert_eq!(Some(
            vec![
                "bar".to_string(),
                "baz".to_string(),
                "foo".to_string(),
                "mun".to_string()
                ]), 
            tag.classes);
    }


    #[test]
    fn should_extract_consider_id_and_class_as_attributes() {
        let html = "<div id='foo' class='bar'>";
        let tag = extract_tag_name(html);
        assert!(tag.attributes.is_none());
        
        assert_eq!("foo", tag.id.unwrap());

        assert_eq!("bar", tag.classes.unwrap().iter().next().unwrap() );
    }
    
    #[test]
    fn should_extract_ignore_id_and_class_as_attributes_and_read_one() {
        let html = "<input id='foo' class='bar' type='password'>";
        let tag = extract_tag_name(html);
        assert!(tag.attributes.is_some());
        let attributes = tag.attributes.unwrap();
        println!("attributes {:?}", attributes);
        assert_eq!(1, attributes.len());


    }

    #[test]
    fn should_extract_tag_name_from_tag_into_multi_line() {
        let html = r#"<div
            id='foo'
            class='bar'
            about
        >"#;

        assert_eq!("div", extract_tag_name(html).name);
    }
    #[test]
    fn should_extract_auto_closing_tag() {
        assert_eq!("br", extract_tag_name("<br/>").name);
    }
    #[test]
    fn should_extract_tag_name_that_does_not_close() {
        assert_eq!("br", extract_tag_name("<br").name);
    }
}
