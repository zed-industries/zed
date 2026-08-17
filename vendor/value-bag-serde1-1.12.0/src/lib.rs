/*!
Implementation detail for `value-bag`; it should not be depended on directly.
*/

#![no_std]

pub use erased_serde as erased;
pub use serde_core as lib;
pub use serde_fmt as fmt;

#[cfg(feature = "owned")]
pub use serde_buf as buf;

#[cfg(feature = "json")]
pub use serde_json as json;

#[cfg(feature = "test")]
pub use serde_test as test;
