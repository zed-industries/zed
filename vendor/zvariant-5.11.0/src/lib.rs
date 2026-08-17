#![allow(clippy::unusual_byte_groupings)]
#![deny(rust_2018_idioms)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/z-galaxy/zbus/9f7a90d2b594ddc48b7a5f39fda5e00cd56a7dfb/logo.png"
)]
#![doc = include_str!("../README.md")]
#![doc(test(attr(
    warn(unused),
    deny(warnings),
    allow(dead_code),
    // W/o this, we seem to get some bogus warning about `extern crate zbus`.
    allow(unused_extern_crates),
)))]
#![cfg_attr(test, recursion_limit = "256")]

#[macro_use]
mod utils;
pub use utils::*;

mod array;
pub use array::*;

mod basic;
pub use basic::*;

mod dict;
pub use dict::*;

pub mod serialized;

#[cfg(unix)]
mod fd;
#[cfg(unix)]
pub use fd::*;

mod object_path;
pub use crate::object_path::*;

mod file_path;
pub use crate::file_path::*;

mod ser;
pub use ser::*;

mod de;

pub mod dbus;
#[cfg(feature = "gvariant")]
pub mod gvariant;

pub mod signature;
pub use signature::Signature;

mod str;
pub use crate::str::*;

mod structure;
pub use crate::structure::*;

#[cfg(feature = "gvariant")]
mod maybe;
#[cfg(feature = "gvariant")]
pub use crate::maybe::*;

mod optional;
pub use crate::optional::*;

mod value;
pub use value::*;

mod error;
pub use error::*;

#[macro_use]
mod r#type;
pub use r#type::*;

mod tuple;
pub use tuple::*;

mod from_value;

mod into_value;

mod owned_value;
pub use owned_value::*;

#[cfg(feature = "gvariant")]
mod framing_offset_size;
#[cfg(feature = "gvariant")]
mod framing_offsets;

mod container_depths;

pub mod as_value;
#[deprecated(since = "5.5.0", note = "Use `as_value::Deserialize` instead.")]
pub use as_value::Deserialize as DeserializeValue;
#[deprecated(since = "5.5.0", note = "Use `as_value::Serialize` instead.")]
pub use as_value::Serialize as SerializeValue;

pub use zvariant_derive::{DeserializeDict, OwnedValue, SerializeDict, Type, Value, signature};

// Required for the macros to function within this crate.
extern crate self as zvariant;

// Macro support module, not part of the public API.
#[doc(hidden)]
pub mod export {
    pub use serde;
}

// Re-export all of the `endi` API for ease of use.
pub use endi::*;
