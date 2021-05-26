pub fn select_by_type(tag_path_str: &str, selector_chain: &str) -> bool {
    tag_path_str.ends_with(selector_chain)
}

pub fn select_by_class(tag_path_str: &str, selector_chain: &str) -> bool {
    tag_path_str.ends_with(selector_chain)
}

pub fn select_by_id(tag_path_str: &str, selector_chain: &str) -> bool {
    tag_path_str.ends_with(selector_chain)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_true_given_the_selector_type_is_the_last_tag_in_the_path() {
        // given a tag path
        let tag_path_str = "html body div p";

        let selector_chain = "p";

        let is_present = select_by_type(tag_path_str, selector_chain);

        assert_eq!(true, is_present);
    }

    #[test]
    fn should_return_false_given_the_selector_type_is_the_not_last_tag_in_the_path() {
        // given a tag path
        let tag_path_str = "html body div p";

        let selector_chain = "div";

        let is_present = select_by_type(tag_path_str, selector_chain);

        assert_eq!(false, is_present);
    }

    #[test]
    fn should_return_true_given_the_class_selector_is_in_the_last_tag_path() {
        // given a tag path
        let tag_path_str = "html body div p.important";

        let selector_chain = ".important";

        let is_present = select_by_class(tag_path_str, selector_chain);

        assert_eq!(true, is_present);
    }
    #[test]
    fn should_return_false_given_the_class_selector_is_not_in_the_last_tag_path() {
        // given a tag path
        let tag_path_str = "html body div p.important";

        let selector_chain = ".alert";

        let is_present = select_by_class(tag_path_str, selector_chain);

        assert_eq!(false, is_present);
    }

    #[test]
    fn should_return_true_given_the_id_selector_is_in_the_last_tag_path() {
        // given a tag path
        let tag_path_str = "html body div p#clock";

        let selector_chain = "#clock";

        let is_present = select_by_id(tag_path_str, selector_chain);

        assert_eq!(true, is_present);
    }
    #[test]
    fn should_return_false_given_the_id_selector_is_not_in_the_last_tag_path() {
        // given a tag path
        let tag_path_str = "html body div p#clock";

        let selector_chain = "#chair";

        let is_present = select_by_id(tag_path_str, selector_chain);

        assert_eq!(false, is_present);
    }
}
