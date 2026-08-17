/*!
Implementation detail for `value-bag`; it should not be depended on directly.
*/

#![no_std]

pub use sval as lib;
pub use sval_buffer as buffer;
pub use sval_dynamic as dynamic;
pub use sval_fmt as fmt;
pub use sval_ref as lib_ref;

#[cfg(feature = "serde1")]
pub use sval_serde as serde1;

#[cfg(feature = "json")]
pub use sval_json as json;

#[cfg(feature = "test")]
pub use sval_test as test;
