// Symphonia
// Copyright (c) 2019-2023 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

use symphonia_core::codecs::{
    CODEC_TYPE_PCM_ALAW, CODEC_TYPE_PCM_F32BE, CODEC_TYPE_PCM_F64BE, CODEC_TYPE_PCM_MULAW,
    CODEC_TYPE_PCM_S16BE, CODEC_TYPE_PCM_S16LE, CODEC_TYPE_PCM_S24BE, CODEC_TYPE_PCM_S32BE,
    CODEC_TYPE_PCM_S8,
};
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::io::{MediaSourceStream, ReadBytes};

use crate::common::{
    try_channel_count_to_mask, ChunkParser, FormatALaw, FormatData, FormatIeeeFloat, FormatMuLaw,
    FormatPcm, PacketInfo, ParseChunk, ParseChunkTag,
};

use extended::Extended;

/// `CommonChunk` is a required AIFF chunk, containing metadata.
pub struct CommonChunk {
    /// The number of channels.
    pub n_channels: i16,
    /// The number of audio frames.
    #[allow(dead_code)]
    pub n_sample_frames: u32,
    /// The sample size in bits.
    pub sample_size: i16,
    /// The sample rate in Hz.
    pub sample_rate: u32,
    /// Extra data associated with the format block conditional upon the format tag.
    pub format_data: FormatData,
}

impl CommonChunk {
    fn read_pcm_fmt(bits_per_sample: u16, n_channels: u16) -> Result<FormatData> {
        // Bits per sample for PCM is both the encoded sample width, and the actual sample width.
        // Strictly, this must either be 8 or 16 bits, but there is no reason why 24 and 32 bits
        // can't be supported. Since these files do exist, allow for 8/16/24/32-bit samples, but
        // error if not a multiple of 8 or greater than 32-bits.
        //
        // It is possible though for AIFF to have a sample size not divisible by 8.
        // Data is left justified, with the remaining bits zeroed. Currently not supported.
        //
        // Select the appropriate codec using bits per sample. Samples are always interleaved and
        // little-endian encoded for the PCM format.
        let codec = match bits_per_sample {
            8 => CODEC_TYPE_PCM_S8,
            16 => CODEC_TYPE_PCM_S16BE,
            24 => CODEC_TYPE_PCM_S24BE,
            32 => CODEC_TYPE_PCM_S32BE,
            _ => return decode_error("aiff: bits per sample for pcm must be 8, 16, 24 or 32 bits"),
        };

        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::Pcm(FormatPcm { bits_per_sample, channels, codec }))
    }

    fn read_alaw_pcm_fmt(n_channels: u16) -> Result<FormatData> {
        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::ALaw(FormatALaw { codec: CODEC_TYPE_PCM_ALAW, channels }))
    }

    fn read_mulaw_pcm_fmt(n_channels: u16) -> Result<FormatData> {
        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::MuLaw(FormatMuLaw { codec: CODEC_TYPE_PCM_MULAW, channels }))
    }

    fn read_ieee_fmt(bits_per_sample: u16, n_channels: u16) -> Result<FormatData> {
        // Select the appropriate codec using bits per sample. Samples are always interleaved and
        // little-endian encoded for the IEEE Float format.
        let codec = match bits_per_sample {
            32 => CODEC_TYPE_PCM_F32BE,
            64 => CODEC_TYPE_PCM_F64BE,
            _ => return decode_error("aifc: bits per sample for fmt_ieee must be 32 or 64 bits"),
        };

        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::IeeeFloat(FormatIeeeFloat { channels, codec }))
    }

    fn read_sowt_fmt(bits_per_sample: u16, n_channels: u16) -> Result<FormatData> {
        let codec = match bits_per_sample {
            16 => CODEC_TYPE_PCM_S16LE,
            _ => return decode_error("aiff: bits per sample for sowt must be 16 bits"),
        };

        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::Pcm(FormatPcm { bits_per_sample, channels, codec }))
    }

    fn read_twos_fmt(bits_per_sample: u16, n_channels: u16) -> Result<FormatData> {
        let codec = match bits_per_sample {
            16 => CODEC_TYPE_PCM_S16BE,
            _ => return decode_error("aiff: bits per sample for twos must be 16 bits"),
        };

        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::Pcm(FormatPcm { bits_per_sample, channels, codec }))
    }

    pub fn packet_info(&self) -> Result<PacketInfo> {
        match &self.format_data {
            FormatData::Pcm(_) => {
                let block_align = self.n_channels * self.sample_size / 8;
                Ok(PacketInfo::without_blocks(block_align as u16))
            }
            FormatData::ALaw(_) => {
                // In a-law encoding, each audio sample is represented by an 8-bit value that has been compressed
                let block_align = self.n_channels;
                Ok(PacketInfo::without_blocks(block_align as u16))
            }
            FormatData::MuLaw(_) => {
                // In mu-law encoding, each audio sample is represented by an 8-bit value that has been compressed
                let block_align = self.n_channels;
                Ok(PacketInfo::without_blocks(block_align as u16))
            }
            FormatData::IeeeFloat(_) => {
                let block_align = self.n_channels * self.sample_size / 8;
                Ok(PacketInfo::without_blocks(block_align as u16))
            }
            FormatData::Extensible(_) => {
                unsupported_error("aiff: packet info not implemented for format Extensible")
            }
            FormatData::Adpcm(_) => {
                unsupported_error("aiff: packet info not implemented for format Adpcm")
            }
        }
    }
}

impl ParseChunk for CommonChunk {
    fn parse<B: ReadBytes>(reader: &mut B, _tag: [u8; 4], _: u32) -> Result<CommonChunk> {
        let n_channels = reader.read_be_i16()?;
        let n_sample_frames = reader.read_be_u32()?;
        let sample_size = reader.read_be_i16()?;

        let mut sample_rate: [u8; 10] = [0; 10];
        reader.read_buf_exact(sample_rate.as_mut())?;

        let sample_rate = Extended::from_be_bytes(sample_rate);
        let sample_rate = sample_rate.to_f64() as u32;

        let format_data = Self::read_pcm_fmt(sample_size as u16, n_channels as u16);

        let format_data = format_data?;

        Ok(CommonChunk { n_channels, n_sample_frames, sample_size, sample_rate, format_data })
    }
}

impl fmt::Display for CommonChunk {
    //TODO: perhaps place this in riff.rs to share with wave etc
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CommonChunk {{")?;
        writeln!(f, "\tn_channels: {},", self.n_channels)?;
        writeln!(f, "\tsample_rate: {} Hz,", self.sample_rate)?;

        match self.format_data {
            FormatData::Pcm(ref pcm) => {
                writeln!(f, "\tformat_data: Pcm {{")?;
                writeln!(f, "\t\tbits_per_sample: {},", pcm.bits_per_sample)?;
                writeln!(f, "\t\tchannels: {},", pcm.channels)?;
                writeln!(f, "\t\tcodec: {},", pcm.codec)?;
            }
            FormatData::ALaw(ref alaw) => {
                writeln!(f, "\tformat_data: MuLaw {{")?;
                writeln!(f, "\t\tchannels: {},", alaw.channels)?;
                writeln!(f, "\t\tcodec: {},", alaw.codec)?;
            }
            FormatData::MuLaw(ref mulaw) => {
                writeln!(f, "\tformat_data: MuLaw {{")?;
                writeln!(f, "\t\tchannels: {},", mulaw.channels)?;
                writeln!(f, "\t\tcodec: {},", mulaw.codec)?;
            }
            FormatData::IeeeFloat(ref ieee) => {
                writeln!(f, "\tformat_data: IeeeFloat {{")?;
                writeln!(f, "\t\tchannels: {},", ieee.channels)?;
                writeln!(f, "\t\tcodec: {},", ieee.codec)?;
            }
            FormatData::Extensible(_) => {
                writeln!(f, "\tformat_data: Extensible DISPLAY UNSUPPORTED {{")?;
            }
            FormatData::Adpcm(_) => {
                writeln!(f, "\tformat_data: Adpcm DISPLAY UNSUPPORTED {{")?;
            }
        };

        writeln!(f, "\t}}")?;
        writeln!(f, "}}")
    }
}

pub trait CommonChunkParser {
    fn parse_aiff(self, source: &mut MediaSourceStream) -> Result<CommonChunk>;
    fn parse_aifc(self, source: &mut MediaSourceStream) -> Result<CommonChunk>;
}

impl CommonChunkParser for ChunkParser<CommonChunk> {
    fn parse_aiff(self, source: &mut MediaSourceStream) -> Result<CommonChunk> {
        self.parse(source)
    }

    fn parse_aifc(self, source: &mut MediaSourceStream) -> Result<CommonChunk> {
        let n_channels = source.read_be_i16()?;
        let n_sample_frames = source.read_be_u32()?;
        let sample_size = source.read_be_i16()?;

        let mut sample_rate: [u8; 10] = [0; 10];
        source.read_buf_exact(sample_rate.as_mut())?;

        let sample_rate = Extended::from_be_bytes(sample_rate);
        let sample_rate = sample_rate.to_f64() as u32;

        let compression_type = source.read_quad_bytes()?;

        // Ignore pascal string containing compression_name
        let str_len = source.read_byte()?;
        source.ignore_bytes(str_len as u64)?;
        // Total number of bytes in pascal string must be even, since len is excluded from our var, we add 1
        if (str_len + 1) % 2 != 0 {
            source.ignore_bytes(1)?;
        }

        let format_data = match &compression_type {
            b"none" | b"NONE" => CommonChunk::read_pcm_fmt(sample_size as u16, n_channels as u16),
            b"alaw" | b"ALAW" => CommonChunk::read_alaw_pcm_fmt(n_channels as u16),
            b"ulaw" | b"ULAW" => CommonChunk::read_mulaw_pcm_fmt(n_channels as u16),
            b"fl32" | b"fl64" => CommonChunk::read_ieee_fmt(sample_size as u16, n_channels as u16),
            b"sowt" | b"SOWT" => CommonChunk::read_sowt_fmt(sample_size as u16, n_channels as u16),
            b"twos" | b"TWOS" => CommonChunk::read_twos_fmt(sample_size as u16, n_channels as u16),
            _ => return unsupported_error("aifc: Compression type not implemented"),
        };

        let format_data = format_data?;

        Ok(CommonChunk { n_channels, n_sample_frames, sample_size, sample_rate, format_data })
    }
}

/// `SoundChunk` is a required AIFF chunk, containing the audio data.
pub struct SoundChunk {
    pub len: u32,
    #[allow(dead_code)]
    pub offset: u32,
    #[allow(dead_code)]
    pub block_size: u32,
}

impl ParseChunk for SoundChunk {
    fn parse<B: ReadBytes>(reader: &mut B, _: [u8; 4], len: u32) -> Result<SoundChunk> {
        let offset = reader.read_be_u32()?;
        let block_size = reader.read_be_u32()?;

        if offset != 0 || block_size != 0 {
            return unsupported_error("riff: No support for AIFF block-aligned data");
        }

        Ok(SoundChunk { len, offset, block_size })
    }
}

pub enum RiffAiffChunks {
    Common(ChunkParser<CommonChunk>),
    Sound(ChunkParser<SoundChunk>),
}

macro_rules! parser {
    ($class:expr, $result:ty, $tag:expr, $len:expr) => {
        Some($class(ChunkParser::<$result>::new($tag, $len)))
    };
}

impl ParseChunkTag for RiffAiffChunks {
    fn parse_tag(tag: [u8; 4], len: u32) -> Option<Self> {
        match &tag {
            b"COMM" => parser!(RiffAiffChunks::Common, CommonChunk, tag, len),
            b"SSND" => parser!(RiffAiffChunks::Sound, SoundChunk, tag, len),
            _ => None,
        }
    }
}
