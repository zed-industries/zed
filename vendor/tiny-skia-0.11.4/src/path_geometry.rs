// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use tiny_skia_path::{NormalizedF32, NormalizedF32Exclusive, Point};

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use tiny_skia_path::NoStdFloat;

pub use tiny_skia_path::path_geometry::{
    chop_cubic_at2, chop_quad_at, find_cubic_max_curvature, find_unit_quad_roots, new_t_values,
    CubicCoeff, QuadCoeff,
};

use tiny_skia_path::path_geometry::valid_unit_divide;

// TODO: return custom type
/// Returns 0 for 1 quad, and 1 for two quads, either way the answer is stored in dst[].
///
/// Guarantees that the 1/2 quads will be monotonic.
pub fn chop_quad_at_x_extrema(src: &[Point; 3], dst: &mut [Point; 5]) -> usize {
    let a = src[0].x;
    let mut b = src[1].x;
    let c = src[2].x;

    if is_not_monotonic(a, b, c) {
        if let Some(t_value) = valid_unit_divide(a - b, a - b - b + c) {
            chop_quad_at(src, t_value, dst);

            // flatten double quad extrema
            dst[1].x = dst[2].x;
            dst[3].x = dst[2].x;

            return 1;
        }

        // if we get here, we need to force dst to be monotonic, even though
        // we couldn't compute a unit_divide value (probably underflow).
        b = if (a - b).abs() < (b - c).abs() { a } else { c };
    }

    dst[0] = Point::from_xy(a, src[0].y);
    dst[1] = Point::from_xy(b, src[1].y);
    dst[2] = Point::from_xy(c, src[2].y);
    0
}

/// Returns 0 for 1 quad, and 1 for two quads, either way the answer is stored in dst[].
///
/// Guarantees that the 1/2 quads will be monotonic.
pub fn chop_quad_at_y_extrema(src: &[Point; 3], dst: &mut [Point; 5]) -> usize {
    let a = src[0].y;
    let mut b = src[1].y;
    let c = src[2].y;

    if is_not_monotonic(a, b, c) {
        if let Some(t_value) = valid_unit_divide(a - b, a - b - b + c) {
            chop_quad_at(src, t_value, dst);

            // flatten double quad extrema
            dst[1].y = dst[2].y;
            dst[3].y = dst[2].y;

            return 1;
        }

        // if we get here, we need to force dst to be monotonic, even though
        // we couldn't compute a unit_divide value (probably underflow).
        b = if (a - b).abs() < (b - c).abs() { a } else { c };
    }

    dst[0] = Point::from_xy(src[0].x, a);
    dst[1] = Point::from_xy(src[1].x, b);
    dst[2] = Point::from_xy(src[2].x, c);
    0
}

fn is_not_monotonic(a: f32, b: f32, c: f32) -> bool {
    let ab = a - b;
    let mut bc = b - c;
    if ab < 0.0 {
        bc = -bc;
    }

    ab == 0.0 || bc < 0.0
}

pub fn chop_cubic_at_x_extrema(src: &[Point; 4], dst: &mut [Point; 10]) -> usize {
    let mut t_values = new_t_values();
    let t_values = find_cubic_extrema(src[0].x, src[1].x, src[2].x, src[3].x, &mut t_values);

    chop_cubic_at(src, t_values, dst);
    if !t_values.is_empty() {
        // we do some cleanup to ensure our X extrema are flat
        dst[2].x = dst[3].x;
        dst[4].x = dst[3].x;
        if t_values.len() == 2 {
            dst[5].x = dst[6].x;
            dst[7].x = dst[6].x;
        }
    }

    t_values.len()
}

/// Given 4 points on a cubic bezier, chop it into 1, 2, 3 beziers such that
/// the resulting beziers are monotonic in Y.
///
/// This is called by the scan converter.
///
/// Depending on what is returned, dst[] is treated as follows:
///
/// - 0: dst[0..3] is the original cubic
/// - 1: dst[0..3] and dst[3..6] are the two new cubics
/// - 2: dst[0..3], dst[3..6], dst[6..9] are the three new cubics
pub fn chop_cubic_at_y_extrema(src: &[Point; 4], dst: &mut [Point; 10]) -> usize {
    let mut t_values = new_t_values();
    let t_values = find_cubic_extrema(src[0].y, src[1].y, src[2].y, src[3].y, &mut t_values);

    chop_cubic_at(src, t_values, dst);
    if !t_values.is_empty() {
        // we do some cleanup to ensure our Y extrema are flat
        dst[2].y = dst[3].y;
        dst[4].y = dst[3].y;
        if t_values.len() == 2 {
            dst[5].y = dst[6].y;
            dst[7].y = dst[6].y;
        }
    }

    t_values.len()
}

// Cubic'(t) = At^2 + Bt + C, where
// A = 3(-a + 3(b - c) + d)
// B = 6(a - 2b + c)
// C = 3(b - a)
// Solve for t, keeping only those that fit between 0 < t < 1
fn find_cubic_extrema(
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    t_values: &mut [NormalizedF32Exclusive; 3],
) -> &[NormalizedF32Exclusive] {
    // we divide A,B,C by 3 to simplify
    let na = d - a + 3.0 * (b - c);
    let nb = 2.0 * (a - b - b + c);
    let nc = b - a;

    let roots = find_unit_quad_roots(na, nb, nc, t_values);
    &t_values[0..roots]
}

// http://code.google.com/p/skia/issues/detail?id=32
//
// This test code would fail when we didn't check the return result of
// valid_unit_divide in SkChopCubicAt(... NormalizedF32Exclusives[], int roots). The reason is
// that after the first chop, the parameters to valid_unit_divide are equal
// (thanks to finite float precision and rounding in the subtracts). Thus
// even though the 2nd NormalizedF32Exclusive looks < 1.0, after we renormalize it, we end
// up with 1.0, hence the need to check and just return the last cubic as
// a degenerate clump of 4 points in the same place.
pub fn chop_cubic_at(src: &[Point; 4], t_values: &[NormalizedF32Exclusive], dst: &mut [Point]) {
    if t_values.is_empty() {
        // nothing to chop
        dst[0] = src[0];
        dst[1] = src[1];
        dst[2] = src[2];
        dst[3] = src[3];
    } else {
        let mut t = t_values[0];
        let mut tmp = [Point::zero(); 4];

        // Reduce the `src` lifetime, so we can use `src = &tmp` later.
        let mut src = src;

        let mut dst_offset = 0;
        for i in 0..t_values.len() {
            chop_cubic_at2(src, t, &mut dst[dst_offset..]);
            if i == t_values.len() - 1 {
                break;
            }

            dst_offset += 3;
            // have src point to the remaining cubic (after the chop)
            tmp[0] = dst[dst_offset + 0];
            tmp[1] = dst[dst_offset + 1];
            tmp[2] = dst[dst_offset + 2];
            tmp[3] = dst[dst_offset + 3];
            src = &tmp;

            // watch out in case the renormalized t isn't in range
            let n = valid_unit_divide(
                t_values[i + 1].get() - t_values[i].get(),
                1.0 - t_values[i].get(),
            );

            match n {
                Some(n) => t = n,
                None => {
                    // if we can't, just create a degenerate cubic
                    dst[dst_offset + 4] = src[3];
                    dst[dst_offset + 5] = src[3];
                    dst[dst_offset + 6] = src[3];
                    break;
                }
            }
        }
    }
}

pub fn chop_cubic_at_max_curvature(
    src: &[Point; 4],
    t_values: &mut [NormalizedF32Exclusive; 3],
    dst: &mut [Point],
) -> usize {
    let mut roots = [NormalizedF32::ZERO; 3];
    let roots = find_cubic_max_curvature(src, &mut roots);

    // Throw out values not inside 0..1.
    let mut count = 0;
    for root in roots {
        if 0.0 < root.get() && root.get() < 1.0 {
            t_values[count] = NormalizedF32Exclusive::new_bounded(root.get());
            count += 1;
        }
    }

    if count == 0 {
        dst[0..4].copy_from_slice(src);
    } else {
        chop_cubic_at(src, &t_values[0..count], dst);
    }

    count + 1
}

pub fn chop_mono_cubic_at_x(src: &[Point; 4], x: f32, dst: &mut [Point; 7]) -> bool {
    cubic_dchop_at_intercept(src, x, true, dst)
}

pub fn chop_mono_cubic_at_y(src: &[Point; 4], y: f32, dst: &mut [Point; 7]) -> bool {
    cubic_dchop_at_intercept(src, y, false, dst)
}

fn cubic_dchop_at_intercept(
    src: &[Point; 4],
    intercept: f32,
    is_vertical: bool,
    dst: &mut [Point; 7],
) -> bool {
    use crate::path64::{cubic64::Cubic64, line_cubic_intersections, point64::Point64};

    let src = [
        Point64::from_point(src[0]),
        Point64::from_point(src[1]),
        Point64::from_point(src[2]),
        Point64::from_point(src[3]),
    ];

    let cubic = Cubic64::new(src);
    let mut roots = [0.0; 3];
    let count = if is_vertical {
        line_cubic_intersections::vertical_intersect(&cubic, f64::from(intercept), &mut roots)
    } else {
        line_cubic_intersections::horizontal_intersect(&cubic, f64::from(intercept), &mut roots)
    };

    if count > 0 {
        let pair = cubic.chop_at(roots[0]);
        for i in 0..7 {
            dst[i] = pair.points[i].to_point();
        }

        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chop_cubic_at_y_extrema_1() {
        let src = [
            Point::from_xy(10.0, 20.0),
            Point::from_xy(67.0, 437.0),
            Point::from_xy(298.0, 213.0),
            Point::from_xy(401.0, 214.0),
        ];

        let mut dst = [Point::zero(); 10];
        let n = chop_cubic_at_y_extrema(&src, &mut dst);
        assert_eq!(n, 2);
        assert_eq!(dst[0], Point::from_xy(10.0, 20.0));
        assert_eq!(dst[1], Point::from_xy(37.508274, 221.24475));
        assert_eq!(dst[2], Point::from_xy(105.541855, 273.19803));
        assert_eq!(dst[3], Point::from_xy(180.15599, 273.19803));
        assert_eq!(dst[4], Point::from_xy(259.80502, 273.19803));
        assert_eq!(dst[5], Point::from_xy(346.9527, 213.99666));
        assert_eq!(dst[6], Point::from_xy(400.30844, 213.99666));
        assert_eq!(dst[7], Point::from_xy(400.53958, 213.99666));
        assert_eq!(dst[8], Point::from_xy(400.7701, 213.99777));
        assert_eq!(dst[9], Point::from_xy(401.0, 214.0));
    }
}
