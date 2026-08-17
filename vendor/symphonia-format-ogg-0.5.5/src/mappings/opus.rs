// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::common::SideData;

use super::{MapResult, Mapper, PacketParser};

use symphonia_core::audio::Channels;
use symphonia_core::codecs::{CodecParameters, CODEC_TYPE_OPUS};
use symphonia_core::errors::Result;
use symphonia_core::io::{BufReader, ReadBytes};
use symphonia_core::meta::MetadataBuilder;
use symphonia_core::units::TimeBase;

use symphonia_metadata::vorbis;

use log::warn;

/// The minimum expected size of an Opus identification packet.
const OGG_OPUS_MIN_IDENTIFICATION_PACKET_SIZE: usize = 19;

/// The signature for an Opus identification packet.
const OGG_OPUS_MAGIC_SIGNATURE: &[u8] = b"OpusHead";

/// The signature for an Opus metadata packet.
const OGG_OPUS_COMMENT_SIGNATURE: &[u8] = b"OpusTags";

/// The maximum support Opus OGG mapping version.
const OGG_OPUS_MAPPING_VERSION_MAX: u8 = 0x0f;

pub fn detect(buf: &[u8]) -> Result<Option<Box<dyn Mapper>>> {
    // The identification packet for Opus must be a minimum size.
    if buf.len() < OGG_OPUS_MIN_IDENTIFICATION_PACKET_SIZE {
        return Ok(None);
    }

    let mut reader = BufReader::new(buf);

    // The first 8 bytes are the magic signature ASCII bytes.
    let mut magic = [0; 8];
    reader.read_buf_exact(&mut magic)?;

    if magic != *OGG_OPUS_MAGIC_SIGNATURE {
        return Ok(None);
    }

    // The next byte is the OGG Opus encapsulation version. The version is split into two
    // sub-fields: major and minor. These fields are stored in the upper and lower 4-bit,
    // respectively.
    if reader.read_byte()? > OGG_OPUS_MAPPING_VERSION_MAX {
        return Ok(None);
    }

    // The next byte is the number of channels and must not be 0.
    let channel_count = reader.read_byte()?;

    if channel_count == 0 {
        return Ok(None);
    }

    // The next 16-bit integer is the pre-skip padding (# of samples at 48kHz to subtract from the
    // OGG granule position to obtain the PCM sample position).
    let pre_skip = reader.read_u16()?;

    // The next 32-bit integer is the sample rate of the original audio.
    let _ = reader.read_u32()?;

    // Next, the 16-bit gain value.
    let _ = reader.read_u16()?;

    // The next byte indicates the channel mapping. Most of these values are reserved.
    let channel_mapping = reader.read_byte()?;

    let channels = match channel_mapping {
        // RTP Mapping
        0 if channel_count == 1 => Channels::FRONT_LEFT,
        0 if channel_count == 2 => Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
        // Vorbis Mapping
        1 => match channel_count {
            1 => Channels::FRONT_LEFT,
            2 => Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
            3 => Channels::FRONT_LEFT | Channels::FRONT_CENTRE | Channels::FRONT_RIGHT,
            4 => {
                Channels::FRONT_LEFT
                    | Channels::FRONT_RIGHT
                    | Channels::REAR_LEFT
                    | Channels::REAR_RIGHT
            }
            5 => {
                Channels::FRONT_LEFT
                    | Channels::FRONT_CENTRE
                    | Channels::FRONT_RIGHT
                    | Channels::REAR_LEFT
                    | Channels::REAR_RIGHT
            }
            6 => {
                Channels::FRONT_LEFT
                    | Channels::FRONT_CENTRE
                    | Channels::FRONT_RIGHT
                    | Channels::REAR_LEFT
                    | Channels::REAR_RIGHT
                    | Channels::LFE1
            }
            7 => {
                Channels::FRONT_LEFT
                    | Channels::FRONT_CENTRE
                    | Channels::FRONT_RIGHT
                    | Channels::SIDE_LEFT
                    | Channels::SIDE_RIGHT
                    | Channels::REAR_CENTRE
                    | Channels::LFE1
            }
            8 => {
                Channels::FRONT_LEFT
                    | Channels::FRONT_CENTRE
                    | Channels::FRONT_RIGHT
                    | Channels::SIDE_LEFT
                    | Channels::SIDE_RIGHT
                    | Channels::REAR_LEFT
                    | Channels::REAR_RIGHT
                    | Channels::LFE1
            }
            _ => return Ok(None),
        },
        // Reserved, and should NOT be supported for playback.
        _ => return Ok(None),
    };

    // Populate the codec parameters with the information read from identification header.
    let mut codec_params = CodecParameters::new();

    codec_params
        .for_codec(CODEC_TYPE_OPUS)
        .with_delay(u32::from(pre_skip))
        .with_sample_rate(48_000)
        .with_time_base(TimeBase::new(1, 48_000))
        .with_channels(channels)
        .with_extra_data(Box::from(buf));

    // Instantiate the Opus mapper.
    let mapper = Box::new(OpusMapper { codec_params, need_comment: true });

    Ok(Some(mapper))
}

pub struct OpusPacketParser {}

impl PacketParser for OpusPacketParser {
    fn parse_next_packet_dur(&mut self, packet: &[u8]) -> u64 {
        // See https://www.rfc-editor.org/rfc/rfc6716
        // Read TOC (Table Of Contents) byte which is the first byte in the opus data.
        let toc_byte = match packet.first() {
            Some(b) => b,
            None => {
                warn!("opus packet empty");
                return 0;
            }
        };
        // The configuration number is the 5 most significant bits. Shift out 3 least significant
        // bits.
        let configuration_number = toc_byte >> 3; // max 2^5-1 = 31

        // The configuration number maps to packet length according to this lookup table.
        // See https://www.rfc-editor.org/rfc/rfc6716 top half of page 14.
        // Numbers are in milliseconds in the rfc. Down below they are in TimeBase units, so
        // 10ms = 10*48.
        #[rustfmt::skip]
        const CONFIGURATION_NUMBER_TO_FRAME_DURATION: [u32; 32] = [
            10*48, 20*48, 40*48, 60*48,
            10*48, 20*48, 40*48, 60*48,
            10*48, 20*48, 40*48, 60*48,
            10*48, 20*48,
            10*48, 20*48,
            (2.5*48.0) as u32, 5*48, 10*48, 20*48,
            (2.5*48.0) as u32, 5*48, 10*48, 20*48,
            (2.5*48.0) as u32, 5*48, 10*48, 20*48,
            (2.5*48.0) as u32, 5*48, 10*48, 20*48,
        ];
        // Look up the frame length.
        let frame_duration =
            CONFIGURATION_NUMBER_TO_FRAME_DURATION[configuration_number as usize] as u64;

        // Look up the number of frames in the packet.
        // See https://www.rfc-editor.org/rfc/rfc6716 bottom half of page 14.
        let c = toc_byte & 0b11; // Note: it's actually called "c" in the rfc.
        let num_frames = match c {
            0 => 1,
            1 | 2 => 2,
            3 => match packet.get(1) {
                Some(byte) => {
                    // TOC byte is followed by number of frames. See page 18 section 3.2.5 code 3
                    let m = byte & 0b11111; // Note: it's actually called "M" in the rfc.
                    m as u64
                }
                None => {
                    // What to do here? I'd like to return an error but this is an infalliable
                    // trait.
                    warn!("opus code 3 packet with no following byte containing number of frames");
                    return 0;
                }
            },
            _ => unreachable!("masked 2 bits"),
        };
        // Look up the packet length and return it.
        frame_duration * num_frames
    }
}

struct OpusMapper {
    codec_params: CodecParameters,
    need_comment: bool,
}

impl Mapper for OpusMapper {
    fn name(&self) -> &'static str {
        "opus"
    }

    fn reset(&mut self) {
        // Nothing to do.
    }

    fn codec_params(&self) -> &CodecParameters {
        &self.codec_params
    }

    fn codec_params_mut(&mut self) -> &mut CodecParameters {
        &mut self.codec_params
    }

    fn make_parser(&self) -> Option<Box<dyn super::PacketParser>> {
        Some(Box::new(OpusPacketParser {}))
    }

    fn map_packet(&mut self, packet: &[u8]) -> Result<MapResult> {
        if !self.need_comment {
            Ok(MapResult::StreamData { dur: OpusPacketParser {}.parse_next_packet_dur(packet) })
        }
        else {
            let mut reader = BufReader::new(packet);

            // Read the header signature.
            let mut sig = [0; 8];
            reader.read_buf_exact(&mut sig)?;

            if sig == *OGG_OPUS_COMMENT_SIGNATURE {
                // This packet should be a metadata packet containing a Vorbis Comment.
                let mut builder = MetadataBuilder::new();

                vorbis::read_comment_no_framing(&mut reader, &mut builder)?;

                self.need_comment = false;

                Ok(MapResult::SideData { data: SideData::Metadata(builder.metadata()) })
            }
            else {
                warn!("ogg (opus): invalid packet type");
                Ok(MapResult::Unknown)
            }
        }
    }
}
