use dugong::util;

fn suffix_is_digits(s: &str, prefix: &str) -> bool {
    if !s.starts_with(prefix) {
        return false;
    }
    let rest = &s[prefix.len()..];
    !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
}

#[test]
fn unique_id_name_generates_a_valid_identifier() {
    let id = util::unique_id("_root");
    assert!(!id.contains("[object undefined]"));
    assert!(suffix_is_digits(&id, "_root"));
}

#[test]
fn unique_id_multiple_calls_generate_distinct_values() {
    let first = util::unique_id("name");
    let second = util::unique_id("name");
    let third = util::unique_id("name");
    assert_ne!(first, second);
    assert_ne!(second, third);
}

#[test]
fn unique_id_number_prefix_creates_a_valid_identifier_string() {
    let id = util::unique_id(99);
    assert!(suffix_is_digits(&id, "99"));
}
