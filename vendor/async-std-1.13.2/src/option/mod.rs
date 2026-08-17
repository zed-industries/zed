//! The Rust core optional value type
//!
//! This module provides the `Option<T>` type for returning and
//! propagating optional values.

mod from_stream;

#[allow(unused)]
#[doc(inline)]
pub use std::option::Option;

cfg_unstable! {
    mod product;
    mod sum;
}
