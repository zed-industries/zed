use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

mod common;

use crate::common::deserializes_to;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
struct Example {
    unquoted: String,
    single_quotes: String,
    line_breaks: String,
    hexadecimal: u32,
    leading_decimal_point: f64,
    and_trailing: f64,
    positive_sign: i32,
    trailing_comma: String,
    and_in: Vec<String>,
    backwards_compatible: String,
}

#[test]
fn serializes_example_from_json5_dot_org() {
    let mut contents = String::new();
    File::open("tests/assets/json5_dot_org_example.json5")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let expected = Example {
        unquoted: "and you can quote me on that".to_owned(),
        single_quotes: "I can use \"double quotes\" here".to_owned(),
        line_breaks: "Look, Mom! No \\n's!".to_owned(),
        hexadecimal: 0xdecaf,
        leading_decimal_point: 0.8675309,
        and_trailing: 8675309.0,
        positive_sign: 1,
        trailing_comma: "in objects".to_owned(),
        and_in: vec!["arrays".to_owned()],
        backwards_compatible: "with JSON".to_owned(),
    };

    deserializes_to(&contents, expected)
}
