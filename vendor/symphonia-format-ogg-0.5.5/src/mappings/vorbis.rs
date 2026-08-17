// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{MapResult, Mapper, PacketParser};
use crate::common::SideData;

use symphonia_core::codecs::{CodecParameters, CODEC_TYPE_VORBIS};
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::io::{BitReaderRtl, BufReader, ReadBitsRtl, ReadBytes};
use symphonia_core::meta::MetadataBuilder;
use symphonia_core::units::TimeBase;

use symphonia_metadata::vorbis::*;
use symphonia_utils_xiph::vorbis::*;

use log::warn;

/// The identification header packet size.
const VORBIS_IDENTIFICATION_HEADER_SIZE: usize = 30;

/// The packet type for an identification header.
const VORBIS_PACKET_TYPE_IDENTIFICATION: u8 = 1;
/// The packet type for a comment header.
const VORBIS_PACKET_TYPE_COMMENT: u8 = 3;
/// The packet type for a setup header.
const VORBIS_PACKET_TYPE_SETUP: u8 = 5;

/// The common header packet signature.
const VORBIS_HEADER_PACKET_SIGNATURE: &[u8] = b"vorbis";

/// The Vorbis version supported by this mapper.
const VORBIS_VERSION: u32 = 0;

/// The minimum block size (64) expressed as a power-of-2 exponent.
const VORBIS_BLOCKSIZE_MIN: u8 = 6;
/// The maximum block size (8192) expressed as a power-of-2 exponent.
const VORBIS_BLOCKSIZE_MAX: u8 = 13;

struct VorbisPacketParser {
    modes_block_flags: u64,
    num_modes: u8,
    bs0_exp: u8,
    bs1_exp: u8,
    prev_bs_exp: Option<u8>,
}

impl VorbisPacketParser {
    fn new(bs0_exp: u8, bs1_exp: u8, num_modes: u8, modes_block_flags: u64) -> Self {
        Self { bs0_exp, bs1_exp, num_modes, modes_block_flags, prev_bs_exp: None }
    }

    fn reset(&mut self) {
        self.prev_bs_exp = None;
    }
}

impl PacketParser for VorbisPacketParser {
    fn parse_next_packet_dur(&mut self, packet: &[u8]) -> u64 {
        let mut bs = BitReaderRtl::new(packet);

        // First bit must be 0 to indicate audio packet.
        match bs.read_bool() {
            Ok(bit) if !bit => (),
            _ => return 0,
        }

        // Number of bits for the mode number.
        let mode_num_bits = ilog(u32::from(self.num_modes) - 1);

        // Read the mode number.
        let mode_num = match bs.read_bits_leq32(mode_num_bits) {
            Ok(mode_num) => mode_num as u8,
            _ => return 0,
        };

        // Determine the current block size.
        let cur_bs_exp = if mode_num < self.num_modes {
            let block_flag = (self.modes_block_flags >> mode_num) & 1;
            if block_flag == 1 {
                self.bs1_exp
            }
            else {
                self.bs0_exp
            }
        }
        else {
            return 0;
        };

        // Calculate the duration if the previous block size is available. Otherwise return 0.
        let dur = if let Some(prev_bs_exp) = self.prev_bs_exp {
            ((1 << prev_bs_exp) >> 2) + ((1 << cur_bs_exp) >> 2)
        }
        else {
            0
        };

        self.prev_bs_exp = Some(cur_bs_exp);

        dur
    }
}

pub fn detect(buf: &[u8]) -> Result<Option<Box<dyn Mapper>>> {
    // The identification header packet must be the correct size.
    if buf.len() != VORBIS_IDENTIFICATION_HEADER_SIZE {
        return Ok(None);
    }

    // Read the identification header. Any errors cause detection to fail.
    let ident = match read_ident_header(&mut BufReader::new(buf)) {
        Ok(ident) => ident,
        _ => return Ok(None),
    };

    // Populate the codec parameters with the information above.
    let mut codec_params = CodecParameters::new();

    codec_params
        .for_codec(CODEC_TYPE_VORBIS)
        .with_sample_rate(ident.sample_rate)
        .with_time_base(TimeBase::new(1, ident.sample_rate))
        .with_extra_data(Box::from(buf));

    if let Some(channels) = vorbis_channels_to_channels(ident.n_channels) {
        codec_params.with_channels(channels);
    }

    // Instantiate the Vorbis mapper.
    let mapper =
        Box::new(VorbisMapper { codec_params, ident, parser: None, has_setup_header: false });

    Ok(Some(mapper))
}

struct VorbisMapper {
    codec_params: CodecParameters,
    ident: IdentHeader,
    parser: Option<VorbisPacketParser>,
    has_setup_header: bool,
}

impl Mapper for VorbisMapper {
    fn name(&self) -> &'static str {
        "vorbis"
    }

    fn codec_params(&self) -> &CodecParameters {
        &self.codec_params
    }

    fn codec_params_mut(&mut self) -> &mut CodecParameters {
        &mut self.codec_params
    }

    fn reset(&mut self) {
        if let Some(parser) = &mut self.parser {
            parser.reset()
        }
    }

    fn make_parser(&self) -> Option<Box<dyn PacketParser>> {
        match &self.parser {
            Some(base_parser) => {
                let parser = Box::new(VorbisPacketParser::new(
                    base_parser.bs0_exp,
                    base_parser.bs1_exp,
                    base_parser.num_modes,
                    base_parser.modes_block_flags,
                ));
                Some(parser)
            }
            _ => None,
        }
    }

    fn map_packet(&mut self, packet: &[u8]) -> Result<MapResult> {
        let mut reader = BufReader::new(packet);

        // All Vorbis packets indicate the packet type in the first byte.
        let packet_type = reader.read_u8()?;

        // An even numbered packet type is an audio packet.
        if packet_type & 1 == 0 {
            let dur = match &mut self.parser {
                Some(parser) => parser.parse_next_packet_dur(packet),
                _ => 0,
            };

            Ok(MapResult::StreamData { dur })
        }
        else {
            // Odd numbered packet types are header packets.
            let mut sig = [0; 6];
            reader.read_buf_exact(&mut sig)?;

            // Check if the presumed header packet has the common header packet signature.
            if sig != VORBIS_HEADER_PACKET_SIGNATURE {
                return decode_error("ogg (vorbis): header packet signature invalid");
            }

            // Handle each header packet type specifically.
            match packet_type {
                VORBIS_PACKET_TYPE_COMMENT => {
                    let mut builder = MetadataBuilder::new();

                    read_comment_no_framing(&mut reader, &mut builder)?;

                    Ok(MapResult::SideData { data: SideData::Metadata(builder.metadata()) })
                }
                VORBIS_PACKET_TYPE_SETUP => {
                    // Append the setup headers to the extra data.
                    let mut extra_data = self.codec_params.extra_data.take().unwrap().to_vec();
                    extra_data.extend_from_slice(packet);

                    // Try to read the setup header.
                    if let Ok(modes) = read_setup(&mut BufReader::new(packet), &self.ident) {
                        let num_modes = modes.len();
                        let mut modes_block_flags = 0;

                        assert!(num_modes <= 64);

                        for (i, mode) in modes.iter().enumerate() {
                            if mode.block_flag {
                                modes_block_flags |= 1 << i;
                            }
                        }

                        let parser = VorbisPacketParser::new(
                            self.ident.bs0_exp,
                            self.ident.bs1_exp,
                            num_modes as u8,
                            modes_block_flags,
                        );

                        self.parser.replace(parser);
                    }

                    self.codec_params.with_extra_data(extra_data.into_boxed_slice());
                    self.has_setup_header = true;

                    Ok(MapResult::Setup)
                }
                _ => {
                    warn!("ogg (vorbis): packet type {} unexpected", packet_type);
                    Ok(MapResult::Unknown)
                }
            }
        }
    }

    fn is_ready(&self) -> bool {
        self.has_setup_header
    }
}

struct IdentHeader {
    n_channels: u8,
    sample_rate: u32,
    bs0_exp: u8,
    bs1_exp: u8,
}

fn read_ident_header<B: ReadBytes>(reader: &mut B) -> Result<IdentHeader> {
    // The packet type must be an identification header.
    let packet_type = reader.read_u8()?;

    if packet_type != VORBIS_PACKET_TYPE_IDENTIFICATION {
        return decode_error("ogg (vorbis): invalid packet type for identification header");
    }

    // Next, the header packet signature must be correct.
    let mut packet_sig_buf = [0; 6];
    reader.read_buf_exact(&mut packet_sig_buf)?;

    if packet_sig_buf != VORBIS_HEADER_PACKET_SIGNATURE {
        return decode_error("ogg (vorbis): invalid header signature");
    }

    // Next, the Vorbis version must be 0.
    let version = reader.read_u32()?;

    if version != VORBIS_VERSION {
        return unsupported_error("ogg (vorbis): only vorbis 1 is supported");
    }

    // Next, the number of channels and sample rate must be non-zero.
    let n_channels = reader.read_u8()?;

    if n_channels == 0 {
        return decode_error("ogg (vorbis): number of channels cannot be 0");
    }

    let sample_rate = reader.read_u32()?;

    if sample_rate == 0 {
        return decode_error("ogg (vorbis): sample rate cannot be 0");
    }

    // Read the bitrate range.
    let _bitrate_max = reader.read_u32()?;
    let _bitrate_nom = reader.read_u32()?;
    let _bitrate_min = reader.read_u32()?;

    // Next, blocksize_0 and blocksize_1 are packed into a single byte.
    let block_sizes = reader.read_u8()?;

    let bs0_exp = (block_sizes & 0x0f) >> 0;
    let bs1_exp = (block_sizes & 0xf0) >> 4;

    // The block sizes must not exceed the bounds.
    if bs0_exp < VORBIS_BLOCKSIZE_MIN || bs0_exp > VORBIS_BLOCKSIZE_MAX {
        return decode_error("ogg (vorbis): blocksize_0 out-of-bounds");
    }

    if bs1_exp < VORBIS_BLOCKSIZE_MIN || bs1_exp > VORBIS_BLOCKSIZE_MAX {
        return decode_error("ogg (vorbis): blocksize_1 out-of-bounds");
    }

    // Blocksize_0 must be >= blocksize_1
    if bs0_exp > bs1_exp {
        return decode_error("ogg (vorbis): blocksize_0 exceeds blocksize_1");
    }

    // Framing flag must be set.
    if reader.read_u8()? != 0x1 {
        return decode_error("ogg (vorbis): ident header framing flag unset");
    }

    Ok(IdentHeader { n_channels, sample_rate, bs0_exp, bs1_exp })
}

fn read_setup(reader: &mut BufReader<'_>, ident: &IdentHeader) -> Result<Vec<Mode>> {
    // The packet type must be an setup header.
    let packet_type = reader.read_u8()?;

    if packet_type != VORBIS_PACKET_TYPE_SETUP {
        return decode_error("ogg (vorbis): invalid packet type for setup header");
    }

    // Next, the setup packet signature must be correct.
    let mut packet_sig_buf = [0; 6];
    reader.read_buf_exact(&mut packet_sig_buf)?;

    if packet_sig_buf != VORBIS_HEADER_PACKET_SIGNATURE {
        return decode_error("ogg (vorbis): invalid setup header signature");
    }

    // The remaining portion of the setup header packet is read bitwise.
    let mut bs = BitReaderRtl::new(reader.read_buf_bytes_available_ref());

    // Skip the codebooks.
    skip_codebooks(&mut bs)?;

    // Skip the time-domain transforms (placeholders in Vorbis 1).
    skip_time_domain_transforms(&mut bs)?;

    // Skip the floors.
    skip_floors(&mut bs)?;

    // Skip the residues.
    skip_residues(&mut bs)?;

    // Skip the channel mappings.
    skip_mappings(&mut bs, ident.n_channels)?;

    // Read modes.
    let modes = read_modes(&mut bs)?;

    // Framing flag must be set.
    if !bs.read_bool()? {
        return decode_error("ogg (vorbis): setup header framing flag unset");
    }

    Ok(modes)
}

fn skip_codebooks(bs: &mut BitReaderRtl<'_>) -> Result<()> {
    let count = bs.read_bits_leq32(8)? + 1;
    for _ in 0..count {
        skip_codebook(bs)?;
    }
    Ok(())
}

pub fn skip_codebook(bs: &mut BitReaderRtl<'_>) -> Result<()> {
    // Verify codebook synchronization word.
    let sync = bs.read_bits_leq32(24)?;

    if sync != 0x564342 {
        return decode_error("ogg (vorbis): invalid codebook sync");
    }

    // Read codebook number of dimensions and entries.
    let codebook_dimensions = bs.read_bits_leq32(16)? as u16;
    let codebook_entries = bs.read_bits_leq32(24)?;
    let is_length_ordered = bs.read_bool()?;

    if !is_length_ordered {
        // Codeword list is not length ordered.
        let is_sparse = bs.read_bool()?;

        if is_sparse {
            // Sparsely packed codeword entry list.
            for _ in 0..codebook_entries {
                if bs.read_bool()? {
                    let _ = bs.read_bits_leq32(5)?;
                }
            }
        }
        else {
            bs.ignore_bits(codebook_entries * 5)?;
        }
    }
    else {
        // Codeword list is length ordered.
        let mut cur_entry = 0;
        let mut _cur_len = bs.read_bits_leq32(5)? + 1;

        loop {
            let num_bits =
                if codebook_entries > cur_entry { ilog(codebook_entries - cur_entry) } else { 0 };

            let num = bs.read_bits_leq32(num_bits)?;

            cur_entry += num;

            if cur_entry > codebook_entries {
                return decode_error("ogg (vorbis): invalid codebook");
            }

            if cur_entry == codebook_entries {
                break;
            }
        }
    }

    // Read and unpack vector quantization (VQ) lookup table.
    let lookup_type = bs.read_bits_leq32(4)?;

    match lookup_type & 0xf {
        0 => (),
        1 | 2 => {
            let _min_value = bs.read_bits_leq32(32)?;
            let _delta_value = bs.read_bits_leq32(32)?;
            let value_bits = bs.read_bits_leq32(4)? + 1;
            let _sequence_p = bs.read_bool()?;

            // Lookup type is either 1 or 2 as per outer match.
            let lookup_values = match lookup_type {
                1 => lookup1_values(codebook_entries, codebook_dimensions),
                2 => codebook_entries * u32::from(codebook_dimensions),
                _ => unreachable!(),
            };

            // Multiplicands
            bs.ignore_bits(lookup_values * value_bits)?;
        }
        _ => return decode_error("ogg (vorbis): invalid codeword lookup type"),
    }

    Ok(())
}

fn skip_time_domain_transforms(bs: &mut BitReaderRtl<'_>) -> Result<()> {
    let count = bs.read_bits_leq32(6)? + 1;

    for _ in 0..count {
        // All these values are placeholders and must be 0.
        if bs.read_bits_leq32(16)? != 0 {
            return decode_error("ogg (vorbis): invalid time domain tranform");
        }
    }

    Ok(())
}

fn skip_floors(bs: &mut BitReaderRtl<'_>) -> Result<()> {
    let count = bs.read_bits_leq32(6)? + 1;
    for _ in 0..count {
        skip_floor(bs)?;
    }
    Ok(())
}

fn skip_floor(bs: &mut BitReaderRtl<'_>) -> Result<()> {
    let floor_type = bs.read_bits_leq32(16)?;

    match floor_type {
        0 => skip_floor0_setup(bs),
        1 => skip_floor1_setup(bs),
        _ => decode_error("ogg (vorbis): invalid floor type"),
    }
}

fn skip_floor0_setup(bs: &mut BitReaderRtl<'_>) -> Result<()> {
    // floor0_order
    // floor0_rate
    // floor0_bark_map_size
    // floor0_amplitude_bits
    // floor0_amplitude_offset
    bs.ignore_bits(8 + 16 + 16 + 6 + 8)?;
    let floor0_number_of_books = bs.read_bits_leq32(4)? + 1;
    bs.ignore_bits(floor0_number_of_books * 8)?;
    Ok(())
}

fn skip_floor1_setup(bs: &mut BitReaderRtl<'_>) -> Result<()> {
    // The number of partitions. 5-bit value, 0..31 range.
    let floor1_partitions = bs.read_bits_leq32(5)? as usize;

    // Parition list of up-to 32 partitions (floor1_partitions), with each partition indicating
    // a 4-bit class (0..16) identifier.
    let mut floor1_partition_class_list = [0; 32];
    let mut floor1_classes_dimensions = [0; 16];

    if floor1_partitions > 0 {
        let mut max_class = 0; // 4-bits, 0..15

        for class_idx in &mut floor1_partition_class_list[..floor1_partitions] {
            *class_idx = bs.read_bits_leq32(4)? as u8;
            max_class = max_class.max(*class_idx);
        }

        let num_classes = usize::from(1 + max_class);

        for dimensions in floor1_classes_dimensions[..num_classes].iter_mut() {
            *dimensions = bs.read_bits_leq32(3)? as u8 + 1;

            let subclass_bits = bs.read_bits_leq32(2)?;

            if subclass_bits != 0 {
                let _main_book = bs.read_bits_leq32(8)?;
            }

            let num_subclasses = 1 << subclass_bits;

            // Sub-class books
            bs.ignore_bits(num_subclasses * 8)?;
        }
    }

    let _floor1_multiplier = bs.read_bits_leq32(2)?;

    let rangebits = bs.read_bits_leq32(4)?;

    for &class_idx in &floor1_partition_class_list[..floor1_partitions] {
        let class_dimensions = u32::from(floor1_classes_dimensions[class_idx as usize]);
        // TODO? No more than 65 elements are allowed.
        bs.ignore_bits(class_dimensions * rangebits)?;
    }

    Ok(())
}

fn skip_residues(bs: &mut BitReaderRtl<'_>) -> Result<()> {
    let count = bs.read_bits_leq32(6)? + 1;
    for _ in 0..count {
        let _residue_type = bs.read_bits_leq32(16)?;
        skip_residue_setup(bs)?
    }
    Ok(())
}

fn skip_residue_setup(bs: &mut BitReaderRtl<'_>) -> Result<()> {
    // residue_begin
    // residue_end
    // residue_partition_size
    bs.ignore_bits(24 + 24 + 24)?;
    let residue_classifications = bs.read_bits_leq32(6)? as u8 + 1;

    // residue_classbook
    bs.ignore_bits(8)?;

    let mut num_codebooks = 0;

    for _ in 0..residue_classifications {
        let low_bits = bs.read_bits_leq32(3)? as u8;
        let high_bits = if bs.read_bool()? { bs.read_bits_leq32(5)? as u8 } else { 0 };
        let is_used = (high_bits << 3) | low_bits;
        num_codebooks += is_used.count_ones();
    }

    bs.ignore_bits(num_codebooks * 8)?;

    Ok(())
}

fn skip_mappings(bs: &mut BitReaderRtl<'_>, audio_channels: u8) -> Result<()> {
    let count = bs.read_bits_leq32(6)? + 1;
    for _ in 0..count {
        skip_mapping(bs, audio_channels)?
    }
    Ok(())
}

fn skip_mapping(bs: &mut BitReaderRtl<'_>, audio_channels: u8) -> Result<()> {
    let mapping_type = bs.read_bits_leq32(16)?;

    match mapping_type {
        0 => skip_mapping_type0_setup(bs, audio_channels),
        _ => decode_error("ogg (vorbis): invalid mapping type"),
    }
}

fn skip_mapping_type0_setup(bs: &mut BitReaderRtl<'_>, audio_channels: u8) -> Result<()> {
    let num_submaps = if bs.read_bool()? { bs.read_bits_leq32(4)? + 1 } else { 1 };

    if bs.read_bool()? {
        // Number of channel couplings (up-to 256).
        let coupling_steps = bs.read_bits_leq32(8)? as u16 + 1;

        // The maximum channel number.
        let max_ch = audio_channels - 1;

        // The number of bits to read for the magnitude and angle channel numbers. Never exceeds 8.
        let coupling_bits = ilog(u32::from(max_ch));
        debug_assert!(coupling_bits <= 8);

        // Read each channel coupling.
        for _ in 0..coupling_steps {
            let _magnitude_ch = bs.read_bits_leq32(coupling_bits)?;
            let _angle_ch = bs.read_bits_leq32(coupling_bits)?;
        }
    }

    if bs.read_bits_leq32(2)? != 0 {
        return decode_error("ogg (vorbis): reserved mapping bits non-zero");
    }

    // If the number of submaps is > 1 read the multiplex numbers from the bitstream, otherwise
    // they're all 0.
    if num_submaps > 1 {
        // Mux to use per channel.
        bs.ignore_bits(u32::from(audio_channels) * 4)?;
    }

    // Reserved, floor, and residue to use per submap.
    bs.ignore_bits(num_submaps * (8 + 8 + 8))?;

    Ok(())
}

fn read_modes(bs: &mut BitReaderRtl<'_>) -> Result<Vec<Mode>> {
    let count = bs.read_bits_leq32(6)? + 1;
    (0..count).map(|_| read_mode(bs)).collect()
}

#[derive(Debug)]
struct Mode {
    block_flag: bool,
}

fn read_mode(bs: &mut BitReaderRtl<'_>) -> Result<Mode> {
    let block_flag = bs.read_bool()?;
    let window_type = bs.read_bits_leq32(16)? as u16;
    let transform_type = bs.read_bits_leq32(16)? as u16;
    let _mapping = bs.read_bits_leq32(8)? as u8;

    // Only window type 0 is allowed in Vorbis 1 (section 4.2.4).
    if window_type != 0 {
        return decode_error("ogg (vorbis): invalid window type for mode");
    }

    // Only transform type 0 is allowed in Vorbis 1 (section 4.2.4).
    if transform_type != 0 {
        return decode_error("ogg (vorbis): invalid transform type for mode");
    }

    let mode = Mode { block_flag };

    Ok(mode)
}

#[inline(always)]
pub fn ilog(x: u32) -> u32 {
    32 - x.leading_zeros()
}

#[inline(always)]
fn lookup1_values(entries: u32, dimensions: u16) -> u32 {
    // (value ^ dimensions) <= entries
    // [(value ^ dimensions) ^ (1 / dimensions)] = lower[entries ^ (1 / dimensions)]
    // value = lower[entries ^ (1 / dimensions)]
    let value = (entries as f32).powf(1.0f32 / f32::from(dimensions)).floor() as u32;

    assert!(value.pow(u32::from(dimensions)) <= entries);
    assert!((value + 1).pow(u32::from(dimensions)) > entries);

    value
}
