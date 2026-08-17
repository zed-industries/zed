// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

use symphonia_core::audio::Channels;
use symphonia_core::codecs::CodecParameters;
use symphonia_core::codecs::CodecType;
use symphonia_core::codecs::{
    CODEC_TYPE_ADPCM_IMA_WAV, CODEC_TYPE_ADPCM_MS, CODEC_TYPE_PCM_ALAW, CODEC_TYPE_PCM_F32LE,
    CODEC_TYPE_PCM_F64LE, CODEC_TYPE_PCM_MULAW, CODEC_TYPE_PCM_S16LE, CODEC_TYPE_PCM_S24LE,
    CODEC_TYPE_PCM_S32LE, CODEC_TYPE_PCM_U8,
};
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::io::{MediaSourceStream, ReadBytes};
use symphonia_core::meta::{MetadataBuilder, MetadataRevision, Tag};
use symphonia_metadata::riff;

use crate::common::{
    fix_channel_mask, try_channel_count_to_mask, ByteOrder, ChunkParser, ChunksReader, FormatALaw,
    FormatAdpcm, FormatData, FormatExtensible, FormatIeeeFloat, FormatMuLaw, FormatPcm, NullChunks,
    PacketInfo, ParseChunk, ParseChunkTag,
};

pub struct WaveFormatChunk {
    /// The number of channels.
    pub n_channels: u16,
    /// The sample rate in Hz. For non-PCM formats, this value must be interpreted as per the
    /// format's specifications.
    pub sample_rate: u32,
    /// The required average data rate required in bytes/second. For non-PCM formats, this value
    /// must be interpreted as per the format's specifications.
    pub avg_bytes_per_sec: u32,
    /// The byte alignment of one audio frame. For PCM formats, this is equal to
    /// `(n_channels * format_data.bits_per_sample) / 8`. For non-PCM formats, this value must be
    /// interpreted as per the format's specifications.
    pub block_align: u16,
    /// Extra data associated with the format block conditional upon the format tag.
    pub format_data: FormatData,
}

impl WaveFormatChunk {
    fn read_pcm_fmt<B: ReadBytes>(
        reader: &mut B,
        bits_per_sample: u16,
        n_channels: u16,
        len: u32,
    ) -> Result<FormatData> {
        // WaveFormat for a PCM format may be extended with an extra data length field followed by
        // the extension data itself. Use the chunk length to determine if the format chunk is
        // extended.
        match len {
            // Basic WavFormat struct, no extension.
            16 => (),
            // WaveFormatEx with extension data length field present, but no extension data.
            18 => {
                // Extension data length should be 0.
                let _extension_len = reader.read_be_u16()?;
            }
            // WaveFormatEx with extension data length field present, and extension data.
            40 => {
                // Extension data length should be either 0 or 22 (if valid data is present).
                let _extension_len = reader.read_u16()?;
                reader.ignore_bytes(22)?;
            }
            _ => return decode_error("wav: malformed fmt_pcm chunk"),
        }

        // Bits per sample for PCM is both the encoded sample width, and the actual sample width.
        // Strictly, this must either be 8 or 16 bits, but there is no reason why 24 and 32 bits
        // can't be supported. Since these files do exist, allow for 8/16/24/32-bit samples, but
        // error if not a multiple of 8 or greater than 32-bits.
        //
        // Select the appropriate codec using bits per sample. Samples are always interleaved and
        // little-endian encoded for the PCM format.
        let codec = match bits_per_sample {
            8 => CODEC_TYPE_PCM_U8,
            16 => CODEC_TYPE_PCM_S16LE,
            24 => CODEC_TYPE_PCM_S24LE,
            32 => CODEC_TYPE_PCM_S32LE,
            _ => {
                return decode_error(
                    "wav: bits per sample for fmt_pcm must be 8, 16, 24 or 32 bits",
                )
            }
        };

        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::Pcm(FormatPcm { bits_per_sample, channels, codec }))
    }

    fn read_adpcm_fmt<B: ReadBytes>(
        reader: &mut B,
        bits_per_sample: u16,
        n_channels: u16,
        len: u32,
        codec: CodecType,
    ) -> Result<FormatData> {
        if bits_per_sample != 4 {
            return decode_error("wav: bits per sample for fmt_adpcm must be 4 bits");
        }

        // WaveFormatEx with extension data length field present and with atleast frames per block data.
        if len < 20 {
            return decode_error("wav: malformed fmt_adpcm chunk");
        }

        let extra_size = reader.read_u16()? as u64;

        match codec {
            CODEC_TYPE_ADPCM_MS if extra_size < 32 => {
                return decode_error("wav: malformed fmt_adpcm chunk");
            }
            CODEC_TYPE_ADPCM_IMA_WAV if extra_size != 2 => {
                return decode_error("wav: malformed fmt_adpcm chunk");
            }
            _ => (),
        }
        reader.ignore_bytes(extra_size)?;

        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::Adpcm(FormatAdpcm { bits_per_sample, channels, codec }))
    }

    fn read_ieee_fmt<B: ReadBytes>(
        reader: &mut B,
        bits_per_sample: u16,
        n_channels: u16,
        len: u32,
    ) -> Result<FormatData> {
        // WaveFormat for a IEEE format should not be extended, but it may still have an extra data
        // length parameter.
        match len {
            16 => (),
            18 => {
                let extra_size = reader.read_u16()?;
                if extra_size != 0 {
                    return decode_error("wav: extra data not expected for fmt_ieee chunk");
                }
            }
            40 => {
                // WAVEFORMATEXTENSIBLE is used for formats having more than two channels
                // or higher sample resolutions than allowed by WAVEFORMATEX but for now
                // we just ignore it
                let _ = reader.ignore_bytes(40 - 16);
            }
            _ => return decode_error("wav: malformed fmt_ieee chunk"),
        }

        // Officially, only 32-bit floats are supported, but Symphonia can handle 64-bit floats.
        //
        // Select the appropriate codec using bits per sample. Samples are always interleaved and
        // little-endian encoded for the IEEE Float format.
        let codec = match bits_per_sample {
            32 => CODEC_TYPE_PCM_F32LE,
            64 => CODEC_TYPE_PCM_F64LE,
            _ => return decode_error("wav: bits per sample for fmt_ieee must be 32 or 64 bits"),
        };

        let channels = try_channel_count_to_mask(n_channels)?;

        Ok(FormatData::IeeeFloat(FormatIeeeFloat { channels, codec }))
    }

    fn read_ext_fmt<B: ReadBytes>(
        reader: &mut B,
        bits_per_coded_sample: u16,
        n_channels: u16,
        len: u32,
    ) -> Result<FormatData> {
        // WaveFormat for the extensible format must be extended to 40 bytes in length.
        if len < 40 {
            return decode_error("wav: malformed fmt_ext chunk");
        }

        let extra_size = reader.read_u16()?;

        // The size of the extra data for the Extensible format is exactly 22 bytes.
        if extra_size != 22 {
            return decode_error("wav: extra data size not 22 bytes for fmt_ext chunk");
        }

        let bits_per_sample = reader.read_u16()?;

        // Bits per coded sample for extensible formats is the width per sample as stored in the
        // stream. This must be a multiple of 8.
        if (bits_per_coded_sample & 0x7) != 0 {
            return decode_error("wav: bits per coded sample for fmt_ext must be a multiple of 8");
        }

        // Bits per sample indicates the number of valid bits in the encoded sample. The sample is
        // encoded in a bits per coded sample width value, therefore the valid number of bits must
        // be at most bits per coded sample long.
        if bits_per_sample > bits_per_coded_sample {
            return decode_error(
                "wav: bits per sample must be <= bits per coded sample for fmt_ext",
            );
        }

        let channel_mask = fix_channel_mask(reader.read_u32()?, n_channels);

        // Try to map channels.
        let channels = match Channels::from_bits(channel_mask) {
            Some(channels) => channels,
            _ => return unsupported_error("wav: too many channels in mask for fmt_ext"),
        };

        let mut sub_format_guid = [0u8; 16];
        reader.read_buf_exact(&mut sub_format_guid)?;

        // These GUIDs identifiy the format of the data chunks. These definitions can be found in
        // ksmedia.h of the Microsoft Windows Platform SDK.
        #[rustfmt::skip]
        const KSDATAFORMAT_SUBTYPE_PCM: [u8; 16] = [
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
            0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
        ];
        // #[rustfmt::skip]
        // const KSDATAFORMAT_SUBTYPE_ADPCM: [u8; 16] = [
        //     0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
        //     0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
        // ];
        #[rustfmt::skip]
        const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: [u8; 16] = [
            0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
            0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
        ];
        #[rustfmt::skip]
        const KSDATAFORMAT_SUBTYPE_ALAW: [u8; 16] = [
            0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
            0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
        ];
        #[rustfmt::skip]
        const KSDATAFORMAT_SUBTYPE_MULAW: [u8; 16] = [
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00,
            0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
        ];
        #[rustfmt::skip]
        const KSDATAFORMAT_SUBTYPE_AMBISONIC_B_FORMAT_PCM: [u8; 16] = [
            0x01, 0x00, 0x00, 0x00, 0x21, 0x07, 0xd3, 0x11,
            0x86, 0x44, 0xc8, 0xc1, 0xca, 0x00, 0x00, 0x00,
        ];
        #[rustfmt::skip]
        const KSDATAFORMAT_SUBTYPE_AMBISONIC_B_FORMAT_IEE_FLOAT: [u8; 16] = [
            0x03, 0x00, 0x00, 0x00, 0x21, 0x07, 0xd3, 0x11,
            0x86, 0x44, 0xc8, 0xc1, 0xca, 0x00, 0x00, 0x00,
        ];

        // Verify support based on the format GUID.
        let codec = match sub_format_guid {
            KSDATAFORMAT_SUBTYPE_PCM | KSDATAFORMAT_SUBTYPE_AMBISONIC_B_FORMAT_PCM => {
                // Only support up-to 32-bit integer samples.
                if bits_per_coded_sample > 32 {
                    return decode_error(
                        "bits per sample for fmt_ext PCM sub-type must be <= 32 bits",
                    );
                }

                // Use bits per coded sample to select the codec to use. If bits per sample is less
                // than the bits per coded sample, the codec will expand the sample during decode.
                match bits_per_coded_sample {
                    8 => CODEC_TYPE_PCM_U8,
                    16 => CODEC_TYPE_PCM_S16LE,
                    24 => CODEC_TYPE_PCM_S24LE,
                    32 => CODEC_TYPE_PCM_S32LE,
                    _ => unreachable!(),
                }
            }
            KSDATAFORMAT_SUBTYPE_IEEE_FLOAT | KSDATAFORMAT_SUBTYPE_AMBISONIC_B_FORMAT_IEE_FLOAT => {
                // IEEE floating formats do not support truncated sample widths.
                if bits_per_sample != bits_per_coded_sample {
                    return decode_error(
                        "wav: bits per sample for fmt_ext IEEE sub-type must equal bits per coded sample"
                    );
                }

                // Select the appropriate codec based on the bits per coded sample.
                match bits_per_coded_sample {
                    32 => CODEC_TYPE_PCM_F32LE,
                    64 => CODEC_TYPE_PCM_F64LE,
                    _ => {
                        return decode_error(
                            "wav: bits per sample for fmt_ext IEEE sub-type must be 32 or 64 bits",
                        )
                    }
                }
            }
            KSDATAFORMAT_SUBTYPE_ALAW => CODEC_TYPE_PCM_ALAW,
            KSDATAFORMAT_SUBTYPE_MULAW => CODEC_TYPE_PCM_MULAW,
            _ => return unsupported_error("wav: unsupported fmt_ext sub-type"),
        };

        Ok(FormatData::Extensible(FormatExtensible {
            bits_per_sample,
            bits_per_coded_sample,
            channels,
            sub_format_guid,
            codec,
        }))
    }

    fn read_alaw_pcm_fmt<B: ReadBytes>(
        reader: &mut B,
        n_channels: u16,
        len: u32,
    ) -> Result<FormatData> {
        if len != 18 {
            return decode_error("wav: malformed fmt_alaw chunk");
        }

        let extra_size = reader.read_u16()?;

        if extra_size > 0 {
            reader.ignore_bytes(u64::from(extra_size))?;
        }

        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::ALaw(FormatALaw { codec: CODEC_TYPE_PCM_ALAW, channels }))
    }

    fn read_mulaw_pcm_fmt<B: ReadBytes>(
        reader: &mut B,
        n_channels: u16,
        len: u32,
    ) -> Result<FormatData> {
        if len != 18 {
            return decode_error("wav: malformed fmt_mulaw chunk");
        }

        let extra_size = reader.read_u16()?;

        if extra_size > 0 {
            reader.ignore_bytes(u64::from(extra_size))?;
        }

        let channels = try_channel_count_to_mask(n_channels)?;
        Ok(FormatData::MuLaw(FormatMuLaw { codec: CODEC_TYPE_PCM_MULAW, channels }))
    }

    pub(crate) fn packet_info(&self) -> Result<PacketInfo> {
        match self.format_data {
            FormatData::Adpcm(FormatAdpcm { codec, bits_per_sample, .. })
            //| WaveFormatData::Extensible(WaveFormatExtensible { codec, bits_per_sample, .. })
                if codec == CODEC_TYPE_ADPCM_MS =>
            {
                let frames_per_block = ((((self.block_align - (7 * self.n_channels)) * 8)
                    / (bits_per_sample * self.n_channels))
                    + 2) as u64;
                PacketInfo::with_blocks(self.block_align, frames_per_block)
            }
            FormatData::Adpcm(FormatAdpcm { codec, bits_per_sample, .. })
                if codec == CODEC_TYPE_ADPCM_IMA_WAV =>
            {
                let frames_per_block = (((self.block_align - (4 * self.n_channels)) * 8)
                    / (bits_per_sample * self.n_channels)
                    + 1) as u64;
                PacketInfo::with_blocks(self.block_align, frames_per_block)
            }
            _ => Ok(PacketInfo::without_blocks(self.block_align)),
        }
    }
}

impl ParseChunk for WaveFormatChunk {
    fn parse<B: ReadBytes>(reader: &mut B, _tag: [u8; 4], len: u32) -> Result<WaveFormatChunk> {
        // WaveFormat has a minimal length of 16 bytes. This may be extended with format specific
        // data later.
        if len < 16 {
            return decode_error("wav: malformed fmt chunk");
        }

        let format = reader.read_u16()?;
        let n_channels = reader.read_u16()?;
        let sample_rate = reader.read_u32()?;
        let avg_bytes_per_sec = reader.read_u32()?;
        let block_align = reader.read_u16()?;
        let bits_per_sample = reader.read_u16()?;

        // The definition of these format identifiers can be found in mmreg.h of the Microsoft
        // Windows Platform SDK.
        const WAVE_FORMAT_PCM: u16 = 0x0001;
        const WAVE_FORMAT_ADPCM: u16 = 0x0002;
        const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;
        const WAVE_FORMAT_ALAW: u16 = 0x0006;
        const WAVE_FORMAT_MULAW: u16 = 0x0007;
        const WAVE_FORMAT_ADPCM_IMA: u16 = 0x0011;
        const WAVE_FORMAT_EXTENSIBLE: u16 = 0xfffe;

        let format_data = match format {
            // The PCM Wave Format
            WAVE_FORMAT_PCM => Self::read_pcm_fmt(reader, bits_per_sample, n_channels, len),
            // The Microsoft ADPCM Format
            WAVE_FORMAT_ADPCM => {
                Self::read_adpcm_fmt(reader, bits_per_sample, n_channels, len, CODEC_TYPE_ADPCM_MS)
            }
            // The IEEE Float Wave Format
            WAVE_FORMAT_IEEE_FLOAT => Self::read_ieee_fmt(reader, bits_per_sample, n_channels, len),
            // The Extensible Wave Format
            WAVE_FORMAT_EXTENSIBLE => Self::read_ext_fmt(reader, bits_per_sample, n_channels, len),
            // The Alaw Wave Format.
            WAVE_FORMAT_ALAW => Self::read_alaw_pcm_fmt(reader, n_channels, len),
            // The MuLaw Wave Format.
            WAVE_FORMAT_MULAW => Self::read_mulaw_pcm_fmt(reader, n_channels, len),
            // The IMA ADPCM Format
            WAVE_FORMAT_ADPCM_IMA => Self::read_adpcm_fmt(
                reader,
                bits_per_sample,
                n_channels,
                len,
                CODEC_TYPE_ADPCM_IMA_WAV,
            ),
            // Unsupported format.
            _ => return unsupported_error("wav: unsupported wave format"),
        }?;

        Ok(WaveFormatChunk { n_channels, sample_rate, avg_bytes_per_sec, block_align, format_data })
    }
}

impl fmt::Display for WaveFormatChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WaveFormatChunk {{")?;
        writeln!(f, "\tn_channels: {},", self.n_channels)?;
        writeln!(f, "\tsample_rate: {} Hz,", self.sample_rate)?;
        writeln!(f, "\tavg_bytes_per_sec: {},", self.avg_bytes_per_sec)?;
        writeln!(f, "\tblock_align: {},", self.block_align)?;

        match self.format_data {
            FormatData::Pcm(ref pcm) => {
                writeln!(f, "\tformat_data: Pcm {{")?;
                writeln!(f, "\t\tbits_per_sample: {},", pcm.bits_per_sample)?;
                writeln!(f, "\t\tchannels: {},", pcm.channels)?;
                writeln!(f, "\t\tcodec: {},", pcm.codec)?;
            }
            FormatData::Adpcm(ref adpcm) => {
                writeln!(f, "\tformat_data: Adpcm {{")?;
                writeln!(f, "\t\tbits_per_sample: {},", adpcm.bits_per_sample)?;
                writeln!(f, "\t\tchannels: {},", adpcm.channels)?;
                writeln!(f, "\t\tcodec: {},", adpcm.codec)?;
            }
            FormatData::IeeeFloat(ref ieee) => {
                writeln!(f, "\tformat_data: IeeeFloat {{")?;
                writeln!(f, "\t\tchannels: {},", ieee.channels)?;
                writeln!(f, "\t\tcodec: {},", ieee.codec)?;
            }
            FormatData::Extensible(ref ext) => {
                writeln!(f, "\tformat_data: Extensible {{")?;
                writeln!(f, "\t\tbits_per_sample: {},", ext.bits_per_sample)?;
                writeln!(f, "\t\tbits_per_coded_sample: {},", ext.bits_per_coded_sample)?;
                writeln!(f, "\t\tchannels: {},", ext.channels)?;
                writeln!(f, "\t\tsub_format_guid: {:?},", &ext.sub_format_guid)?;
                writeln!(f, "\t\tcodec: {},", ext.codec)?;
            }
            FormatData::ALaw(ref alaw) => {
                writeln!(f, "\tformat_data: ALaw {{")?;
                writeln!(f, "\t\tchannels: {},", alaw.channels)?;
                writeln!(f, "\t\tcodec: {},", alaw.codec)?;
            }
            FormatData::MuLaw(ref mulaw) => {
                writeln!(f, "\tformat_data: MuLaw {{")?;
                writeln!(f, "\t\tchannels: {},", mulaw.channels)?;
                writeln!(f, "\t\tcodec: {},", mulaw.codec)?;
            }
        };

        writeln!(f, "\t}}")?;
        writeln!(f, "}}")
    }
}

pub struct FactChunk {
    pub n_frames: u32,
}

impl ParseChunk for FactChunk {
    fn parse<B: ReadBytes>(reader: &mut B, _tag: [u8; 4], len: u32) -> Result<Self> {
        // A Fact chunk is exactly 4 bytes long, though there is some mystery as to whether there
        // can be more fields in the chunk.
        if len != 4 {
            return decode_error("wav: malformed fact chunk");
        }

        Ok(FactChunk { n_frames: reader.read_u32()? })
    }
}

impl fmt::Display for FactChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FactChunk {{")?;
        writeln!(f, "\tn_frames: {},", self.n_frames)?;
        writeln!(f, "}}")
    }
}

pub struct ListChunk {
    pub form: [u8; 4],
    pub len: u32,
}

impl ListChunk {
    pub fn skip<B: ReadBytes>(&self, reader: &mut B) -> Result<()> {
        ChunksReader::<NullChunks>::new(self.len, ByteOrder::LittleEndian).finish(reader)
    }
}

impl ParseChunk for ListChunk {
    fn parse<B: ReadBytes>(reader: &mut B, _tag: [u8; 4], len: u32) -> Result<Self> {
        // A List chunk must contain atleast the list/form identifier. However, an empty list
        // (len == 4) is permissible.
        if len < 4 {
            return decode_error("wav: malformed list chunk");
        }

        Ok(ListChunk { form: reader.read_quad_bytes()?, len: len - 4 })
    }
}

impl fmt::Display for ListChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ListChunk {{")?;
        writeln!(f, "\tform: {},", String::from_utf8_lossy(&self.form))?;
        writeln!(f, "\tlen: {},", self.len)?;
        writeln!(f, "}}")
    }
}

pub struct InfoChunk {
    pub tag: Tag,
}

impl ParseChunk for InfoChunk {
    fn parse<B: ReadBytes>(reader: &mut B, tag: [u8; 4], len: u32) -> Result<InfoChunk> {
        // TODO: Apply limit.
        let mut value_buf = vec![0u8; len as usize];
        reader.read_buf_exact(&mut value_buf)?;

        Ok(InfoChunk { tag: riff::parse(tag, &value_buf) })
    }
}

pub struct DataChunk {
    pub len: u32,
}

impl ParseChunk for DataChunk {
    fn parse<B: ReadBytes>(_: &mut B, _: [u8; 4], len: u32) -> Result<DataChunk> {
        Ok(DataChunk { len })
    }
}

pub enum RiffWaveChunks {
    Format(ChunkParser<WaveFormatChunk>),
    List(ChunkParser<ListChunk>),
    Fact(ChunkParser<FactChunk>),
    Data(ChunkParser<DataChunk>),
}

macro_rules! parser {
    ($class:expr, $result:ty, $tag:expr, $len:expr) => {
        Some($class(ChunkParser::<$result>::new($tag, $len)))
    };
}

impl ParseChunkTag for RiffWaveChunks {
    fn parse_tag(tag: [u8; 4], len: u32) -> Option<Self> {
        match &tag {
            b"fmt " => parser!(RiffWaveChunks::Format, WaveFormatChunk, tag, len),
            b"LIST" => parser!(RiffWaveChunks::List, ListChunk, tag, len),
            b"fact" => parser!(RiffWaveChunks::Fact, FactChunk, tag, len),
            b"data" => parser!(RiffWaveChunks::Data, DataChunk, tag, len),
            _ => None,
        }
    }
}

pub enum RiffInfoListChunks {
    Info(ChunkParser<InfoChunk>),
}

impl ParseChunkTag for RiffInfoListChunks {
    fn parse_tag(tag: [u8; 4], len: u32) -> Option<Self> {
        // Right now it is assumed all list chunks are INFO chunks, but that's not really
        // guaranteed.
        //
        // TODO: Actually validate that the chunk is an info chunk.
        parser!(RiffInfoListChunks::Info, InfoChunk, tag, len)
    }
}

pub fn append_fact_params(codec_params: &mut CodecParameters, fact: &FactChunk) {
    codec_params.with_n_frames(u64::from(fact.n_frames));
}

pub fn read_info_chunk(source: &mut MediaSourceStream, len: u32) -> Result<MetadataRevision> {
    let mut info_list = ChunksReader::<RiffInfoListChunks>::new(len, ByteOrder::LittleEndian);

    let mut metadata_builder = MetadataBuilder::new();

    loop {
        let chunk = info_list.next(source)?;

        if let Some(RiffInfoListChunks::Info(info)) = chunk {
            let parsed_info = info.parse(source)?;
            metadata_builder.add_tag(parsed_info.tag);
        }
        else {
            break;
        }
    }

    info_list.finish(source)?;

    Ok(metadata_builder.metadata())
}
