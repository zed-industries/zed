#![allow(
    // We place `#[inline(always)]` only on very small methods where we'd event want
    // a guarantee of them being inlined.
    clippy::inline_always,

    // Marking every potential function as `const` is a bit too much.
    // Especially, this doesn't play well with our MSRV. Trait bounds
    // aren't allowed on const functions in older Rust versions.
    clippy::missing_const_for_fn,
)]

/// Used for providing better IDE hints (completions and syntax highlighting).
pub mod ide;

pub mod better_errors;

mod cfg_eval;

// This reexport is a private implementation detail and should not be used
// directly! This reexport may change or be removed at any time between
// patch releases. Use the export from your generated  builder's state module
// directly instead of using this reexport from `bon::__`.
pub use crate::builder_state::{IsSet, IsUnset};
pub use bon_macros::__privatize;
pub use rustversion;

pub(crate) mod sealed {
    // The purpose of the `Sealed` trait **is** to be unnameable from outside the crate.
    #[allow(unnameable_types)]
    pub trait Sealed: Sized {}

    impl<Name> Sealed for super::Unset<Name> {}
    impl<Name> Sealed for super::Set<Name> {}
}

pub(crate) use sealed::Sealed;

/// Used to implement the `alloc` feature.
#[cfg(feature = "alloc")]
pub extern crate alloc;

#[derive(Debug)]
pub struct Unset<Name>(Name);

#[derive(Debug)]
pub struct Set<Name>(Name);
