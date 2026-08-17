//! Bzip compression for Rust
//!
//! This library contains bindings to libbz2 to support bzip compression and
//! decompression for Rust. The streams offered in this library are primarily
//! found in the `reader` and `writer` modules. Both compressors and
//! decompressors are available in each module depending on what operation you
//! need.
//!
//! Access to the raw decompression/compression stream is also provided through
//! the `raw` module which has a much closer interface to libbz2.
//!
//! # Example
//!
//! ```
//! use std::io::prelude::*;
//! use bzip2::Compression;
//! use bzip2::read::{BzEncoder, BzDecoder};
//!
//! // Round trip some bytes from a byte source, into a compressor, into a
//! // decompressor, and finally into a vector.
//! let data = "Hello, World!".as_bytes();
//! let compressor = BzEncoder::new(data, Compression::best());
//! let mut decompressor = BzDecoder::new(compressor);
//!
//! let mut contents = String::new();
//! decompressor.read_to_string(&mut contents).unwrap();
//! assert_eq!(contents, "Hello, World!");
//! ```
//!
//! # Multistreams (e.g. Wikipedia or pbzip2)
//!
//! Some tools such as pbzip2 or data from sources such as Wikipedia
//! are encoded as so called bzip2 "multistreams," meaning they
//! contain back to back chunks of bzip'd data. `BzDecoder` does not
//! attempt to convert anything after the the first bzip chunk in the
//! source stream. Thus, if you wish to decode all bzip chunks from
//! the input until end of file, use `MultiBzDecoder`.
//!
//! *Protip*: If you use `BzDecoder` to decode data and the output is
//! incomplete and exactly 900K bytes, you probably need a
//! `MultiBzDecoder`.
//!
//! # Async I/O
//!
//! This crate optionally can support async I/O streams with the Tokio stack via
//! the `tokio` feature of this crate:
//!
//! ```toml
//! bzip2 = { version = "0.4", features = ["tokio"] }
//! ```
//!
//! All methods are internally capable of working with streams that may return
//! `ErrorKind::WouldBlock` when they're not ready to perform the particular
//! operation.
//!
//! Note that care needs to be taken when using these objects, however. The
//! Tokio runtime, in particular, requires that data is fully flushed before
//! dropping streams. For compatibility with blocking streams all streams are
//! flushed/written when they are dropped, and this is not always a suitable
//! time to perform I/O. If I/O streams are flushed before drop, however, then
//! these operations will be a noop.

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bzip2/")]

extern crate bzip2_sys as ffi;
extern crate libc;
#[cfg(test)]
extern crate partial_io;
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;
#[cfg(feature = "tokio")]
#[macro_use]
extern crate tokio_io;
#[cfg(feature = "tokio")]
extern crate futures;

pub use mem::{Action, Compress, Decompress, Error, Status};

mod mem;

pub mod bufread;
pub mod read;
pub mod write;

/// When compressing data, the compression level can be specified by a value in
/// this enum.
#[derive(Copy, Clone, Debug)]
pub struct Compression(u32);

impl Compression {
    /// Create a new compression spec with a specific numeric level (0-9).
    pub fn new(level: u32) -> Compression {
        Compression(level)
    }

    /// Do not compress.
    pub fn none() -> Compression {
        Compression(0)
    }

    /// Optimize for the best speed of encoding.
    pub fn fast() -> Compression {
        Compression(1)
    }

    /// Optimize for the size of data being encoded.
    pub fn best() -> Compression {
        Compression(9)
    }

    /// Return the compression level as an integer.
    pub fn level(&self) -> u32 {
        self.0
    }
}

impl Default for Compression {
    /// Choose the default compression, a balance between speed and size.
    fn default() -> Compression {
        Compression(6)
    }
}
