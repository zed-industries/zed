//! Rust library to easily compare version numbers with no specific format, and test against various comparison operators.
//!
//! Comparing version numbers is hard, especially with weird version number formats.
//!
//! This library helps you to easily compare any kind of version number with no
//! specific format using a best-effort approach.
//! Two version numbers can be compared to each other to get a comparison operator
//! (`<`, `==`, `>`), or test them against a comparison operator.
//!
//! Along with version comparison, the library provides various other tools for
//! working with version numbers.
//!
//! Inspired by PHPs [version_compare()](http://php.net/manual/en/function.version-compare.php).
//!
//! ### Formats
//!
//! Version numbers that would parse successfully include:  
//! `1`, `3.10.4.1`, `1.2.alpha`, `1.2.dev.4`, ` `, ` .   -32 . 1`, `MyApp 3.2.0 / build 0932` ...
//!
//! See a list of how version numbers compare [here](https://github.com/timvisee/version-compare/blob/411ed7135741ed7cf2fcf4919012fb5412dc122b/src/test.rs#L50-L103).
//!
//! ## Examples
//!
//! [example.rs](examples/example.rs):
//! ```rust
//! use version_compare::{compare, compare_to, Cmp, Version};
//!
//! let a = "1.2";
//! let b = "1.5.1";
//!
//! // The following comparison operators are used:
//! // - Cmp::Eq -> Equal
//! // - Cmp::Ne -> Not equal
//! // - Cmp::Lt -> Less than
//! // - Cmp::Le -> Less than or equal
//! // - Cmp::Ge -> Greater than or equal
//! // - Cmp::Gt -> Greater than
//!
//! // Easily compare version strings
//! assert_eq!(compare(a, b), Ok(Cmp::Lt));
//! assert_eq!(compare_to(a, b, Cmp::Le), Ok(true));
//! assert_eq!(compare_to(a, b, Cmp::Gt), Ok(false));
//!
//! // Parse and wrap version strings as a Version
//! let a = Version::from(a).unwrap();
//! let b = Version::from(b).unwrap();
//!
//! // The Version can easily be compared with
//! assert_eq!(a < b, true);
//! assert_eq!(a <= b, true);
//! assert_eq!(a > b, false);
//! assert_eq!(a != b, true);
//! assert_eq!(a.compare(&b), Cmp::Lt);
//! assert_eq!(a.compare_to(&b, Cmp::Lt), true);
//!
//! // Or match the comparison operators
//! match a.compare(b) {
//!     Cmp::Lt => println!("Version a is less than b"),
//!     Cmp::Eq => println!("Version a is equal to b"),
//!     Cmp::Gt => println!("Version a is greater than b"),
//!     _ => unreachable!(),
//! }
//! ```
//!
//! See the [`examples`](https://github.com/timvisee/version-compare/tree/master/examples) directory for more.
//!
//! ## Features
//!
//! * Compare version numbers, get: `<`, `==`, `>`
//! * Compare against a comparison operator
//!   (`<`, `<=`, `==`, `!=`, `>=`, `>`)
//! * Parse complex and unspecified formats
//! * Static, standalone methods to easily compare version strings in a single line
//!   of code
//!
//! ### Semver
//!
//! Version numbers using the [semver](http://semver.org/) format are compared
//! correctly with no additional configuration.
//!
//! If your version number strings follow this exact format you may be better off
//! using the [`semver`](https://crates.io/crates/semver) crate for more format
//! specific features.
//!
//! If that isn't certain however, `version-compare` makes comparing a breeze.
//!
//! _[View complete README](https://github.com/timvisee/version-compare/blob/master/README.md)_

mod cmp;
mod compare;
mod manifest;
mod part;
mod version;

#[cfg(test)]
mod test;

// Re-exports
pub use crate::cmp::Cmp;
pub use crate::compare::{compare, compare_to};
pub use crate::manifest::Manifest;
pub use crate::part::Part;
pub use crate::version::Version;
