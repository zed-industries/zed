#![doc(
    html_logo_url = "https://bon-rs.com/bon-logo-thumb.png",
    html_favicon_url = "https://bon-rs.com/bon-logo-medium.png"
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// We mark all items from the `private` module as deprecated to signal that they are
// implementation details and should not be used directly. Unfortunately, this triggers
// the deprecation warnings within this crate itself everywhere we use them, so we just
// suppress this lint for the entire crate.
#![allow(deprecated)]

// Rexport all macros from the proc-macro crate.
pub use bon_macros::{bon, builder, map, set, Builder};

/// Small utility declarative macros for creating collections with [`Into`] conversions.
mod collections;

#[doc(hidden)]
#[deprecated = "the items from the `bon::__` module are an implementation detail; \
    they should not be used directly; if you found a need for this, then you are probably \
    doing something wrong; feel free to open an issue/discussion in our GitHub repository \
    (https://github.com/elastio/bon) or ask for help in our Discord server \
    (https://bon-rs.com/discord)"]
pub mod __;

mod builder_state;
