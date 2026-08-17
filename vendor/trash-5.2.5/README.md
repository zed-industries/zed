
[![Crates.io](https://img.shields.io/crates/v/trash?color=mediumvioletred)](https://crates.io/crates/trash)
[![Docs.rs](https://docs.rs/trash/badge.svg)](https://docs.rs/trash)
[![CI](https://github.com/Byron/trash-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Byron/trash-rs/actions/workflows/rust.yml)


## About

The `trash` is a Rust library for moving files and folders to the operating system's Recycle Bin or Trash or Rubbish Bin or what have you :D

The library supports Windows, macOS, and all FreeDesktop Trash compliant environments (including GNOME, KDE, XFCE, and more). 
See more about the FreeDesktop Trash implementation in the `freedesktop.rs` file.

## Usage

```toml
# In Cargo.toml
[dependencies]
trash = "3"
```

```rust
// In main.rs
use std::fs::File;
use trash;

fn main() {
    // Let's create and remove a single file
    File::create_new("remove-me").unwrap();
    trash::delete("remove-me").unwrap();
    assert!(File::open("remove-me").is_err());

    // Now let's remove multiple files at once
    let the_others = ["remove-me-too", "dont-forget-about-me-either"];
    for name in the_others.iter() {
        File::create_new(name).unwrap();
    }
    trash::delete_all(&the_others).unwrap();
    for name in the_others.iter() {
        assert!(File::open(name).is_err());
    }
}
```
