mod common;

use crate::common::assert_linked_with;
use linkify::LinkFinder;
use linkify::LinkKind;

#[test]
fn no_links() {
    assert_not_linked("");
    assert_not_linked("foo");
    assert_not_linked("@");
    assert_not_linked("a@");
    assert_not_linked("@a");
    assert_not_linked("@@@");
}

#[test]
fn simple() {
    assert_linked("foo@example.com", "|foo@example.com|");
    assert_linked("foo.bar@example.com", "|foo.bar@example.com|");
}

#[test]
fn allowed_text() {
    // I know, I know...
    assert_linked(
        "#!$%&'*+-/=?^_`{}|~@example.org",
        "|#!$%&'*+-/=?^_`{}|~@example.org|",
    );
}

#[test]
fn space_separation() {
    assert_linked("foo a@b.com", "foo |a@b.com|");
    assert_linked("a@b.com foo", "|a@b.com| foo");
    assert_linked("\na@b.com", "\n|a@b.com|");
    assert_linked("a@b.com\n", "|a@b.com|\n");
}

#[test]
fn special_separation() {
    assert_linked("(a@example.com)", "(|a@example.com|)");
    assert_linked("\"a@example.com\"", "\"|a@example.com|\"");
    assert_linked("\"a@example.com\"", "\"|a@example.com|\"");
    assert_linked(",a@example.com,", ",|a@example.com|,");
    assert_linked(":a@example.com:", ":|a@example.com|:");
    assert_linked(";a@example.com;", ";|a@example.com|;");
}

#[test]
fn dots() {
    assert_not_linked(".@example.com");
    assert_not_linked("foo.@example.com");
    assert_linked(".foo@example.com", ".|foo@example.com|");
    assert_linked(".foo@example.com", ".|foo@example.com|");
    assert_linked("a..b@example.com", "a..|b@example.com|");
    assert_linked("a@example.com.", "|a@example.com|.");
}

#[test]
fn domain_without_dot() {
    assert_not_linked("a@b");
    assert_not_linked("a@b.");
    assert_linked("a@b.com.", "|a@b.com|.");
}

#[test]
fn dashes() {
    assert_linked("a@example.com-", "|a@example.com|-");
    assert_linked("a@foo-bar.com", "|a@foo-bar.com|");
    assert_not_linked("a@-foo.com");
    assert_not_linked("a@b-.");
}

#[test]
fn domain_must_have_dot_false() {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Email]);
    finder.email_domain_must_have_dot(false);

    assert_linked_with(&finder, "a@b", "|a@b|");
    assert_linked_with(&finder, "a@b.", "|a@b|.");

    assert_linked_with(&finder, "a@b.", "|a@b|.");
}

#[test]
fn multiple() {
    assert_linked(
        "a@example.com b@example.com",
        "|a@example.com| |b@example.com|",
    );
    assert_linked(
        "a@example.com @ b@example.com",
        "|a@example.com| @ |b@example.com|",
    );
}

#[test]
fn multiple_delimited_hard() {
    assert_linked(
        "a@xy.com;b@xy.com,c@xy.com",
        "|a@xy.com|;|b@xy.com|,|c@xy.com|",
    );
}

#[test]
fn international() {
    assert_linked("üñîçøðé@example.com", "|üñîçøðé@example.com|");
    assert_linked("üñîçøðé@üñîçøðé.com", "|üñîçøðé@üñîçøðé.com|");
}

#[test]
fn trigger_overlap() {
    let finder = LinkFinder::new();

    // 'w' is a trigger character for WWW links. Make sure we can rewind enough.
    assert_linked_with(&finder, "www@example.com", "|www@example.com|");
}

#[test]
fn fuzz() {
    assert_linked("a@a.xyϸ", "|a@a.xyϸ|");
}

fn assert_not_linked(s: &str) {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Email]);
    let result = finder.links(s);
    assert_eq!(result.count(), 0, "expected no links in {:?}", s);
}

fn assert_linked(input: &str, expected: &str) {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Email]);
    assert_linked_with(&finder, input, expected);
}
