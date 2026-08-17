//! Interface for building object files.
//!
//! This module provides common types and traits used in the builders.
//!
//! The submodules define the builders for each file format.

mod error;
pub use error::{Error, Result};

mod bytes;
pub use bytes::{ByteString, Bytes};

mod table;
use table::IdPrivate;
pub use table::{Id, Item, Table};

#[cfg(feature = "elf")]
pub mod elf;
