// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io::{Seek, SeekFrom};

use symphonia_core::codecs::CodecParameters;
use symphonia_core::errors::{seek_error, unsupported_error};
use symphonia_core::errors::{Result, SeekErrorKind};
use symphonia_core::formats::prelude::*;
use symphonia_core::io::*;
use symphonia_core::meta::{Metadata, MetadataLog};
use symphonia_core::probe::{Descriptor, Instantiate, QueryDescriptor};
use symphonia_core::support_format;

use log::debug;

use crate::common::{
    append_data_params, append_format_params, next_packet, ByteOrder, ChunksReader, PacketInfo,
};
mod chunks;
use chunks::*;

/// Aiff is actually a RIFF stream, with a "FORM" ASCII stream marker.
const AIFF_STREAM_MARKER: [u8; 4] = *b"FORM";
/// A possible RIFF form is "aiff".
const AIFF_RIFF_FORM: [u8; 4] = *b"AIFF";
/// A possible RIFF form is "aifc", using compressed data.
const AIFC_RIFF_FORM: [u8; 4] = *b"AIFC";

/// Audio Interchange File Format (AIFF) format reader.
///
/// `AiffReader` implements a demuxer for the AIFF container format.
pub struct AiffReader {
    reader: MediaSourceStream,
    tracks: Vec<Track>,
    cues: Vec<Cue>,
    metadata: MetadataLog,
    packet_info: PacketInfo,
    data_start_pos: u64,
    data_end_pos: u64,
}

impl QueryDescriptor for AiffReader {
    fn query() -> &'static [Descriptor] {
        &[
            // AIFF RIFF form
            support_format!(
                "riff",
                " Resource Interchange File Format",
                &["aiff", "aif", "aifc"],
                &["audio/aiff", "audio/x-aiff", " sound/aiff", "audio/x-pn-aiff"],
                &[b"FORM"]
            ),
        ]
    }

    fn score(_context: &[u8]) -> u8 {
        255
    }
}

impl FormatReader for AiffReader {
    fn try_new(mut source: MediaSourceStream, _options: &FormatOptions) -> Result<Self> {
        // The FORM marker should be present.
        let marker = source.read_quad_bytes()?;
        if marker != AIFF_STREAM_MARKER {
            return unsupported_error("aiff: missing riff stream marker");
        }

        // File is basically one RIFF chunk, with the actual meta and audio data as sub-chunks (called local chunks).
        // Therefore, the header was the chunk ID, and the next 4 bytes is the length of the RIFF
        // chunk.
        let riff_len = source.read_be_u32()?;
        let riff_form = source.read_quad_bytes()?;

        let mut riff_chunks = ChunksReader::<RiffAiffChunks>::new(riff_len, ByteOrder::BigEndian);

        let mut codec_params = CodecParameters::new();
        //TODO: Chunks such as marker contain metadata, get it.
        let metadata: MetadataLog = Default::default();
        let mut packet_info = PacketInfo::without_blocks(0);

        loop {
            let chunk = riff_chunks.next(&mut source)?;

            // The last chunk should always be a data chunk, if it is not, then the stream is
            // unsupported.
            // TODO: According to the spec additional chunks can be added after the sound data chunk. In fact any order can be possible.
            if chunk.is_none() {
                return unsupported_error("aiff: missing sound chunk");
            }

            match chunk.unwrap() {
                RiffAiffChunks::Common(common) => {
                    let common = match riff_form {
                        AIFF_RIFF_FORM => common.parse_aiff(&mut source)?,
                        AIFC_RIFF_FORM => common.parse_aifc(&mut source)?,
                        _ => return unsupported_error("aiff: riff form is not supported"),
                    };

                    // The Format chunk contains the block_align field and possible additional information
                    // to handle packetization and seeking.
                    packet_info = common.packet_info()?;
                    codec_params
                        .with_max_frames_per_packet(packet_info.get_max_frames_per_packet())
                        .with_frames_per_block(packet_info.frames_per_block);

                    // Append Format chunk fields to codec parameters.
                    append_format_params(
                        &mut codec_params,
                        &common.format_data,
                        common.sample_rate,
                    );
                }
                RiffAiffChunks::Sound(dat) => {
                    let data = dat.parse(&mut source)?;

                    // Record the bounds of the data chunk.
                    let data_start_pos = source.pos();
                    let data_end_pos = data_start_pos + u64::from(data.len);

                    // Append Sound chunk fields to codec parameters.
                    append_data_params(&mut codec_params, data.len as u64, &packet_info);

                    // Add a new track using the collected codec parameters.
                    return Ok(AiffReader {
                        reader: source,
                        tracks: vec![Track::new(0, codec_params)],
                        cues: Vec::new(),
                        metadata,
                        packet_info,
                        data_start_pos,
                        data_end_pos,
                    });
                }
            }
        }
    }

    fn next_packet(&mut self) -> Result<Packet> {
        next_packet(
            &mut self.reader,
            &self.packet_info,
            &self.tracks,
            self.data_start_pos,
            self.data_end_pos,
        )
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
        if self.tracks.is_empty() || self.packet_info.is_empty() {
            return seek_error(SeekErrorKind::Unseekable);
        }

        let params = &self.tracks[0].codec_params;

        let ts = match to {
            // Frame timestamp given.
            SeekTo::TimeStamp { ts, .. } => ts,
            // Time value given, calculate frame timestamp from sample rate.
            SeekTo::Time { time, .. } => {
                // Use the sample rate to calculate the frame timestamp. If sample rate is not
                // known, the seek cannot be completed.
                if let Some(sample_rate) = params.sample_rate {
                    TimeBase::new(1, sample_rate).calc_timestamp(time)
                }
                else {
                    return seek_error(SeekErrorKind::Unseekable);
                }
            }
        };

        // If the total number of frames in the track is known, verify the desired frame timestamp
        // does not exceed it.
        if let Some(n_frames) = params.n_frames {
            if ts > n_frames {
                return seek_error(SeekErrorKind::OutOfRange);
            }
        }

        debug!("seeking to frame_ts={}", ts);

        // RIFF is not internally packetized for PCM codecs. Packetization is simulated by trying to
        // read a constant number of samples or blocks every call to next_packet. Therefore, a packet begins
        // wherever the data stream is currently positioned. Since timestamps on packets should be
        // determinstic, instead of seeking to the exact timestamp requested and starting the next
        // packet there, seek to a packet boundary. In this way, packets will have have the same
        // timestamps regardless if the stream was seeked or not.
        let actual_ts = self.packet_info.get_actual_ts(ts);

        // Calculate the absolute byte offset of the desired audio frame.
        let seek_pos = self.data_start_pos + (actual_ts * self.packet_info.block_size);

        // If the reader supports seeking we can seek directly to the frame's offset wherever it may
        // be.
        if self.reader.is_seekable() {
            self.reader.seek(SeekFrom::Start(seek_pos))?;
        }
        // If the reader does not support seeking, we can only emulate forward seeks by consuming
        // bytes. If the reader has to seek backwards, return an error.
        else {
            let current_pos = self.reader.pos();
            if seek_pos >= current_pos {
                self.reader.ignore_bytes(seek_pos - current_pos)?;
            }
            else {
                return seek_error(SeekErrorKind::ForwardOnly);
            }
        }

        debug!("seeked to packet_ts={} (delta={})", actual_ts, actual_ts as i64 - ts as i64);

        Ok(SeekedTo { track_id: 0, actual_ts, required_ts: ts })
    }

    fn into_inner(self: Box<Self>) -> MediaSourceStream {
        self.reader
    }
}
