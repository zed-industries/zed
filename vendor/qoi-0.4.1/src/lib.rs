//! Fast encoder/decoder for [QOI image format](https://qoiformat.org/), implemented in pure and safe Rust.
//!
//! - One of the [fastest](#benchmarks) QOI encoders/decoders out there.
//! - Compliant with the [latest](https://qoiformat.org/qoi-specification.pdf) QOI format specification.
//! - Zero unsafe code.
//! - Supports decoding from / encoding to `std::io` streams directly.
//! - `no_std` support.
//! - Roundtrip-tested vs the reference C implementation; fuzz-tested.
//!
//! ### Examples
//!
//! ```rust
//! use qoi::{encode_to_vec, decode_to_vec};
//!
//! let encoded = encode_to_vec(&pixels, width, height)?;
//! let (header, decoded) = decode_to_vec(&encoded)?;
//!
//! assert_eq!(header.width, width);
//! assert_eq!(header.height, height);
//! assert_eq!(decoded, pixels);
//! ```
//!
//! ### Benchmarks
//!
//! ```
//!              decode:Mp/s  encode:Mp/s  decode:MB/s  encode:MB/s
//! qoi.h              282.9        225.3        978.3        778.9
//! qoi-rust           427.4        290.0       1477.7       1002.9
//! ```
//!
//! - Reference C implementation:
//!   [phoboslab/qoi@00e34217](https://github.com/phoboslab/qoi/commit/00e34217).
//! - Benchmark timings were collected on an Apple M1 laptop.
//! - 2846 images from the suite provided upstream
//!   ([tarball](https://phoboslab.org/files/qoibench/qoi_benchmark_suite.tar)):
//!   all pngs except two with broken checksums.
//! - 1.32 GPixels in total with 4.46 GB of raw pixel data.
//!
//! Benchmarks have also been run for all of the other Rust implementations
//! of QOI for comparison purposes and, at the time of writing this document,
//! this library proved to be the fastest one by a noticeable margin.
//!
//! ### Rust version
//!
//! The minimum supported Rust version is 1.51.0 (any changes to this would be
//! considered to be a breaking change).
//!
//! ### `no_std`
//!
//! This crate supports `no_std` mode. By default, std is enabled via the `std`
//! feature. You can deactivate the `default-features` to target core instead.
//! In that case anything related to `std::io`, `std::error::Error` and heap
//! allocations is disabled. There is an additional `alloc` feature that can
//! be activated to bring back the support for heap allocations.

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::inline_always,
    clippy::similar_names,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::cargo_common_metadata,
    clippy::doc_markdown,
    clippy::return_self_not_must_use,
)]
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#[cfg(all(feature = "alloc", not(any(feature = "std", test))))]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std as alloc;

mod decode;
mod encode;
mod error;
mod header;
mod pixel;
mod types;
mod utils;

#[doc(hidden)]
pub mod consts;

#[cfg(any(feature = "alloc", feature = "std"))]
pub use crate::decode::decode_to_vec;
pub use crate::decode::{decode_header, decode_to_buf, Decoder};

#[cfg(any(feature = "alloc", feature = "std"))]
pub use crate::encode::encode_to_vec;
pub use crate::encode::{encode_max_len, encode_to_buf, Encoder};

pub use crate::error::{Error, Result};
pub use crate::header::Header;
pub use crate::types::{Channels, ColorSpace};
