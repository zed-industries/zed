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

use log::{debug, error};

use crate::common::{
    append_data_params, append_format_params, next_packet, ByteOrder, ChunksReader, PacketInfo,
};
mod chunks;
use chunks::*;

/// WAVE is actually a RIFF stream, with a "RIFF" ASCII stream marker.
const WAVE_STREAM_MARKER: [u8; 4] = *b"RIFF";
/// A possible RIFF form is "wave".
const WAVE_RIFF_FORM: [u8; 4] = *b"WAVE";

/// Waveform Audio File Format (WAV) format reader.
///
/// `WavReader` implements a demuxer for the WAVE container format.
pub struct WavReader {
    reader: MediaSourceStream,
    tracks: Vec<Track>,
    cues: Vec<Cue>,
    metadata: MetadataLog,
    packet_info: PacketInfo,
    data_start_pos: u64,
    data_end_pos: u64,
}

impl QueryDescriptor for WavReader {
    fn query() -> &'static [Descriptor] {
        &[
            // WAVE RIFF form
            support_format!(
                "wave",
                "Waveform Audio File Format",
                &["wav", "wave"],
                &["audio/vnd.wave", "audio/x-wav", "audio/wav", "audio/wave"],
                &[b"RIFF"]
            ),
        ]
    }

    fn score(_context: &[u8]) -> u8 {
        255
    }
}

impl FormatReader for WavReader {
    fn try_new(mut source: MediaSourceStream, _options: &FormatOptions) -> Result<Self> {
        // The RIFF marker should be present.
        let marker = source.read_quad_bytes()?;

        if marker != WAVE_STREAM_MARKER {
            return unsupported_error("wav: missing riff stream marker");
        }

        // A Wave file is one large RIFF chunk, with the actual meta and audio data as sub-chunks.
        // Therefore, the header was the chunk ID, and the next 4 bytes is the length of the RIFF
        // chunk.
        let riff_len = source.read_u32()?;
        let riff_form = source.read_quad_bytes()?;

        // The RIFF chunk contains WAVE data.
        if riff_form != WAVE_RIFF_FORM {
            error!("riff form is not wave ({})", String::from_utf8_lossy(&riff_form));

            return unsupported_error("wav: riff form is not wave");
        }

        let mut riff_chunks =
            ChunksReader::<RiffWaveChunks>::new(riff_len, ByteOrder::LittleEndian);

        let mut codec_params = CodecParameters::new();
        let mut metadata: MetadataLog = Default::default();
        let mut packet_info = PacketInfo::without_blocks(0);

        loop {
            let chunk = riff_chunks.next(&mut source)?;

            // The last chunk should always be a data chunk, if it is not, then the stream is
            // unsupported.
            if chunk.is_none() {
                return unsupported_error("wav: missing data chunk");
            }

            match chunk.unwrap() {
                RiffWaveChunks::Format(fmt) => {
                    let format = fmt.parse(&mut source)?;

                    // The Format chunk contains the block_align field and possible additional information
                    // to handle packetization and seeking.
                    packet_info = format.packet_info()?;
                    codec_params
                        .with_max_frames_per_packet(packet_info.get_max_frames_per_packet())
                        .with_frames_per_block(packet_info.frames_per_block);

                    // Append Format chunk fields to codec parameters.
                    append_format_params(
                        &mut codec_params,
                        &format.format_data,
                        format.sample_rate,
                    );
                }
                RiffWaveChunks::Fact(fct) => {
                    let fact = fct.parse(&mut source)?;

                    // Append Fact chunk fields to codec parameters.
                    append_fact_params(&mut codec_params, &fact);
                }
                RiffWaveChunks::List(lst) => {
                    let list = lst.parse(&mut source)?;

                    // Riff Lists can have many different forms, but WavReader only supports Info
                    // lists.
                    match &list.form {
                        b"INFO" => metadata.push(read_info_chunk(&mut source, list.len)?),
                        _ => list.skip(&mut source)?,
                    }
                }
                RiffWaveChunks::Data(dat) => {
                    let data = dat.parse(&mut source)?;

                    // Record the bounds of the data chunk.
                    let data_start_pos = source.pos();
                    let data_end_pos = data_start_pos + u64::from(data.len);

                    // Append Data chunk fields to codec parameters.
                    append_data_params(&mut codec_params, data.len as u64, &packet_info);

                    // Add a new track using the collected codec parameters.
                    return Ok(WavReader {
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

        // WAVE is not internally packetized for PCM codecs. Packetization is simulated by trying to
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
