/*
 * Copyright (c) 2023.
 *
 * This software is free software; You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

//! Core routines shared by all libraries
//!
//! This crate provides a set of core routines shared
//! by the decoders and encoders under `zune` umbrella
//!
//! It currently contains
//!
//! - A bytestream reader and writer with endian aware reads and writes
//! - Colorspace and bit depth information shared by images
//! - Image decoder and encoder options
//! - A simple enum type to hold image decoding results.
//!
//! This library is `#[no_std]` with `alloc` feature needed for defining `Vec`
//! which we need for storing decoded  bytes.
//!
//!
//! # Features
//!  - `no_std`: Enables `#[no_std]` compilation support.
//!
//!  - `serde`: Enables serializing of some of the data structures
//!     present in the crate
//!
//!
//! # Input/Output
//!
//! zune-image supports many different input and output devices. For input readers
//! we can read anything that implements `BufRead` + `Seek` and provide an optimized routine for
//! handling in memory buffers by using [`ZCursor`](crate::bytestream::ZCursor).
//!
//! For output, we support anything that implements `Write` trait, this includes files, standard io streams
//! network sockets, etc
//!
//! In a `no_std` environment. We can write to in memory buffers `&mut [u8]` and `&mut Vec<u8>`
//!
//! If you have an in memory buffer, use [`ZCursor`](crate::bytestream::ZCursor),
//! it's optimized for in memory buffers.
//!
//!  
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![macro_use]
extern crate alloc;
extern crate core;

#[cfg(not(feature = "log"))]
pub mod log;

#[cfg(feature = "log")]
pub use log;

pub mod bit_depth;
pub mod bytestream;
pub mod colorspace;
pub mod options;
pub mod result;
mod serde;
