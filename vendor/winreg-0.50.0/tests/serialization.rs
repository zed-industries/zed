// Copyright 2023, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
#![cfg(feature = "serialization-serde")]
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

mod common;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Coords {
    x: u32,
    y: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Size {
    w: u32,
    h: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Rectangle {
    coords: Coords,
    size: Size,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AllFields {
    t_bool: bool,
    t_u8: u8,
    t_u16: u16,
    t_u32: u32,
    t_u64: u64,
    t_usize: usize,
    t_struct: Rectangle,
    t_string: String,
    t_map: HashMap<String, HashMap<String, u32>>,
    t_i8: i8,
    t_i16: i16,
    t_i32: i32,
    t_i64: i64,
    t_isize: isize,
    t_f64: f64,
    t_f32: f32,
    t_char: char,
    #[serde(with = "serde_bytes")]
    t_bytes: Vec<u8>,
}

impl AllFields {
    pub fn test_val() -> Self {
        let mut k1 = HashMap::new();
        k1.insert("val1".to_owned(), 32);
        k1.insert("val2".to_owned(), 64);
        k1.insert("val3".to_owned(), 128);

        let mut k2 = HashMap::new();
        k2.insert("val1".to_owned(), 256);
        k2.insert("val2".to_owned(), 512);
        k2.insert("val3".to_owned(), 1024);

        let mut map = HashMap::new();
        map.insert("key1".to_owned(), k1);
        map.insert("key2".to_owned(), k2);

        AllFields {
            t_bool: false,
            t_u8: 127,
            t_u16: 32768,
            t_u32: 123_456_789,
            t_u64: 123_456_789_101_112,
            t_usize: 1_234_567_891,
            t_struct: Rectangle {
                coords: Coords { x: 55, y: 77 },
                size: Size { w: 500, h: 300 },
            },
            t_map: map,
            t_string: "Test123 \n$%^&|+-*/\\()".to_owned(),
            t_i8: -123,
            t_i16: -2049,
            t_i32: 20100,
            t_i64: -12_345_678_910,
            t_isize: -1_234_567_890,
            t_f64: -0.01,
            t_f32: 3.15,
            t_char: 'a',
            t_bytes: vec![0xDE, 0xAD, 0xBE, 0xEF],
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SomeFields {
    t_usize: usize,
    t_struct: Rectangle,
    t_string: String,
    t_u32: Option<u32>,
    t_none: Option<u32>,
}

impl PartialEq<AllFields> for SomeFields {
    fn eq(&self, other: &AllFields) -> bool {
        *self.t_string == other.t_string
            && self.t_usize == other.t_usize
            && self.t_struct == other.t_struct
            && self.t_u32 == Some(other.t_u32)
            && self.t_none.is_none()
    }
}

#[test]
fn test_serialization_some() {
    let v1 = AllFields::test_val();

    with_key!(key, "SerializationSome" => {
        key.encode(&v1).unwrap();
        let v2: SomeFields = key.decode().unwrap();
        assert_eq!(v2, v1);
    });
}

#[test]
fn test_serialization_all() {
    let v1 = AllFields::test_val();

    with_key!(key, "SerializationAll" => {
        key.encode(&v1).unwrap();
        let v2: AllFields = key.decode().unwrap();
        assert_eq!(v2, v1);
    });
}
