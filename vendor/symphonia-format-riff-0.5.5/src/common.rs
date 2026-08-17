// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// `PacketInfo` helps to simulate packetization over a number of blocks of data.
/// In case the codec is blockless the block size equals one full audio frame in bytes.
use std::marker::PhantomData;

use symphonia_core::audio::Channels;
use symphonia_core::codecs::CodecParameters;
use symphonia_core::codecs::CodecType;
use symphonia_core::errors::{decode_error, end_of_stream_error, Error, Result};
use symphonia_core::formats::prelude::*;
use symphonia_core::io::{MediaSourceStream, ReadBytes};

use log::{debug, info};

pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

/// The maximum number of frames that will be in a packet.
/// Since there are no real packets in AIFF, this is arbitrary, used same value as MP3.
const MAX_FRAMES_PER_PACKET: u64 = 1152;

/// `ParseChunkTag` implements `parse_tag` to map between the 4-byte chunk identifier and the
/// enumeration
pub trait ParseChunkTag: Sized {
    fn parse_tag(tag: [u8; 4], len: u32) -> Option<Self>;
}

pub enum NullChunks {}

impl ParseChunkTag for NullChunks {
    fn parse_tag(_tag: [u8; 4], _len: u32) -> Option<Self> {
        None
    }
}

pub fn fix_channel_mask(mut channel_mask: u32, n_channels: u16) -> u32 {
    let channel_diff = n_channels as i32 - channel_mask.count_ones() as i32;

    if channel_diff != 0 {
        info!("Channel mask not set correctly, channel positions may be incorrect!");
    }

    // Check that the number of ones in the channel mask match the number of channels.
    if channel_diff > 0 {
        // Too few ones in mask so add extra ones above the most significant one
        let shift = 32 - (!channel_mask).leading_ones();
        channel_mask |= ((1 << channel_diff) - 1) << shift;
    }
    else {
        // Too many ones in mask so remove the most significant extra ones
        while channel_mask.count_ones() != n_channels as u32 {
            let highest_one = 31 - (!channel_mask).leading_ones();
            channel_mask &= !(1 << highest_one);
        }
    }

    channel_mask
}

#[test]
fn test_fix_channel_mask() {
    // Too few
    assert_eq!(fix_channel_mask(0, 9), 0b111111111);
    assert_eq!(fix_channel_mask(0b101000, 5), 0b111101000);

    // Too many
    assert_eq!(fix_channel_mask(0b1111111, 0), 0);
    assert_eq!(fix_channel_mask(0b101110111010, 5), 0b10111010);
    assert_eq!(fix_channel_mask(0xFFFFFFFF, 8), 0b11111111);
}

pub fn try_channel_count_to_mask(count: u16) -> Result<Channels> {
    (1..=32)
        .contains(&count)
        .then(|| Channels::from_bits(((1u64 << count) - 1) as u32))
        .flatten()
        .ok_or(Error::DecodeError("riff: invalid channel count"))
}

#[test]
fn test_try_channel_count_to_mask() {
    assert!(try_channel_count_to_mask(0).is_err());

    for i in 1..27 {
        assert!(try_channel_count_to_mask(i).is_ok());
    }

    for i in 27..u16::MAX {
        assert!(try_channel_count_to_mask(i).is_err());
    }
}

/// `ChunksReader` reads chunks from a `ByteStream`. It is generic across a type, usually an enum,
/// implementing the `ParseChunkTag` trait. When a new chunk is encountered in the stream,
/// `parse_tag` on T is called to return an object capable of parsing/reading that chunk or `None`.
/// This makes reading the actual chunk data lazy in that the  chunk is not read until the object is
/// consumed.
pub struct ChunksReader<T: ParseChunkTag> {
    len: u32,
    byte_order: ByteOrder,
    consumed: u32,
    phantom: PhantomData<T>,
}

impl<T: ParseChunkTag> ChunksReader<T> {
    pub fn new(len: u32, byte_order: ByteOrder) -> Self {
        ChunksReader { len, byte_order, consumed: 0, phantom: PhantomData }
    }

    pub fn next<B: ReadBytes>(&mut self, reader: &mut B) -> Result<Option<T>> {
        // Loop until a chunk is recognized and returned, or the end of stream is reached.
        loop {
            // Align to the next 2-byte boundary if not currently aligned.
            if self.consumed & 0x1 == 1 {
                reader.read_u8()?;
                self.consumed += 1;
            }

            // Check if there are enough bytes for another chunk, if not, there are no more chunks.
            if self.consumed + 8 > self.len {
                return Ok(None);
            }

            // Read tag and len, the chunk header.
            let tag = reader.read_quad_bytes()?;

            let len = match self.byte_order {
                ByteOrder::LittleEndian => reader.read_u32()?,
                ByteOrder::BigEndian => reader.read_be_u32()?,
            };

            self.consumed += 8;

            // Check if the ChunkReader has enough unread bytes to fully read the chunk.
            //
            // Warning: the formulation of this conditional is critical because len is untrusted
            // input, it may overflow when if added to anything.
            if self.len - self.consumed < len {
                // When ffmpeg encodes wave to stdout the riff (parent) and data chunk lengths are
                // (2^32)-1 since the size can't be known ahead of time.
                if !(self.len == len && len == u32::MAX) {
                    debug!(
                        "chunk length of {} exceeds parent (list) chunk length",
                        String::from_utf8_lossy(&tag)
                    );
                    return decode_error("riff: chunk length exceeds parent (list) chunk length");
                }
            }

            // The length of the chunk has been validated, so "consume" the chunk.
            self.consumed = self.consumed.saturating_add(len);

            match T::parse_tag(tag, len) {
                Some(chunk) => return Ok(Some(chunk)),
                None => {
                    // As per the RIFF spec, unknown chunks are to be ignored.
                    info!(
                        "ignoring unknown chunk: tag={}, len={}.",
                        String::from_utf8_lossy(&tag),
                        len
                    );

                    reader.ignore_bytes(u64::from(len))?
                }
            }
        }
    }
    pub fn finish<B: ReadBytes>(&mut self, reader: &mut B) -> Result<()> {
        // If data is remaining in this chunk, skip it.
        if self.consumed < self.len {
            let remaining = self.len - self.consumed;
            reader.ignore_bytes(u64::from(remaining))?;
            self.consumed += remaining;
        }

        // Pad the chunk to the next 2-byte boundary.
        if self.len & 0x1 == 1 {
            reader.read_u8()?;
        }

        Ok(())
    }
}

/// Common trait implemented for all chunks that are parsed by a `ChunkParser`.
pub trait ParseChunk: Sized {
    fn parse<B: ReadBytes>(reader: &mut B, tag: [u8; 4], len: u32) -> Result<Self>;
}

/// `ChunkParser` is a utility struct for unifying the parsing of chunks.
pub struct ChunkParser<P: ParseChunk> {
    tag: [u8; 4],
    pub len: u32,
    phantom: PhantomData<P>,
}

impl<P: ParseChunk> ChunkParser<P> {
    pub fn new(tag: [u8; 4], len: u32) -> Self {
        ChunkParser { tag, len, phantom: PhantomData }
    }

    pub fn parse<B: ReadBytes>(&self, reader: &mut B) -> Result<P> {
        P::parse(reader, self.tag, self.len)
    }
}

pub enum FormatData {
    Pcm(FormatPcm),
    Adpcm(FormatAdpcm),
    IeeeFloat(FormatIeeeFloat),
    Extensible(FormatExtensible),
    ALaw(FormatALaw),
    MuLaw(FormatMuLaw),
}

pub struct FormatPcm {
    /// The number of bits per sample. In the PCM format, this is always a multiple of 8-bits.
    pub bits_per_sample: u16,
    /// Channel bitmask.
    pub channels: Channels,
    /// Codec type.
    pub codec: CodecType,
}

pub struct FormatAdpcm {
    /// The number of bits per sample. At the moment only 4bit is supported.
    pub bits_per_sample: u16,
    /// Channel bitmask.
    pub channels: Channels,
    /// Codec type.
    pub codec: CodecType,
}

pub struct FormatIeeeFloat {
    /// Channel bitmask.
    pub channels: Channels,
    /// Codec type.
    pub codec: CodecType,
}

pub struct FormatExtensible {
    /// The number of bits per sample as stored in the stream. This value is always a multiple of
    /// 8-bits.
    pub bits_per_sample: u16,
    /// The number of bits per sample that are valid. This number is always less than the number
    /// of bits per sample.
    pub bits_per_coded_sample: u16,
    /// Channel bitmask.
    pub channels: Channels,
    /// Globally unique identifier of the format.
    pub sub_format_guid: [u8; 16],
    /// Codec type.
    pub codec: CodecType,
}

pub struct FormatALaw {
    /// Channel bitmask.
    pub channels: Channels,
    /// Codec type.
    pub codec: CodecType,
}

pub struct FormatMuLaw {
    /// Channel bitmask.
    pub channels: Channels,
    /// Codec type.
    pub codec: CodecType,
}

pub struct PacketInfo {
    pub block_size: u64,
    pub frames_per_block: u64,
    pub max_blocks_per_packet: u64,
}

impl PacketInfo {
    pub fn with_blocks(block_size: u16, frames_per_block: u64) -> Result<Self> {
        if frames_per_block == 0 {
            return decode_error("riff: frames per block is 0");
        }
        Ok(Self {
            block_size: u64::from(block_size),
            frames_per_block,
            max_blocks_per_packet: frames_per_block.max(MAX_FRAMES_PER_PACKET) / frames_per_block,
        })
    }

    pub fn without_blocks(frame_len: u16) -> Self {
        Self {
            block_size: u64::from(frame_len),
            frames_per_block: 1,
            max_blocks_per_packet: MAX_FRAMES_PER_PACKET,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.block_size == 0
    }

    pub fn get_max_frames_per_packet(&self) -> u64 {
        self.max_blocks_per_packet * self.frames_per_block
    }

    pub fn get_frames(&self, data_len: u64) -> u64 {
        data_len / self.block_size * self.frames_per_block
    }

    pub fn get_actual_ts(&self, ts: u64) -> u64 {
        let max_frames_per_packet = self.get_max_frames_per_packet();
        ts / max_frames_per_packet * max_frames_per_packet
    }
}

pub fn next_packet(
    reader: &mut MediaSourceStream,
    packet_info: &PacketInfo,
    tracks: &[Track],
    data_start_pos: u64,
    data_end_pos: u64,
) -> Result<Packet> {
    let pos = reader.pos();
    if tracks.is_empty() {
        return decode_error("riff: no tracks");
    }
    if packet_info.is_empty() {
        return decode_error("riff: block size is 0");
    }

    // Determine the number of complete blocks remaining in the data chunk.
    let num_blocks_left =
        if pos < data_end_pos { (data_end_pos - pos) / packet_info.block_size } else { 0 };

    if num_blocks_left == 0 {
        return end_of_stream_error();
    }

    let blocks_per_packet = num_blocks_left.min(packet_info.max_blocks_per_packet);

    let dur = blocks_per_packet * packet_info.frames_per_block;
    let packet_len = blocks_per_packet * packet_info.block_size;

    // Copy the frames.
    let packet_buf = reader.read_boxed_slice(packet_len as usize)?;

    // The packet timestamp is the position of the first byte of the first frame in the
    // packet relative to the start of the data chunk divided by the length per frame.
    let pts = packet_info.get_frames(pos - data_start_pos);

    Ok(Packet::new_from_boxed_slice(0, pts, dur, packet_buf))
}

/// TODO: format here refers to format chunk in Wave terminology, but the data being handled here is generic - find a better name, or combine with append_data_params
pub fn append_format_params(
    codec_params: &mut CodecParameters,
    format_data: &FormatData,
    sample_rate: u32,
) {
    codec_params.with_sample_rate(sample_rate).with_time_base(TimeBase::new(1, sample_rate));

    match format_data {
        FormatData::Pcm(pcm) => {
            codec_params
                .for_codec(pcm.codec)
                .with_bits_per_coded_sample(u32::from(pcm.bits_per_sample))
                .with_bits_per_sample(u32::from(pcm.bits_per_sample))
                .with_channels(pcm.channels);
        }
        FormatData::Adpcm(adpcm) => {
            codec_params.for_codec(adpcm.codec).with_channels(adpcm.channels);
        }
        FormatData::IeeeFloat(ieee) => {
            codec_params.for_codec(ieee.codec).with_channels(ieee.channels);
        }
        FormatData::Extensible(ext) => {
            codec_params
                .for_codec(ext.codec)
                .with_bits_per_coded_sample(u32::from(ext.bits_per_coded_sample))
                .with_bits_per_sample(u32::from(ext.bits_per_sample))
                .with_channels(ext.channels);
        }
        FormatData::ALaw(alaw) => {
            codec_params.for_codec(alaw.codec).with_channels(alaw.channels);
        }
        FormatData::MuLaw(mulaw) => {
            codec_params.for_codec(mulaw.codec).with_channels(mulaw.channels);
        }
    }
}

/// TODO: format here refers to format chunk in Wave terminology, but the data being handled here is generic - find a better name, or combine with append_data_params append_format_params
pub fn append_data_params(
    codec_params: &mut CodecParameters,
    data_len: u64,
    packet_info: &PacketInfo,
) {
    if !packet_info.is_empty() {
        //let n_frames = packet_info.get_frames(u64::from(data.len));
        let n_frames = packet_info.get_frames(data_len);
        codec_params.with_n_frames(n_frames);
    }
}
