use crate::SyntaxTree;

use super::*;
use indoc::indoc;
use util::test::{generate_marked_text, marked_text_ranges};

fn parse_json(source: &str) -> SyntaxTree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .expect("failed to set language");
    let tree = parser.parse(source, None).expect("failed to parse");
    build_tree(tree.walk(), source)
}

fn parse_rust(source: &str) -> SyntaxTree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("failed to set language");
    let tree = parser.parse(source, None).expect("failed to parse");
    build_tree(tree.walk(), source)
}

#[track_caller]
fn assert_diff(lhs_marked: &str, rhs_marked: &str, parser: fn(&str) -> SyntaxTree) {
    let (lhs_text, expected_lhs_ranges) = marked_text_ranges(lhs_marked, false);
    let (rhs_text, expected_rhs_ranges) = marked_text_ranges(rhs_marked, false);

    let lhs_tree = parser(&lhs_text);
    let rhs_tree = parser(&rhs_text);

    let diff = diff_trees(&lhs_tree, &rhs_tree, &DiffOptions::default())
        .expect("diff should not exceed graph limit");

    let actual_lhs_marked = generate_marked_text(&lhs_text, &diff.lhs_ranges, false);
    let actual_rhs_marked = generate_marked_text(&rhs_text, &diff.rhs_ranges, false);

    assert_eq!(
        diff.lhs_ranges, expected_lhs_ranges,
        "LHS ranges mismatch.\nExpected: {lhs_marked}\nActual:   {actual_lhs_marked}"
    );
    assert_eq!(
        diff.rhs_ranges, expected_rhs_ranges,
        "RHS ranges mismatch.\nExpected: {rhs_marked}\nActual:   {actual_rhs_marked}"
    );
}

#[test]
fn test_diff_trees_identical_json() {
    assert_diff(r#"{"a": 1, "b": 2}"#, r#"{"a": 1, "b": 2}"#, parse_json);
}

#[test]
fn test_diff_trees_changed_value() {
    assert_diff(r#"{"a": «1»}"#, r#"{"a": «2»}"#, parse_json);
}

#[test]
fn test_diff_trees_added_key() {
    assert_diff(r#"{"a": 1}"#, r#"{"a": 1«,» «"b":» «2»}"#, parse_json);
}

#[test]
fn test_diff_trees_removed_key() {
    assert_diff(r#"{"a": 1«,» «"b":» «2»}"#, r#"{"a": 1}"#, parse_json);
}

#[test]
fn test_diff_trees_rust_changed_function_body() {
    assert_diff(
        indoc! {r#"
                fn main() {
                    println!("«hello»");
                }
            "#},
        indoc! {r#"
                fn main() {
                    println!("«world»");
                }
            "#},
        parse_rust,
    );
}

#[test]
fn test_diff_trees_rust_added_function() {
    assert_diff(
        indoc! {r#"
                fn foo() {
                    println!("foo");
                }
            "#},
        indoc! {r#"
                fn foo() {
                    println!("foo");
                }

                «fn» «bar()» «{»
                    «println!("bar");»
                «}»
            "#},
        parse_rust,
    );
}

#[test]
fn test_diff_trees_rust_changed_function_signature() {
    assert_diff(
        indoc! {r#"
                fn process(x: i32) -> i32 {
                    x «*» «2»
                }
            "#},
        indoc! {r#"
                fn process(x: i32«,» «y:» «i32») -> i32 {
                    x «+» «y»
                }
            "#},
        parse_rust,
    );
}

#[test]
fn test_diff_trees_rust_struct_field_change() {
    assert_diff(
        indoc! {r#"
                struct Point {
                    x: f64,
                    y: f64,
                }
            "#},
        indoc! {r#"
                struct Point {
                    x: f64,
                    y: f64,
                    «z:» «f64,»
                }
            "#},
        parse_rust,
    );
}

#[test]
fn test_diff_trees_rust_match_arm_change() {
    assert_diff(
        indoc! {r#"
                fn classify(n: i32) -> &'static str {
                    match n {
                        0 => "zero",
                        «1» => "«one»",
                        _ => "other",
                    }
                }
            "#},
        indoc! {r#"
                fn classify(n: i32) -> &'static str {
                    match n {
                        0 => "zero",
                        «1» «|» «2» => "«small»",
                        _ => "other",
                    }
                }
            "#},
        parse_rust,
    );
}
