#![allow(clippy::bool_assert_comparison)]

//! Usage examples of the version-compare crate.
//!
//! This shows various ways this library provides for comparing version numbers.
//! The `assert_eq!(...)` macros are used to assert and show the expected output.
//!
//! Run this example by invoking `cargo run --example example`.

use version_compare::{compare, compare_to, Cmp, Version};

fn main() {
    let a = "1.2";
    let b = "1.5.1";

    // The following comparison operators are used:
    // - Cmp::Eq -> Equal
    // - Cmp::Ne -> Not equal
    // - Cmp::Lt -> Less than
    // - Cmp::Le -> Less than or equal
    // - Cmp::Ge -> Greater than or equal
    // - Cmp::Gt -> Greater than

    // Easily compare version strings
    assert_eq!(compare(a, b), Ok(Cmp::Lt));
    assert_eq!(compare_to(a, b, Cmp::Le), Ok(true));
    assert_eq!(compare_to(a, b, Cmp::Gt), Ok(false));

    // Parse and wrap version strings as a Version
    let a = Version::from(a).unwrap();
    let b = Version::from(b).unwrap();

    // The Version can easily be compared with
    assert_eq!(a < b, true);
    assert_eq!(a <= b, true);
    assert_eq!(a > b, false);
    assert_eq!(a != b, true);
    assert_eq!(a.compare(&b), Cmp::Lt);
    assert_eq!(a.compare_to(&b, Cmp::Lt), true);

    // Or match the comparison operators
    match a.compare(b) {
        Cmp::Lt => println!("Version a is less than b"),
        Cmp::Eq => println!("Version a is equal to b"),
        Cmp::Gt => println!("Version a is greater than b"),
        _ => unreachable!(),
    }
}
