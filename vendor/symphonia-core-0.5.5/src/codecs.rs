// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `codec` module provides the traits and support structures necessary to implement audio codec
//! decoders.

use std::collections::HashMap;
use std::default::Default;
use std::fmt;

use crate::audio::{AudioBufferRef, Channels, Layout};
use crate::errors::{unsupported_error, Result};
use crate::formats::Packet;
use crate::sample::SampleFormat;
use crate::units::TimeBase;

/// A `CodecType` is a unique identifier used to identify a specific codec.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodecType(u32);

/// Declares a new `CodecType` given a character code. A character code is an ASCII string
/// containing a maximum of 5 alphanumeric characters.
///
/// Note: Due to the current limitations of const fn, this function is not able to panic on error.
/// Therefore, if the character code contains an invalid character, then the character is dropped.
/// Additionally, any extra characters will be truncated.
pub const fn decl_codec_type(cc: &[u8]) -> CodecType {
    /// Map alphanumeric ASCII characters into a 6-bit code.
    const fn map_ascii_to_bits(cc: u8) -> u32 {
        // The mapping is defined as:
        //  b'0'..=b'9' maps to  1..=10
        //  b'a'..=b'z' maps to 11..=36
        //  b'A'..=b'Z' maps to 37..=62
        if cc.is_ascii_digit() {
            1 + (cc - b'0') as u32
        }
        else if cc.is_ascii_lowercase() {
            11 + (cc - b'a') as u32
        }
        else if cc.is_ascii_uppercase() {
            37 + (cc - b'A') as u32
        }
        else {
            0
        }
    }

    // TODO: assert!(cc.len() <= 5);

    // The upper-bit indicates the user codec namespace.
    let mut id = 0x8000_0000;

    let mut i = 0;
    let mut j = 0;

    while i < cc.len() && j < 5 {
        // TODO: When const panic is stabilized, assert that the character is alphanumeric to
        // generate an error rather than silently dropping invalid characters.
        // assert!(cc[i].is_ascii_alphanumeric());

        // Pack the ASCII characters into the allocated 30 bits (6 bits per character) in MSb order.
        if cc[i].is_ascii_alphanumeric() {
            id |= map_ascii_to_bits(cc[i]) << (24 - (6 * j));
            j += 1;
        }
        i += 1;
    }

    CodecType(id)
}

impl fmt::Display for CodecType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

/// Null codec
pub const CODEC_TYPE_NULL: CodecType = CodecType(0x0);

// Uncompressed PCM audio codecs
//------------------------------

/// PCM signed 32-bit little-endian interleaved
pub const CODEC_TYPE_PCM_S32LE: CodecType = CodecType(0x100);
/// PCM signed 32-bit little-endian planar
pub const CODEC_TYPE_PCM_S32LE_PLANAR: CodecType = CodecType(0x101);
/// PCM signed 32-bit big-endian interleaved
pub const CODEC_TYPE_PCM_S32BE: CodecType = CodecType(0x102);
/// PCM signed 32-bit big-endian planar
pub const CODEC_TYPE_PCM_S32BE_PLANAR: CodecType = CodecType(0x103);
/// PCM signed 24-bit little-endian interleaved
pub const CODEC_TYPE_PCM_S24LE: CodecType = CodecType(0x104);
/// PCM signed 24-bit little-endian planar
pub const CODEC_TYPE_PCM_S24LE_PLANAR: CodecType = CodecType(0x105);
/// PCM signed 24-bit big-endian interleaved
pub const CODEC_TYPE_PCM_S24BE: CodecType = CodecType(0x106);
/// PCM signed 24-bit big-endian planar
pub const CODEC_TYPE_PCM_S24BE_PLANAR: CodecType = CodecType(0x107);
/// PCM signed 16-bit little-endian interleaved
pub const CODEC_TYPE_PCM_S16LE: CodecType = CodecType(0x108);
/// PCM signed 16-bit little-endian planar
pub const CODEC_TYPE_PCM_S16LE_PLANAR: CodecType = CodecType(0x109);
/// PCM signed 16-bit big-endian interleaved
pub const CODEC_TYPE_PCM_S16BE: CodecType = CodecType(0x10a);
/// PCM signed 16-bit big-endian planar
pub const CODEC_TYPE_PCM_S16BE_PLANAR: CodecType = CodecType(0x10b);
/// PCM signed 8-bit interleaved
pub const CODEC_TYPE_PCM_S8: CodecType = CodecType(0x10c);
/// PCM signed 8-bit planar
pub const CODEC_TYPE_PCM_S8_PLANAR: CodecType = CodecType(0x10d);
/// PCM unsigned 32-bit little-endian interleaved
pub const CODEC_TYPE_PCM_U32LE: CodecType = CodecType(0x10e);
/// PCM unsigned 32-bit little-endian planar
pub const CODEC_TYPE_PCM_U32LE_PLANAR: CodecType = CodecType(0x10f);
/// PCM unsigned 32-bit big-endian interleaved
pub const CODEC_TYPE_PCM_U32BE: CodecType = CodecType(0x110);
/// PCM unsigned 32-bit big-endian planar
pub const CODEC_TYPE_PCM_U32BE_PLANAR: CodecType = CodecType(0x111);
/// PCM unsigned 24-bit little-endian interleaved
pub const CODEC_TYPE_PCM_U24LE: CodecType = CodecType(0x112);
/// PCM unsigned 24-bit little-endian planar
pub const CODEC_TYPE_PCM_U24LE_PLANAR: CodecType = CodecType(0x113);
/// PCM unsigned 24-bit big-endian interleaved
pub const CODEC_TYPE_PCM_U24BE: CodecType = CodecType(0x114);
/// PCM unsigned 24-bit big-endian planar
pub const CODEC_TYPE_PCM_U24BE_PLANAR: CodecType = CodecType(0x115);
/// PCM unsigned 16-bit little-endian interleaved
pub const CODEC_TYPE_PCM_U16LE: CodecType = CodecType(0x116);
/// PCM unsigned 16-bit little-endian planar
pub const CODEC_TYPE_PCM_U16LE_PLANAR: CodecType = CodecType(0x117);
/// PCM unsigned 16-bit big-endian interleaved
pub const CODEC_TYPE_PCM_U16BE: CodecType = CodecType(0x118);
/// PCM unsigned 16-bit big-endian planar
pub const CODEC_TYPE_PCM_U16BE_PLANAR: CodecType = CodecType(0x119);
/// PCM unsigned 8-bit interleaved
pub const CODEC_TYPE_PCM_U8: CodecType = CodecType(0x11a);
/// PCM unsigned 8-bit planar
pub const CODEC_TYPE_PCM_U8_PLANAR: CodecType = CodecType(0x11b);
/// PCM 32-bit little-endian floating point interleaved
pub const CODEC_TYPE_PCM_F32LE: CodecType = CodecType(0x11c);
/// PCM 32-bit little-endian floating point planar
pub const CODEC_TYPE_PCM_F32LE_PLANAR: CodecType = CodecType(0x11d);
/// PCM 32-bit big-endian floating point interleaved
pub const CODEC_TYPE_PCM_F32BE: CodecType = CodecType(0x11e);
/// PCM 32-bit big-endian floating point planar
pub const CODEC_TYPE_PCM_F32BE_PLANAR: CodecType = CodecType(0x11f);
/// PCM 64-bit little-endian floating point interleaved
pub const CODEC_TYPE_PCM_F64LE: CodecType = CodecType(0x120);
/// PCM 64-bit little-endian floating point planar
pub const CODEC_TYPE_PCM_F64LE_PLANAR: CodecType = CodecType(0x121);
/// PCM 64-bit big-endian floating point interleaved
pub const CODEC_TYPE_PCM_F64BE: CodecType = CodecType(0x122);
/// PCM 64-bit big-endian floating point planar
pub const CODEC_TYPE_PCM_F64BE_PLANAR: CodecType = CodecType(0x123);
/// PCM A-law (G.711)
pub const CODEC_TYPE_PCM_ALAW: CodecType = CodecType(0x124);
/// PCM Mu-law (G.711)
pub const CODEC_TYPE_PCM_MULAW: CodecType = CodecType(0x125);

// ADPCM audio codecs
//-------------------

/// G.722 ADPCM
pub const CODEC_TYPE_ADPCM_G722: CodecType = CodecType(0x200);
/// G.726 ADPCM
pub const CODEC_TYPE_ADPCM_G726: CodecType = CodecType(0x201);
/// G.726 ADPCM little-endian
pub const CODEC_TYPE_ADPCM_G726LE: CodecType = CodecType(0x202);
/// Microsoft ADPCM
pub const CODEC_TYPE_ADPCM_MS: CodecType = CodecType(0x203);
/// ADPCM IMA WAV
pub const CODEC_TYPE_ADPCM_IMA_WAV: CodecType = CodecType(0x204);
/// ADPCM IMA QuickTime
pub const CODEC_TYPE_ADPCM_IMA_QT: CodecType = CodecType(0x205);

// Compressed lossy audio codecs
//------------------------------

/// Vorbis
pub const CODEC_TYPE_VORBIS: CodecType = CodecType(0x1000);
/// MPEG Layer 1 (MP1)
pub const CODEC_TYPE_MP1: CodecType = CodecType(0x1001);
/// MPEG Layer 2 (MP2)
pub const CODEC_TYPE_MP2: CodecType = CodecType(0x1002);
/// MPEG Layer 3 (MP3)
pub const CODEC_TYPE_MP3: CodecType = CodecType(0x1003);
/// Advanced Audio Coding (AAC)
pub const CODEC_TYPE_AAC: CodecType = CodecType(0x1004);
/// Opus
pub const CODEC_TYPE_OPUS: CodecType = CodecType(0x1005);
/// Speex
pub const CODEC_TYPE_SPEEX: CodecType = CodecType(0x1006);
/// Musepack
pub const CODEC_TYPE_MUSEPACK: CodecType = CodecType(0x1007);
/// Adaptive Transform Acoustic Coding (ATRAC1)
pub const CODEC_TYPE_ATRAC1: CodecType = CodecType(0x1008);
/// Adaptive Transform Acoustic Coding 3 (ATRAC3)
pub const CODEC_TYPE_ATRAC3: CodecType = CodecType(0x1009);
/// Adaptive Transform Acoustic Coding 3+ (ATRAC3+)
pub const CODEC_TYPE_ATRAC3PLUS: CodecType = CodecType(0x100a);
/// Adaptive Transform Acoustic Coding 9 (ATRAC9)
pub const CODEC_TYPE_ATRAC9: CodecType = CodecType(0x100b);
/// AC-3, E-AC-3, Dolby Digital (ATSC A/52)
pub const CODEC_TYPE_EAC3: CodecType = CodecType(0x100c);
/// Dolby AC-4 (ETSI TS 103 190)
pub const CODEC_TYPE_AC4: CodecType = CodecType(0x100d);
/// DTS Coherent Acoustics (DCA/DTS)
pub const CODEC_TYPE_DCA: CodecType = CodecType(0x100e);
/// Windows Media Audio
pub const CODEC_TYPE_WMA: CodecType = CodecType(0x100f);

// Compressed lossless audio codecs
//---------------------------------

/// Free Lossless Audio Codec (FLAC)
pub const CODEC_TYPE_FLAC: CodecType = CodecType(0x2000);
/// WavPack
pub const CODEC_TYPE_WAVPACK: CodecType = CodecType(0x2001);
/// Monkey's Audio (APE)
pub const CODEC_TYPE_MONKEYS_AUDIO: CodecType = CodecType(0x2002);
/// Apple Lossless Audio Codec (ALAC)
pub const CODEC_TYPE_ALAC: CodecType = CodecType(0x2003);
/// True Audio (TTA)
pub const CODEC_TYPE_TTA: CodecType = CodecType(0x2004);

/// A method and expected value to perform verification on the decoded audio.
#[derive(Copy, Clone, Debug)]
pub enum VerificationCheck {
    /// CRC8 of interleaved PCM audio samples.
    Crc8(u8),
    /// CRC16 of interleaved PCM audio samples.
    Crc16([u8; 2]),
    /// CRC32 of interleaved PCM audio samples.
    Crc32([u8; 4]),
    /// MD5 of interleaved PCM audio samples.
    Md5([u8; 16]),
    /// Codec defined, up-to 16-byte code.
    Other([u8; 16]),
}

/// Codec parameters stored in a container format's headers and metadata may be passed to a codec
/// using the `CodecParameters` structure.
#[derive(Clone, Debug)]
pub struct CodecParameters {
    /// The codec type.
    pub codec: CodecType,

    /// The sample rate of the audio in Hz.
    pub sample_rate: Option<u32>,

    /// The timebase of the stream.
    ///
    /// The timebase is the length of time in seconds of a single tick of a timestamp or duration.
    /// It can be used to convert any timestamp or duration related to the stream into seconds.
    pub time_base: Option<TimeBase>,

    /// The length of the stream in number of frames.
    ///
    /// If a timebase is available, this field can be used to calculate the total duration of the
    /// stream in seconds by using [`TimeBase::calc_time`] and passing the number of frames as the
    /// timestamp.
    pub n_frames: Option<u64>,

    /// The timestamp of the first frame.
    pub start_ts: u64,

    /// The sample format of an audio sample.
    pub sample_format: Option<SampleFormat>,

    /// The number of bits per one decoded audio sample.
    pub bits_per_sample: Option<u32>,

    /// The number of bits per one encoded audio sample.
    pub bits_per_coded_sample: Option<u32>,

    /// A bitmask of all channels in the stream.
    pub channels: Option<Channels>,

    /// The channel layout.
    pub channel_layout: Option<Layout>,

    /// The number of leading frames inserted by the encoder that should be skipped during playback.
    pub delay: Option<u32>,

    /// The number of trailing frames inserted by the encoder for padding that should be skipped
    /// during playback.
    pub padding: Option<u32>,

    /// The maximum number of frames a packet will contain.
    pub max_frames_per_packet: Option<u64>,

    /// The demuxer guarantees packet data integrity.
    pub packet_data_integrity: bool,

    /// A method and expected value that may be used to perform verification on the decoded audio.
    pub verification_check: Option<VerificationCheck>,

    /// The number of frames per block, in case packets are seperated in multiple blocks.
    pub frames_per_block: Option<u64>,

    /// Extra data (defined by the codec).
    pub extra_data: Option<Box<[u8]>>,
}

impl CodecParameters {
    pub fn new() -> CodecParameters {
        CodecParameters {
            codec: CODEC_TYPE_NULL,
            sample_rate: None,
            time_base: None,
            n_frames: None,
            start_ts: 0,
            sample_format: None,
            bits_per_sample: None,
            bits_per_coded_sample: None,
            channels: None,
            channel_layout: None,
            delay: None,
            padding: None,
            max_frames_per_packet: None,
            packet_data_integrity: false,
            verification_check: None,
            frames_per_block: None,
            extra_data: None,
        }
    }

    /// Provide the `CodecType`.
    pub fn for_codec(&mut self, codec: CodecType) -> &mut Self {
        self.codec = codec;
        self
    }

    /// Provide the sample rate in Hz.
    pub fn with_sample_rate(&mut self, sample_rate: u32) -> &mut Self {
        self.sample_rate = Some(sample_rate);
        self
    }

    /// Provide the `TimeBase`.
    pub fn with_time_base(&mut self, time_base: TimeBase) -> &mut Self {
        self.time_base = Some(time_base);
        self
    }

    /// Provide the total number of frames.
    pub fn with_n_frames(&mut self, n_frames: u64) -> &mut Self {
        self.n_frames = Some(n_frames);
        self
    }

    /// Provide the timestamp of the first frame.
    pub fn with_start_ts(&mut self, start_ts: u64) -> &mut Self {
        self.start_ts = start_ts;
        self
    }

    /// Provide the codec's decoded audio sample format.
    pub fn with_sample_format(&mut self, sample_format: SampleFormat) -> &mut Self {
        self.sample_format = Some(sample_format);
        self
    }

    /// Provide the bit per sample of a decoded audio sample.
    pub fn with_bits_per_sample(&mut self, bits_per_sample: u32) -> &mut Self {
        self.bits_per_sample = Some(bits_per_sample);
        self
    }

    /// Provide the bits per sample of an encoded audio sample.
    pub fn with_bits_per_coded_sample(&mut self, bits_per_coded_sample: u32) -> &mut Self {
        self.bits_per_coded_sample = Some(bits_per_coded_sample);
        self
    }

    /// Provide the channel map.
    pub fn with_channels(&mut self, channels: Channels) -> &mut Self {
        self.channels = Some(channels);
        self
    }

    /// Provide the channel layout.
    pub fn with_channel_layout(&mut self, channel_layout: Layout) -> &mut Self {
        self.channel_layout = Some(channel_layout);
        self
    }

    /// Provide the number of delay frames.
    pub fn with_delay(&mut self, delay: u32) -> &mut Self {
        self.delay = Some(delay);
        self
    }

    /// Provide the number of padding frames.
    pub fn with_padding(&mut self, padding: u32) -> &mut Self {
        self.padding = Some(padding);
        self
    }

    /// Provide the maximum number of frames per packet.
    pub fn with_max_frames_per_packet(&mut self, len: u64) -> &mut Self {
        self.max_frames_per_packet = Some(len);
        self
    }

    /// Specify if the packet's data integrity was guaranteed.
    pub fn with_packet_data_integrity(&mut self, integrity: bool) -> &mut Self {
        self.packet_data_integrity = integrity;
        self
    }

    /// Provide the maximum number of frames per packet.
    pub fn with_frames_per_block(&mut self, len: u64) -> &mut Self {
        self.frames_per_block = Some(len);
        self
    }

    /// Provide codec extra data.
    pub fn with_extra_data(&mut self, data: Box<[u8]>) -> &mut Self {
        self.extra_data = Some(data);
        self
    }

    /// Provide a verification code of the final decoded audio.
    pub fn with_verification_code(&mut self, code: VerificationCheck) -> &mut Self {
        self.verification_check = Some(code);
        self
    }
}

impl Default for CodecParameters {
    fn default() -> Self {
        Self::new()
    }
}

/// `FinalizeResult` contains optional information that can only be found, calculated, or
/// determined after decoding is complete.
#[derive(Copy, Clone, Debug, Default)]
pub struct FinalizeResult {
    /// If verification is enabled and supported by the decoder, provides the verification result
    /// if available.
    pub verify_ok: Option<bool>,
}

/// `DecoderOptions` is a common set of options that all decoders use.
#[derive(Copy, Clone, Debug, Default)]
pub struct DecoderOptions {
    /// The decoded audio should be verified if possible during the decode process.
    pub verify: bool,
}

/// A `Decoder` implements a codec's decode algorithm. It consumes `Packet`s and produces
/// `AudioBuffer`s.
pub trait Decoder: Send + Sync {
    /// Attempts to instantiates a `Decoder` using the provided `CodecParameters`.
    fn try_new(params: &CodecParameters, options: &DecoderOptions) -> Result<Self>
    where
        Self: Sized;

    /// Gets a list of codec descriptors for the codecs supported by this Decoder.
    fn supported_codecs() -> &'static [CodecDescriptor]
    where
        Self: Sized;

    /// Reset the `Decoder`.
    ///
    /// A decoder must be reset when the next packet is discontinuous with respect to the last
    /// decoded packet. Most notably, this occurs after a seek.
    ///
    /// For codecs that do a lot of pre-computation, reset should only reset the absolute minimum
    /// amount of state.
    fn reset(&mut self);

    /// Gets a reference to an updated set of `CodecParameters` based on the parameters the
    /// `Decoder` was instantiated with.
    fn codec_params(&self) -> &CodecParameters;

    /// Decodes a `Packet` of audio data and returns a copy-on-write generic (untyped) audio buffer
    /// of the decoded audio.
    ///
    /// If a `DecodeError` or `IoError` is returned, the packet is undecodeable and should be
    /// discarded. Decoding may be continued with the next packet. If `ResetRequired` is returned,
    /// consumers of the decoded audio data should expect the duration and `SignalSpec` of the
    /// decoded audio buffer to change. All other errors are unrecoverable.
    ///
    /// Implementors of decoders *must* `clear` the internal buffer if an error occurs.
    fn decode(&mut self, packet: &Packet) -> Result<AudioBufferRef<'_>>;

    /// Optionally, obtain post-decode information such as the verification status.
    fn finalize(&mut self) -> FinalizeResult;

    /// Allows read access to the internal audio buffer.
    ///
    /// After a successful call to `decode`, this will contain the audio content of the last decoded
    /// `Packet`. If the last call to `decode` resulted in an error, then implementors *must* ensure
    /// the returned audio buffer has zero length.
    fn last_decoded(&self) -> AudioBufferRef<'_>;
}

/// A `CodecDescriptor` stores a description of a single logical codec. Common information such as
/// the `CodecType`, a short name, and a long name are provided. The `CodecDescriptor` also provides
/// an instantiation function. When the instantiation function is called, a `Decoder` for the codec
/// is returned.
#[derive(Copy, Clone)]
pub struct CodecDescriptor {
    /// The `CodecType` identifier.
    pub codec: CodecType,
    /// A short ASCII-only string identifying the codec.
    pub short_name: &'static str,
    /// A longer, more descriptive, string identifying the codec.
    pub long_name: &'static str,
    // An instantiation function for the codec.
    pub inst_func: fn(&CodecParameters, &DecoderOptions) -> Result<Box<dyn Decoder>>,
}

/// A `CodecRegistry` allows the registration of codecs, and provides a method to instantiate a
/// `Decoder` given a `CodecParameters` object.
pub struct CodecRegistry {
    codecs: HashMap<CodecType, CodecDescriptor>,
}

impl CodecRegistry {
    /// Instantiate a new `CodecRegistry`.
    pub fn new() -> Self {
        CodecRegistry { codecs: HashMap::new() }
    }

    /// Gets the `CodecDescriptor` for a registered codec.
    pub fn get_codec(&self, codec: CodecType) -> Option<&CodecDescriptor> {
        self.codecs.get(&codec)
    }

    /// Registers all codecs supported by `Decoder`. If a supported codec was previously registered
    /// by another `Decoder` it will be replaced within the registry.
    pub fn register_all<D: Decoder>(&mut self) {
        for descriptor in D::supported_codecs() {
            self.register(descriptor);
        }
    }

    /// Register a single codec. If the codec was previously registered it will be replaced within
    /// the registry.
    pub fn register(&mut self, descriptor: &CodecDescriptor) {
        self.codecs.insert(descriptor.codec, *descriptor);
    }

    /// Searches the registry for a `Decoder` that supports the codec. If one is found, it will be
    /// instantiated with the provided `CodecParameters` and returned. If a `Decoder` could not be
    /// found, or the `CodecParameters` are either insufficient or invalid for the `Decoder`, an
    /// error will be returned.
    pub fn make(
        &self,
        params: &CodecParameters,
        options: &DecoderOptions,
    ) -> Result<Box<dyn Decoder>> {
        if let Some(descriptor) = self.codecs.get(&params.codec) {
            Ok((descriptor.inst_func)(params, options)?)
        }
        else {
            unsupported_error("core (codec):unsupported codec")
        }
    }
}

impl Default for CodecRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience macro for declaring a `CodecDescriptor`.
#[macro_export]
macro_rules! support_codec {
    ($type:expr, $short_name:expr, $long_name:expr) => {
        CodecDescriptor {
            codec: $type,
            short_name: $short_name,
            long_name: $long_name,
            inst_func: |params, opt| Ok(Box::new(Self::try_new(&params, &opt)?)),
        }
    };
}
