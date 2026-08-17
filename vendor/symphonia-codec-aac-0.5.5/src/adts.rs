// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::errors::unsupported_error;
use symphonia_core::support_format;

use symphonia_core::audio::Channels;
use symphonia_core::codecs::{CodecParameters, CODEC_TYPE_AAC};
use symphonia_core::errors::{decode_error, seek_error, Result, SeekErrorKind};
use symphonia_core::formats::prelude::*;
use symphonia_core::io::*;
use symphonia_core::meta::{Metadata, MetadataLog};
use symphonia_core::probe::{Descriptor, Instantiate, QueryDescriptor};

use std::io::{Seek, SeekFrom};

use super::common::{map_channels, M4AType, AAC_SAMPLE_RATES, M4A_TYPES};

use log::{debug, info};

const SAMPLES_PER_AAC_PACKET: u64 = 1024;

/// Audio Data Transport Stream (ADTS) format reader.
///
/// `AdtsReader` implements a demuxer for ADTS (AAC native frames).
pub struct AdtsReader {
    reader: MediaSourceStream,
    tracks: Vec<Track>,
    cues: Vec<Cue>,
    metadata: MetadataLog,
    first_frame_pos: u64,
    next_packet_ts: u64,
}

impl QueryDescriptor for AdtsReader {
    fn query() -> &'static [Descriptor] {
        &[support_format!(
            "aac",
            "Audio Data Transport Stream (native AAC)",
            &["aac"],
            &["audio/aac"],
            &[&[0xff, 0xf1]]
        )]
    }

    fn score(_context: &[u8]) -> u8 {
        255
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct AdtsHeader {
    profile: M4AType,
    channels: Option<Channels>,
    sample_rate: u32,
    frame_len: usize,
}

impl AdtsHeader {
    const SIZE: usize = 7;

    fn sync<B: ReadBytes>(reader: &mut B) -> Result<()> {
        let mut sync = 0u16;

        while sync != 0xfff1 {
            sync = (sync << 8) | u16::from(reader.read_u8()?);
        }

        Ok(())
    }

    fn read<B: ReadBytes>(reader: &mut B) -> Result<Self> {
        AdtsHeader::sync(reader)?;

        // The header may be 5 or 7 bytes (without or with protection).
        let mut buf = [0u8; 7];
        reader.read_buf_exact(&mut buf[..5])?;

        let mut bs = BitReaderLtr::new(&buf);

        // Profile
        let profile = M4A_TYPES[bs.read_bits_leq32(2)? as usize + 1];

        // Sample rate index.
        let sample_rate = match bs.read_bits_leq32(4)? as usize {
            15 => return decode_error("adts: forbidden sample rate"),
            13 | 14 => return decode_error("adts: reserved sample rate"),
            idx => AAC_SAMPLE_RATES[idx],
        };

        // Private bit.
        bs.ignore_bit()?;

        // Channel configuration
        let channels = match bs.read_bits_leq32(3)? {
            0 => None,
            idx => map_channels(idx),
        };

        // Originality, Home, Copyrighted ID bit, Copyright ID start bits. Only used for encoding.
        bs.ignore_bits(4)?;

        // Frame length = Header size (7) + AAC frame size
        let frame_len = bs.read_bits_leq32(13)? as usize;

        if frame_len < AdtsHeader::SIZE {
            return decode_error("adts: invalid adts frame length");
        }

        let _fullness = bs.read_bits_leq32(11)?;
        let num_aac_frames = bs.read_bits_leq32(2)? + 1;

        // TODO: Support multiple AAC packets per ADTS packet.
        if num_aac_frames > 1 {
            return unsupported_error("adts: only 1 aac frame per adts packet is supported");
        }

        Ok(AdtsHeader { profile, channels, sample_rate, frame_len: frame_len - AdtsHeader::SIZE })
    }
}

impl FormatReader for AdtsReader {
    fn try_new(mut source: MediaSourceStream, _options: &FormatOptions) -> Result<Self> {
        let header = AdtsHeader::read(&mut source)?;

        // Use the header to populate the codec parameters.
        let mut params = CodecParameters::new();

        params
            .for_codec(CODEC_TYPE_AAC)
            .with_sample_rate(header.sample_rate)
            .with_time_base(TimeBase::new(1, header.sample_rate));

        if let Some(channels) = header.channels {
            params.with_channels(channels);
        }

        // Rewind back to the start of the frame.
        source.seek_buffered_rev(AdtsHeader::SIZE);

        let first_frame_pos = source.pos();

        let n_frames = approximate_frame_count(&mut source)?;
        if let Some(n_frames) = n_frames {
            info!("estimating duration from bitrate, may be inaccurate for vbr files");
            params.with_n_frames(n_frames);
        }

        Ok(AdtsReader {
            reader: source,
            tracks: vec![Track::new(0, params)],
            cues: Vec::new(),
            metadata: Default::default(),
            first_frame_pos,
            next_packet_ts: 0,
        })
    }

    fn next_packet(&mut self) -> Result<Packet> {
        // Parse the header to get the calculated frame size.
        let header = AdtsHeader::read(&mut self.reader)?;

        // TODO: Support multiple AAC packets per ADTS packet.

        let ts = self.next_packet_ts;

        self.next_packet_ts += SAMPLES_PER_AAC_PACKET;

        Ok(Packet::new_from_boxed_slice(
            0,
            ts,
            SAMPLES_PER_AAC_PACKET,
            self.reader.read_boxed_slice_exact(header.frame_len)?,
        ))
    }

    fn metadata(&mut self) -> Metadata<'_> {
        self.metadata.metadata()
    }

    fn cues(&self) -> &[Cue] {
        &self.cues
    }

    fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    fn seek(&mut self, _mode: SeekMode, to: SeekTo) -> Result<SeekedTo> {
        // Get the timestamp of the desired audio frame.
        let required_ts = match to {
            // Frame timestamp given.
            SeekTo::TimeStamp { ts, .. } => ts,
            // Time value given, calculate frame timestamp from sample rate.
            SeekTo::Time { time, .. } => {
                // Use the sample rate to calculate the frame timestamp. If sample rate is not
                // known, the seek cannot be completed.
                if let Some(sample_rate) = self.tracks[0].codec_params.sample_rate {
                    TimeBase::new(1, sample_rate).calc_timestamp(time)
                }
                else {
                    return seek_error(SeekErrorKind::Unseekable);
                }
            }
        };

        debug!("seeking to ts={}", required_ts);

        // If the desired timestamp is less-than the next packet timestamp, attempt to seek
        // to the start of the stream.
        if required_ts < self.next_packet_ts {
            // If the reader is not seekable then only forward seeks are possible.
            if self.reader.is_seekable() {
                let seeked_pos = self.reader.seek(SeekFrom::Start(self.first_frame_pos))?;

                // Since the elementary stream has no timestamp information, the position seeked
                // to must be exactly as requested.
                if seeked_pos != self.first_frame_pos {
                    return seek_error(SeekErrorKind::Unseekable);
                }
            }
            else {
                return seek_error(SeekErrorKind::ForwardOnly);
            }

            // Successfuly seeked to the start of the stream, reset the next packet timestamp.
            self.next_packet_ts = 0;
        }

        // Parse frames from the stream until the frame containing the desired timestamp is
        // reached.
        loop {
            // Parse the next frame header.
            let header = AdtsHeader::read(&mut self.reader)?;

            // TODO: Support multiple AAC packets per ADTS packet.

            // If the next frame's timestamp would exceed the desired timestamp, rewind back to the
            // start of this frame and end the search.
            if self.next_packet_ts + SAMPLES_PER_AAC_PACKET > required_ts {
                self.reader.seek_buffered_rev(AdtsHeader::SIZE);
                break;
            }

            // Otherwise, ignore the frame body.
            self.reader.ignore_bytes(header.frame_len as u64)?;

            // Increment the timestamp for the next packet.
            self.next_packet_ts += SAMPLES_PER_AAC_PACKET;
        }

        debug!(
            "seeked to ts={} (delta={})",
            self.next_packet_ts,
            required_ts as i64 - self.next_packet_ts as i64
        );

        Ok(SeekedTo { track_id: 0, required_ts, actual_ts: self.next_packet_ts })
    }

    fn into_inner(self: Box<Self>) -> MediaSourceStream {
        self.reader
    }
}

fn approximate_frame_count(mut source: &mut MediaSourceStream) -> Result<Option<u64>> {
    let original_pos = source.pos();
    let total_len = match source.byte_len() {
        Some(len) => len - original_pos,
        _ => return Ok(None),
    };

    let mut parsed_n_frames = 0;
    let mut n_bytes = 0;

    if !source.is_seekable() {
        // The maximum length in bytes of frames to consume from the stream to sample.
        const MAX_LEN: u64 = 16 * 1024;

        source.ensure_seekback_buffer(MAX_LEN as usize);
        let mut scoped_stream = ScopedStream::new(&mut source, MAX_LEN);

        while let Ok(header) = AdtsHeader::read(&mut scoped_stream) {
            if scoped_stream.ignore_bytes(header.frame_len as u64).is_err() {
                break;
            }

            parsed_n_frames += 1;
            n_bytes += header.frame_len + AdtsHeader::SIZE;
        }

        let _ = source.seek_buffered(original_pos);
    }
    else {
        // The number of points to sample within the stream.
        const NUM_SAMPLE_POINTS: u64 = 4;

        let step = (total_len - original_pos) / NUM_SAMPLE_POINTS;

        // Skip the first sample point (start of file) since it is an outlier.
        for new_pos in (original_pos..total_len - step).step_by(step as usize).skip(1) {
            let res = source.seek(SeekFrom::Start(new_pos));
            if res.is_err() {
                break;
            }

            for _ in 0..=100 {
                let header = match AdtsHeader::read(&mut source) {
                    Ok(header) => header,
                    _ => break,
                };

                if source.ignore_bytes(header.frame_len as u64).is_err() {
                    break;
                }

                parsed_n_frames += 1;
                n_bytes += header.frame_len + AdtsHeader::SIZE;
            }
        }

        let _ = source.seek(SeekFrom::Start(original_pos))?;
    }

    debug!("adts: Parsed {} of {} bytes to approximate duration", n_bytes, total_len);

    match parsed_n_frames {
        0 => Ok(None),
        _ => Ok(Some(total_len / (n_bytes as u64 / parsed_n_frames) * SAMPLES_PER_AAC_PACKET)),
    }
}
