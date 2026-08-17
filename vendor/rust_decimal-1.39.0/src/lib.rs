#![doc = include_str!(concat!(env!("OUT_DIR"), "/README-lib.md"))]
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
extern crate alloc;

mod constants;
mod decimal;
mod error;
mod ops;
pub mod str;

// We purposely place this here for documentation ordering
mod arithmetic_impls;

#[cfg(feature = "rust-fuzz")]
mod fuzz;
#[cfg(feature = "maths")]
mod maths;
#[cfg(feature = "db-diesel-mysql")]
mod mysql;
#[cfg(any(
    feature = "db-tokio-postgres",
    feature = "db-postgres",
    feature = "db-diesel-postgres",
))]
mod postgres;
#[cfg(feature = "proptest")]
mod proptest;
#[cfg(feature = "rand")]
mod rand;
#[cfg(feature = "rand-0_9")]
mod rand_0_9;
#[cfg(feature = "rocket-traits")]
mod rocket;
#[cfg(all(
    feature = "serde",
    not(any(
        feature = "serde-with-str",
        feature = "serde-with-float",
        feature = "serde-with-arbitrary-precision"
    ))
))]
mod serde;
/// Serde specific functionality to customize how a decimal is serialized/deserialized (`serde_with`)
#[cfg(all(
    feature = "serde",
    any(
        feature = "serde-with-str",
        feature = "serde-with-float",
        feature = "serde-with-arbitrary-precision"
    )
))]
pub mod serde;

pub use decimal::{Decimal, RoundingStrategy};
pub use error::Error;
#[cfg(feature = "maths")]
pub use maths::MathematicalOps;

/// A convenience module appropriate for glob imports (`use rust_decimal::prelude::*;`).
pub mod prelude {
    #[cfg(feature = "macros")]
    pub use super::dec;
    #[cfg(feature = "maths")]
    pub use crate::maths::MathematicalOps;
    pub use crate::{Decimal, RoundingStrategy};
    pub use core::str::FromStr;
    pub use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
}

#[cfg(feature = "macros")]
pub use rust_decimal_macros::dec;

#[cfg(feature = "diesel")]
extern crate diesel;

/// Shortcut for `core::result::Result<T, rust_decimal::Error>`. Useful to distinguish
/// between `rust_decimal` and `std` types.
pub type Result<T> = core::result::Result<T, Error>;

// #[cfg(feature = "legacy-ops")]
// compiler_error!("legacy-ops has been removed as 1.x");
