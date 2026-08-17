//! The Rust hash map, implemented with quadratic probing and SIMD lookup.

mod extend;
mod from_stream;

#[doc(inline)]
pub use std::collections::HashMap;
