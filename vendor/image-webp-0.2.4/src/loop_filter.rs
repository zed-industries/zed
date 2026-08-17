//! Does loop filtering on webp lossy images

#[inline]
fn c(val: i32) -> i32 {
    val.clamp(-128, 127)
}

//unsigned to signed
#[inline]
fn u2s(val: u8) -> i32 {
    i32::from(val) - 128
}

//signed to unsigned
#[inline]
fn s2u(val: i32) -> u8 {
    (c(val) + 128) as u8
}

#[inline]
const fn diff(val1: u8, val2: u8) -> u8 {
    u8::abs_diff(val1, val2)
}

/// Used in both the simple and normal filters described in 15.2 and 15.3
///
/// Adjusts the 2 middle pixels in a vertical loop filter
fn common_adjust_vertical(
    use_outer_taps: bool,
    pixels: &mut [u8],
    point: usize,
    stride: usize,
) -> i32 {
    let p1 = u2s(pixels[point - 2 * stride]);
    let p0 = u2s(pixels[point - stride]);
    let q0 = u2s(pixels[point]);
    let q1 = u2s(pixels[point + stride]);

    //value for the outer 2 pixels
    let outer = if use_outer_taps { c(p1 - q1) } else { 0 };

    let a = c(outer + 3 * (q0 - p0));

    let b = (c(a + 3)) >> 3;

    let a = (c(a + 4)) >> 3;

    pixels[point] = s2u(q0 - a);
    pixels[point - stride] = s2u(p0 + b);

    a
}

/// Used in both the simple and normal filters described in 15.2 and 15.3
///
/// Adjusts the 2 middle pixels in a horizontal loop filter
fn common_adjust_horizontal(use_outer_taps: bool, pixels: &mut [u8]) -> i32 {
    let p1 = u2s(pixels[2]);
    let p0 = u2s(pixels[3]);
    let q0 = u2s(pixels[4]);
    let q1 = u2s(pixels[5]);

    //value for the outer 2 pixels
    let outer = if use_outer_taps { c(p1 - q1) } else { 0 };

    let a = c(outer + 3 * (q0 - p0));

    let b = (c(a + 3)) >> 3;

    let a = (c(a + 4)) >> 3;

    pixels[4] = s2u(q0 - a);
    pixels[3] = s2u(p0 + b);

    a
}

#[inline]
fn simple_threshold_vertical(
    filter_limit: i32,
    pixels: &[u8],
    point: usize,
    stride: usize,
) -> bool {
    i32::from(diff(pixels[point - stride], pixels[point])) * 2
        + i32::from(diff(pixels[point - 2 * stride], pixels[point + stride])) / 2
        <= filter_limit
}

#[inline]
fn simple_threshold_horizontal(filter_limit: i32, pixels: &[u8]) -> bool {
    assert!(pixels.len() >= 6); // one bounds check up front eliminates all subsequent checks in this function
    i32::from(diff(pixels[3], pixels[4])) * 2 + i32::from(diff(pixels[2], pixels[5])) / 2
        <= filter_limit
}

fn should_filter_vertical(
    interior_limit: u8,
    edge_limit: u8,
    pixels: &[u8],
    point: usize,
    stride: usize,
) -> bool {
    simple_threshold_vertical(i32::from(edge_limit), pixels, point, stride)
        // this looks like an erroneous way to compute differences between 8 points, but isn't:
        // there are actually only 6 diff comparisons required as per the spec:
        // https://www.rfc-editor.org/rfc/rfc6386#section-20.6
        && diff(pixels[point - 4 * stride], pixels[point - 3 * stride]) <= interior_limit
        && diff(pixels[point - 3 * stride], pixels[point - 2 * stride]) <= interior_limit
        && diff(pixels[point - 2 * stride], pixels[point - stride]) <= interior_limit
        && diff(pixels[point + 3 * stride], pixels[point + 2 * stride]) <= interior_limit
        && diff(pixels[point + 2 * stride], pixels[point + stride]) <= interior_limit
        && diff(pixels[point + stride], pixels[point]) <= interior_limit
}

fn should_filter_horizontal(interior_limit: u8, edge_limit: u8, pixels: &[u8]) -> bool {
    assert!(pixels.len() >= 8); // one bounds check up front eliminates all subsequent checks in this function
    simple_threshold_horizontal(i32::from(edge_limit), pixels)
        // this looks like an erroneous way to compute differences between 8 points, but isn't:
        // there are actually only 6 diff comparisons required as per the spec:
        // https://www.rfc-editor.org/rfc/rfc6386#section-20.6
        && diff(pixels[0], pixels[1]) <= interior_limit
        && diff(pixels[1], pixels[2]) <= interior_limit
        && diff(pixels[2], pixels[3]) <= interior_limit
        && diff(pixels[7], pixels[6]) <= interior_limit
        && diff(pixels[6], pixels[5]) <= interior_limit
        && diff(pixels[5], pixels[4]) <= interior_limit
}

#[inline]
fn high_edge_variance_vertical(threshold: u8, pixels: &[u8], point: usize, stride: usize) -> bool {
    diff(pixels[point - 2 * stride], pixels[point - stride]) > threshold
        || diff(pixels[point + stride], pixels[point]) > threshold
}

#[inline]
fn high_edge_variance_horizontal(threshold: u8, pixels: &[u8]) -> bool {
    diff(pixels[2], pixels[3]) > threshold || diff(pixels[5], pixels[4]) > threshold
}

/// Part of the simple filter described in 15.2 in the specification
///
/// Affects 4 pixels on an edge(2 each side)
pub(crate) fn simple_segment_vertical(
    edge_limit: u8,
    pixels: &mut [u8],
    point: usize,
    stride: usize,
) {
    if simple_threshold_vertical(i32::from(edge_limit), pixels, point, stride) {
        common_adjust_vertical(true, pixels, point, stride);
    }
}

/// Part of the simple filter described in 15.2 in the specification
///
/// Affects 4 pixels on an edge(2 each side)
pub(crate) fn simple_segment_horizontal(edge_limit: u8, pixels: &mut [u8]) {
    if simple_threshold_horizontal(i32::from(edge_limit), pixels) {
        common_adjust_horizontal(true, pixels);
    }
}

/// Part of the normal filter described in 15.3 in the specification
///
/// Filters on the 8 pixels on the edges between subblocks inside a macroblock
pub(crate) fn subblock_filter_vertical(
    hev_threshold: u8,
    interior_limit: u8,
    edge_limit: u8,
    pixels: &mut [u8],
    point: usize,
    stride: usize,
) {
    if should_filter_vertical(interior_limit, edge_limit, pixels, point, stride) {
        let hv = high_edge_variance_vertical(hev_threshold, pixels, point, stride);

        let a = (common_adjust_vertical(hv, pixels, point, stride) + 1) >> 1;

        if !hv {
            pixels[point + stride] = s2u(u2s(pixels[point + stride]) - a);
            pixels[point - 2 * stride] = s2u(u2s(pixels[point - 2 * stride]) + a);
        }
    }
}

/// Part of the normal filter described in 15.3 in the specification
///
/// Filters on the 8 pixels on the edges between subblocks inside a macroblock
pub(crate) fn subblock_filter_horizontal(
    hev_threshold: u8,
    interior_limit: u8,
    edge_limit: u8,
    pixels: &mut [u8],
) {
    if should_filter_horizontal(interior_limit, edge_limit, pixels) {
        let hv = high_edge_variance_horizontal(hev_threshold, pixels);

        let a = (common_adjust_horizontal(hv, pixels) + 1) >> 1;

        if !hv {
            pixels[5] = s2u(u2s(pixels[5]) - a);
            pixels[2] = s2u(u2s(pixels[2]) + a);
        }
    }
}

/// Part of the normal filter described in 15.3 in the specification
///
/// Filters on the 8 pixels on the vertical edges between macroblocks\
/// The point passed in must be the first vertical pixel on the bottom macroblock
pub(crate) fn macroblock_filter_vertical(
    hev_threshold: u8,
    interior_limit: u8,
    edge_limit: u8,
    pixels: &mut [u8],
    point: usize,
    stride: usize,
) {
    if should_filter_vertical(interior_limit, edge_limit, pixels, point, stride) {
        if !high_edge_variance_vertical(hev_threshold, pixels, point, stride) {
            // p0-3 are the pixels on the left macroblock from right to left
            let p2 = u2s(pixels[point - 3 * stride]);
            let p1 = u2s(pixels[point - 2 * stride]);
            let p0 = u2s(pixels[point - stride]);
            // q0-3 are the pixels on the right macroblock from left to right
            let q0 = u2s(pixels[point]);
            let q1 = u2s(pixels[point + stride]);
            let q2 = u2s(pixels[point + 2 * stride]);

            let w = c(c(p1 - q1) + 3 * (q0 - p0));

            let a = c((27 * w + 63) >> 7);

            pixels[point] = s2u(q0 - a);
            pixels[point - stride] = s2u(p0 + a);

            let a = c((18 * w + 63) >> 7);

            pixels[point + stride] = s2u(q1 - a);
            pixels[point - 2 * stride] = s2u(p1 + a);

            let a = c((9 * w + 63) >> 7);

            pixels[point + 2 * stride] = s2u(q2 - a);
            pixels[point - 3 * stride] = s2u(p2 + a);
        } else {
            common_adjust_vertical(true, pixels, point, stride);
        }
    }
}

/// Part of the normal filter described in 15.3 in the specification
///
/// Filters on the 8 pixels on the horizontal edges between macroblocks\
/// The pixels passed in must be a slice containing the 4 pixels on each macroblock
pub(crate) fn macroblock_filter_horizontal(
    hev_threshold: u8,
    interior_limit: u8,
    edge_limit: u8,
    pixels: &mut [u8],
) {
    assert!(pixels.len() >= 8);
    if should_filter_horizontal(interior_limit, edge_limit, pixels) {
        if !high_edge_variance_horizontal(hev_threshold, pixels) {
            // p0-3 are the pixels on the left macroblock from right to left
            let p2 = u2s(pixels[1]);
            let p1 = u2s(pixels[2]);
            let p0 = u2s(pixels[3]);
            // q0-3 are the pixels on the right macroblock from left to right
            let q0 = u2s(pixels[4]);
            let q1 = u2s(pixels[5]);
            let q2 = u2s(pixels[6]);

            let w = c(c(p1 - q1) + 3 * (q0 - p0));

            let a = c((27 * w + 63) >> 7);

            pixels[4] = s2u(q0 - a);
            pixels[3] = s2u(p0 + a);

            let a = c((18 * w + 63) >> 7);

            pixels[5] = s2u(q1 - a);
            pixels[2] = s2u(p1 + a);

            let a = c((9 * w + 63) >> 7);

            pixels[6] = s2u(q2 - a);
            pixels[1] = s2u(p2 + a);
        } else {
            common_adjust_horizontal(true, pixels);
        }
    }
}

#[cfg(all(test, feature = "_benchmarks"))]
mod benches {
    use super::*;
    use test::{black_box, Bencher};

    #[rustfmt::skip]
    const TEST_DATA: [u8; 8 * 8] = [
        177, 192, 179, 181, 185, 174, 186, 193,
        185, 180, 175, 179, 175, 190, 189, 190,
        185, 181, 177, 190, 190, 174, 176, 188,
        192, 179, 186, 175, 190, 184, 190, 175,
        175, 183, 183, 190, 187, 186, 176, 181,
        183, 177, 182, 185, 183, 179, 178, 181,
        191, 183, 188, 181, 180, 193, 185, 180,
        177, 182, 177, 178, 179, 178, 191, 178,
    ];

    #[bench]
    fn measure_horizontal_macroblock_filter(b: &mut Bencher) {
        let hev_threshold = 5;
        let interior_limit = 15;
        let edge_limit = 15;

        let mut data = TEST_DATA.clone();
        let stride = 8;

        b.iter(|| {
            for y in 0..8 {
                black_box(macroblock_filter_horizontal(
                    hev_threshold,
                    interior_limit,
                    edge_limit,
                    &mut data[y * stride..][..8],
                ));
            }
        });
    }

    #[bench]
    fn measure_vertical_macroblock_filter(b: &mut Bencher) {
        let hev_threshold = 5;
        let interior_limit = 15;
        let edge_limit = 15;

        let mut data = TEST_DATA.clone();
        let stride = 8;

        b.iter(|| {
            for x in 0..8 {
                black_box(macroblock_filter_vertical(
                    hev_threshold,
                    interior_limit,
                    edge_limit,
                    &mut data,
                    4 * stride + x,
                    stride,
                ));
            }
        });
    }

    #[bench]
    fn measure_horizontal_subblock_filter(b: &mut Bencher) {
        let hev_threshold = 5;
        let interior_limit = 15;
        let edge_limit = 15;

        let mut data = TEST_DATA.clone();
        let stride = 8;

        b.iter(|| {
            for y in 0usize..8 {
                black_box(subblock_filter_horizontal(
                    hev_threshold,
                    interior_limit,
                    edge_limit,
                    &mut data[y * stride..][..8],
                ))
            }
        });
    }

    #[bench]
    fn measure_vertical_subblock_filter(b: &mut Bencher) {
        let hev_threshold = 5;
        let interior_limit = 15;
        let edge_limit = 15;

        let mut data = TEST_DATA.clone();
        let stride = 8;

        b.iter(|| {
            for x in 0..8 {
                black_box(subblock_filter_vertical(
                    hev_threshold,
                    interior_limit,
                    edge_limit,
                    &mut data,
                    4 * stride + x,
                    stride,
                ))
            }
        });
    }

    #[bench]
    fn measure_simple_segment_horizontal_filter(b: &mut Bencher) {
        let edge_limit = 15;

        let mut data = TEST_DATA.clone();
        let stride = 8;

        b.iter(|| {
            for y in 0usize..8 {
                black_box(simple_segment_horizontal(
                    edge_limit,
                    &mut data[y * stride..][..8],
                ))
            }
        });
    }

    #[bench]
    fn measure_simple_segment_vertical_filter(b: &mut Bencher) {
        let edge_limit = 15;

        let mut data = TEST_DATA.clone();
        let stride = 8;

        b.iter(|| {
            for x in 0usize..16 {
                black_box(simple_segment_vertical(
                    edge_limit,
                    &mut data,
                    4 * stride + x,
                    stride,
                ))
            }
        });
    }
}
