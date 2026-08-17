//! Bzip compression for Rust
//!
//! This library contains bindings to [`libbz2`] to support bzip compression and
//! decompression for Rust. The streams offered in this library are primarily
//! found in the [`mod@read`] and [`mod@write`] modules. Both compressors and
//! decompressors are available in each module depending on what operation you
//! need.
//!
//! A more low-level interface, much closer to the interface of [`libbz2`], is
//! available via the [`Compress`] and [`Decompress`] structs.
//!
//! [`libbz2`]: https://sourceware.org/bzip2/manual/manual.html
//!
//! # Example
//!
//! ```
//! use std::io::{BufRead, Read, Write};
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
//! attempt to convert anything after the first bzip chunk in the
//! source stream. Thus, if you wish to decode all bzip chunks from
//! the input until end of file, use `MultiBzDecoder`.
//!
//! *Protip*: If you use `BzDecoder` to decode data and the output is
//! incomplete and exactly 900K bytes, you probably need a
//! `MultiBzDecoder`.
//!
//! All methods are internally capable of working with streams that may return
//! [`ErrorKind::WouldBlock`](std::io::ErrorKind::WouldBlock) when they're not
//! ready to perform the particular operation.
//!
//! Note that care needs to be taken when using these objects, however. The
//! Tokio runtime, in particular, requires that data is fully flushed before
//! dropping streams. For compatibility with blocking streams all streams are
//! flushed/written when they are dropped, and this is not always a suitable
//! time to perform I/O. If I/O streams are flushed before drop, however, then
//! these operations will be a noop.

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bzip2/")]

#[cfg(feature = "bzip2-sys")]
extern crate bzip2_sys as ffi;
#[cfg(not(feature = "bzip2-sys"))]
extern crate libbz2_rs_sys as ffi;
#[cfg(test)]
extern crate partial_io;
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
extern crate rand;

pub use mem::{Action, Compress, Decompress, Error, Status};

mod mem;

pub mod bufread;
pub mod read;
pub mod write;

/// When compressing data, the compression level can be specified by a value in
/// this enum.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Compression(u32);

impl Compression {
    /// Create a new compression spec with a specific numeric level in the range `1..=9`.
    ///
    /// # Panics
    ///
    /// A level outside of the `1..=9` range will throw a panic. Use [`Self::try_new`] to
    /// gracefully handle invalid levels (e.g. from user input).
    #[track_caller]
    pub const fn new(level: u32) -> Self {
        match Self::try_new(level) {
            Some(v) => v,
            None => panic!("expected a compression level in the range 1..=9"),
        }
    }

    /// Create a new compression spec with a specific numeric level in the range `1..=9`.
    pub const fn try_new(level: u32) -> Option<Self> {
        match level {
            1..=9 => Some(Self(level)),
            _ => None,
        }
    }

    /// Do not compress.
    #[deprecated(since = "0.5.1", note = "libbz2 does not support compression level 0")]
    pub fn none() -> Self {
        Self(0)
    }

    /// Optimize for the best speed of encoding.
    pub const fn fast() -> Self {
        Self(1)
    }

    /// Optimize for smallest output size.
    pub const fn best() -> Self {
        Self(9)
    }

    /// Return the compression level as an integer.
    pub const fn level(&self) -> u32 {
        self.0
    }
}

impl Default for Compression {
    /// Choose the default compression, a balance between speed and size.
    fn default() -> Self {
        Self(6)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn new_level_0() {
        Compression::new(0);
    }

    #[test]
    #[should_panic]
    fn new_level_10() {
        Compression::new(10);
    }

    #[test]
    fn try_new() {
        assert!(Compression::try_new(0).is_none());
        assert!(Compression::try_new(10).is_none());

        assert_eq!(Compression::try_new(1), Some(Compression::fast()));
        assert_eq!(Compression::try_new(6), Some(Compression::default()));
        assert_eq!(Compression::try_new(9), Some(Compression::best()));
    }
}
