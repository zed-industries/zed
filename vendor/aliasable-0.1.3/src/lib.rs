//! Basic aliasable (non `core::ptr::Unique`) types.
//!
//! # Why?
//!
//! Used for escaping `noalias` when multiple raw pointers may point to the same
//! data.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(
    clippy::pedantic,
    rust_2018_idioms,
    anonymous_parameters,
    unused_qualifications,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    warnings
)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::wrong_self_convention,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

mod mut_ref;

#[cfg(feature = "alloc")]
pub mod boxed;
#[cfg(feature = "alloc")]
pub mod string;
#[cfg(feature = "alloc")]
pub mod vec;

pub use crate::mut_ref::AliasableMut;

/// Export of all types enabled.
pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use crate::boxed::*;
    #[cfg(feature = "alloc")]
    pub use crate::string::*;
    #[cfg(feature = "alloc")]
    pub use crate::vec::*;

    pub use crate::mut_ref::*;
}

#[cfg(feature = "traits")]
pub use aliasable_deref_trait::AliasableDeref;
#[cfg(feature = "traits")]
pub use stable_deref_trait::StableDeref;
