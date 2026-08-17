// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::common::SideData;

use symphonia_core::codecs::CodecParameters;
use symphonia_core::errors::Result;

mod flac;
mod opus;
mod vorbis;

/// Detect a `Mapper` for a logical stream given the identification packet of the stream.
pub fn detect(buf: &[u8]) -> Result<Option<Box<dyn Mapper>>> {
    let mapper = flac::detect(buf)?
        .or(vorbis::detect(buf)?)
        .or(opus::detect(buf)?)
        .or_else(make_null_mapper);

    Ok(mapper)
}

/// Result of a packet map operation.
pub enum MapResult {
    /// The packet contained side data.
    SideData { data: SideData },
    /// The packet contained setup data.
    Setup,
    /// The packet contained stream data.
    StreamData { dur: u64 },
    /// The packet contained unknown data.
    Unknown,
}

/// A `PacketParser` implements a packet parser that decodes the timestamp and duration for a
/// packet.
pub trait PacketParser: Send + Sync {
    fn parse_next_packet_dur(&mut self, packet: &[u8]) -> u64;
}

/// A `Mapper` implements packet-handling for a specific `Codec`.
pub trait Mapper: Send + Sync {
    /// Gets the name of the mapper.
    fn name(&self) -> &'static str;

    /// Soft-reset the mapper after a discontinuity in packets.
    fn reset(&mut self);

    /// Gets an immutable reference `CodecParameters` for the stream belonging to this `Mapper`. If
    /// the stream is not ready then the set of parameters may be incomplete.
    fn codec_params(&self) -> &CodecParameters;

    /// Gets a mutable reference to the `CodecParameters` for the stream belonging to this `Mapper`.
    /// If the stream is not ready then the set of parameters may be incomplete.
    fn codec_params_mut(&mut self) -> &mut CodecParameters;

    /// Convert an absolute granular position to a timestamp.
    fn absgp_to_ts(&self, ts: u64) -> u64 {
        ts
    }

    /// Make a packet parser for parsing packet timing.
    fn make_parser(&self) -> Option<Box<dyn PacketParser>>;

    /// Map a packet.
    fn map_packet(&mut self, packet: &[u8]) -> Result<MapResult>;

    /// Returns `true` if the stream can is ready for usage. If the stream is not ready then the
    /// mapper needs to consume more setup packets.
    fn is_ready(&self) -> bool {
        true
    }
}

fn make_null_mapper() -> Option<Box<dyn Mapper>> {
    Some(Box::new(NullMapper::new()))
}

struct NullMapper {
    params: CodecParameters,
}

impl NullMapper {
    fn new() -> Self {
        NullMapper { params: CodecParameters::new() }
    }
}

impl Mapper for NullMapper {
    fn name(&self) -> &'static str {
        "null"
    }

    fn codec_params(&self) -> &CodecParameters {
        &self.params
    }

    fn codec_params_mut(&mut self) -> &mut CodecParameters {
        &mut self.params
    }

    fn reset(&mut self) {
        // Nothing to do!
    }

    fn make_parser(&self) -> Option<Box<dyn PacketParser>> {
        Some(Box::new(NullPacketParser {}))
    }

    fn map_packet(&mut self, _: &[u8]) -> Result<MapResult> {
        Ok(MapResult::Unknown)
    }
}

struct NullPacketParser {}

impl PacketParser for NullPacketParser {
    fn parse_next_packet_dur(&mut self, _: &[u8]) -> u64 {
        0
    }
}
