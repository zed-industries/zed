//! The Rust hash set, implemented as a `HashMap` where the value is `()`.

mod extend;
mod from_stream;

#[doc(inline)]
pub use std::collections::HashSet;
