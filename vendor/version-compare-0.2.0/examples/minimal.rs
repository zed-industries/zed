//! A minimal usage example of the version-compare crate.
//!
//! This compares two given version number strings, and outputs which is greater.
//!
//! Run this example by invoking `cargo run --example minimal`.

use version_compare::{compare, Cmp};

fn main() {
    let a = "1.3";
    let b = "1.2.4";

    match compare(a, b) {
        Ok(Cmp::Lt) => println!("Version a is less than b"),
        Ok(Cmp::Eq) => println!("Version a is equal to b"),
        Ok(Cmp::Gt) => println!("Version a is greater than b"),
        _ => panic!("Invalid version number"),
    }
}
