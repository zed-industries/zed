//! This module contains zstd-specific types for async-compression.

use compression_core::Level;

/// A compression parameter for zstd. This is a stable wrapper around zstd's own `CParameter`
/// type, to abstract over different versions of the zstd library.
///
/// See the [zstd documentation](https://facebook.github.io/zstd/zstd_manual.html) for more
/// information on these parameters.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CParameter(libzstd::stream::raw::CParameter);

impl From<CParameter> for libzstd::stream::raw::CParameter {
    fn from(value: CParameter) -> Self {
        value.0
    }
}

impl CParameter {
    pub fn quality(level: Level) -> i32 {
        let (fastest, best) = libzstd::compression_level_range().into_inner();

        // NOTE: zstd's "fastest" level is -131072 which can create outputs larger than inputs.
        // This library chooses a "fastest" level which has a more-or-less equivalent compression
        // ratio to gzip's fastest mode. We still allow precise levels to go negative.
        // See discussion in https://github.com/Nullus157/async-compression/issues/352
        const OUR_FASTEST: i32 = 1;

        match level {
            Level::Fastest => OUR_FASTEST,
            Level::Best => best,
            Level::Precise(quality) => quality.clamp(fastest, best),
            _ => libzstd::DEFAULT_COMPRESSION_LEVEL,
        }
    }

    /// Window size in bytes (as a power of two)
    pub fn window_log(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::WindowLog(value))
    }

    /// Size of the initial probe table in 4-byte entries (as a power of two)
    pub fn hash_log(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::HashLog(value))
    }

    /// Size of the multi-probe table in 4-byte entries (as a power of two)
    pub fn chain_log(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::ChainLog(value))
    }

    /// Number of search attempts (as a power of two)
    pub fn search_log(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::SearchLog(value))
    }

    /// Minimum size of matches searched for
    pub fn min_match(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::MinMatch(value))
    }

    /// Strategy-dependent length modifier
    pub fn target_length(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::TargetLength(value))
    }

    /// Enable long-distance matching mode to look for and emit long-distance references.
    ///
    /// This increases the default window size.
    pub fn enable_long_distance_matching(value: bool) -> Self {
        Self(libzstd::stream::raw::CParameter::EnableLongDistanceMatching(value))
    }

    /// Size of the long-distance matching table (as a power of two)
    pub fn ldm_hash_log(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::LdmHashLog(value))
    }

    /// Minimum size of long-distance matches searched for
    pub fn ldm_min_match(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::LdmMinMatch(value))
    }

    /// Size of each bucket in the LDM hash table for collision resolution (as a power of two)
    pub fn ldm_bucket_size_log(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::LdmBucketSizeLog(value))
    }

    /// Frequency of using the LDM hash table (as a power of two)
    pub fn ldm_hash_rate_log(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::LdmHashRateLog(value))
    }

    /// Emit the size of the content (default: true).
    pub fn content_size_flag(value: bool) -> Self {
        Self(libzstd::stream::raw::CParameter::ContentSizeFlag(value))
    }

    /// Emit a checksum (default: false).
    pub fn checksum_flag(value: bool) -> Self {
        Self(libzstd::stream::raw::CParameter::ChecksumFlag(value))
    }

    /// Emit a dictionary ID when using a custom dictionary (default: true).
    pub fn dict_id_flag(value: bool) -> Self {
        Self(libzstd::stream::raw::CParameter::DictIdFlag(value))
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
        Self(libzstd::stream::raw::CParameter::NbWorkers(value))
    }

    /// Number of bytes given to each worker.
    ///
    /// If set to 0, zstd selects a job size based on compression parameters.
    pub fn job_size(value: u32) -> Self {
        Self(libzstd::stream::raw::CParameter::JobSize(value))
    }

    pub(crate) fn as_zstd(&self) -> libzstd::stream::raw::CParameter {
        self.0
    }
}

/// A decompression parameter for zstd. This is a stable wrapper around zstd's own `DParameter`
/// type, to abstract over different versions of the zstd library.
///
/// See the [zstd documentation](https://facebook.github.io/zstd/zstd_manual.html) for more
/// information on these parameters.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DParameter(libzstd::stream::raw::DParameter);

impl DParameter {
    /// Maximum window size in bytes (as a power of two)
    pub fn window_log_max(value: u32) -> Self {
        Self(libzstd::stream::raw::DParameter::WindowLogMax(value))
    }

    pub(crate) fn as_zstd(&self) -> libzstd::stream::raw::DParameter {
        self.0
    }
}
