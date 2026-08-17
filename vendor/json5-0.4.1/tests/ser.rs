use serde_derive::Serialize;

use std::collections::HashMap;

mod common;

use crate::common::serializes_to;

#[test]
fn serializes_bool() {
    serializes_to(true, "true");
    serializes_to(false, "false");
}

#[test]
fn serializes_i8() {
    let x: i8 = 42;
    serializes_to(x, "42");
}

#[test]
fn serializes_u8() {
    let x: u8 = 42;
    serializes_to(x, "42");
}

#[test]
fn serializes_i16() {
    let x: i16 = 42;
    serializes_to(x, "42");
}

#[test]
fn serializes_u16() {
    let x: u16 = 42;
    serializes_to(x, "42");
}

#[test]
fn serializes_i32() {
    let x: i32 = 42;
    serializes_to(x, "42");
}

#[test]
fn serializes_u32() {
    let x: u32 = 42;
    serializes_to(x, "42");
}

#[test]
fn serializes_i64() {
    let x: i64 = 42;
    serializes_to(x, "42");
}

#[test]
fn serializes_u64() {
    let x: u64 = 42;
    serializes_to(x, "42");
}

#[test]
fn serializes_f32() {
    let x: f32 = 42.42;
    serializes_to(x, "42.42");

    serializes_to(std::f32::INFINITY, "Infinity");
    serializes_to(std::f32::NEG_INFINITY, "-Infinity");
    serializes_to(std::f32::NAN, "NaN");
}

#[test]
fn serializes_f64() {
    let x: f64 = 42.42;
    serializes_to(x, "42.42");

    serializes_to(std::f64::INFINITY, "Infinity");
    serializes_to(std::f64::NEG_INFINITY, "-Infinity");
    serializes_to(std::f64::NAN, "NaN");
}

#[test]
fn serializes_char() {
    serializes_to('x', "\"x\"");
    serializes_to('자', "\"자\"");
    serializes_to('"', r#""\"""#);
    serializes_to('\r', r#""\r""#);
    serializes_to('\n', r#""\n""#);
    serializes_to('\t', r#""\t""#);
    serializes_to('\\', r#""\\""#);
    serializes_to('/', r#""\/""#);
    serializes_to('\u{0008}', r#""\b""#);
    serializes_to('\u{000c}', r#""\f""#);
}

#[test]
fn serializes_str() {
    serializes_to("Hello!", "\"Hello!\"");
    serializes_to("안녕하세요", "\"안녕하세요\"");
    serializes_to("\"quotes!\"", "\"\\\"quotes!\\\"\"");
    serializes_to("new\nlines", "\"new\\nlines\"");
    serializes_to("\\", "\"\\\\\"");
}

#[test]
fn serializes_string() {
    serializes_to("Hello!".to_owned(), "\"Hello!\"");
    serializes_to("안녕하세요".to_owned(), "\"안녕하세요\"");
    serializes_to("\"quotes!\"".to_owned(), "\"\\\"quotes!\\\"\"");
    serializes_to("new\nlines".to_owned(), "\"new\\nlines\"");
    serializes_to("\\".to_owned(), "\"\\\\\"");
}

#[test]
#[ignore] // TODO currently unsupported
fn serializes_bytes() {}

#[test]
#[ignore] // TODO currently unsupported
fn serializes_byte_buf() {}

#[test]
fn serializes_option() {
    serializes_to::<Option<i32>>(None, "null");
    serializes_to(Some(42), "42");
    serializes_to(Some(Some(42)), "42");
}

#[test]
fn serializes_unit() {
    serializes_to((), "null");
}

#[test]
fn serializes_unit_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct A;
    serializes_to(A, "null");
}

#[test]
fn serializes_newtype_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct A(i32);

    #[derive(Serialize, PartialEq, Debug)]
    struct B(f64);

    serializes_to(A(42), "42");
    serializes_to(B(42.), "42");
}

#[test]
fn serializes_seq() {
    #[derive(Serialize, PartialEq, Debug)]
    #[serde(untagged)]
    enum Val {
        Number(f64),
        Bool(bool),
        String(String),
    }

    serializes_to(vec![1, 2, 3], "[1,2,3]");
    serializes_to(
        vec![
            Val::Number(42.),
            Val::Bool(true),
            Val::String("hello".to_owned()),
        ],
        "[42,true,\"hello\"]",
    )
}

#[test]
fn serializes_tuple() {
    serializes_to((1, 2, 3), "[1,2,3]");
}

#[test]
fn serializes_tuple_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct A(i32, f64);

    #[derive(Serialize, PartialEq, Debug)]
    struct B(f64, i32);

    serializes_to(A(1, 2.), "[1,2]");
    serializes_to(B(1., 2), "[1,2]");
}

#[test]
fn serializes_map() {
    let mut inner = HashMap::new();
    inner.insert("b".to_owned(), true);

    let mut outer = HashMap::new();
    outer.insert("a".to_owned(), inner);

    serializes_to(outer, "{\"a\":{\"b\":true}}");
}

#[test]
fn serializes_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct S {
        a: i32,
        b: i32,
        c: i32,
    }

    serializes_to(S { a: 1, b: 2, c: 3 }, "{\"a\":1,\"b\":2,\"c\":3}");
}

#[test]
fn serializes_enum() {
    #[derive(Serialize, PartialEq, Debug)]
    enum E {
        A,
        B(i32),
        C(i32, i32),
        D { a: i32, b: i32 },
    }

    serializes_to(E::A, "\"A\"");
    serializes_to(E::B(2), "{\"B\":2}");
    serializes_to(E::C(3, 5), "{\"C\":[3,5]}");
    serializes_to(E::D { a: 7, b: 11 }, "{\"D\":{\"a\":7,\"b\":11}}");
}
