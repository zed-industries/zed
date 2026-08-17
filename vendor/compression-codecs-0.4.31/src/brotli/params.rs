//! This module contains Brotli-specific types for async-compression.

use brotli::enc::backward_references::{BrotliEncoderMode, BrotliEncoderParams};
use compression_core::Level;

/// Brotli compression parameters builder. This is a stable wrapper around Brotli's own encoder
/// params type, to abstract over different versions of the Brotli library.
///
/// See the [Brotli documentation](https://www.brotli.org/encode.html#a9a8) for more information on
/// these parameters.
///
/// # Examples
///
/// ```
/// use compression_codecs::brotli;
///
/// let params = brotli::params::EncoderParams::default()
///     .window_size(12)
///     .text_mode();
/// ```
#[derive(Debug, Clone, Default)]
pub struct EncoderParams {
    inner: BrotliEncoderParams,
}

impl From<EncoderParams> for BrotliEncoderParams {
    fn from(value: EncoderParams) -> Self {
        value.inner
    }
}

impl EncoderParams {
    pub fn quality(mut self, level: Level) -> Self {
        let quality = match level {
            Level::Fastest => Some(0),
            Level::Best => Some(11),
            Level::Precise(quality) => Some(quality.clamp(0, 11)),
            _ => None,
        };
        match quality {
            Some(quality) => self.inner.quality = quality,
            None => {
                let default_params = BrotliEncoderParams::default();
                self.inner.quality = default_params.quality;
            }
        }

        self
    }

    /// Sets window size in bytes (as a power of two).
    ///
    /// Used as Brotli's `lgwin` parameter.
    ///
    /// `window_size` is clamped to `0 <= window_size <= 24`.
    pub fn window_size(mut self, window_size: i32) -> Self {
        self.inner.lgwin = window_size.clamp(0, 24);
        self
    }

    /// Sets input block size in bytes (as a power of two).
    ///
    /// Used as Brotli's `lgblock` parameter.
    ///
    /// `block_size` is clamped to `16 <= block_size <= 24`.
    pub fn block_size(mut self, block_size: i32) -> Self {
        self.inner.lgblock = block_size.clamp(16, 24);
        self
    }

    /// Sets hint for size of data to be compressed.
    pub fn size_hint(mut self, size_hint: usize) -> Self {
        self.inner.size_hint = size_hint;
        self
    }

    /// Sets encoder to text mode.
    ///
    /// If input data is known to be UTF-8 text, this allows the compressor to make assumptions and
    /// optimizations.
    ///
    /// Used as Brotli's `mode` parameter.
    pub fn text_mode(mut self) -> Self {
        self.inner.mode = BrotliEncoderMode::BROTLI_MODE_TEXT;
        self
    }

    pub fn mode(mut self, mode: BrotliEncoderMode) -> Self {
        self.inner.mode = mode;
        self
    }
}
