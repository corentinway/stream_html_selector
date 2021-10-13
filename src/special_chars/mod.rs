mod map;

use map::special_char_to_char;

fn decode(html: &str) -> &str  { 
    let mut init = String::new();
    let encoded = html
        .chars()
        .fold(&mut init, |acc, c| {
            if c == '&' {
                acc.push(c);
            } else if !acc.is_empty() {
                acc.push(c);
            } else if c == ';' {
                acc.push(c);
            }
            acc
        });
        println!("Encoded {:?}", encoded);
        let decoded = special_char_to_char(&encoded).unwrap();
        decoded
}


#[cfg(test)]
mod test_special_char_decoding {

    use super::*;

    #[test]
    fn should_decode_to_a_single_char() {
        let html = "&eacute;";

        let decoded = decode(html);

        assert_eq!("é", decoded);
    }


}