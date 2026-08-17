//! Rust module prelude for Wasmtime crates.
//!
//! Wasmtime crates that use `no_std` use `core::prelude::*` by default which
//! does not include `alloc`-related functionality such as `String` and `Vec`.
//! To have similar ergonomics to `std` and additionally group up some common
//! functionality this module is intended to be imported at the top of all
//! modules with:
//!
//! ```rust,ignore
//! use crate::*;
//! ```
//!
//! Externally for crates that depend on `wasmtime-environ` they should have this
//! in the root of the crate:
//!
//! ```rust,ignore
//! use wasmtime_environ::prelude;
//! ```
//!
//! and then `use crate::*` works as usual.

pub use alloc::borrow::ToOwned;
pub use alloc::boxed::Box;
pub use alloc::format;
pub use alloc::string::{String, ToString};
pub use alloc::vec;
pub use alloc::vec::Vec;
pub use wasmparser::collections::{IndexMap, IndexSet};
