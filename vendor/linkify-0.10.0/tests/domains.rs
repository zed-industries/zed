mod common;

use crate::common::assert_linked_with;
use linkify::{LinkFinder, LinkKind};

#[test]
fn domain_valid() {
    assert_linked("9292.nl", "|9292.nl|");
    assert_linked("a12.b-c.com", "|a12.b-c.com|");
}

#[test]
fn domain_invalid_tld() {
    assert_not_linked("v1.2.3");
    assert_not_linked("https://12-7.0.0.1/");
}

#[test]
fn domain_with_userinfo() {
    assert_linked(
        "https://user:pass@example.com/",
        "|https://user:pass@example.com/|",
    );
    assert_linked(
        "https://user:-.!$@example.com/",
        "|https://user:-.!$@example.com/|",
    );
    assert_linked(
        "https://user:!$&\'()*+,;=@example.com/",
        "|https://user:!$&\'()*+,;=@example.com/|",
    );

    // Can't have another @
    assert_not_linked("https://user:pass@ex@mple.com/");
}

#[test]
fn domain_with_port() {
    assert_linked("https://localhost:8080!", "|https://localhost:8080|!");
    assert_linked("https://localhost:8080/", "|https://localhost:8080/|");
}

#[test]
fn domain_with_userinfo_and_port() {
    assert_linked(
        "https://user:pass@example.com:8080/hi",
        "|https://user:pass@example.com:8080/hi|",
    );
}

#[test]
fn domain_ipv4() {
    assert_linked("https://127.0.0.1/", "|https://127.0.0.1/|");
    assert_linked("1.0.0.0", "|1.0.0.0|");
    assert_linked("1.0.0.0/foo/bar", "|1.0.0.0/foo/bar|");
    assert_not_linked("1.0 ");
    assert_not_linked("1.0.0");
    assert_not_linked("1.0.0.0.0");
    assert_not_linked("1.0.0.");
}

#[test]
fn domain_trailing_dot() {
    // assert_linked("https://example.com./test", "|https://example.com./test|");
    assert_linked(
        "https://example.com.:8080/test",
        "|https://example.com.:8080/test|",
    );
}

#[test]
fn domain_delimited() {
    // Delimiter at end of domain should *not* be included
    assert_linked("https://example.org'", "|https://example.org|'");
    // Unless it's a userinfo of course (sike!)
    assert_linked(
        "https://example.org'a@example.com",
        "|https://example.org'a@example.com|",
    );
}

#[test]
fn domain_delimited_multiple() {
    assert_linked(
        "https://a.com'https://b.com",
        "https://a.com'|https://b.com|",
    );
}

#[test]
fn domain_dots() {
    assert_linked("https://example.com...", "|https://example.com|...")
}

#[test]
fn domain_labels_cant_be_empty() {
    assert_not_linked("www.example..com");
    assert_not_linked("https://.www.example.com");
}

#[test]
fn domain_labels_cant_start_with_hyphen() {
    assert_not_linked("-a.com");
    assert_not_linked("https://a.-b.com");
}

#[test]
fn domain_labels_cant_end_with_hyphen() {
    assert_not_linked("a-.com");
    assert_not_linked("a.b-.com");

    assert_not_linked("https://a.b-.com");
    // Could also argue that it should not be linked at all
    assert_linked("https://example.com-/", "|https://example.com|-/");
    assert_linked("https://example.org-", "|https://example.org|-");
}

#[test]
fn domain_cant_contain_at() {
    // Looks like an email but was recognized as a schemeless link before.
    assert_not_linked("example.com@about");
    // As part of path it's ok.
    assert_linked("example.com/@about", "|example.com/@about|");
    assert_linked("https://example.com/@about", "|https://example.com/@about|");
}

#[test]
fn domain_cant_end_numeric() {
    assert_not_linked("info@v1.1.1");
}

#[test]
fn no_authority_part() {
    assert_linked("file:///", "|file:///|");
    assert_linked("file:///home/foo", "|file:///home/foo|");
    assert_linked("file://localhost/home/foo", "|file://localhost/home/foo|");
}

#[test]
fn authority_thats_not_domain() {
    // Not valid according to DNS but we should allow it for other schemes (or all, not sure).
    assert_linked("facetime://+19995551234", "|facetime://+19995551234|");
}

#[test]
fn authority_without_slash_should_stop_at_delimiters() {
    // What's going on here? For schemes where we don't enforce domainyness,
    // we want to stop at delimiter characters. Note that `!` is valid in authorities.
    assert_linked("test://123'456!!!", "|test://123'456|!!!");
    assert_linked("test://123'456...", "|test://123'456|...");
    // ... unless there is a "/" terminating the authority.
    assert_linked("test://123'456!!!/", "|test://123'456!!!/|");
    assert_linked("test://123'456.../", "|test://123'456.../|");
}

#[test]
fn without_scheme_should_stop() {
    // assert_linked("ab/example.com", "ab/|example.com|");
    // This is not a valid scheme. Even the schemeless parser should not accept it, nor extract
    // only example.com out of it.
    assert_not_linked("1abc://example.com");
}

#[test]
pub fn test_international_not_allowed() {
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(false);
    finder.url_can_be_iri(false);
    let link = finder.links("\u{A1}\u{A2}example.com").next().unwrap();
    assert_eq!(link.kind(), &LinkKind::Url);
    assert_eq!(link.as_str(), "example.com");
}

#[test]
pub fn test_international_allowed() {
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(false);
    finder.url_can_be_iri(true);
    let link = finder.links("\u{A1}\u{A2}example.com").next().unwrap();
    assert_eq!(link.kind(), &LinkKind::Url);
    assert_eq!(link.as_str(), "\u{A1}\u{A2}example.com");
}

fn assert_linked(input: &str, expected: &str) {
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(false);

    assert_linked_with(&finder, input, expected);
}

fn assert_not_linked(s: &str) {
    assert_linked(s, s);
}
