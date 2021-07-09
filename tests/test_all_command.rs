extern crate stream_html_selector;

use stream_html_selector::css_selector;

fn get_command() -> String {
    let path = "./amazon_command.html";
    std::fs::read_to_string(path).unwrap()
}

#[test]
fn should_get_all_details_of_single_command() {
    // #header > tbody > tr:nth-child(2) > td > a
    let command_number_matcher = vec![
        css_selector!(#header),
        css_selector!(tbody),
        css_selector!(tr: nth - child(2)),
        css_selector!(td),
        css_selector!(a),
    ];

    // #orderDetails > tbody > tr > td > span
    let command_date = vec![
        css_selector!(#orderDetails),
        css_selector!(tbody),
        css_selector!(tr),
        css_selector!(td),
        css_selector!(span),
    ];

    let total_matcher = vec![
        css_selector!(#costBreakdown),
        css_selector!(tbody),
        css_selector!(tr: nth - child(9)),
        css_selector!(td: nth - child(2)),
        css_selector!(strong),
    ];

    // #itemDetails > tbody > tr > td.name > a:nth-child(1)
    let label_item_1 = vec![
        css_selector!(#itemDetails),
        css_selector!(tbody),
        css_selector!(tr),
        css_selector!(td.name),
        css_selector!(a: nth - child(1)),
    ];

    // #itemDetails > tbody > tr > td.price > strong
    let amount_item_1 = vec![
        css_selector!(#itemDetails),
        css_selector!(tbody),
        css_selector!(tr),
        css_selector!(td.price),
        css_selector!(strong),
    ];

    let matchers = vec![
        &command_number_matcher,
        &command_date,
        &total_matcher,
        &label_item_1,
        &amount_item_1,
    ];

    let html = get_command();

    let founds = stream_html_selector::by_tag_path::find_first(html.as_str(), &matchers);

    let expected = vec![
        String::from("405-5855855-9921124"),             // command number
        String::from("Effectu&eacute;e le 8 juin 2019"), // command date
        String::from("EUR 61,90"),                       // total amount
        String::from(" 2&nbsp;x Rehausseur Si&egrave;ge auto EOS ROUGE 15-36&nbsp;kg ECE R44/04 "), // label item 1
        String::from("EUR 55,95"), // amount 1
    ];

    assert_eq!(expected, founds);
}
