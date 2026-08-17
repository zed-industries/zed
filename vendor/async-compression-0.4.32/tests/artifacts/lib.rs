//! Adaptors between compression crates and Rust's modern asynchronous IO types.
//!

//! # Feature Organization
//!
//! This crate is divided up along two axes, which can each be individually selected via Cargo
//! features.
//!
//! All features are disabled by default, you should enable just the ones you need from the lists
//! below.
//!
//! If you want to pull in everything there are three group features defined:
//!

//!  Feature | Does
//! ---------|------
//!  `all`   | Activates all implementations and algorithms.
//!  `all-implementations` | Activates all implementations, needs to be paired with a selection of algorithms
//!  `all-algorithms` | Activates all algorithms, needs to be paired with a selection of implementations
//!

//! ## IO implementation
//!
//! The first division is which underlying asynchronous IO trait will be wrapped, these are
//! available as separate features that have corresponding top-level modules:
//!

//!  Feature | Type
//! ---------|------
// TODO: Kill rustfmt on this section, `#![rustfmt::skip::attributes(cfg_attr)]` should do it, but
// that's unstable
#![cfg_attr(
    feature = "futures-io",
    doc = "[`futures-io`](crate::futures) | [`futures::io::AsyncBufRead`](futures_io::AsyncBufRead), [`futures::io::AsyncWrite`](futures_io::AsyncWrite)"
)]
#![cfg_attr(
    not(feature = "futures-io"),
    doc = "`futures-io` (*inactive*) | `futures::io::AsyncBufRead`, `futures::io::AsyncWrite`"
)]
#![cfg_attr(
    feature = "tokio",
    doc = "[`tokio`](crate::tokio) | [`tokio::io::AsyncBufRead`](::tokio::io::AsyncBufRead), [`tokio::io::AsyncWrite`](::tokio::io::AsyncWrite)"
)]
#![cfg_attr(
    not(feature = "tokio"),
    doc = "`tokio` (*inactive*) | `tokio::io::AsyncBufRead`, `tokio::io::AsyncWrite`"
)]
//!

//! ## Compression algorithm
//!
//! The second division is which compression schemes to support, there are currently a few
//! available choices, these determine which types will be available inside the above modules:
//!

//!  Feature | Types
//! ---------|------
#![cfg_attr(
    feature = "brotli",
    doc = "`brotli` | [`BrotliEncoder`](?search=BrotliEncoder), [`BrotliDecoder`](?search=BrotliDecoder)"
)]
#![cfg_attr(
    not(feature = "brotli"),
    doc = "`brotli` (*inactive*) | `BrotliEncoder`, `BrotliDecoder`"
)]
#![cfg_attr(
    feature = "bzip2",
    doc = "`bzip2` | [`BzEncoder`](?search=BzEncoder), [`BzDecoder`](?search=BzDecoder)"
)]
#![cfg_attr(
    not(feature = "bzip2"),
    doc = "`bzip2` (*inactive*) | `BzEncoder`, `BzDecoder`"
)]
#![cfg_attr(
    feature = "deflate",
    doc = "`deflate` | [`DeflateEncoder`](?search=DeflateEncoder), [`DeflateDecoder`](?search=DeflateDecoder)"
)]
#![cfg_attr(
    not(feature = "deflate"),
    doc = "`deflate` (*inactive*) | `DeflateEncoder`, `DeflateDecoder`"
)]
#![cfg_attr(
    feature = "gzip",
    doc = "`gzip` | [`GzipEncoder`](?search=GzipEncoder), [`GzipDecoder`](?search=GzipDecoder)"
)]
#![cfg_attr(
    not(feature = "gzip"),
    doc = "`gzip` (*inactive*) | `GzipEncoder`, `GzipDecoder`"
)]
#![cfg_attr(
    feature = "lzma",
    doc = "`lzma` | [`LzmaEncoder`](?search=LzmaEncoder), [`LzmaDecoder`](?search=LzmaDecoder)"
)]
#![cfg_attr(
    not(feature = "lzma"),
    doc = "`lzma` (*inactive*) | `LzmaEncoder`, `LzmaDecoder`"
)]
#![cfg_attr(
    feature = "xz",
    doc = "`xz` | [`XzEncoder`](?search=XzEncoder), [`XzDecoder`](?search=XzDecoder)"
)]
#![cfg_attr(
    not(feature = "xz"),
    doc = "`xz` (*inactive*) | `XzEncoder`, `XzDecoder`"
)]
#![cfg_attr(
    feature = "zlib",
    doc = "`zlib` | [`ZlibEncoder`](?search=ZlibEncoder), [`ZlibDecoder`](?search=ZlibDecoder)"
)]
#![cfg_attr(
    not(feature = "zlib"),
    doc = "`zlib` (*inactive*) | `ZlibEncoder`, `ZlibDecoder`"
)]
#![cfg_attr(
    feature = "zstd",
    doc = "`zstd` | [`ZstdEncoder`](?search=ZstdEncoder), [`ZstdDecoder`](?search=ZstdDecoder)"
)]
#![cfg_attr(
    not(feature = "zstd"),
    doc = "`zstd` (*inactive*) | `ZstdEncoder`, `ZstdDecoder`"
)]
//!

#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_copy_implementations,
    missing_debug_implementations
)]
#![cfg_attr(not(all), allow(unused))]

#[cfg(any(feature = "bzip2", feature = "flate2", feature = "xz2"))]
use std::convert::TryInto;

#[macro_use]
mod macros;
mod codec;

#[cfg(feature = "futures-io")]
pub mod futures;
#[cfg(feature = "tokio")]
pub mod tokio;

mod unshared;
mod util;

#[cfg(feature = "brotli")]
use brotli::enc::backward_references::BrotliEncoderParams;

/// Level of compression data should be compressed with.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Level {
    /// Fastest quality of compression, usually produces bigger size.
    Fastest,
    /// Best quality of compression, usually produces the smallest size.
    Best,
    /// Default quality of compression defined by the selected compression algorithm.
    Default,
    /// Precise quality based on the underlying compression algorithms'
    /// qualities. The interpretation of this depends on the algorithm chosen
    /// and the specific implementation backing it.
    /// Qualities are implicitly clamped to the algorithm's maximum.
    Precise(i32),
}

impl Level {
    #[cfg(feature = "brotli")]
    fn into_brotli(self, mut params: BrotliEncoderParams) -> BrotliEncoderParams {
        match self {
            Self::Fastest => params.quality = 0,
            Self::Best => params.quality = 11,
            Self::Precise(quality) => params.quality = quality.clamp(0, 11),
            Self::Default => (),
        }

        params
    }

    #[cfg(feature = "bzip2")]
    fn into_bzip2(self) -> bzip2::Compression {
        let fastest = bzip2::Compression::fast();
        let best = bzip2::Compression::best();

        match self {
            Self::Fastest => fastest,
            Self::Best => best,
            Self::Precise(quality) => bzip2::Compression::new(
                quality
                    .try_into()
                    .unwrap_or(0)
                    .clamp(fastest.level(), best.level()),
            ),
            Self::Default => bzip2::Compression::default(),
        }
    }

    #[cfg(feature = "flate2")]
    fn into_flate2(self) -> flate2::Compression {
        let fastest = flate2::Compression::fast();
        let best = flate2::Compression::best();

        match self {
            Self::Fastest => fastest,
            Self::Best => best,
            Self::Precise(quality) => flate2::Compression::new(
                quality
                    .try_into()
                    .unwrap_or(0)
                    .clamp(fastest.level(), best.level()),
            ),
            Self::Default => flate2::Compression::default(),
        }
    }

    #[cfg(feature = "zstd")]
    fn into_zstd(self) -> i32 {
        let (fastest, best) = libzstd::compression_level_range().into_inner();
        match self {
            Self::Fastest => fastest,
            Self::Best => best,
            Self::Precise(quality) => quality.clamp(fastest, best),
            Self::Default => libzstd::DEFAULT_COMPRESSION_LEVEL,
        }
    }

    #[cfg(feature = "xz2")]
    fn into_xz2(self) -> u32 {
        match self {
            Self::Fastest => 0,
            Self::Best => 9,
            Self::Precise(quality) => quality.try_into().unwrap_or(0).min(9),
            Self::Default => 5,
        }
    }
}

#[cfg(feature = "zstd")]
/// This module contains zstd-specific types for async-compression.
pub mod zstd {
    use libzstd::stream::raw::CParameter::*;

    /// A compression parameter for zstd. This is a stable wrapper around zstd's own `CParameter`
    /// type, to abstract over different versions of the zstd library.
    ///
    /// See the [zstd documentation](https://facebook.github.io/zstd/zstd_manual.html) for more
    /// information on these parameters.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct CParameter(libzstd::stream::raw::CParameter);

    impl CParameter {
        /// Window size in bytes (as a power of two)
        pub fn window_log(value: u32) -> Self {
            Self(WindowLog(value))
        }

        /// Size of the initial probe table in 4-byte entries (as a power of two)
        pub fn hash_log(value: u32) -> Self {
            Self(HashLog(value))
        }

        /// Size of the multi-probe table in 4-byte entries (as a power of two)
        pub fn chain_log(value: u32) -> Self {
            Self(ChainLog(value))
        }

        /// Number of search attempts (as a power of two)
        pub fn search_log(value: u32) -> Self {
            Self(SearchLog(value))
        }

        /// Minimum size of matches searched for
        pub fn min_match(value: u32) -> Self {
            Self(MinMatch(value))
        }

        /// Strategy-dependent length modifier
        pub fn target_length(value: u32) -> Self {
            Self(TargetLength(value))
        }

        /// Enable long-distance matching mode to look for and emit long-distance references.
        ///
        /// This increases the default window size.
        pub fn enable_long_distance_matching(value: bool) -> Self {
            Self(EnableLongDistanceMatching(value))
        }

        /// Size of the long-distance matching table (as a power of two)
        pub fn ldm_hash_log(value: u32) -> Self {
            Self(LdmHashLog(value))
        }

        /// Minimum size of long-distance matches searched for
        pub fn ldm_min_match(value: u32) -> Self {
            Self(LdmMinMatch(value))
        }

        /// Size of each bucket in the LDM hash table for collision resolution (as a power of two)
        pub fn ldm_bucket_size_log(value: u32) -> Self {
            Self(LdmBucketSizeLog(value))
        }

        /// Frequency of using the LDM hash table (as a power of two)
        pub fn ldm_hash_rate_log(value: u32) -> Self {
            Self(LdmHashRateLog(value))
        }

        /// Emit the size of the content (default: true).
        pub fn content_size_flag(value: bool) -> Self {
            Self(ContentSizeFlag(value))
        }

        /// Emit a checksum (default: false).
        pub fn checksum_flag(value: bool) -> Self {
            Self(ChecksumFlag(value))
        }

        /// Emit a dictionary ID when using a custom dictionary (default: true).
        pub fn dict_id_flag(value: bool) -> Self {
            Self(DictIdFlag(value))
        }

        /// Number of threads to spawn.
        ///
        /// If set to 0, compression functions will block; if set to 1 or more, compression will
        /// run in background threads and `flush` pushes bytes through the compressor.
        ///
        /// # Panics
        ///
        /// This parameter requires feature `zstdmt` to be enabled, otherwise it will cause a panic
        /// when used in `ZstdEncoder::with_quality_and_params()` calls.
        //
        // TODO: make this a normal feature guarded fn on next breaking release
        #[cfg_attr(docsrs, doc(cfg(feature = "zstdmt")))]
        pub fn nb_workers(value: u32) -> Self {
            Self(NbWorkers(value))
        }

        /// Number of bytes given to each worker.
        ///
        /// If set to 0, zstd selects a job size based on compression parameters.
        pub fn job_size(value: u32) -> Self {
            Self(JobSize(value))
        }

        pub(crate) fn as_zstd(&self) -> libzstd::stream::raw::CParameter {
            self.0
        }
    }
}
