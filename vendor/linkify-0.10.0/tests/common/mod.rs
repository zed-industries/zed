use linkify::LinkFinder;

pub fn assert_linked_with(finder: &LinkFinder, input: &str, expected: &str) {
    let actual = show_links(input, finder);
    assert_eq!(actual, expected);
}

pub fn show_links(input: &str, finder: &LinkFinder) -> String {
    let mut result = String::new();

    for span in finder.spans(input) {
        if span.kind().is_some() {
            result.push('|');
            result.push_str(span.as_str());
            result.push('|');
        } else {
            result.push_str(span.as_str());
        }
    }
    result
}
