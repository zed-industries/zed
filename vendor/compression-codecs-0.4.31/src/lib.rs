//! Adaptors for various compression algorithms.

#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

use std::io::Result;

#[cfg(feature = "brotli")]
pub mod brotli;
#[cfg(feature = "bzip2")]
pub mod bzip2;
#[cfg(feature = "deflate")]
pub mod deflate;
#[cfg(feature = "deflate64")]
pub mod deflate64;
#[cfg(feature = "flate2")]
pub mod flate;
#[cfg(feature = "gzip")]
pub mod gzip;
#[cfg(feature = "lz4")]
pub mod lz4;
#[cfg(feature = "lzma")]
pub mod lzma;
#[cfg(feature = "xz")]
pub mod xz;
#[cfg(feature = "lzma")]
pub mod xz2;
#[cfg(feature = "zlib")]
pub mod zlib;
#[cfg(feature = "zstd")]
pub mod zstd;

use compression_core::util::PartialBuffer;

#[cfg(feature = "brotli")]
pub use self::brotli::{BrotliDecoder, BrotliEncoder};
#[cfg(feature = "bzip2")]
pub use self::bzip2::{BzDecoder, BzEncoder};
#[cfg(feature = "deflate")]
pub use self::deflate::{DeflateDecoder, DeflateEncoder};
#[cfg(feature = "deflate64")]
pub use self::deflate64::Deflate64Decoder;
#[cfg(feature = "flate2")]
pub use self::flate::{FlateDecoder, FlateEncoder};
#[cfg(feature = "gzip")]
pub use self::gzip::{GzipDecoder, GzipEncoder};
#[cfg(feature = "lz4")]
pub use self::lz4::{Lz4Decoder, Lz4Encoder};
#[cfg(feature = "lzma")]
pub use self::lzma::{LzmaDecoder, LzmaEncoder};
#[cfg(feature = "xz")]
pub use self::xz::{XzDecoder, XzEncoder};
#[cfg(feature = "lzma")]
pub use self::xz2::{Xz2Decoder, Xz2Encoder, Xz2FileFormat};
#[cfg(feature = "zlib")]
pub use self::zlib::{ZlibDecoder, ZlibEncoder};
#[cfg(feature = "zstd")]
pub use self::zstd::{ZstdDecoder, ZstdEncoder};

pub trait Encode {
    fn encode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<()>;

    /// Returns whether the internal buffers are flushed
    fn flush(&mut self, output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>)
        -> Result<bool>;

    /// Returns whether the internal buffers are flushed and the end of the stream is written
    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool>;
}

pub trait Decode {
    /// Reinitializes this decoder ready to decode a new member/frame of data.
    fn reinit(&mut self) -> Result<()>;

    /// Returns whether the end of the stream has been read
    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool>;

    /// Returns whether the internal buffers are flushed
    fn flush(&mut self, output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>)
        -> Result<bool>;

    /// Returns whether the internal buffers are flushed
    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool>;
}
