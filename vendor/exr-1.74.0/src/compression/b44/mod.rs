mod table;

use crate::compression::{mod_p, ByteVec};
use crate::error::usize_to_i32;
use crate::io::Data;
use crate::meta::attribute::ChannelList;
use crate::prelude::*;
use std::cmp::min;
use std::mem::size_of;
use table::{EXP_TABLE, LOG_TABLE};
use lebe::io::{ReadPrimitive, WriteEndian};

const BLOCK_SAMPLE_COUNT: usize = 4;

// As B44 compression is only use on f16 channels, we can have a conste for this value.
const BLOCK_X_BYTE_COUNT: usize = BLOCK_SAMPLE_COUNT * size_of::<u16>();

#[inline]
fn convert_from_linear(s: &mut [u16; 16]) {
    for v in s {
        *v = EXP_TABLE[*v as usize];
    }
}

#[inline]
fn convert_to_linear(s: &mut [u16; 16]) {
    for v in s {
        *v = LOG_TABLE[*v as usize];
    }
}

#[inline]
fn shift_and_round(x: i32, shift: i32) -> i32 {
    let x = x << 1;
    let a = (1 << shift) - 1;
    let shift = shift + 1;
    let b = (x >> shift) & 1;
    (x + a + b) >> shift
}

/// Pack a block of 4 by 4 16-bit pixels (32 bytes, the array `s`) into either 14 or 3 bytes.
fn pack(s: [u16; 16], b: &mut [u8], optimize_flat_fields: bool, exact_max: bool) -> usize {

    let mut t = [0u16; 16];

    for i in 0..16 {
        if (s[i] & 0x7c00) == 0x7c00 {
            t[i] = 0x8000;
        } else if (s[i] & 0x8000) != 0 {
            t[i] = !s[i];
        } else {
            t[i] = s[i] | 0x8000;
        }
    }

    let t_max = t.iter().max().unwrap();

    // Compute a set of running differences, r[0] ... r[14]:
    // Find a shift value such that after rounding off the
    // rightmost bits and shifting all differences are between
    // -32 and +31.  Then bias the differences so that they
    // end up between 0 and 63.
    let mut shift = -1;
    let mut d = [0i32; 16];
    let mut r = [0i32; 15];
    let mut r_min: i32;
    let mut r_max: i32;

    const BIAS: i32 = 0x20;

    loop {
        shift += 1;

        // Compute absolute differences, d[0] ... d[15],
        // between t_max and t[0] ... t[15].
        //
        // Shift and round the absolute differences.
        d.iter_mut()
            .zip(&t)
            .for_each(|(d_v, t_v)| *d_v = shift_and_round((t_max - t_v).into(), shift));

        // Convert d[0] .. d[15] into running differences
        r[0] = d[0] - d[4] + BIAS;
        r[1] = d[4] - d[8] + BIAS;
        r[2] = d[8] - d[12] + BIAS;

        r[3] = d[0] - d[1] + BIAS;
        r[4] = d[4] - d[5] + BIAS;
        r[5] = d[8] - d[9] + BIAS;
        r[6] = d[12] - d[13] + BIAS;

        r[7] = d[1] - d[2] + BIAS;
        r[8] = d[5] - d[6] + BIAS;
        r[9] = d[9] - d[10] + BIAS;
        r[10] = d[13] - d[14] + BIAS;

        r[11] = d[2] - d[3] + BIAS;
        r[12] = d[6] - d[7] + BIAS;
        r[13] = d[10] - d[11] + BIAS;
        r[14] = d[14] - d[15] + BIAS;

        r_min = r[0];
        r_max = r[0];

        r.iter().copied().for_each(|v| {
            if r_min > v {
                r_min = v;
            }

            if r_max < v {
                r_max = v;
            }
        });

        if !(r_min < 0 || r_max > 0x3f) {
            break;
        }
    }

    if r_min == BIAS && r_max == BIAS && optimize_flat_fields {
        // Special case - all pixels have the same value.
        // We encode this in 3 instead of 14 bytes by
        // storing the value 0xfc in the third output byte,
        // which cannot occur in the 14-byte encoding.
        b[0] = (t[0] >> 8) as u8;
        b[1] = t[0] as u8;
        b[2] = 0xfc;

        return 3;
    }

    if exact_max {
        // Adjust t[0] so that the pixel whose value is equal
        // to t_max gets represented as accurately as possible.
        t[0] = t_max - (d[0] << shift) as u16;
    }

    // Pack t[0], shift and r[0] ... r[14] into 14 bytes:
    b[0] = (t[0] >> 8) as u8;
    b[1] = t[0] as u8;

    b[2] = ((shift << 2) | (r[0] >> 4)) as u8;
    b[3] = ((r[0] << 4) | (r[1] >> 2)) as u8;
    b[4] = ((r[1] << 6) | r[2]) as u8;

    b[5] = ((r[3] << 2) | (r[4] >> 4)) as u8;
    b[6] = ((r[4] << 4) | (r[5] >> 2)) as u8;
    b[7] = ((r[5] << 6) | r[6]) as u8;

    b[8] = ((r[7] << 2) | (r[8] >> 4)) as u8;
    b[9] = ((r[8] << 4) | (r[9] >> 2)) as u8;
    b[10] = ((r[9] << 6) | r[10]) as u8;

    b[11] = ((r[11] << 2) | (r[12] >> 4)) as u8;
    b[12] = ((r[12] << 4) | (r[13] >> 2)) as u8;
    b[13] = ((r[13] << 6) | r[14]) as u8;

    return 14;
}

// Tiny macro to simply get block array value as a u32.
macro_rules! b32 {
    ($b:expr, $i:expr) => {
        $b[$i] as u32
    };
}

// 0011 1111
const SIX_BITS: u32 = 0x3f;

// Unpack a 14-byte block into 4 by 4 16-bit pixels.
fn unpack14(b: &[u8], s: &mut [u16; 16]) {
    debug_assert_eq!(b.len(), 14);
    debug_assert_ne!(b[2], 0xfc);

    s[0] = ((b32!(b, 0) << 8) | b32!(b, 1)) as u16;

    let shift = b32!(b, 2) >> 2;
    let bias = 0x20 << shift;

    s[4] = (s[0] as u32 + ((((b32!(b, 2) << 4) | (b32!(b, 3) >> 4)) & SIX_BITS) << shift) - bias) as u16;
    s[8] = (s[4] as u32 + ((((b32!(b, 3) << 2) | (b32!(b, 4) >> 6)) & SIX_BITS) << shift) - bias) as u16;
    s[12] = (s[8] as u32 + ((b32!(b, 4) & SIX_BITS) << shift) - bias) as u16;

    s[1] = (s[0] as u32 + ((b32!(b, 5) >> 2) << shift) - bias) as u16;
    s[5] = (s[4] as u32 + ((((b32!(b, 5) << 4) | (b32!(b, 6) >> 4)) & SIX_BITS) << shift) - bias) as u16;
    s[9] = (s[8] as u32 + ((((b32!(b, 6) << 2) | (b32!(b, 7) >> 6)) & SIX_BITS) << shift) - bias) as u16;
    s[13] = (s[12] as u32 + ((b32!(b, 7) & SIX_BITS) << shift) - bias) as u16;

    s[2] = (s[1] as u32 + ((b32!(b, 8) >> 2) << shift) - bias) as u16;
    s[6] = (s[5] as u32 + ((((b32!(b, 8) << 4) | (b32!(b, 9) >> 4)) & SIX_BITS) << shift)  - bias) as u16;
    s[10] = (s[9] as u32 + ((((b32!(b, 9) << 2) | (b32!(b, 10) >> 6)) & SIX_BITS) << shift) - bias) as u16;
    s[14] = (s[13] as u32 + ((b32!(b, 10) & SIX_BITS) << shift) - bias) as u16;

    s[3] = (s[2] as u32 + ((b32!(b, 11) >> 2) << shift) - bias) as u16;
    s[7] = (s[6] as u32 + ((((b32!(b, 11) << 4) | (b32!(b, 12) >> 4)) & SIX_BITS) << shift) - bias) as u16;
    s[11] = (s[10] as u32 + ((((b32!(b, 12) << 2) | (b32!(b, 13) >> 6)) & SIX_BITS) << shift) - bias) as u16;
    s[15] = (s[14] as u32 + ((b32!(b, 13) & SIX_BITS) << shift) - bias) as u16;

    for i in 0..16 {
        if (s[i] & 0x8000) != 0 {
            s[i] &= 0x7fff;
        } else {
            s[i] = !s[i];
        }
    }
}

// Unpack a 3-byte block `b` into 4 by 4 identical 16-bit pixels in `s` array.
fn unpack3(b: &[u8], s: &mut [u16; 16]) {
    // this assertion panics for fuzzed images.
    // assuming this debug assertion is an overly strict check to catch potential compression errors.
    // disabling because it panics when fuzzed.
    // when commenting out, it simply works (maybe it should return an error instead?).
    // debug_assert_eq!(b[2], 0xfc);

    // Get the 16-bit value from the block.
    let mut value = ((b32!(b, 0) << 8) | b32!(b, 1)) as u16;

    if (value & 0x8000) != 0 {
        value &= 0x7fff;
    } else {
        value = !value;
    }

    s.fill(value); // All pixels have save value.
}

#[derive(Debug)]
struct ChannelData {
    tmp_start_index: usize,
    tmp_end_index: usize,
    resolution: Vec2<usize>,
    y_sampling: usize,
    sample_type: SampleType,
    quantize_linearly: bool,
    samples_per_pixel: usize,
}

// TODO: Unsafe seems to be required to efficiently copy whole slice of u16 ot u8. For now, we use
//   a less efficient, yet safe, implementation.
#[inline]
fn memcpy_u16_to_u8(src: &[u16], mut dst: &mut [u8]) {
    use lebe::prelude::*;
    dst.write_as_native_endian(src).expect("byte copy error");
}

#[inline]
fn memcpy_u8_to_u16(mut src: &[u8], dst: &mut [u16]) {
    use lebe::prelude::*;
    src.read_from_native_endian_into(dst).expect("byte copy error");
}

#[inline]
fn cpy_u8(src: &[u16], src_i: usize, dst: &mut [u8], dst_i: usize, n: usize) {
    memcpy_u16_to_u8(&src[src_i..src_i + n], &mut dst[dst_i..dst_i + 2 * n]);
}

pub fn decompress(
    channels: &ChannelList,
    compressed_le: ByteVec,
    rectangle: IntegerBounds,
    expected_byte_size: usize,
    _pedantic: bool,
) -> Result<ByteVec> {
    debug_assert_eq!(
        expected_byte_size,
        rectangle.size.area() * channels.bytes_per_pixel,
        "expected byte size does not match header" // TODO compute instead of passing argument?
    );

    debug_assert!(!channels.list.is_empty(), "no channels found");

    if compressed_le.is_empty() {
        return Ok(Vec::new());
    }

    // Extract channel information needed for decompression.
    let mut channel_data: Vec<ChannelData> = Vec::with_capacity(channels.list.len());
    let mut tmp_read_index = 0;

    for channel in channels.list.iter() {
        let channel = ChannelData {
            tmp_start_index: tmp_read_index,
            tmp_end_index: tmp_read_index,
            resolution: channel.subsampled_resolution(rectangle.size),
            y_sampling: channel.sampling.y(),
            sample_type: channel.sample_type,
            quantize_linearly: channel.quantize_linearly,
            samples_per_pixel: channel.sampling.area(),
        };

        tmp_read_index += channel.resolution.area()
            * channel.samples_per_pixel
            * channel.sample_type.bytes_per_sample();

        channel_data.push(channel);
    }

    // Temporary buffer is used to decompress B44 datas the way they are stored in the compressed
    // buffer (channel by channel). We interleave the final result later.
    let mut tmp = Vec::with_capacity(expected_byte_size);

    // Index in the compressed buffer.
    let mut in_i = 0usize;

    let mut remaining_le = compressed_le.len();

    for channel in &channel_data {

        debug_assert_eq!(remaining_le, compressed_le.len() - in_i);

        // Compute information for current channel.
        let sample_count = channel.resolution.area() * channel.samples_per_pixel;
        let byte_count = sample_count * channel.sample_type.bytes_per_sample();

        // Sample types that does not support B44 compression (u32 and f32) are raw copied.
        // In this branch, "compressed" array is actually raw, uncompressed data.
        if channel.sample_type != SampleType::F16 {

            debug_assert_eq!(channel.sample_type.bytes_per_sample(), 4);

            if remaining_le < byte_count {
                return Err(Error::invalid("not enough data"));
            }

            tmp.extend_from_slice(&compressed_le[in_i..(in_i + byte_count)]);

            in_i += byte_count;
            remaining_le -= byte_count;

            continue;
        }

        // HALF channel
        // The rest of the code assume we are manipulating u16 (2 bytes) values.
        debug_assert_eq!(channel.sample_type, SampleType::F16);
        debug_assert_eq!(channel.sample_type.bytes_per_sample(), size_of::<u16>());

        // Increase buffer to get new uncompressed datas.
        tmp.resize(tmp.len() + byte_count, 0);

        let x_sample_count = channel.resolution.x() * channel.samples_per_pixel;
        let y_sample_count = channel.resolution.y() * channel.samples_per_pixel;

        let bytes_per_sample = size_of::<u16>();

        let x_byte_count = x_sample_count * bytes_per_sample;
        let cd_start = channel.tmp_start_index;

        for y in (0..y_sample_count).step_by(BLOCK_SAMPLE_COUNT) {
            // Compute index in output (decompressed) buffer. We have 4 rows, because we will
            // uncompress 4 by 4 data blocks.
            let mut row0 = cd_start + y * x_byte_count;
            let mut row1 = row0 + x_byte_count;
            let mut row2 = row1 + x_byte_count;
            let mut row3 = row2 + x_byte_count;

            // Move in pixel x line, 4 by 4.
            for x in (0..x_sample_count).step_by(BLOCK_SAMPLE_COUNT) {

                // Extract the 4 by 4 block of 16-bit floats from the compressed buffer.
                let mut s = [0u16; 16];

                if remaining_le < 3 {
                    return Err(Error::invalid("not enough data"));
                }

                // If shift exponent is 63, call unpack14 (ignoring unused bits)
                if compressed_le[in_i + 2] >= (13 << 2) {
                    if remaining_le < 3 {
                        return Err(Error::invalid("not enough data"));
                    }

                    unpack3(&compressed_le[in_i..(in_i + 3)], &mut s);

                    in_i += 3;
                    remaining_le -= 3;
                } else {
                    if remaining_le < 14 {
                        return Err(Error::invalid("not enough data"));
                    }

                    unpack14(&compressed_le[in_i..(in_i + 14)], &mut s);

                    in_i += 14;
                    remaining_le -= 14;
                }

                if channel.quantize_linearly {
                    convert_to_linear(&mut s);
                }

                // Get resting samples from the line to copy in temp buffer (without going outside channel).
                let x_resting_sample_count = match x + 3 < x_sample_count {
                    true => BLOCK_SAMPLE_COUNT,
                    false => x_sample_count - x,
                };

                debug_assert!(x_resting_sample_count > 0);
                debug_assert!(x_resting_sample_count <= BLOCK_SAMPLE_COUNT);

                // Copy rows (without going outside channel).
                if y + 3 < y_sample_count {
                    cpy_u8(&s, 0, &mut tmp, row0, x_resting_sample_count);
                    cpy_u8(&s, 4, &mut tmp, row1, x_resting_sample_count);
                    cpy_u8(&s, 8, &mut tmp, row2, x_resting_sample_count);
                    cpy_u8(&s, 12, &mut tmp, row3, x_resting_sample_count);
                } else {
                    debug_assert!(y < y_sample_count);

                    cpy_u8(&s, 0, &mut tmp, row0, x_resting_sample_count);

                    if y + 1 < y_sample_count {
                        cpy_u8(&s, 4, &mut tmp, row1, x_resting_sample_count);
                    }

                    if y + 2 < y_sample_count {
                        cpy_u8(&s, 8, &mut tmp, row2, x_resting_sample_count);
                    }
                }

                // Update row's array index to 4 next pixels.
                row0 += BLOCK_X_BYTE_COUNT;
                row1 += BLOCK_X_BYTE_COUNT;
                row2 += BLOCK_X_BYTE_COUNT;
                row3 += BLOCK_X_BYTE_COUNT;
            }
        }
    }

    debug_assert_eq!(tmp.len(), expected_byte_size);

    // Interleave uncompressed channel data.
    let mut out = Vec::with_capacity(expected_byte_size);

    for y in rectangle.position.y()..rectangle.end().y() {
        for channel in &mut channel_data {
            if mod_p(y, usize_to_i32(channel.y_sampling, "sampling")?) != 0 {
                continue;
            }

            // Find data location in temporary buffer.
            let x_sample_count = channel.resolution.x() * channel.samples_per_pixel;
            let bytes_per_line = x_sample_count * channel.sample_type.bytes_per_sample();
            let next_tmp_end_index = channel.tmp_end_index + bytes_per_line;
            let channel_bytes = &tmp[channel.tmp_end_index..next_tmp_end_index];

            channel.tmp_end_index = next_tmp_end_index;

            // TODO do not convert endianness for f16-only images
            //      see https://github.com/AcademySoftwareFoundation/openexr/blob/3bd93f85bcb74c77255f28cdbb913fdbfbb39dfe/OpenEXR/IlmImf/ImfTiledOutputFile.cpp#L750-L842
            // We can support uncompressed data in the machine's native format
            // if all image channels are of type HALF, and if the Xdr and the
            // native representations of a half have the same size.

            if channel.sample_type == SampleType::F16 {
                // TODO simplify this and make it memcpy on little endian systems
                // https://github.com/AcademySoftwareFoundation/openexr/blob/a03aca31fa1ce85d3f28627dbb3e5ded9494724a/src/lib/OpenEXR/ImfB44Compressor.cpp#L943
                for mut f16_bytes in channel_bytes.chunks(std::mem::size_of::<f16>()) {
                    let native_endian_f16_bits = u16::read_from_little_endian(&mut f16_bytes).expect("memory read failed");
                    out.write_as_native_endian(&native_endian_f16_bits).expect("memory write failed");
                }
            }
            else {
                u8::write_slice_ne(&mut out, channel_bytes)
                    .expect("write to in-memory failed");
            }
        }
    }

    for index in 1..channel_data.len() {
        debug_assert_eq!(
            channel_data[index - 1].tmp_end_index,
            channel_data[index].tmp_start_index
        );
    }

    debug_assert_eq!(out.len(), expected_byte_size);

    // TODO do not convert endianness for f16-only images
    //      see https://github.com/AcademySoftwareFoundation/openexr/blob/3bd93f85bcb74c77255f28cdbb913fdbfbb39dfe/OpenEXR/IlmImf/ImfTiledOutputFile.cpp#L750-L842
    super::convert_little_endian_to_current(out, channels, rectangle)
}

pub fn compress(
    channels: &ChannelList,
    uncompressed_ne: ByteVec,
    rectangle: IntegerBounds,
    optimize_flat_fields: bool,
) -> Result<ByteVec> {
    if uncompressed_ne.is_empty() {
        return Ok(Vec::new());
    }

    // TODO do not convert endianness for f16-only images
    //      see https://github.com/AcademySoftwareFoundation/openexr/blob/3bd93f85bcb74c77255f28cdbb913fdbfbb39dfe/OpenEXR/IlmImf/ImfTiledOutputFile.cpp#L750-L842
    let uncompressed_le = super::convert_current_to_little_endian(uncompressed_ne, channels, rectangle)?;
    let uncompressed_le = uncompressed_le.as_slice(); // TODO no alloc

    let mut channel_data = Vec::new();

    let mut tmp_end_index = 0;
    for channel in &channels.list {
        let number_samples = channel.subsampled_resolution(rectangle.size);

        let sample_count = channel.subsampled_resolution(rectangle.size).area();
        let byte_count = sample_count * channel.sample_type.bytes_per_sample();

        let channel = ChannelData {
            tmp_start_index: tmp_end_index,
            tmp_end_index,
            y_sampling: channel.sampling.y(),
            resolution: number_samples,
            sample_type: channel.sample_type,
            quantize_linearly: channel.quantize_linearly,
            samples_per_pixel: channel.sampling.area(),
        };

        tmp_end_index += byte_count;
        channel_data.push(channel);
    }

    let mut tmp = vec![0_u8; uncompressed_le.len()];

    debug_assert_eq!(tmp_end_index, tmp.len());

    let mut remaining_uncompressed_bytes = uncompressed_le;

    for y in rectangle.position.y()..rectangle.end().y() {
        for channel in &mut channel_data {
            if mod_p(y, usize_to_i32(channel.y_sampling, "sampling")?) != 0 {
                continue;
            }

            let x_sample_count = channel.resolution.x() * channel.samples_per_pixel;
            let bytes_per_line = x_sample_count * channel.sample_type.bytes_per_sample();
            let next_tmp_end_index = channel.tmp_end_index + bytes_per_line;
            let target = &mut tmp[channel.tmp_end_index..next_tmp_end_index];

            channel.tmp_end_index = next_tmp_end_index;

            // TODO do not convert endianness for f16-only images
            //      see https://github.com/AcademySoftwareFoundation/openexr/blob/3bd93f85bcb74c77255f28cdbb913fdbfbb39dfe/OpenEXR/IlmImf/ImfTiledOutputFile.cpp#L750-L842
            // We can support uncompressed data in the machine's native format
            // if all image channels are of type HALF, and if the Xdr and the
            // native representations of a half have the same size.

            if channel.sample_type == SampleType::F16 {

                // TODO simplify this and make it memcpy on little endian systems
                // https://github.com/AcademySoftwareFoundation/openexr/blob/a03aca31fa1ce85d3f28627dbb3e5ded9494724a/src/lib/OpenEXR/ImfB44Compressor.cpp#L640

                for mut out_f16_bytes in target.chunks_mut(2) {
                    let native_endian_f16_bits = u16::read_from_native_endian(&mut remaining_uncompressed_bytes).expect("memory read failed");
                    out_f16_bytes.write_as_little_endian(&native_endian_f16_bits).expect("memory write failed");
                }
            }
            else {
                u8::read_slice_ne(&mut remaining_uncompressed_bytes, target)
                    .expect("in-memory read failed");
            }
        }
    }

    // Generate a whole buffer that we will crop to proper size once compression is done.
    let mut b44_compressed = vec![0; std::cmp::max(2048, uncompressed_le.len())];
    let mut b44_end = 0; // Buffer byte index for storing next compressed values.

    for channel in &channel_data {
        // U32 and F32 channels are raw copied.
        if channel.sample_type != SampleType::F16 {

            debug_assert_eq!(channel.sample_type.bytes_per_sample(), 4);

            // Raw byte copy.
            let slice = &tmp[channel.tmp_start_index..channel.tmp_end_index];
            slice.iter().copied().for_each(|b| {
                b44_compressed[b44_end] = b;
                b44_end += 1;
            });

            continue;
        }

        // HALF channel
        debug_assert_eq!(channel.sample_type, SampleType::F16);
        debug_assert_eq!(channel.sample_type.bytes_per_sample(), size_of::<u16>());

        let x_sample_count = channel.resolution.x() * channel.samples_per_pixel;
        let y_sample_count = channel.resolution.y() * channel.samples_per_pixel;

        let x_byte_count = x_sample_count * size_of::<u16>();
        let cd_start = channel.tmp_start_index;

        for y in (0..y_sample_count).step_by(BLOCK_SAMPLE_COUNT) {
            //
            // Copy the next 4x4 pixel block into array s.
            // If the width, cd.nx, or the height, cd.ny, of
            // the pixel data in _tmpBuffer is not divisible
            // by 4, then pad the data by repeating the
            // rightmost column and the bottom row.
            //

            // Compute row index in temp buffer.
            let mut row0 = cd_start + y * x_byte_count;
            let mut row1 = row0 + x_byte_count;
            let mut row2 = row1 + x_byte_count;
            let mut row3 = row2 + x_byte_count;

            if y + 3 >= y_sample_count {
                if y + 1 >= y_sample_count {
                    row1 = row0;
                }

                if y + 2 >= y_sample_count {
                    row2 = row1;
                }

                row3 = row2;
            }

            for x in (0..x_sample_count).step_by(BLOCK_SAMPLE_COUNT) {
                let mut s = [0u16; 16];

                if x + 3 >= x_sample_count {
                    let n = x_sample_count - x;

                    for i in 0..BLOCK_SAMPLE_COUNT {
                        let j = min(i, n - 1) * 2;

                        // TODO: Make [u8; 2] to u16 fast.
                        s[i + 0] = u16::from_ne_bytes([tmp[row0 + j], tmp[row0 + j + 1]]);
                        s[i + 4] = u16::from_ne_bytes([tmp[row1 + j], tmp[row1 + j + 1]]);
                        s[i + 8] = u16::from_ne_bytes([tmp[row2 + j], tmp[row2 + j + 1]]);
                        s[i + 12] = u16::from_ne_bytes([tmp[row3 + j], tmp[row3 + j + 1]]);
                    }
                } else {
                    memcpy_u8_to_u16(&tmp[row0..(row0 + BLOCK_X_BYTE_COUNT)], &mut s[0..4]);
                    memcpy_u8_to_u16(&tmp[row1..(row1 + BLOCK_X_BYTE_COUNT)], &mut s[4..8]);
                    memcpy_u8_to_u16(&tmp[row2..(row2 + BLOCK_X_BYTE_COUNT)], &mut s[8..12]);
                    memcpy_u8_to_u16(&tmp[row3..(row3 + BLOCK_X_BYTE_COUNT)], &mut s[12..16]);
                }

                // Move to next block.
                row0 += BLOCK_X_BYTE_COUNT;
                row1 += BLOCK_X_BYTE_COUNT;
                row2 += BLOCK_X_BYTE_COUNT;
                row3 += BLOCK_X_BYTE_COUNT;

                // Compress the contents of array `s` and append the results to the output buffer.
                if channel.quantize_linearly {
                    convert_from_linear(&mut s);
                }

                b44_end += pack(
                    s,
                    &mut b44_compressed[b44_end..(b44_end + 14)],
                    optimize_flat_fields,
                    !channel.quantize_linearly,
                );
            }
        }
    }

    b44_compressed.resize(b44_end, 0);

    Ok(b44_compressed)
}

#[cfg(test)]
mod test {
    use crate::compression::b44;
    use crate::compression::b44::{convert_from_linear, convert_to_linear};
    use crate::compression::ByteVec;
    use crate::image::validate_results::ValidateResult;
    use crate::meta::attribute::ChannelList;
    use crate::prelude::f16;
    use crate::prelude::*;

    #[test]
    fn test_convert_from_to_linear() {
        // Create two identical arrays with random floats.
        let mut s1 = [0u16; 16];

        for i in 0..16 {
            s1[i] = f16::from_f32(rand::random::<f32>()).to_bits();
        }

        let s2 = s1.clone();

        // Apply two reversible conversion.
        convert_from_linear(&mut s1);
        convert_to_linear(&mut s1);

        // And check.
        for (u1, u2) in s1.iter().zip(&s2) {
            let f1 = f16::from_bits(*u1).to_f64();
            let f2 = f16::from_bits(*u2).to_f64();
            assert!((f1 - f2).abs() < 0.01);
        }
    }

    fn test_roundtrip_noise_with(
        channels: ChannelList,
        rectangle: IntegerBounds,
    ) -> (ByteVec, ByteVec, ByteVec) {
        let byte_count = channels
            .list
            .iter()
            .map(|c| {
                c.subsampled_resolution(rectangle.size).area() * c.sample_type.bytes_per_sample()
            })
            .sum();

        assert!(byte_count > 0);

        let pixel_bytes: ByteVec = (0..byte_count).map(|_| rand::random()).collect();

        assert_eq!(pixel_bytes.len(), byte_count);

        let compressed = b44::compress(&channels, pixel_bytes.clone(), rectangle, true).unwrap();

        let decompressed =
            b44::decompress(&channels, compressed.clone(), rectangle, pixel_bytes.len(), true).unwrap();

        assert_eq!(decompressed.len(), pixel_bytes.len());

        (pixel_bytes, compressed, decompressed)
    }

    #[test]
    fn roundtrip_noise_f16() {
        let channel = ChannelDescription {
            sample_type: SampleType::F16,
            name: Default::default(),
            quantize_linearly: false,
            sampling: Vec2(1, 1),
        };

        // Two similar channels.
        let channels = ChannelList::new(smallvec![channel.clone(), channel]);

        let rectangle = IntegerBounds {
            position: Vec2(-30, 100),
            size: Vec2(322, 731),
        };

        let (pixel_bytes, compressed, decompressed) =
            test_roundtrip_noise_with(channels, rectangle);

        // On my tests, B44 give a size of 44.08% the original data (this assert implies enough
        // pixels to be relevant).
        assert_eq!(pixel_bytes.len(), 941528);
        assert_eq!(compressed.len(), 415044);
        assert_eq!(decompressed.len(), 941528);
    }

    #[test]
    fn roundtrip_noise_f16_tiny() {
        let channel = ChannelDescription {
            sample_type: SampleType::F16,
            name: Default::default(),
            quantize_linearly: false,
            sampling: Vec2(1, 1),
        };

        // Two similar channels.
        let channels = ChannelList::new(smallvec![channel.clone(), channel]);

        let rectangle = IntegerBounds {
            position: Vec2(0, 0),
            size: Vec2(3, 2),
        };

        let (pixel_bytes, compressed, decompressed) =
            test_roundtrip_noise_with(channels, rectangle);

        // B44 being 4 by 4 block, compression is less efficient for tiny images.
        assert_eq!(pixel_bytes.len(), 24);
        assert_eq!(compressed.len(), 28);
        assert_eq!(decompressed.len(), 24);
    }

    #[test]
    fn roundtrip_noise_f32() {
        let channel = ChannelDescription {
            sample_type: SampleType::F32,
            name: Default::default(),
            quantize_linearly: false,
            sampling: Vec2(1, 1),
        };

        // Two similar channels.
        let channels = ChannelList::new(smallvec![channel.clone(), channel]);

        let rectangle = IntegerBounds {
            position: Vec2(-30, 100),
            size: Vec2(322, 731),
        };

        let (pixel_bytes, compressed, decompressed) =
            test_roundtrip_noise_with(channels, rectangle);

        assert_eq!(pixel_bytes.len(), 1883056);
        assert_eq!(compressed.len(), 1883056);
        assert_eq!(decompressed.len(), 1883056);
        assert_eq!(pixel_bytes, decompressed);
    }

    #[test]
    fn roundtrip_noise_f32_tiny() {
        let channel = ChannelDescription {
            sample_type: SampleType::F32,
            name: Default::default(),
            quantize_linearly: false,
            sampling: Vec2(1, 1),
        };

        // Two similar channels.
        let channels = ChannelList::new(smallvec![channel.clone(), channel]);

        let rectangle = IntegerBounds {
            position: Vec2(0, 0),
            size: Vec2(3, 2),
        };

        let (pixel_bytes, compressed, decompressed) =
            test_roundtrip_noise_with(channels, rectangle);

        assert_eq!(pixel_bytes.len(), 48);
        assert_eq!(compressed.len(), 48);
        assert_eq!(decompressed.len(), 48);
        assert_eq!(pixel_bytes, decompressed);
    }

    #[test]
    fn roundtrip_noise_u32() {
        let channel = ChannelDescription {
            sample_type: SampleType::U32,
            name: Default::default(),
            quantize_linearly: false,
            sampling: Vec2(1, 1),
        };

        // Two similar channels.
        let channels = ChannelList::new(smallvec![channel.clone(), channel]);

        let rectangle = IntegerBounds {
            position: Vec2(-30, 100),
            size: Vec2(322, 731),
        };

        let (pixel_bytes, compressed, decompressed) =
            test_roundtrip_noise_with(channels, rectangle);

        assert_eq!(pixel_bytes.len(), 1883056);
        assert_eq!(compressed.len(), 1883056);
        assert_eq!(decompressed.len(), 1883056);
        assert_eq!(pixel_bytes, decompressed);
    }

    #[test]
    fn roundtrip_noise_u32_tiny() {
        let channel = ChannelDescription {
            sample_type: SampleType::U32,
            name: Default::default(),
            quantize_linearly: false,
            sampling: Vec2(1, 1),
        };

        // Two similar channels.
        let channels = ChannelList::new(smallvec![channel.clone(), channel]);

        let rectangle = IntegerBounds {
            position: Vec2(0, 0),
            size: Vec2(3, 2),
        };

        let (pixel_bytes, compressed, decompressed) =
            test_roundtrip_noise_with(channels, rectangle);

        assert_eq!(pixel_bytes.len(), 48);
        assert_eq!(compressed.len(), 48);
        assert_eq!(decompressed.len(), 48);
        assert_eq!(pixel_bytes, decompressed);
    }

    #[test]
    fn roundtrip_noise_mix_f32_f16_u32() {
        let channels = ChannelList::new(smallvec![
            ChannelDescription {
                sample_type: SampleType::F32,
                name: Default::default(),
                quantize_linearly: false,
                sampling: Vec2(1, 1),
            },
            ChannelDescription {
                sample_type: SampleType::F16,
                name: Default::default(),
                quantize_linearly: false,
                sampling: Vec2(1, 1),
            },
            ChannelDescription {
                sample_type: SampleType::U32,
                name: Default::default(),
                quantize_linearly: false,
                sampling: Vec2(1, 1),
            }
        ]);

        let rectangle = IntegerBounds {
            position: Vec2(-30, 100),
            size: Vec2(322, 731),
        };

        let (pixel_bytes, compressed, decompressed) =
            test_roundtrip_noise_with(channels, rectangle);

        assert_eq!(pixel_bytes.len(), 2353820);
        assert_eq!(compressed.len(), 2090578);
        assert_eq!(decompressed.len(), 2353820);
    }

    #[test]
    fn roundtrip_noise_mix_f32_f16_u32_tiny() {
        let channels = ChannelList::new(smallvec![
            ChannelDescription {
                sample_type: SampleType::F32,
                name: Default::default(),
                quantize_linearly: false,
                sampling: Vec2(1, 1),
            },
            ChannelDescription {
                sample_type: SampleType::F16,
                name: Default::default(),
                quantize_linearly: false,
                sampling: Vec2(1, 1),
            },
            ChannelDescription {
                sample_type: SampleType::U32,
                name: Default::default(),
                quantize_linearly: false,
                sampling: Vec2(1, 1),
            }
        ]);

        let rectangle = IntegerBounds {
            position: Vec2(0, 0),
            size: Vec2(3, 2),
        };

        let (pixel_bytes, compressed, decompressed) =
            test_roundtrip_noise_with(channels, rectangle);

        assert_eq!(pixel_bytes.len(), 60);
        assert_eq!(compressed.len(), 62);
        assert_eq!(decompressed.len(), 60);
    }

    #[test]
    fn border_on_multiview() {
        // This test is hard to reproduce, so we use the direct image.
        let path = "tests/images/valid/openexr/MultiView/Adjuster.exr";

        let read_image = read()
            .no_deep_data()
            .all_resolution_levels()
            .all_channels()
            .all_layers()
            .all_attributes()
            .non_parallel();

        let image = read_image.clone().from_file(path).unwrap();

        let mut tmp_bytes = Vec::new();
        image
            .write()
            .non_parallel()
            .to_buffered(std::io::Cursor::new(&mut tmp_bytes))
            .unwrap();

        let image2 = read_image
            .from_buffered(std::io::Cursor::new(tmp_bytes))
            .unwrap();

        image.assert_equals_result(&image2);
    }
}
