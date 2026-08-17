use json5::{Error, Location};
use matches::assert_matches;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use std::f64;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Val {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Val>),
    Object(HashMap<String, Val>),
}

#[allow(unused)]
pub fn deserializes_to<'a, T>(s: &'a str, v: T)
where
    T: ::std::fmt::Debug + ::std::cmp::PartialEq + serde::de::Deserialize<'a>,
{
    assert_matches!(json5::from_str::<T>(s), Ok(value) if value == v);
}

#[allow(unused)]
pub fn deserializes_to_nan_f32<'a>(s: &'a str) {
    assert_matches!(json5::from_str::<f32>(s), Ok(value) if value.is_nan());
}

#[allow(unused)]
pub fn deserializes_to_nan_f64<'a>(s: &'a str) {
    assert_matches!(json5::from_str::<f64>(s), Ok(value) if value.is_nan());
}

#[allow(unused)]
pub fn deserializes_with_error<'a, T>(s: &'a str, error_expected: Error)
where
    T: ::std::fmt::Debug + ::std::cmp::PartialEq + serde::de::Deserialize<'a>,
{
    assert_matches!(json5::from_str::<T>(s), Err(e) if e == error_expected);
}

#[allow(unused)]
pub fn serializes_to<T>(v: T, s: &'static str)
where
    T: ::std::fmt::Debug + ::std::cmp::PartialEq + serde::ser::Serialize,
{
    assert_matches!(json5::to_string::<T>(&v), Ok(value) if value == s);
}

#[allow(unused)]
pub fn make_error(msg: impl Into<String>, line: usize, column: usize) -> Error {
    Error::Message {
        msg: msg.into(),
        location: Some(Location { line, column }),
    }
}
