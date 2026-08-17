//! The Rust core string library
//!
//! This library provides a UTF-8 encoded, growable string.

mod extend;
mod from_stream;

#[allow(unused)]
#[doc(inline)]
pub use std::string::String;
