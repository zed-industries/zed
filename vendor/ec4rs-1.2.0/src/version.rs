//! Information about the version of the EditorConfig specification this library complies with.
//!
//! The constants in this module specify the latest version of EditorConfig that ec4rs
//! is known to be compliant with.
//! Compliance is determined by running the `ec4rs_parse` tool
//! against the same core test suite used by the reference implementation of EditorConfig.
#![allow(missing_docs)]

pub static STRING: &str = "0.17.2";
pub static MAJOR: usize = 0;
pub static MINOR: usize = 17;
pub static PATCH: usize = 2;
