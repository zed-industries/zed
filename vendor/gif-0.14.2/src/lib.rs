#![forbid(unsafe_code)]
//! # GIF en- and decoding library [![Build Status](https://github.com/image-rs/image-gif/workflows/Rust%20CI/badge.svg)](https://github.com/image-rs/image-gif/actions)
//!
//! GIF en- and decoder written in Rust ([API Documentation](https://docs.rs/gif)).
//!
//! # GIF encoding and decoding library
//!
//! This library provides all functions necessary to de- and encode GIF files.
//!
//! ## High level interface
//!
//! The high level interface consists of the two types
//! [`Encoder`](struct.Encoder.html) and [`Decoder`](struct.Decoder.html).
//!
//! ### Decoding GIF files
//!
//! ```rust
//! // Open the file
//! use std::fs::File;
//! let mut decoder = gif::DecodeOptions::new();
//! // Configure the decoder such that it will expand the image to RGBA.
//! decoder.set_color_output(gif::ColorOutput::RGBA);
//! // Read the file header
//! let file = File::open("tests/samples/sample_1.gif").unwrap();
//! let mut decoder = decoder.read_info(file).unwrap();
//! while let Some(frame) = decoder.read_next_frame().unwrap() {
//!     // Process every frame
//! }
//! ```
//!
//!
//!
//! ### Encoding GIF files
//!
//! The encoder can be used so save simple computer generated images:
//!
//! ```rust
//! use gif::{Frame, Encoder, Repeat};
//! use std::fs::File;
//! use std::borrow::Cow;
//!
//! let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
//! let (width, height) = (6, 6);
//! let mut beacon_states = [[
//!     0, 0, 0, 0, 0, 0,
//!     0, 1, 1, 0, 0, 0,
//!     0, 1, 1, 0, 0, 0,
//!     0, 0, 0, 1, 1, 0,
//!     0, 0, 0, 1, 1, 0,
//!     0, 0, 0, 0, 0, 0,
//! ], [
//!     0, 0, 0, 0, 0, 0,
//!     0, 1, 1, 0, 0, 0,
//!     0, 1, 0, 0, 0, 0,
//!     0, 0, 0, 0, 1, 0,
//!     0, 0, 0, 1, 1, 0,
//!     0, 0, 0, 0, 0, 0,
//! ]];
//! let mut image = File::create("tests/samples/beacon.gif").unwrap();;
//! let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
//! encoder.set_repeat(Repeat::Infinite).unwrap();
//! for state in &beacon_states {
//!     let mut frame = Frame::default();
//!     frame.width = width;
//!     frame.height = height;
//!     frame.buffer = Cow::Borrowed(&*state);
//!     encoder.write_frame(&frame).unwrap();
//! }
//! ```
//!
//! [`Frame::from_*`](struct.Frame.html) can be used to convert a true color image to a paletted
//! image with a maximum of 256 colors:
//!
//! ```rust
//! # #[cfg(feature = "color_quant")] {
//! use std::fs::File;
//!
//! // Get pixel data from some source
//! let mut pixels: Vec<u8> = vec![0; 30_000];
//! // Create frame from data
//! let frame = gif::Frame::from_rgb(100, 100, &mut *pixels);
//! // Create encoder
//! let mut image = File::create("target/indexed_color.gif").unwrap();
//! let mut encoder = gif::Encoder::new(&mut image, frame.width, frame.height, &[]).unwrap();
//! // Write frame to file
//! encoder.write_frame(&frame).unwrap();
//! # }
//! ```

// TODO: make this compile
// ```rust
// use gif::{Frame, Encoder};
// use std::fs::File;
// let color_map = &[0, 0, 0, 0xFF, 0xFF, 0xFF];
// let mut frame = Frame::default();
// // Generate checkerboard lattice
// for (i, j) in (0..10).zip(0..10) {
//     frame.buffer.push(if (i * j) % 2 == 0 {
//         1
//     } else {
//         0
//     })
// }
// # (|| {
// {
// let mut file = File::create("test.gif")?;
// let mut encoder = Encoder::new(&mut file, 100, 100);
// encoder.write_global_palette(color_map)?.write_frame(&frame)
// }
// # })().unwrap();
// ```
#![deny(missing_docs)]
#![cfg(feature = "std")]
#![allow(unknown_lints)] // Certain lints only apply to later versions of Rust
#![allow(clippy::manual_range_contains)]
#![allow(clippy::new_without_default)]
#![deny(clippy::alloc_instead_of_core)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate std;

mod common;
mod encoder;
mod reader;
mod traits;

pub use crate::common::{AnyExtension, DisposalMethod, Extension, Frame};

pub use crate::reader::{ColorOutput, MemoryLimit};
pub use crate::reader::{DecodeOptions, Decoder, Version};
pub use crate::reader::{DecodingError, DecodingFormatError};

pub use crate::encoder::{Encoder, EncodingError, EncodingFormatError, ExtensionData, Repeat};

/// Low-level, advanced decoder. Prefer [`Decoder`] instead, which can stream frames too.
pub mod streaming_decoder {
    pub use crate::common::Block;
    pub use crate::reader::{Decoded, FrameDataType, FrameDecoder, OutputBuffer, StreamingDecoder};
}

#[cfg(feature = "color_quant")]
macro_rules! insert_as_doc {
    { $content:expr } => {
        #[allow(unused_doc_comments)]
        #[doc = $content] extern "C" { }
    }
}

// Provides the README.md as doc, to ensure the example works!
#[cfg(feature = "color_quant")]
insert_as_doc!(include_str!("../README.md"));
