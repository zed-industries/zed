// Copyright 2015, Igor Shaula
// Licensed under the MIT License <LICENSE or
// http://opensource.org/licenses/MIT>. This file
// may not be copied, modified, or distributed
// except according to those terms.
#[macro_use]
extern crate serde_derive;
extern crate winreg;
use std::collections::HashMap;
use std::error::Error;
use winreg::enums::*;

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Test {
    t_bool: bool,
    t_u8: u8,
    t_u16: u16,
    t_u32: u32,
    t_u64: u64,
    t_usize: usize,
    t_struct: Rectangle,
    t_map: HashMap<String, u32>,
    t_string: String,
    #[serde(rename = "")] // empty name becomes the (Default) value in the registry
    t_char: char,
    t_i8: i8,
    t_i16: i16,
    t_i32: i32,
    t_i64: i64,
    t_isize: isize,
    t_f64: f64,
    t_f32: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let (key, _disp) = hkcu.create_subkey("Software\\RustEncode")?;

    let mut map = HashMap::new();
    map.insert("".to_owned(), 0); // empty name becomes the (Default) value in the registry
    map.insert("v1".to_owned(), 1);
    map.insert("v2".to_owned(), 2);
    map.insert("v3".to_owned(), 3);

    let v1 = Test {
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
        t_string: "test 123!".to_owned(),
        t_char: 'a',
        t_i8: -123,
        t_i16: -2049,
        t_i32: 20100,
        t_i64: -12_345_678_910,
        t_isize: -1_234_567_890,
        t_f64: -0.01,
        t_f32: 3.15,
    };

    key.encode(&v1)?;

    let v2: Test = key.decode()?;
    println!("Decoded {:?}", v2);

    println!("Equal to encoded: {:?}", v1 == v2);
    Ok(())
}
