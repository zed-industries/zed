/*
 * Copyright (c) 2023.
 *
 * This software is free software; You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

//! An incredibly spiffy deflate decoder.
//!
//! This crate features a deflate/zlib decoder inspired by
//! Eric Bigger's [libdeflate].
//!
//! This libary has a smaller set of features hence you should use it
//! if it aligns with your end goals.
//!
//! Use it if
//! - You want a smaller library footprint when compared to flate/miniz-oxide
//! - You want faster speeds than zlib-ng/zlib/miniz-oxide.
//! - You do full buffer decompression and not streaming decompression.
//! - You don't need compression support for now, it will come soon enough.
//! - You want a 100% safe, pure rust implementation with above.
//!
//!Do not use it if
//!  - You want compression support, not yet there
//!  - You stream your data, not compatible with this library
//!
//! ## Alternatives
//!- For the fastest speeds, check out [libdeflate] (C), if using Rust there is [libdeflater] which
//! provides bindings to [libdeflate]
//!
//!- For streaming support use [flate2-rs] with an appropriate backend(zlib-ng is recommended for speed)
//!  
//! # Features
//! You can disable features depending on what you need. the following are
//! features present
//! - gzip: Enable gzip decoding
//! - zlib: Enable zlib decoding
//!
//! These features are enabled by default
//!
//! To disable a feature , modify Cargo.toml to disable default features
//! and add the needed feature , e.g below will include zlib decoding and disable gzip decoding
//! ```toml
//! zune-inflate={ version="0.2",default-features=false,feature=["zlib"]}
//! ```
//!
//! # Errors
//! In case of an error, the library returns the error and the decoded
//! data up to when the error was encountered hence that data can be recovered
//! but no data further than that can be recovered
//!
//!
//! # Usage
//!
//! Decoding delfate data
//
//! ```no_run
//! use zune_inflate::DeflateDecoder;
//! let totally_valid_data = [0;23];
//! let mut decoder = DeflateDecoder::new(&totally_valid_data);
//!
//! let decompressed =decoder.decode_deflate().unwrap();
//! ```
//!
//! Decoding zlib data
//! ```no_run
//! use zune_inflate::DeflateDecoder;
//! // yea this isn't valid
//! let totally_valid_data = [0;23];
//! let mut decoder = DeflateDecoder::new(&totally_valid_data);
//!
//! let decompressed =decoder.decode_zlib().unwrap();
//! ```
//!
//! Decoding zlib data without confirming the adler32 checksum
//! ```no_run
//! use zune_inflate::DeflateDecoder;
//! use zune_inflate::DeflateOptions;
//! let totally_valid_data=[0;23];
//! let mut options = DeflateOptions::default()
//!                     .set_confirm_checksum(false);
//! let decoder =  DeflateDecoder::new_with_options(&totally_valid_data,options);
//!
//! ```
//!
//! [libdeflate]: https://github.com/ebiggers/libdeflate
//! [libdeflater]: https://github.com/adamkewley/libdeflater
//! [flate2-rs]: https://github.com/rust-lang/flate2-rs
//!
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub use crate::decoder::{DeflateDecoder, DeflateOptions};
pub use crate::encoder::DeflateEncoder;

mod bitstream;
mod constants;
mod crc;
mod decoder;
mod encoder;
pub mod errors;
mod gzip_constants;
mod utils;
