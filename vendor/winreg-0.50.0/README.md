winreg
[![Github Actions][actions-image]][actions]
[![Winreg on crates.io][cratesio-image]][cratesio]
[![Winreg on docs.rs][docsrs-image]][docsrs]
======

[actions-image]: https://github.com/gentoo90/winreg-rs/actions/workflows/ci.yaml/badge.svg
[actions]: https://github.com/gentoo90/winreg-rs/actions
[cratesio-image]: https://img.shields.io/crates/v/winreg.svg
[cratesio]: https://crates.io/crates/winreg
[docsrs-image]: https://docs.rs/winreg/badge.svg
[docsrs]: https://docs.rs/winreg

Rust bindings to MS Windows Registry API. Work in progress.

Current features:
* Basic registry operations:
    * open/create/delete keys
    * load application hive from a file
    * read and write values
    * seamless conversion between `REG_*` types and rust primitives
        * `String` and `OsString` <= `REG_SZ`, `REG_EXPAND_SZ` or `REG_MULTI_SZ`
        * `String`, `&str`, `OsString`, `&OsStr` => `REG_SZ`
        * `Vec<String>`, `Vec<OsString>` <= `REG_MULTI_SZ`
        * `Vec<String>`, `Vec<&str>`, `Vec<OsString>`, `Vec<&OsStr>` => `REG_MULTI_SZ`
        * `u32` <=> `REG_DWORD`
        * `u64` <=> `REG_QWORD`
* Iteration through key names and through values
* Transactions
* Transacted serialization of rust types into/from registry (only primitives, structures and maps for now)

## Usage

### Basic usage

```toml
# Cargo.toml
[dependencies]
winreg = "0.50"
```

```rust
use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    println!("Reading some system info...");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")?;
    let pf: String = cur_ver.get_value("ProgramFilesDir")?;
    let dp: String = cur_ver.get_value("DevicePath")?;
    println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);
    let info = cur_ver.query_info()?;
    println!("info = {:?}", info);
    let mt = info.get_last_write_time_system();
    println!(
        "last_write_time as windows_sys::Win32::Foundation::SYSTEMTIME = {}-{:02}-{:02} {:02}:{:02}:{:02}",
        mt.wYear, mt.wMonth, mt.wDay, mt.wHour, mt.wMinute, mt.wSecond
    );

    // enable `chrono` feature on `winreg` to make this work
    // println!(
    //     "last_write_time as chrono::NaiveDateTime = {}",
    //     info.get_last_write_time_chrono()
    // );

    println!("And now lets write something...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join("WinregRsExample1");
    let (key, disp) = hkcu.create_subkey(&path)?;

    match disp {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }

    key.set_value("TestSZ", &"written by Rust")?;
    let sz_val: String = key.get_value("TestSZ")?;
    key.delete_value("TestSZ")?;
    println!("TestSZ = {}", sz_val);

    key.set_value("TestMultiSZ", &vec!["written", "by", "Rust"])?;
    let multi_sz_val: Vec<String> = key.get_value("TestMultiSZ")?;
    key.delete_value("TestMultiSZ")?;
    println!("TestMultiSZ = {:?}", multi_sz_val);

    key.set_value("TestDWORD", &1234567890u32)?;
    let dword_val: u32 = key.get_value("TestDWORD")?;
    println!("TestDWORD = {}", dword_val);

    key.set_value("TestQWORD", &1234567891011121314u64)?;
    let qword_val: u64 = key.get_value("TestQWORD")?;
    println!("TestQWORD = {}", qword_val);

    key.create_subkey("sub\\key")?;
    hkcu.delete_subkey_all(&path)?;

    println!("Trying to open nonexistent key...");
    hkcu.open_subkey(&path).unwrap_or_else(|e| match e.kind() {
        io::ErrorKind::NotFound => panic!("Key doesn't exist"),
        io::ErrorKind::PermissionDenied => panic!("Access denied"),
        _ => panic!("{:?}", e),
    });
    Ok(())
}
```

### Iterators

```rust
use std::io;
use winreg::RegKey;
use winreg::enums::*;

fn main() -> io::Result<()> {
    println!("File extensions, registered in system:");
    for i in RegKey::predef(HKEY_CLASSES_ROOT)
        .enum_keys().map(|x| x.unwrap())
        .filter(|x| x.starts_with("."))
    {
        println!("{}", i);
    }

    let system = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("HARDWARE\\DESCRIPTION\\System")?;
    for (name, value) in system.enum_values().map(|x| x.unwrap()) {
        println!("{} = {:?}", name, value);
    }

    Ok(())
}
```

### Transactions

```toml
# Cargo.toml
[dependencies]
winreg = { version = "0.50", features = ["transactions"] }
```

```rust
use std::io;
use winreg::RegKey;
use winreg::enums::*;
use winreg::transaction::Transaction;

fn main() -> io::Result<()> {
    let t = Transaction::new()?;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _disp) = hkcu.create_subkey_transacted("Software\\RustTransaction", &t)?;
    key.set_value("TestQWORD", &1234567891011121314u64)?;
    key.set_value("TestDWORD", &1234567890u32)?;

    println!("Commit transaction? [y/N]:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input = input.trim_right().to_owned();
    if input == "y" || input == "Y" {
        t.commit()?;
        println!("Transaction committed.");
    }
    else {
        // this is optional, if transaction wasn't committed,
        // it will be rolled back on disposal
        t.rollback()?;

        println!("Transaction wasn't committed, it will be rolled back.");
    }

    Ok(())
}
```

### Serialization

```toml
# Cargo.toml
[dependencies]
winreg = { version = "0.50", features = ["serialization-serde"] }
serde = "1"
serde_derive = "1"
```

```rust
use serde_derive::{Deserialize, Serialize};
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
```
