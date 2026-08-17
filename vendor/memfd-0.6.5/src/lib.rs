//! A pure-Rust library to work with Linux memfd and seals.
//!
//! It provides support for creating `memfd` objects on Linux
//! and handling seals on them. This was first introduced in
//! Linux kernel 3.17.
//! For further details, see `memfd_create(2)` manpage.
//!
//! ```rust
//! use memfd;
//!
//! fn new_sized_memfd() -> Result<memfd::Memfd, Box<dyn std::error::Error>> {
//!     // Create a sealable memfd.
//!     let opts = memfd::MemfdOptions::default().allow_sealing(true);
//!     let mfd = opts.create("sized-1K")?;
//!
//!     // Resize to 1024B.
//!     mfd.as_file().set_len(1024)?;
//!
//!     // Add seals to prevent further resizing.
//!     mfd.add_seals(&[
//!         memfd::FileSeal::SealShrink,
//!         memfd::FileSeal::SealGrow
//!     ])?;
//!
//!     // Prevent further sealing changes.
//!     mfd.add_seal(memfd::FileSeal::SealSeal)?;
//!
//!     Ok(mfd)
//! }
//! ```
#![deny(
    missing_docs,
    broken_intra_doc_links,
    clippy::all,
    unreachable_pub,
    unused
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(any(target_os = "android", target_os = "linux"))))]
// No-op crate on platforms that do not support memfd_create, instead of failing to link, or at
// runtime.
#![cfg(any(target_os = "android", target_os = "linux"))]

mod errors;
mod memfd;
mod sealing;

pub use crate::{
    errors::Error,
    memfd::{HugetlbSize, Memfd, MemfdOptions},
    sealing::{FileSeal, SealsHashSet},
};
