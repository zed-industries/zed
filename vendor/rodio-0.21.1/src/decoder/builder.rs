//! Builder pattern for configuring and constructing decoders.
//!
//! This module provides a flexible builder API for creating decoders with custom settings.
//! The builder allows configuring format hints, seeking behavior, byte length and other
//! parameters that affect decoder behavior.
//!
//! # Examples
//!
//! ```no_run
//! use std::fs::File;
//! use rodio::Decoder;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let file = File::open("audio.mp3")?;
//!     let len = file.metadata()?.len();
//!
//!     Decoder::builder()
//!         .with_data(file)
//!         .with_byte_len(len)      // Enable seeking and duration calculation
//!         .with_hint("mp3")        // Optional format hint
//!         .with_gapless(true)      // Enable gapless playback
//!         .build()?;
//!
//!     // Use the decoder...
//!     Ok(())
//! }
//! ```
//!
//! # Settings
//!
//! The following settings can be configured:
//!
//! - `byte_len` - Total length of the input data in bytes
//! - `hint` - Format hint like "mp3", "wav", etc
//! - `mime_type` - MIME type hint for container formats
//! - `seekable` - Whether seeking operations are enabled
//! - `gapless` - Enable gapless playback
//! - `coarse_seek` - Use faster but less precise seeking

use std::io::{Read, Seek};

#[cfg(feature = "symphonia")]
use self::read_seek_source::ReadSeekSource;
#[cfg(feature = "symphonia")]
use ::symphonia::core::io::{MediaSource, MediaSourceStream};

use super::*;

/// Audio decoder configuration settings.
/// Support for these settings depends on the underlying decoder implementation.
/// Currently, settings are only used by the Symphonia decoder.
#[derive(Clone, Debug)]
pub struct Settings {
    /// The length of the stream in bytes.
    /// This is required for:
    /// - Reliable seeking operations
    /// - Duration calculations in formats that lack timing information (e.g. MP3, Vorbis)
    ///
    /// Can be obtained from file metadata or by seeking to the end of the stream.
    pub(crate) byte_len: Option<u64>,

    /// Whether to use coarse seeking, or sample-accurate seeking instead.
    pub(crate) coarse_seek: bool,

    /// Whether to trim frames for gapless playback.
    /// Note: Disabling this may affect duration calculations for some formats
    /// as padding frames will be included.
    pub(crate) gapless: bool,

    /// An extension hint for the decoder about the format of the stream.
    /// When known, this can help the decoder to select the correct codec.
    pub(crate) hint: Option<String>,

    /// An MIME type hint for the decoder about the format of the stream.
    /// When known, this can help the decoder to select the correct demuxer.
    pub(crate) mime_type: Option<String>,

    /// Whether the decoder should report as seekable.
    pub(crate) is_seekable: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            byte_len: None,
            coarse_seek: false,
            gapless: true,
            hint: None,
            mime_type: None,
            is_seekable: false,
        }
    }
}

/// Builder for configuring and creating a decoder.
///
/// This provides a flexible way to configure decoder settings before creating
/// the actual decoder instance.
///
/// # Examples
///
/// ```no_run
/// use std::fs::File;
/// use rodio::decoder::DecoderBuilder;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let file = File::open("audio.mp3")?;
///     let decoder = DecoderBuilder::new()
///         .with_data(file)
///         .with_hint("mp3")
///         .with_gapless(true)
///         .build()?;
///
///     // Use the decoder...
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug)]
pub struct DecoderBuilder<R> {
    /// The input data source to decode.
    data: Option<R>,
    /// Configuration settings for the decoder.
    settings: Settings,
}

impl<R> Default for DecoderBuilder<R> {
    fn default() -> Self {
        Self {
            data: None,
            settings: Settings::default(),
        }
    }
}

impl<R: Read + Seek + Send + Sync + 'static> DecoderBuilder<R> {
    /// Creates a new decoder builder with default settings.
    ///
    /// # Examples
    /// ```no_run
    /// use std::fs::File;
    /// use rodio::decoder::DecoderBuilder;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let file = File::open("audio.mp3")?;
    ///     let decoder = DecoderBuilder::new()
    ///         .with_data(file)
    ///         .build()?;
    ///
    ///     // Use the decoder...
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the input data source to decode.
    pub fn with_data(mut self, data: R) -> Self {
        self.data = Some(data);
        self
    }

    /// Sets the byte length of the stream.
    /// This is required for:
    /// - Reliable seeking operations
    /// - Duration calculations in formats that lack timing information (e.g. MP3, Vorbis)
    ///
    /// Note that this also sets `is_seekable` to `true`.
    ///
    /// The byte length should typically be obtained from file metadata:
    /// ```no_run
    /// use std::fs::File;
    /// use rodio::Decoder;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let file = File::open("audio.mp3")?;
    ///     let len = file.metadata()?.len();
    ///     let decoder = Decoder::builder()
    ///         .with_data(file)
    ///         .with_byte_len(len)
    ///         .build()?;
    ///
    ///     // Use the decoder...
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Alternatively, it can be obtained by seeking to the end of the stream.
    ///
    /// An incorrect byte length can lead to unexpected behavior, including but not limited to
    /// incorrect duration calculations and seeking errors.
    pub fn with_byte_len(mut self, byte_len: u64) -> Self {
        self.settings.byte_len = Some(byte_len);
        self.settings.is_seekable = true;
        self
    }

    /// Enables or disables coarse seeking. This is disabled by default.
    ///
    /// This needs `byte_len` to be set. Coarse seeking is faster but less accurate:
    /// it may seek to a position slightly before or after the requested one,
    /// especially when the bitrate is variable.
    pub fn with_coarse_seek(mut self, coarse_seek: bool) -> Self {
        self.settings.coarse_seek = coarse_seek;
        self
    }

    /// Enables or disables gapless playback. This is enabled by default.
    ///
    /// When enabled, removes silence between tracks for formats that support it.
    pub fn with_gapless(mut self, gapless: bool) -> Self {
        self.settings.gapless = gapless;
        self
    }

    /// Sets a format hint for the decoder.
    ///
    /// When known, this can help the decoder to select the correct codec faster.
    /// Common values are "mp3", "wav", "flac", "ogg", etc.
    pub fn with_hint(mut self, hint: &str) -> Self {
        self.settings.hint = Some(hint.to_string());
        self
    }

    /// Sets a mime type hint for the decoder.
    ///
    /// When known, this can help the decoder to select the correct demuxer faster.
    /// Common values are "audio/mpeg", "audio/vnd.wav", "audio/flac", "audio/ogg", etc.
    pub fn with_mime_type(mut self, mime_type: &str) -> Self {
        self.settings.mime_type = Some(mime_type.to_string());
        self
    }

    /// Configure whether the data supports random access seeking. Without this,
    /// only forward seeking may work.
    ///
    /// For reliable seeking behavior, `byte_len` should be set either from file metadata
    /// or by seeking to the end of the stream. While seeking may work without `byte_len`
    /// for some formats, it is not guaranteed.
    ///
    /// # Examples
    /// ```no_run
    /// use std::fs::File;
    /// use rodio::Decoder;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let file = File::open("audio.mp3")?;
    ///     let len = file.metadata()?.len();
    ///
    ///     // Recommended: Set both byte_len and seekable
    ///     let decoder = Decoder::builder()
    ///         .with_data(file)
    ///         .with_byte_len(len)
    ///         .with_seekable(true)
    ///         .build()?;
    ///
    ///     // Use the decoder...
    ///     Ok(())
    /// }
    /// ```
    pub fn with_seekable(mut self, is_seekable: bool) -> Self {
        self.settings.is_seekable = is_seekable;
        self
    }

    /// Creates the decoder implementation with configured settings.
    fn build_impl(self) -> Result<(DecoderImpl<R>, Settings), DecoderError> {
        let data = self.data.ok_or(DecoderError::UnrecognizedFormat)?;

        #[cfg(all(feature = "hound", not(feature = "symphonia-wav")))]
        let data = match wav::WavDecoder::new(data) {
            Ok(decoder) => return Ok((DecoderImpl::Wav(decoder), self.settings)),
            Err(data) => data,
        };
        #[cfg(all(feature = "claxon", not(feature = "symphonia-flac")))]
        let data = match flac::FlacDecoder::new(data) {
            Ok(decoder) => return Ok((DecoderImpl::Flac(decoder), self.settings)),
            Err(data) => data,
        };

        #[cfg(all(feature = "lewton", not(feature = "symphonia-vorbis")))]
        let data = match vorbis::VorbisDecoder::new(data) {
            Ok(decoder) => return Ok((DecoderImpl::Vorbis(decoder), self.settings)),
            Err(data) => data,
        };

        #[cfg(all(feature = "minimp3", not(feature = "symphonia-mp3")))]
        let data = match mp3::Mp3Decoder::new(data) {
            Ok(decoder) => return Ok((DecoderImpl::Mp3(decoder), self.settings)),
            Err(data) => data,
        };

        #[cfg(feature = "symphonia")]
        {
            let mss = MediaSourceStream::new(
                Box::new(ReadSeekSource::new(data, &self.settings)) as Box<dyn MediaSource>,
                Default::default(),
            );

            symphonia::SymphoniaDecoder::new(mss, &self.settings)
                .map(|decoder| (DecoderImpl::Symphonia(decoder, PhantomData), self.settings))
        }

        #[cfg(not(feature = "symphonia"))]
        Err(DecoderError::UnrecognizedFormat)
    }

    /// Creates a new decoder with previously configured settings.
    ///
    /// # Errors
    ///
    /// Returns `DecoderError::UnrecognizedFormat` if the audio format could not be determined
    /// or is not supported.
    pub fn build(self) -> Result<Decoder<R>, DecoderError> {
        let (decoder, _) = self.build_impl()?;
        Ok(Decoder(decoder))
    }

    /// Creates a new looped decoder with previously configured settings.
    ///
    /// # Errors
    ///
    /// Returns `DecoderError::UnrecognizedFormat` if the audio format could not be determined
    /// or is not supported.
    pub fn build_looped(self) -> Result<LoopedDecoder<R>, DecoderError> {
        let (decoder, settings) = self.build_impl()?;
        Ok(LoopedDecoder {
            inner: Some(decoder),
            settings,
        })
    }
}
