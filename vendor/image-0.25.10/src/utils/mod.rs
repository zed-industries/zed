//!  Utilities

use std::collections::TryReserveError;
use std::iter::repeat;

#[inline(always)]
pub(crate) fn expand_packed<F>(buf: &mut [u8], channels: usize, bit_depth: u8, mut func: F)
where
    F: FnMut(u8, &mut [u8]),
{
    let pixels = buf.len() / channels * bit_depth as usize;
    let extra = pixels % 8;
    let entries = pixels / 8
        + match extra {
            0 => 0,
            _ => 1,
        };
    let mask = ((1u16 << bit_depth) - 1) as u8;
    let i = (0..entries)
        .rev() // Reverse iterator
        .flat_map(|idx|
            // This has to be reversed to
            (0..8/bit_depth).map(|i| i*bit_depth).zip(repeat(idx)))
        .skip(extra);
    let buf_len = buf.len();
    let j_inv = (channels..buf_len).step_by(channels);
    for ((shift, i), j_inv) in i.zip(j_inv) {
        let j = buf_len - j_inv;
        let pixel = (buf[i] & (mask << shift)) >> shift;
        func(pixel, &mut buf[j..(j + channels)]);
    }
}

/// Expand a buffer of packed 1, 2, or 4 bits integers into u8's. Assumes that
/// every `row_size` entries there are padding bits up to the next byte boundary.
#[allow(dead_code)]
// When no image formats that use it are enabled
pub(crate) fn expand_bits(bit_depth: u8, row_size: u32, buf: &[u8]) -> Vec<u8> {
    // Note: this conversion assumes that the scanlines begin on byte boundaries
    let mask = (1u8 << bit_depth as usize) - 1;
    let scaling_factor = 255 / ((1 << bit_depth as usize) - 1);
    let bit_width = row_size * u32::from(bit_depth);
    let skip = if bit_width.is_multiple_of(8) {
        0
    } else {
        (8 - bit_width % 8) / u32::from(bit_depth)
    };
    let row_len = row_size + skip;
    let mut p = Vec::new();
    let mut i = 0;
    for v in buf {
        for shift_inv in 1..=8 / bit_depth {
            let shift = 8 - bit_depth * shift_inv;
            // skip the pixels that can be neglected because scanlines should
            // start at byte boundaries
            if i % (row_len as usize) < (row_size as usize) {
                let pixel = (v & (mask << shift as usize)) >> shift as usize;
                p.push(pixel * scaling_factor);
            }
            i += 1;
        }
    }
    p
}

#[inline(always)]
pub(crate) fn interleave_planes(out: &mut [u8], color: crate::ColorType, planes: &[&[u8]]) {
    #[track_caller]
    pub(crate) fn trampoline<const PLANES: usize, const N: usize>(
        out: &mut [u8],
        planes: &[&[u8]],
    ) {
        interleave_planes_inner::<PLANES, N>(
            out.as_chunks_mut::<N>().0,
            <[_; PLANES]>::try_from(planes)
                .unwrap()
                .map(|arr| arr.as_chunks::<N>().0),
        )
    }

    assert_eq!(planes.len(), usize::from(color.channel_count()));

    match color {
        crate::ColorType::L8 => trampoline::<1, 1>(out, planes),
        crate::ColorType::La8 => trampoline::<2, 1>(out, planes),
        crate::ColorType::Rgb8 => trampoline::<3, 1>(out, planes),
        crate::ColorType::Rgba8 => trampoline::<4, 1>(out, planes),
        crate::ColorType::L16 => trampoline::<1, 2>(out, planes),
        crate::ColorType::La16 => trampoline::<2, 2>(out, planes),
        crate::ColorType::Rgb16 => trampoline::<3, 2>(out, planes),
        crate::ColorType::Rgba16 => trampoline::<4, 2>(out, planes),
        crate::ColorType::Rgb32F => trampoline::<3, 4>(out, planes),
        crate::ColorType::Rgba32F => trampoline::<4, 4>(out, planes),
    }
}

#[inline(always)]
fn interleave_planes_inner<const PLANES: usize, const N: usize>(
    out: &mut [[u8; N]],
    planes: [&[[u8; N]]; PLANES],
) {
    let mut iters = planes.map(|plane| plane.iter().copied());
    for out in out.as_chunks_mut::<PLANES>().0 {
        let vals = iters.each_mut().map(Iterator::next);

        // I'd like to use array::zip once stable.
        for i in 0..PLANES {
            out[i] = vals[i].unwrap_or(out[i]);
        }
    }
}

/// Checks if the provided dimensions would cause an overflow.
#[allow(dead_code)]
// When no image formats that use it are enabled
pub(crate) fn check_dimension_overflow(width: u32, height: u32, bytes_per_pixel: u8) -> bool {
    u64::from(width) * u64::from(height) > u64::MAX / u64::from(bytes_per_pixel)
}

#[allow(dead_code)]
// When no image formats that use it are enabled
pub(crate) fn vec_copy_to_u8<T>(vec: &[T]) -> Vec<u8>
where
    T: bytemuck::Pod,
{
    bytemuck::cast_slice(vec).to_owned()
}

#[inline]
pub(crate) fn clamp<N>(a: N, min: N, max: N) -> N
where
    N: PartialOrd,
{
    if a < min {
        min
    } else if a > max {
        max
    } else {
        a
    }
}

#[inline]
pub(crate) fn vec_try_with_capacity<T>(capacity: usize) -> Result<Vec<T>, TryReserveError> {
    let mut vec = Vec::new();
    vec.try_reserve_exact(capacity)?;
    Ok(vec)
}

#[cfg(test)]
mod test {
    #[test]
    fn gray_to_luma8_skip() {
        let check = |bit_depth, w, from, to| {
            assert_eq!(super::expand_bits(bit_depth, w, from), to);
        };
        // Bit depth 1, skip is more than half a byte
        check(
            1,
            10,
            &[0b11110000, 0b11000000, 0b00001111, 0b11000000],
            vec![
                255, 255, 255, 255, 0, 0, 0, 0, 255, 255, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
            ],
        );
        // Bit depth 2, skip is more than half a byte
        check(
            2,
            5,
            &[0b11110000, 0b11000000, 0b00001111, 0b11000000],
            vec![255, 255, 0, 0, 255, 0, 0, 255, 255, 255],
        );
        // Bit depth 2, skip is 0
        check(
            2,
            4,
            &[0b11110000, 0b00001111],
            vec![255, 255, 0, 0, 0, 0, 255, 255],
        );
        // Bit depth 4, skip is half a byte
        check(4, 1, &[0b11110011, 0b00001100], vec![255, 0]);
    }
}
