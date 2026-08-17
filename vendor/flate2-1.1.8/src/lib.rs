//! A DEFLATE-based stream compression/decompression library
//!
//! This library provides support for compression and decompression of
//! DEFLATE-based streams:
//!
//! * the DEFLATE format itself
//! * the zlib format
//! * gzip
//!
//! These three formats are all closely related and largely only differ in their
//! headers/footers. This crate has three types in each submodule for dealing
//! with these three formats.
//!
//! # Implementation
//!
//! In addition to supporting three formats, this crate supports several different
//! backends, controlled through this crate's *features flags*:
//!
//! * `default`, or `rust_backend` - this implementation currently uses the `miniz_oxide`
//!   crate which is a port of `miniz.c` to Rust. This feature does not
//!   require a C compiler, and only uses safe Rust code.
//!
//!   Note that the `rust_backend` feature may at some point be switched to use `zlib-rs`,
//!   and that `miniz_oxide` should be used explicitly if this is not desired.
//!
//! * `zlib-rs` - this implementation utilizes the `zlib-rs` crate, a Rust rewrite of zlib.
//!   This backend is the fastest, at the cost of some `unsafe` Rust code.
//!
//! Several backends implemented in C are also available.
//! These are useful in case you are already using a specific C implementation
//! and need the result of compression to be bit-identical.
//! See the crate's README for details on the available C backends.
//!
//! The `zlib-rs` backend typically outperforms all the C implementations.
//!
//! # Feature Flags
#![cfg_attr(
    not(feature = "document-features"),
    doc = "Activate the `document-features` cargo feature to see feature docs here"
)]
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!
//! ## Ambiguous feature selection
//!
//! As Cargo features are additive, while backends are not, there is an order in which backends
//! become active if multiple are selected.
//!
//! * zlib-ng
//! * zlib-rs
//! * cloudflare_zlib
//! * miniz_oxide
//!
//! # Organization
//!
//! This crate consists of three main modules: `bufread`, `read`, and `write`. Each module
//! implements DEFLATE, zlib, and gzip for [`std::io::BufRead`] input types, [`std::io::Read`] input
//! types, and [`std::io::Write`] output types respectively.
//!
//! Use the [`mod@bufread`] implementations if you can provide a `BufRead` type for the input.
//! The `&[u8]` slice type implements the `BufRead` trait.
//!
//! The [`mod@read`] implementations conveniently wrap a `Read` type in a `BufRead` implementation.
//! However, the `read` implementations may
//! [read past the end of the input data](https://github.com/rust-lang/flate2-rs/issues/338),
//! making the `Read` type useless for subsequent reads of the input. If you need to re-use the
//! `Read` type, wrap it in a [`std::io::BufReader`], use the `bufread` implementations,
//! and perform subsequent reads on the `BufReader`.
//!
//! The [`mod@write`] implementations are most useful when there is no way to create a `BufRead`
//! type, notably when reading async iterators (streams).
//!
//! ```
//! use futures::{Stream, StreamExt};
//! use std::io::{Result, Write as _};
//!
//! async fn decompress_gzip_stream<S, I>(stream: S) -> Result<Vec<u8>>
//! where
//!     S: Stream<Item = I>,
//!     I: AsRef<[u8]>
//! {
//!     let mut stream = std::pin::pin!(stream);
//!     let mut w = Vec::<u8>::new();
//!     let mut decoder = flate2::write::GzDecoder::new(w);
//!     while let Some(input) = stream.next().await {
//!         decoder.write_all(input.as_ref())?;
//!     }
//!     decoder.finish()
//! }
//! ```
//!
//!
//! Note that types which operate over a specific trait often implement the mirroring trait as well.
//! For example a `bufread::DeflateDecoder<T>` *also* implements the
//! [`Write`] trait if `T: Write`. That is, the "dual trait" is forwarded directly
//! to the underlying object if available.
//!
//! # About multi-member Gzip files
//!
//! While most `gzip` files one encounters will have a single *member* that can be read
//! with the [`GzDecoder`], there may be some files which have multiple members.
//!
//! A [`GzDecoder`] will only read the first member of gzip data, which may unexpectedly
//! provide partial results when a multi-member gzip file is encountered. `GzDecoder` is appropriate
//! for data that is designed to be read as single members from a multi-member file. `bufread::GzDecoder`
//! and `write::GzDecoder` also allow non-gzip data following gzip data to be handled.
//!
//! The [`MultiGzDecoder`] on the other hand will decode all members of a `gzip` file
//! into one consecutive stream of bytes, which hides the underlying *members* entirely.
//! If a file contains non-gzip data after the gzip data, MultiGzDecoder will
//! emit an error after decoding the gzip data. This behavior matches the `gzip`,
//! `gunzip`, and `zcat` command line tools.
//!
//! [`Bufread`]: std::io::BufRead
//! [`BufReader`]: std::io::BufReader
//! [`Read`]: std::io::Read
//! [`Write`]: std::io::Write
//! [`GzDecoder`]: bufread::GzDecoder
//! [`MultiGzDecoder`]: bufread::MultiGzDecoder
#![doc(html_root_url = "https://docs.rs/flate2/0.2")]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(trivial_numeric_casts)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(not(feature = "any_impl",))]
compile_error!("You need to choose a zlib backend");

pub use crate::crc::{Crc, CrcReader, CrcWriter};
pub use crate::gz::GzBuilder;
pub use crate::gz::GzHeader;
pub use crate::mem::{Compress, CompressError, Decompress, DecompressError, Status};
pub use crate::mem::{FlushCompress, FlushDecompress};

mod bufreader;
mod crc;
mod deflate;
mod ffi;
mod gz;
mod mem;
mod zio;
mod zlib;

/// Types which operate over [`Read`] streams, both encoders and decoders for
/// various formats.
///
/// Note that the `read` decoder types may read past the end of the compressed
/// data while decoding. If the caller requires subsequent reads to start
/// immediately following the compressed data  wrap the `Read` type in a
/// [`BufReader`] and use the `BufReader` with the equivalent decoder from the
/// `bufread` module and also for the subsequent reads.
///
/// [`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
/// [`BufReader`]: https://doc.rust-lang.org/std/io/struct.BufReader.html
pub mod read {
    pub use crate::deflate::read::DeflateDecoder;
    pub use crate::deflate::read::DeflateEncoder;
    pub use crate::gz::read::GzDecoder;
    pub use crate::gz::read::GzEncoder;
    pub use crate::gz::read::MultiGzDecoder;
    pub use crate::zlib::read::ZlibDecoder;
    pub use crate::zlib::read::ZlibEncoder;
}

/// Types which operate over [`Write`] streams, both encoders and decoders for
/// various formats.
///
/// [`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
pub mod write {
    pub use crate::deflate::write::DeflateDecoder;
    pub use crate::deflate::write::DeflateEncoder;
    pub use crate::gz::write::GzDecoder;
    pub use crate::gz::write::GzEncoder;
    pub use crate::gz::write::MultiGzDecoder;
    pub use crate::zlib::write::ZlibDecoder;
    pub use crate::zlib::write::ZlibEncoder;
}

/// Types which operate over [`BufRead`] streams, both encoders and decoders for
/// various formats.
///
/// [`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
pub mod bufread {
    pub use crate::deflate::bufread::DeflateDecoder;
    pub use crate::deflate::bufread::DeflateEncoder;
    pub use crate::gz::bufread::GzDecoder;
    pub use crate::gz::bufread::GzEncoder;
    pub use crate::gz::bufread::MultiGzDecoder;
    pub use crate::zlib::bufread::ZlibDecoder;
    pub use crate::zlib::bufread::ZlibEncoder;
}

fn _assert_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}

    _assert_send_sync::<read::DeflateEncoder<&[u8]>>();
    _assert_send_sync::<read::DeflateDecoder<&[u8]>>();
    _assert_send_sync::<read::ZlibEncoder<&[u8]>>();
    _assert_send_sync::<read::ZlibDecoder<&[u8]>>();
    _assert_send_sync::<read::GzEncoder<&[u8]>>();
    _assert_send_sync::<read::GzDecoder<&[u8]>>();
    _assert_send_sync::<read::MultiGzDecoder<&[u8]>>();
    _assert_send_sync::<write::DeflateEncoder<Vec<u8>>>();
    _assert_send_sync::<write::DeflateDecoder<Vec<u8>>>();
    _assert_send_sync::<write::ZlibEncoder<Vec<u8>>>();
    _assert_send_sync::<write::ZlibDecoder<Vec<u8>>>();
    _assert_send_sync::<write::GzEncoder<Vec<u8>>>();
    _assert_send_sync::<write::GzDecoder<Vec<u8>>>();
}

/// When compressing data, the compression level can be specified by a value in
/// this struct.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Compression(u32);

impl Compression {
    /// Creates a new description of the compression level with an explicitly
    /// specified integer.
    ///
    /// The integer here is typically on a scale of 0-9 where 0 means "no
    /// compression" and 9 means "take as long as you'd like".
    pub const fn new(level: u32) -> Compression {
        Compression(level)
    }

    /// No compression is to be performed, this may actually inflate data
    /// slightly when encoding.
    pub const fn none() -> Compression {
        Compression(0)
    }

    /// Optimize for the best speed of encoding.
    pub const fn fast() -> Compression {
        Compression(1)
    }

    /// Optimize for the size of data being encoded.
    pub const fn best() -> Compression {
        Compression(9)
    }

    /// Returns an integer representing the compression level, typically on a
    /// scale of 0-9. See [`new`](Self::new) for details about compression levels.
    pub fn level(&self) -> u32 {
        self.0
    }
}

impl Default for Compression {
    fn default() -> Compression {
        Compression(6)
    }
}

#[cfg(test)]
fn random_bytes() -> impl Iterator<Item = u8> {
    use rand::Rng;
    use std::iter;

    iter::repeat(()).map(|_| rand::rng().random())
}
