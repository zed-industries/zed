// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

use symphonia_core::audio::{AudioBuffer, Signal};
use symphonia_core::errors::{decode_error, Error, Result};
use symphonia_core::io::{BitReaderLtr, BufReader, ReadBitsLtr, ReadBytes};

mod bitstream;
mod codebooks;
mod common;
mod hybrid_synthesis;
mod requantize;
mod stereo;

use crate::{common::*, synthesis};

use common::BlockType;

use log::warn;

/// `BitResevoir` implements the bit resevoir mechanism for main_data. Since frames have a
/// deterministic length based on the bit-rate, low-complexity portions of the audio may not need
/// every byte allocated to the frame. The bit resevoir mechanism allows these unused portions of
/// frames to be used by future frames.
pub struct BitResevoir {
    buf: Box<[u8]>,
    len: usize,
    consumed: usize,
}

impl BitResevoir {
    pub fn new() -> Self {
        BitResevoir { buf: vec![0u8; 2048].into_boxed_slice(), len: 0, consumed: 0 }
    }

    pub fn fill(&mut self, pkt_main_data: &[u8], main_data_begin: usize) -> Result<u32> {
        let main_data_len = pkt_main_data.len();

        // The value `main_data_begin` indicates the number of bytes from the previous frame(s) to
        // reuse. It must always be less than or equal to maximum amount of bytes the resevoir can
        // hold taking into account the additional data being added to the resevoir.
        let main_data_end = main_data_begin + main_data_len;

        if main_data_end > self.buf.len() {
            return decode_error("mpa: invalid main_data length, will exceed resevoir buffer");
        }

        let unread = self.len - self.consumed;

        // If the offset is less-than or equal to the amount of unread data in the resevoir, shift
        // the re-used bytes to the beginning of the resevoir, then copy the main data of the
        // current packet into the resevoir.
        let underflow = if main_data_begin <= unread {
            // Shift all the re-used bytes as indicated by main_data_begin to the front of the
            // resevoir.
            self.buf.copy_within(self.len - main_data_begin..self.len, 0);

            // Copy the new main data from the packet buffer after the re-used bytes.
            self.buf[main_data_begin..main_data_end].copy_from_slice(pkt_main_data);
            self.len = main_data_end;

            0
        }
        else {
            // Shift all the unread bytes to the front of the resevoir. Since this is an underflow
            // condition, all unread bytes will be unconditionally reused.
            self.buf.copy_within(self.len - unread..self.len, 0);

            // If the offset is greater than the amount of data in the resevoir, then the stream is
            // malformed. This can occur if the decoder is starting in the middle of a stream. This
            // is particularly common with online radio streams. In this case, copy the main data
            // of the current packet into the resevoir, then return the number of bytes that are
            // missing.
            self.buf[unread..unread + main_data_len].copy_from_slice(pkt_main_data);
            self.len = unread + main_data_len;

            // The number of bytes that will be missing.
            let underflow = (main_data_begin - unread) as u32;

            warn!("mpa: invalid main_data_begin, underflow by {} bytes", underflow);

            underflow
        };

        self.consumed = 0;

        Ok(underflow)
    }

    pub fn consume(&mut self, len: usize) {
        self.consumed = self.len.min(self.consumed + len);
    }

    pub fn bytes_ref(&self) -> &[u8] {
        &self.buf[self.consumed..self.len]
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.consumed = 0;
    }
}

/// `FrameData` contains the side_info and main_data portions of a MPEG audio frame.
#[derive(Default, Debug)]
struct FrameData {
    /// The byte offset into the bit resevoir indicating the location of the first bit of main_data.
    /// If 0, main_data begins after the side_info of this frame.
    main_data_begin: u16,
    /// Scale factor selector information, per channel. Each channel has 4 groups of bands that may
    /// be scaled in each granule. Scale factors may optionally be used by both granules to save
    /// bits. Bands that share scale factors for both granules are indicated by a true. Otherwise,
    /// each granule must store its own set of scale factors.
    ///
    /// Mapping of array indicies to bands [0..6, 6..11, 11..16, 16..21].
    scfsi: [[bool; 4]; 2],
    /// The granules.
    granules: [Granule; 2],
}

impl FrameData {
    /// Get a mutable slice to the granule(s) in side_info. For MPEG1, a slice of 2 granules are
    /// returned. For MPEG2/2.5, a single granule slice is returned.
    #[inline(always)]
    fn granules_mut(&mut self, version: MpegVersion) -> &mut [Granule] {
        match version {
            MpegVersion::Mpeg1 => &mut self.granules[..2],
            _ => &mut self.granules[..1],
        }
    }
}

#[derive(Default, Debug)]
struct Granule {
    /// Channels in the granule.
    channels: [GranuleChannel; 2],
}

struct GranuleChannel {
    /// Total number of bits used for scale factors (part2) and Huffman encoded data (part3).
    part2_3_length: u16,
    /// HALF the number of samples in the big_values partition (sum of all samples in
    /// `region[0..3]`).
    big_values: u16,
    /// Logarithmic quantization step size.
    global_gain: u8,
    /// Depending on the MPEG version, `scalefac_compress` determines how many bits are allocated
    /// per scale factor.
    ///
    /// - For MPEG1 bitstreams, `scalefac_compress` is a 4-bit index into
    ///   `SCALE_FACTOR_SLEN[0..16]` to obtain a number of bits per scale factor pair.
    ///
    /// - For MPEG2/2.5 bitstreams, `scalefac_compress` is a 9-bit value that decodes into
    ///   `slen[0..3]` (referred to as slen1-4 in the standard) for the number of bits per scale
    ///   factor, and depending on which range the value falls into, for which bands.
    scalefac_compress: u16,
    /// Indicates the block type (type of window) for the channel in the granule.
    block_type: BlockType,
    /// Gain factors for region[0..3] in big_values. Each gain factor has a maximum value of 7
    /// (3 bits).
    subblock_gain: [u8; 3],
    /// The Huffman table to use for decoding `region[0..3]` of big_values.
    table_select: [u8; 3],
    /// The index of the first sample in region1 of big_values.
    region1_start: usize,
    /// The index of the first sample in region2 of big_values.
    region2_start: usize,
    /// Indicates if the pre-emphasis amount for each scale factor band should be added on to each
    /// scale factor before requantization.
    preflag: bool,
    /// A 0.5x (false) or 1x (true) multiplier for scale factors.
    scalefac_scale: bool,
    /// Use Huffman Quads table A (0) or B (1), for decoding the count1 partition.
    count1table_select: u8,
    /// Long (scalefac_l) and short (scalefac_s) window scale factor bands. Must be interpreted
    /// based on the block type of the granule.
    ///
    /// For `block_type == BlockType::Short { is_mixed: false }`:
    ///   - `scalefac_s[0..36]` -> `scalefacs[0..36]`
    ///
    /// For `block_type == BlockType::Short { is_mixed: true }`:
    ///   - `scalefac_l[0..8]`  -> `scalefacs[0..8]`
    ///   - `scalefac_s[0..27]` -> `scalefacs[8..35]`
    ///
    /// For `block_type != BlockType::Short { .. }`:
    ///   - `scalefac_l[0..21]` -> `scalefacs[0..21]`
    ///
    /// Note: The standard doesn't explicitly call it out, but for Short blocks, there are three
    ///       additional scale factors, `scalefacs[36..39]`, that are always 0 and are not
    ///       transmitted in the bitstream.
    ///
    /// For MPEG1, and MPEG2 without intensity stereo coding, a scale factor will not exceed 4 bits
    /// in length (maximum value 15). For MPEG2 with intensity stereo, a scale factor will not
    /// exceed 5 bits (maximum value 31) in length.
    scalefacs: [u8; 39],
    /// The starting sample index of the rzero partition, or the count of big_values and count1
    /// samples.
    rzero: usize,
}

impl Default for GranuleChannel {
    fn default() -> Self {
        GranuleChannel {
            part2_3_length: 0,
            big_values: 0,
            global_gain: 0,
            scalefac_compress: 0,
            block_type: BlockType::Long,
            subblock_gain: [0; 3],
            table_select: [0; 3],
            region1_start: 0,
            region2_start: 0,
            preflag: false,
            scalefac_scale: false,
            count1table_select: 0,
            scalefacs: [0; 39],
            rzero: 0,
        }
    }
}

impl fmt::Debug for GranuleChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GranuleChannel {{")?;
        writeln!(f, "\tpart2_3_length={}", self.part2_3_length)?;
        writeln!(f, "\tbig_values={}", self.big_values)?;
        writeln!(f, "\tglobal_gain={}", self.global_gain)?;
        writeln!(f, "\tscalefac_compress={}", self.scalefac_compress)?;
        writeln!(f, "\tblock_type={:?}", self.block_type)?;
        writeln!(f, "\tsubblock_gain={:?}", self.subblock_gain)?;
        writeln!(f, "\ttable_select={:?}", self.table_select)?;
        writeln!(f, "\tregion1_start={}", self.region1_start)?;
        writeln!(f, "\tregion2_start={}", self.region2_start)?;
        writeln!(f, "\tpreflag={}", self.preflag)?;
        writeln!(f, "\tscalefac_scale={}", self.scalefac_scale)?;
        writeln!(f, "\tcount1table_select={}", self.count1table_select)?;

        write!(f, "\tscalefacs=[ ")?;
        for sf in &self.scalefacs[..] {
            write!(f, "{}, ", sf)?;
        }
        writeln!(f, "]")?;
        writeln!(f, "\trzero={}", self.rzero)?;
        writeln!(f, "}}")
    }
}

pub struct Layer3 {
    pub samples: [[[f32; 576]; 2]; 2],
    pub overlap: [[[f32; 18]; 32]; 2],
    pub synthesis: [synthesis::SynthesisState; 2],
    pub resevoir: BitResevoir,
}

impl Layer3 {
    pub fn new() -> Self {
        Self {
            samples: [[[0f32; 576]; 2]; 2],
            overlap: [[[0f32; 18]; 32]; 2],
            synthesis: Default::default(),
            resevoir: BitResevoir::new(),
        }
    }

    /// Reads the main_data portion of a MPEG audio frame from a `BitStream` into `FrameData`.
    fn read_main_data(
        &mut self,
        header: &FrameHeader,
        underflow_bits: u32,
        frame_data: &mut FrameData,
    ) -> Result<usize> {
        let main_data = self.resevoir.bytes_ref();
        let mut part2_3_begin = 0;
        let mut part2_3_skipped = 0;

        for gr in 0..header.n_granules() {
            // If the resevoir underflowed (i.e., main_data_begin references bits not present in the
            // resevoir) then skip the granule(s) the missing bits would belong to.
            if part2_3_skipped < underflow_bits {
                // Zero the samples in the granule channel(s) and sum the part2/3 bits that were
                // skipped.
                for ch in 0..header.n_channels() {
                    requantize::zero(&mut self.samples[gr][ch]);
                    part2_3_skipped +=
                        u32::from(frame_data.granules[gr].channels[ch].part2_3_length);
                }

                // Adjust the start position of the next granule in the buffer of main data that is
                // available.
                if part2_3_skipped > underflow_bits {
                    part2_3_begin = (part2_3_skipped - underflow_bits) as usize;
                }

                // Continue at the next granule.
                continue;
            }

            for ch in 0..header.n_channels() {
                let byte_index = part2_3_begin >> 3;

                // Create a bit reader at the expected starting bit position.
                let mut bs = if byte_index < main_data.len() {
                    let mut bs = BitReaderLtr::new(&main_data[byte_index..]);

                    let bit_index = part2_3_begin & 0x7;

                    if bit_index > 0 {
                        bs.ignore_bits(bit_index as u32)?;
                    }

                    bs
                }
                else {
                    return decode_error("mpa: invalid main_data offset");
                };

                // Read the scale factors (part2) and get the number of bits read.
                let part2_len = if header.is_mpeg1() {
                    bitstream::read_scale_factors_mpeg1(&mut bs, gr, ch, frame_data)
                }
                else {
                    bitstream::read_scale_factors_mpeg2(
                        &mut bs,
                        ch > 0 && header.is_intensity_stereo(),
                        &mut frame_data.granules[gr].channels[ch],
                    )
                }?;

                let part2_3_length = u32::from(frame_data.granules[gr].channels[ch].part2_3_length);

                // The part2 length must be less than or equal to the part2_3_length.
                if part2_len > part2_3_length {
                    return decode_error("mpa: part2_3_length is not valid");
                }

                // The Huffman code length (part3).
                let part3_len = part2_3_length - part2_len;

                // Decode the Huffman coded spectral samples and get the starting index of the rzero
                // partition.
                let huffman_result = requantize::read_huffman_samples(
                    &mut bs,
                    &frame_data.granules[gr].channels[ch],
                    part3_len,
                    &mut self.samples[gr][ch],
                );

                // Huffman decoding errors are returned as an IO error by the bit reader. IO errors
                // are unrecoverable, which is not the case for huffman decoding errors. Convert the
                // IO error to a decode error.
                frame_data.granules[gr].channels[ch].rzero = match huffman_result {
                    Ok(rzero) => rzero,
                    Err(Error::IoError(e)) if e.kind() == std::io::ErrorKind::Other => {
                        return decode_error("mpa: huffman decode overrun");
                    }
                    Err(err) => return Err(err),
                };

                part2_3_begin += part2_3_length as usize;
            }
        }

        Ok((part2_3_begin + 7) >> 3)
    }
}

impl Layer for Layer3 {
    fn decode(
        &mut self,
        reader: &mut BufReader<'_>,
        header: &FrameHeader,
        out: &mut AudioBuffer<f32>,
    ) -> Result<()> {
        // Initialize an empty FrameData to store the side_info and main_data portions of the
        // frame.
        let mut frame_data: FrameData = Default::default();

        let _crc = if header.has_crc { Some(reader.read_be_u16()?) } else { None };

        let buf = reader.read_buf_bytes_available_ref();

        let mut bs = BitReaderLtr::new(buf);

        // Read side_info into the frame data.
        // TODO: Use a MonitorStream to compute the CRC.
        let side_info_len = match bitstream::read_side_info(&mut bs, header, &mut frame_data) {
            Ok(len) => len,
            Err(e) => {
                // A failure in reading this packet will cause a discontinuity in the codec
                // bitstream. Therefore, clear the bit reservoir since it will not be valid for the
                // next packet.
                self.resevoir.clear();
                return Err(e);
            }
        };

        // Buffer main data into the bit resevoir.
        let underflow =
            self.resevoir.fill(&buf[side_info_len..], frame_data.main_data_begin as usize)?;

        // Read the main data (scale factors and spectral samples).
        match self.read_main_data(header, 8 * underflow, &mut frame_data) {
            Ok(len) => {
                // Consume the bytes of main data read from the resevoir.
                self.resevoir.consume(len);
            }
            Err(e) => {
                // The bit reservoir was likely filled with invalid data. Clear it for the next
                // packet.
                self.resevoir.clear();
                return Err(e);
            }
        }

        for gr in 0..header.n_granules() {
            let granule = &mut frame_data.granules[gr];

            // Requantize all non-zero (big_values and count1 partition) spectral samples.
            requantize::requantize(header, &granule.channels[0], &mut self.samples[gr][0]);

            // If there is a second channel...
            if header.channel_mode != ChannelMode::Mono {
                // Requantize all non-zero spectral samples in the second channel.
                requantize::requantize(header, &granule.channels[1], &mut self.samples[gr][1]);

                // Apply joint stereo processing if it is used.
                stereo::stereo(header, granule, &mut self.samples[gr])?;
            }

            // Each granule will yield 576 samples. After reserving frames, all steps must be
            // infalliable.
            out.render_reserved(Some(576));

            // The next steps are independant of channel count.
            for ch in 0..header.n_channels() {
                // Reorder the spectral samples in short blocks into sub-band order.
                hybrid_synthesis::reorder(
                    header,
                    &mut granule.channels[ch],
                    &mut self.samples[gr][ch],
                );

                // Apply the anti-aliasing filter to all block types other than short.
                hybrid_synthesis::antialias(&mut granule.channels[ch], &mut self.samples[gr][ch]);

                // Perform hybrid-synthesis (IMDCT and windowing). After this step, rzero is invalid
                // due to the overlap-add operation.
                hybrid_synthesis::hybrid_synthesis(
                    &granule.channels[ch],
                    &mut self.overlap[ch],
                    &mut self.samples[gr][ch],
                );

                // Invert every second sample in every second sub-band to negate the frequency
                // inversion of the polyphase filterbank.
                hybrid_synthesis::frequency_inversion(&mut self.samples[gr][ch]);

                // Perform polyphase synthesis and generate PCM samples.
                let out_ch_samples = out.chan_mut(ch);

                synthesis::synthesis(
                    &mut self.synthesis[ch],
                    18,
                    &self.samples[gr][ch],
                    &mut out_ch_samples[(gr * 576)..((gr + 1) * 576)],
                );
            }
        }

        Ok(())
    }
}
