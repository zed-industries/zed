/*!
JSON support for `sval`.

Values are serialized in a `serde`-compatible way.
*/

#![no_std]
#![deny(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;

mod value;
pub use self::value::*;

mod to_fmt;
pub use self::{error::*, to_fmt::*};

pub mod tags;

#[cfg(feature = "alloc")]
mod to_string;

#[cfg(feature = "alloc")]
pub use self::to_string::*;

#[cfg(feature = "std")]
mod to_io;

#[cfg(feature = "std")]
pub use self::to_io::*;

#[cfg(feature = "std")]
mod to_vec;

#[cfg(feature = "std")]
pub use self::to_vec::*;
