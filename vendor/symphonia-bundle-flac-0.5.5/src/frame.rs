// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::checksum::Crc8Ccitt;
use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::{Monitor, MonitorStream, ReadBytes};

/// The minimum FLAC frame header size including the sync bytes.
pub const FLAC_MIN_FRAME_HEADER_SIZE: usize = 6;
/// The maximum FLAC frame header size including the sync bytes.
pub const FLAC_MAX_FRAME_HEADER_SIZE: usize = 16;

/// The maximum FLAC frame size.
pub const FLAC_MAX_FRAME_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug)]
enum BlockingStrategy {
    Fixed,
    Variable,
}

#[derive(Debug)]
pub enum BlockSequence {
    BySample(u64),
    ByFrame(u32),
}

/// `ChannelAssignment` describes the mapping between the samples decoded from a subframe and the
/// channel those samples belong to. It is also through the `ChannelAssignment` that the decoder is
/// instructed on how to decorrelate stereo channels.
//
/// For LeftSide or RightSide channel assignments, one channel is stored independently while the
/// other stores a difference. The Difference is always stored as Left - Right. For the MidSide
/// channel assignment, no channels are stored independently, rather, a Mid (average) channel and a
/// Difference channel are stored.
#[derive(Debug)]
pub enum ChannelAssignment {
    /// All channels are independantly coded and no decorrelation step is required.
    Independant(u32),
    /// Channel 0 is the Left channel, and channel 1 is a Difference channel. The Right channel
    /// is restored by subtracting the Difference channel from the Left channel (R = L - D).
    LeftSide,
    /// Channel 0 is the Mid channel (Left/2 + Right/2), and channel 1 is the Difference channel
    /// (Left - Right). Therefore, if M = L/2 + R/2 and D = L - R, solving for L and R the left
    /// and right channels are: L = S/2 + M, and R = M - S/2.
    MidSide,
    /// Channel 0 is the Difference channel, and channel 1 is the Right channel. The Left channel
    /// is restored by adding the Difference channel to the Right channel (L = R + D).
    RightSide,
}

pub struct FrameHeader {
    pub block_sequence: BlockSequence,
    pub block_num_samples: u16,
    pub channel_assignment: ChannelAssignment,
    pub bits_per_sample: Option<u32>,
    pub sample_rate: Option<u32>,
}

pub fn sync_frame<B: ReadBytes>(reader: &mut B) -> Result<u16> {
    let mut sync = 0u16;

    // Synchronize stream to Frame Header. FLAC specifies a byte-aligned 14 bit sync code of
    // `0b11_1111_1111_1110`. This would be difficult to find on its own. Expand the search to
    // a 16-bit field of `0b1111_1111_1111_10xx` and search a word at a time.
    while (sync & 0xfffc) != 0xfff8 {
        sync = sync.wrapping_shl(8) | u16::from(reader.read_u8()?);
    }

    Ok(sync)
}

pub fn read_frame_header<B: ReadBytes>(reader: &mut B, sync: u16) -> Result<FrameHeader> {
    // The header is checksummed with a CRC8 hash. Include the sync code in this CRC.
    let mut crc8 = Crc8Ccitt::new(0);
    crc8.process_buf_bytes(&sync.to_be_bytes());

    let mut reader_crc8 = MonitorStream::new(reader, crc8);

    // Extract the blocking strategy from the expanded synchronization code.
    let blocking_strategy = match sync & 0x1 {
        0 => BlockingStrategy::Fixed,
        _ => BlockingStrategy::Variable,
    };

    // Read all the standard frame description fields as one 16-bit value and extract the
    // fields.
    let desc = reader_crc8.read_be_u16()?;

    let block_size_enc = u32::from((desc & 0xf000) >> 12);
    let sample_rate_enc = u32::from((desc & 0x0f00) >> 8);
    let channels_enc = u32::from((desc & 0x00f0) >> 4);
    let bits_per_sample_enc = u32::from((desc & 0x000e) >> 1);

    if (desc & 0x0001) == 1 {
        return decode_error("flac: frame header reserved bit is not set to mandatory value");
    }

    let block_sequence = match blocking_strategy {
        // Fixed-blocksize stream sequence blocks by a frame number.
        BlockingStrategy::Fixed => {
            let frame = match utf8_decode_be_u64(&mut reader_crc8)? {
                Some(frame) => frame,
                None => return decode_error("flac: frame sequence number is not valid"),
            };

            // The frame number should only be 31-bits. Since it is UTF8 encoded, the actual length
            // cannot be enforced by the decoder. Return an error if the frame number exceeds the
            // maximum 31-bit value.
            if frame > 0x7fff_ffff {
                return decode_error("flac: frame sequence number exceeds 31-bits");
            }

            BlockSequence::ByFrame(frame as u32)
        }
        // Variable-blocksize streams sequence blocks by a sample number.
        BlockingStrategy::Variable => {
            let sample = match utf8_decode_be_u64(&mut reader_crc8)? {
                Some(sample) => sample,
                None => return decode_error("flac: sample sequence number is not valid"),
            };

            // The sample number should only be 36-bits. Since it is UTF8 encoded, the actual length
            // cannot be enforced by the decoder. Return an error if the frame number exceeds the
            // maximum 36-bit value.
            if sample > 0x000f_ffff_ffff {
                return decode_error("flac: sample sequence number exceeds 36-bits");
            }

            BlockSequence::BySample(sample)
        }
    };

    let block_num_samples = match block_size_enc {
        0x1 => 192,
        0x2..=0x5 => 576 * (1 << (block_size_enc - 2)),
        0x6 => u16::from(reader_crc8.read_u8()?) + 1,
        0x7 => {
            let block_size = reader_crc8.read_be_u16()?;
            if block_size == 0xffff {
                return decode_error("flac: block size not allowed to be greater than 65535");
            }
            block_size + 1
        }
        0x8..=0xf => 256 * (1 << (block_size_enc - 8)),
        _ => {
            return decode_error("flac: block size set to reserved value");
        }
    };

    let sample_rate = match sample_rate_enc {
        0x0 => None, // Get from StreamInfo if possible.
        0x1 => Some(88_200),
        0x2 => Some(176_400),
        0x3 => Some(192_000),
        0x4 => Some(8_000),
        0x5 => Some(16_000),
        0x6 => Some(22_050),
        0x7 => Some(24_000),
        0x8 => Some(32_000),
        0x9 => Some(44_100),
        0xa => Some(48_000),
        0xb => Some(96_000),
        0xc => Some(u32::from(reader_crc8.read_u8()?) * 1000),
        0xd => Some(u32::from(reader_crc8.read_be_u16()?)),
        0xe => Some(u32::from(reader_crc8.read_be_u16()?) * 10),
        _ => {
            return decode_error("flac: sample rate set to reserved value");
        }
    };

    if let Some(rate) = sample_rate {
        if rate < 1 || rate > 655_350 {
            return decode_error("flac: sample rate out of bounds");
        }
    }

    let bits_per_sample = match bits_per_sample_enc {
        0x0 => None, // Get from StreamInfo if possible.
        0x1 => Some(8),
        0x2 => Some(12),
        0x4 => Some(16),
        0x5 => Some(20),
        0x6 => Some(24),
        _ => {
            return decode_error("flac: bits per sample set to reserved value");
        }
    };

    let channel_assignment = match channels_enc {
        0x0..=0x7 => ChannelAssignment::Independant(channels_enc + 1),
        0x8 => ChannelAssignment::LeftSide,
        0x9 => ChannelAssignment::RightSide,
        0xa => ChannelAssignment::MidSide,
        _ => {
            return decode_error("flac: channel assignment set to reserved value");
        }
    };

    // End of freame header, pop off CRC8 checksum.
    let crc8_computed = reader_crc8.monitor().crc();

    // Get expected CRC8 checksum from the header.
    let crc8_expected = reader_crc8.into_inner().read_u8()?;

    if crc8_expected != crc8_computed && cfg!(not(fuzzing)) {
        return decode_error("flac: computed frame header CRC does not match expected CRC");
    }

    Ok(FrameHeader {
        block_sequence,
        block_num_samples,
        channel_assignment,
        bits_per_sample,
        sample_rate,
    })
}

/// A very quick check if the provided buffer is likely be a FLAC frame header.
pub fn is_likely_frame_header(buf: &[u8]) -> bool {
    // Minimum frame header size.
    if buf.len() < FLAC_MIN_FRAME_HEADER_SIZE {
        return false;
    }

    // First sync word.
    if buf[0] != 0xff {
        return false;
    }

    // Second sync word.
    if (buf[1] & 0xfc) != 0xf8 {
        return false;
    }

    // Reserved block size.
    if (buf[2] & 0xf0) == 0x00 {
        return false;
    }

    // Reserved sample rate.
    if (buf[2] & 0x0f) == 0x0f {
        return false;
    }

    // Reserved channel assignments 0xb to 0xf.
    if ((buf[3] & 0xf0) >> 4) >= 0xb {
        return false;
    }

    // Reserved sample size.
    if (buf[3] & 0x0e == 0x6) || (buf[3] & 0x0e == 0x0e) {
        return false;
    }

    // Reserved bit.
    if buf[3] & 0x1 == 1 {
        return false;
    }

    true
}

/// Decodes a big-endian unsigned integer encoded via extended UTF8. In this context, extended UTF8
/// simply means the encoded UTF8 value may be up to 7 bytes for a maximum integer bit width of
/// 36-bits.
fn utf8_decode_be_u64<B: ReadBytes>(src: &mut B) -> Result<Option<u64>> {
    // Read the first byte of the UTF8 encoded integer.
    let mut state = u64::from(src.read_u8()?);

    // UTF8 prefixes 1s followed by a 0 to indicate the total number of bytes within the multi-byte
    // sequence. Using ranges, determine the mask that will overlap the data bits within the first
    // byte of the sequence. For values 0-128, return the value immediately. If the value falls out
    // of range return None as this is either not the start of a UTF8 sequence or the prefix is
    // incorrect.
    let mask: u8 = match state {
        0x00..=0x7f => return Ok(Some(state)),
        0xc0..=0xdf => 0x1f,
        0xe0..=0xef => 0x0f,
        0xf0..=0xf7 => 0x07,
        0xf8..=0xfb => 0x03,
        0xfc..=0xfd => 0x01,
        0xfe => 0x00,
        _ => return Ok(None),
    };

    // Obtain the data bits from the first byte by using the data mask.
    state &= u64::from(mask);

    // Read the remaining bytes within the UTF8 sequence. Since the mask 0s out the UTF8 prefix
    // of 1s which indicate the length of the multi-byte sequence in bytes, plus an additional 0
    // bit, the number of remaining bytes to read is the number of zeros in the mask minus 2.
    // To avoid extra computation, simply loop from 2 to the number of zeros.
    for _i in 2..mask.leading_zeros() {
        // Each subsequent byte after the first in UTF8 is prefixed with 0b10xx_xxxx, therefore
        // only 6 bits are useful. Append these six bits to the result by shifting the result left
        // by 6 bit positions, and appending the next subsequent byte with the first two high-order
        // bits masked out.
        state = (state << 6) | u64::from(src.read_u8()? & 0x3f);

        // TODO: Validation? Invalid if the byte is greater than 0x3f.
    }

    Ok(Some(state))
}

#[cfg(test)]
mod tests {
    use super::utf8_decode_be_u64;
    use symphonia_core::io::BufReader;

    #[test]
    fn verify_utf8_decode_be_u64() {
        let mut stream = BufReader::new(&[
            0x24, 0xc2, 0xa2, 0xe0, 0xa4, 0xb9, 0xe2, 0x82, //
            0xac, 0xf0, 0x90, 0x8d, 0x88, 0xff, 0x80, 0xbf, //
        ]);

        assert_eq!(utf8_decode_be_u64(&mut stream).unwrap(), Some(36));
        assert_eq!(utf8_decode_be_u64(&mut stream).unwrap(), Some(162));
        assert_eq!(utf8_decode_be_u64(&mut stream).unwrap(), Some(2361));
        assert_eq!(utf8_decode_be_u64(&mut stream).unwrap(), Some(8364));
        assert_eq!(utf8_decode_be_u64(&mut stream).unwrap(), Some(66376));
        assert_eq!(utf8_decode_be_u64(&mut stream).unwrap(), None);
        assert_eq!(utf8_decode_be_u64(&mut stream).unwrap(), None);
        assert_eq!(utf8_decode_be_u64(&mut stream).unwrap(), None);
    }
}
