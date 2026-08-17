use std::collections::HashMap;
use std::f64;

mod common;

use crate::common::{deserializes_to, deserializes_to_nan_f64};

// The following tests are adapted from https://github.com/json5/json5/blob/d828908384ce8dc40d8dde017ae82afd1b952d79/test/parse.js

// objects

#[test]
fn parses_empty_objects() {
    let m: HashMap<String, i32> = HashMap::new();
    deserializes_to("{}", m);
}

#[test]
fn parses_double_string_property_names() {
    let mut m = HashMap::new();
    m.insert("a".to_owned(), 1);
    deserializes_to("{\"a\":1}", m);
}

#[test]
fn parses_single_string_property_names() {
    let mut m = HashMap::new();
    m.insert("a".to_owned(), 1);
    deserializes_to("{'a':1}", m);
}

#[test]
fn parses_unquoted_property_names() {
    let mut m = HashMap::new();
    m.insert("a".to_owned(), 1);
    deserializes_to("{a:1}", m);
}

#[test]
fn parses_special_character_property_names() {
    let mut m = HashMap::new();
    m.insert("$_".to_owned(), 1);
    m.insert("_$".to_owned(), 2);
    m.insert("a\u{200C}".to_owned(), 3);
    deserializes_to("{$_:1,_$:2,a\u{200C}:3}", m);
}

#[test]
fn parses_unicode_property_names() {
    let mut m = HashMap::new();
    m.insert("ùńîċõďë".to_owned(), 9);
    deserializes_to("{ùńîċõďë:9}", m);
}

#[test]
fn parses_escaped_property_names() {
    let mut m = HashMap::new();
    m.insert("ab".to_owned(), 1);
    m.insert("$_".to_owned(), 2);
    m.insert("_$".to_owned(), 3);
    deserializes_to("{\\u0061\\u0062:1,\\u0024\\u005F:2,\\u005F\\u0024:3}", m);
}

#[test]
fn parses_multiple_properties() {
    let mut m = HashMap::new();
    m.insert("abc".to_owned(), 1);
    m.insert("def".to_owned(), 2);
    deserializes_to("{abc:1,def:2}", m);
}

#[test]
fn parses_nested_objects() {
    let mut inner = HashMap::new();
    inner.insert("b".to_owned(), 2);
    let mut outer = HashMap::new();
    outer.insert("a".to_owned(), inner);
    deserializes_to("{a:{b:2}}", outer);
}

// arrays

#[test]
fn parses_empty_arrays() {
    let v: Vec<i32> = vec![];
    deserializes_to("[]", v);
}

#[test]
fn parses_array_values() {
    deserializes_to("[1]", vec![1]);
}

#[test]
fn parses_multiple_array_values() {
    deserializes_to("[1,2]", vec![1, 2]);
}

#[test]
fn parses_nested_arrays() {
    deserializes_to("[1,[2,3]]", (1, vec![2, 3]));
}

#[test]
fn parses_nulls() {
    deserializes_to("null", ());
}

#[test]
fn parses_true() {
    deserializes_to("true", true);
}

#[test]
fn parses_false() {
    deserializes_to("false", false);
}

// numbers

#[test]
fn parses_leading_zeroes() {
    deserializes_to("[0,0,0e0]", vec![0, 0, 0]);
}

#[test]
fn parses_integers() {
    deserializes_to("[1,23,456,7890]", vec![1, 23, 456, 7890]);
}

#[test]
fn parses_signed_numbers() {
    deserializes_to("[-1,+2,-.1,-0]", vec![-1., 2., -0.1, -0.]);
}

#[test]
fn parses_leading_decimal_points() {
    deserializes_to("[.1,.23]", vec![0.1, 0.23]);
}

#[test]
fn parses_fractional_numbers() {
    deserializes_to("[1.0,1.23]", vec![1., 1.23]);
}

#[test]
fn parses_exponents() {
    deserializes_to(
        "[1e0,1e1,1e01,1.e0,1.1e0,1e-1,1e+1]",
        vec![1., 10., 10., 1., 1.1, 0.1, 10.],
    );
}

#[test]
fn parses_hexadecimal_numbers() {
    deserializes_to("[0x1,0x10,0xff,0xFF]", vec![1, 16, 255, 255]);
}

#[test]
fn parses_signed_and_unsiged_infinity() {
    deserializes_to(
        "[Infinity,-Infinity]",
        vec![f64::INFINITY, f64::NEG_INFINITY],
    );
}

#[test]
fn parses_signed_and_unsigned_nan() {
    deserializes_to_nan_f64("NaN");
    deserializes_to_nan_f64("-NaN");
}

// strings

#[test]
fn parses_double_quoted_strings() {
    deserializes_to("\"abc\"", "abc".to_owned());
}

#[test]
fn parses_single_quoted_strings() {
    deserializes_to("'abc'", "abc".to_owned());
}

#[test]
fn parses_nested_quotes_strings() {
    deserializes_to("['\"',\"'\"]", vec!["\"".to_owned(), "'".to_owned()]);
}

#[test]
fn parses_escaped_characters() {
    deserializes_to(
        "'\\b\\f\\n\\r\\t\\v\\0\\x0f\\u01fF\\\n\\\r\n\\\r\\\u{2028}\\\u{2029}\\a\\'\\\"'",
        "\u{0008}\u{000C}\n\r\t\u{000B}\0\x0f\u{01FF}a'\"".to_owned(),
    );
}

// comments

#[test]
fn parses_single_line_comments() {
    let m: HashMap<String, i32> = HashMap::new();
    deserializes_to("{//comment\n}", m);
}

#[test]
fn parses_single_line_comments_at_end_of_input() {
    let m: HashMap<String, i32> = HashMap::new();
    deserializes_to("{}//comment", m);
}

#[test]
fn parses_multi_line_comments() {
    let m: HashMap<String, i32> = HashMap::new();
    deserializes_to("{/*comment\n** */}", m);
}

#[test]
fn parses_whitespace() {
    let m: HashMap<String, i32> = HashMap::new();
    deserializes_to(
        "{\t\u{000B}\u{000C} \u{00A0}\u{FEFF}\n\r\u{2028}\u{2029}\u{2003}}",
        m,
    );
}
