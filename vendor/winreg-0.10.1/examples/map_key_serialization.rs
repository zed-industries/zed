// Copyright 2020, Igor Shaula
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

fn main() -> Result<(), Box<dyn Error>> {
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let (key, _disp) = hkcu.create_subkey("Software\\RustEncodeMapKey")?;
    let mut v1 = HashMap::new();
    v1.insert(
        "first".to_owned(),
        Rectangle {
            coords: Coords { x: 55, y: 77 },
            size: Size { w: 500, h: 300 },
        },
    );
    v1.insert(
        "second".to_owned(),
        Rectangle {
            coords: Coords { x: 11, y: 22 },
            size: Size { w: 1000, h: 600 },
        },
    );

    key.encode(&v1)?;

    let v2: HashMap<String, Rectangle> = key.decode()?;
    println!("Decoded {:?}", v2);

    println!("Equal to encoded: {:?}", v1 == v2);
    Ok(())
}
