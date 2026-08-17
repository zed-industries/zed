winreg
[![Winreg on Appveyor][appveyor-image]][appveyor]
[![Winreg on crates.io][cratesio-image]][cratesio]
[![Winreg on docs.rs][docsrs-image]][docsrs]
======

[appveyor-image]: https://ci.appveyor.com/api/projects/status/f3lwrt67ghrf5omd?svg=true
[appveyor]: https://ci.appveyor.com/project/gentoo90/winreg-rs
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
winreg = "0.10"
```

```rust
extern crate winreg;
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
        "last_write_time as winapi::um::minwinbase::SYSTEMTIME = {}-{:02}-{:02} {:02}:{:02}:{:02}",
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
extern crate winreg;
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
winreg = { version = "0.10", features = ["transactions"] }
```

```rust
extern crate winreg;
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
winreg = { version = "0.10", features = ["serialization-serde"] }
serde = "1"
serde_derive = "1"
```

```rust
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
```

## Changelog

### 0.10.1

* Bump minimal required version of `winapi` to `0.3.9` (required for `load_app_key`)
* Reexport `REG_PROCESS_APPKEY` and use it in the `load_app_key` example

### 0.10.0

* Add `RegKey::load_app_key()` and `RegKey::load_app_key_with_flags()` ([#30](https://github.com/gentoo90/winreg-rs/issues/30))
* Update dev dependency `rand` to `0.8`
* Add Github actions
* Fix some clippy warnings

### 0.9.0

* Breaking change: `OsStr` and `OsString` registry values are not `NULL`-terminated any more ([#34](https://github.com/gentoo90/winreg-rs/issues/34), [#42](https://github.com/gentoo90/winreg-rs/issues/42))
* Refactoring: use macros for `ToRegValue` impls and tests for string values
* Fix `bare_trait_objects` warning in the doctests
* Add `impl ToRegValue for OsString`
* Add conversion between `REG_MULTI_SZ` and vectors of strings ([#16](https://github.com/gentoo90/winreg-rs/issues/16))
* Fix: set minimal `winapi` version to 0.3.7 (earlier versions don't have `impl-default` and `impl-debug` features which we use)
* Appveyor now checks the crate against `rust-1.31.1` too

### 0.8.0

* Implement serialization of `char` and maps
* Implement `std::fmt::Display` for `RegValue`
* Make `RegKey::{predef,raw_handle,enum_keys,enum_values}` functions `const`
* Give a better error message when compiling on platforms other than Windows ([#38](https://github.com/gentoo90/winreg-rs/pull/38))
* Tests are moved from `src/lib.rs` to `tests/reg_key.rs`

### 0.7.0

* Breaking change: remove deprecated `Error::description` ([#28](https://github.com/gentoo90/winreg-rs/pull/28))
* Optimize `Iterator::nth()` for the `Enum*` iterators ([#29](https://github.com/gentoo90/winreg-rs/pull/29))

### 0.6.2

* Add `RegKey::delete_subkey_with_flags()` ([#27](https://github.com/gentoo90/winreg-rs/pull/27))

### 0.6.1

* Add `last_write_time` field to `RegKeyMetadata` (returned by `RegKey::query_info()`) ([#25](https://github.com/gentoo90/winreg-rs/pull/25)).
* Add `get_last_write_time_system()` and `get_last_write_time_chrono()` (under `chrono` feature) methods to `RegKeyMetadata`.

### 0.6.0

* Breaking change: `create_subkey`, `create_subkey_with_flags`, `create_subkey_transacted` and
`create_subkey_transacted_with_flags` now return a tuple which contains the subkey and its disposition
which can be `REG_CREATED_NEW_KEY` or `REG_OPENED_EXISTING_KEY` ([#21](https://github.com/gentoo90/winreg-rs/issues/21)).
* Examples fixed to not use `unwrap` according to [Rust API guidelines](https://rust-lang-nursery.github.io/api-guidelines/documentation.html#examples-use--not-try-not-unwrap-c-question-mark).

### 0.5.1

* Reexport `HKEY` ([#15](https://github.com/gentoo90/winreg-rs/issues/15)).
* Add `raw_handle` method ([#18](https://github.com/gentoo90/winreg-rs/pull/18)).

### 0.5.0

* Breaking change: `open_subkey` now opens a key with readonly permissions.
Use `create_subkey` or `open_subkey_with_flags` to open with read-write permissins.
* Breaking change: features `transactions` and `serialization-serde` are now disabled by default.
* Breaking change: serialization now uses `serde` instead of `rustc-serialize`.
* `winapi` updated to `0.3`.
* Documentation fixes ([#14](https://github.com/gentoo90/winreg-rs/pull/14))

### 0.4.0

* Make transactions and serialization otional features
* Update dependensies + minor fixes ([#12](https://github.com/gentoo90/winreg-rs/pull/12))

### 0.3.5

* Implement `FromRegValue` for `OsString` and `ToRegValue` for `OsStr` ([#8](https://github.com/gentoo90/winreg-rs/issues/8))
* Minor fixes

### 0.3.4

* Add `copy_tree` method to `RegKey`
* Now checked with [rust-clippy](https://github.com/Manishearth/rust-clippy)
    * no more `unwrap`s
    * replaced `to_string` with `to_owned`
* Fix: reading strings longer than 2048 characters ([#6](https://github.com/gentoo90/winreg-rs/pull/6))

### 0.3.3

* Fix: now able to read values longer than 2048 bytes ([#3](https://github.com/gentoo90/winreg-rs/pull/3))

### 0.3.2

* Fix: `FromRegValue` trait now requires `Sized` (fixes build with rust 1.4)

### 0.3.1

* Fix: bump `winapi` version to fix build

### 0.3.0

* Add transactions support and make serialization transacted
* Breaking change: use `std::io::{Error,Result}` instead of own `RegError` and `RegResult`
