// Copyright 2014 Google Inc.
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This module is a mix of SkDashPath, SkDashPathEffect, SkContourMeasure and SkPathMeasure.

use alloc::vec::Vec;

use arrayref::array_ref;

use crate::{Path, Point};

use crate::floating_point::{FiniteF32, NonZeroPositiveF32, NormalizedF32, NormalizedF32Exclusive};
use crate::path::{PathSegment, PathSegmentsIter, PathVerb};
use crate::path_builder::PathBuilder;
use crate::path_geometry;
use crate::scalar::Scalar;

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use crate::NoStdFloat;

/// A stroke dashing properties.
///
/// Contains an array of pairs, where the first number indicates an "on" interval
/// and the second one indicates an "off" interval;
/// a dash offset value and internal properties.
///
/// # Guarantees
///
/// - The dash array always have an even number of values.
/// - All dash array values are finite and >= 0.
/// - There is at least two dash array values.
/// - The sum of all dash array values is positive and finite.
/// - Dash offset is finite.
#[derive(Clone, PartialEq, Debug)]
pub struct StrokeDash {
    array: Vec<f32>,
    offset: f32,
    interval_len: NonZeroPositiveF32,
    first_len: f32, // TODO: PositiveF32
    first_index: usize,
}

impl StrokeDash {
    /// Creates a new stroke dashing object.
    pub fn new(dash_array: Vec<f32>, dash_offset: f32) -> Option<Self> {
        let dash_offset = FiniteF32::new(dash_offset)?;

        if dash_array.len() < 2 || dash_array.len() % 2 != 0 {
            return None;
        }

        if dash_array.iter().any(|n| *n < 0.0) {
            return None;
        }

        let interval_len: f32 = dash_array.iter().sum();
        let interval_len = NonZeroPositiveF32::new(interval_len)?;

        let dash_offset = adjust_dash_offset(dash_offset.get(), interval_len.get());
        debug_assert!(dash_offset >= 0.0);
        debug_assert!(dash_offset < interval_len.get());

        let (first_len, first_index) = find_first_interval(&dash_array, dash_offset);
        debug_assert!(first_len >= 0.0);
        debug_assert!(first_index < dash_array.len());

        Some(StrokeDash {
            array: dash_array,
            offset: dash_offset,
            interval_len,
            first_len,
            first_index,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test() {
        assert_eq!(StrokeDash::new(vec![], 0.0), None);
        assert_eq!(StrokeDash::new(vec![1.0], 0.0), None);
        assert_eq!(StrokeDash::new(vec![1.0, 2.0, 3.0], 0.0), None);
        assert_eq!(StrokeDash::new(vec![1.0, -2.0], 0.0), None);
        assert_eq!(StrokeDash::new(vec![0.0, 0.0], 0.0), None);
        assert_eq!(StrokeDash::new(vec![1.0, -1.0], 0.0), None);
        assert_eq!(StrokeDash::new(vec![1.0, 1.0], f32::INFINITY), None);
        assert_eq!(StrokeDash::new(vec![1.0, f32::INFINITY], 0.0), None);
    }

    #[test]
    fn bug_26() {
        let mut pb = PathBuilder::new();
        pb.move_to(665.54, 287.3);
        pb.line_to(675.67, 273.04);
        pb.line_to(675.52, 271.32);
        pb.line_to(674.79, 269.61);
        pb.line_to(674.05, 268.04);
        pb.line_to(672.88, 266.47);
        pb.line_to(671.27, 264.9);
        let path = pb.finish().unwrap();

        let stroke_dash = StrokeDash::new(vec![6.0, 4.5], 0.0).unwrap();

        assert!(path.dash(&stroke_dash, 1.0).is_some());
    }
}

// Adjust phase to be between 0 and len, "flipping" phase if negative.
// e.g., if len is 100, then phase of -20 (or -120) is equivalent to 80.
fn adjust_dash_offset(mut offset: f32, len: f32) -> f32 {
    if offset < 0.0 {
        offset = -offset;
        if offset > len {
            offset %= len;
        }

        offset = len - offset;

        // Due to finite precision, it's possible that phase == len,
        // even after the subtract (if len >>> phase), so fix that here.
        debug_assert!(offset <= len);
        if offset == len {
            offset = 0.0;
        }

        offset
    } else if offset >= len {
        offset % len
    } else {
        offset
    }
}

fn find_first_interval(dash_array: &[f32], mut dash_offset: f32) -> (f32, usize) {
    for (i, gap) in dash_array.iter().copied().enumerate() {
        if dash_offset > gap || (dash_offset == gap && gap != 0.0) {
            dash_offset -= gap;
        } else {
            return (gap - dash_offset, i);
        }
    }

    // If we get here, phase "appears" to be larger than our length. This
    // shouldn't happen with perfect precision, but we can accumulate errors
    // during the initial length computation (rounding can make our sum be too
    // big or too small. In that event, we just have to eat the error here.
    (dash_array[0], 0)
}

impl Path {
    /// Converts the current path into a dashed one.
    ///
    /// `resolution_scale` can be obtained via
    /// [`compute_resolution_scale`](crate::PathStroker::compute_resolution_scale).
    ///
    /// Returns `None` when more than 1_000_000 dashes had to be produced
    /// or when the final path has an invalid bounding box.
    pub fn dash(&self, dash: &StrokeDash, resolution_scale: f32) -> Option<Path> {
        dash_impl(self, dash, resolution_scale)
    }
}

fn dash_impl(src: &Path, dash: &StrokeDash, res_scale: f32) -> Option<Path> {
    // We do not support the `cull_path` branch here.
    // Skia has a lot of code for cases when a path contains only a single zero-length line
    // or when a path is a rect. Not sure why.
    // We simply ignoring it for the sake of simplicity.

    // We also doesn't support the `SpecialLineRec` case.
    // I have no idea what the point in it.

    fn is_even(x: usize) -> bool {
        x % 2 == 0
    }

    let mut pb = PathBuilder::new();
    let mut dash_count = 0.0;
    for contour in ContourMeasureIter::new(src, res_scale) {
        let mut skip_first_segment = contour.is_closed;
        let mut added_segment = false;
        let length = contour.length;
        let mut index = dash.first_index;

        // Since the path length / dash length ratio may be arbitrarily large, we can exert
        // significant memory pressure while attempting to build the filtered path. To avoid this,
        // we simply give up dashing beyond a certain threshold.
        //
        // The original bug report (http://crbug.com/165432) is based on a path yielding more than
        // 90 million dash segments and crashing the memory allocator. A limit of 1 million
        // segments seems reasonable: at 2 verbs per segment * 9 bytes per verb, this caps the
        // maximum dash memory overhead at roughly 17MB per path.
        const MAX_DASH_COUNT: usize = 1000000;
        dash_count += length * (dash.array.len() >> 1) as f32 / dash.interval_len.get();
        if dash_count > MAX_DASH_COUNT as f32 {
            return None;
        }

        // Using double precision to avoid looping indefinitely due to single precision rounding
        // (for extreme path_length/dash_length ratios). See test_infinite_dash() unittest.
        let mut distance = 0.0;
        let mut d_len = dash.first_len;

        while distance < length {
            debug_assert!(d_len >= 0.0);
            added_segment = false;
            if is_even(index) && !skip_first_segment {
                added_segment = true;
                contour.push_segment(distance, distance + d_len, true, &mut pb);
            }

            distance += d_len;

            // clear this so we only respect it the first time around
            skip_first_segment = false;

            // wrap around our intervals array if necessary
            index += 1;
            debug_assert!(index <= dash.array.len());
            if index == dash.array.len() {
                index = 0;
            }

            // fetch our next d_len
            d_len = dash.array[index];
        }

        // extend if we ended on a segment and we need to join up with the (skipped) initial segment
        if contour.is_closed && is_even(dash.first_index) && dash.first_len >= 0.0 {
            contour.push_segment(0.0, dash.first_len, !added_segment, &mut pb);
        }
    }

    pb.finish()
}

const MAX_T_VALUE: u32 = 0x3FFFFFFF;

struct ContourMeasureIter<'a> {
    iter: PathSegmentsIter<'a>,
    tolerance: f32,
}

impl<'a> ContourMeasureIter<'a> {
    fn new(path: &'a Path, res_scale: f32) -> Self {
        // can't use tangents, since we need [0..1..................2] to be seen
        // as definitely not a line (it is when drawn, but not parametrically)
        // so we compare midpoints
        const CHEAP_DIST_LIMIT: f32 = 0.5; // just made this value up

        ContourMeasureIter {
            iter: path.segments(),
            tolerance: CHEAP_DIST_LIMIT * res_scale.invert(),
        }
    }
}

impl Iterator for ContourMeasureIter<'_> {
    type Item = ContourMeasure;

    // If it encounters a zero-length contour, it is skipped.
    fn next(&mut self) -> Option<Self::Item> {
        // Note:
        // as we accumulate distance, we have to check that the result of +=
        // actually made it larger, since a very small delta might be > 0, but
        // still have no effect on distance (if distance >>> delta).
        //
        // We do this check below, and in compute_quad_segs and compute_cubic_segs

        let mut contour = ContourMeasure::default();

        let mut point_index = 0;
        let mut distance = 0.0;
        let mut have_seen_close = false;
        let mut prev_p = Point::zero();
        while let Some(seg) = self.iter.next() {
            match seg {
                PathSegment::MoveTo(p0) => {
                    contour.points.push(p0);
                    prev_p = p0;
                }
                PathSegment::LineTo(p0) => {
                    let prev_d = distance;
                    distance = contour.compute_line_seg(prev_p, p0, distance, point_index);

                    if distance > prev_d {
                        contour.points.push(p0);
                        point_index += 1;
                    }

                    prev_p = p0;
                }
                PathSegment::QuadTo(p0, p1) => {
                    let prev_d = distance;
                    distance = contour.compute_quad_segs(
                        prev_p,
                        p0,
                        p1,
                        distance,
                        0,
                        MAX_T_VALUE,
                        point_index,
                        self.tolerance,
                    );

                    if distance > prev_d {
                        contour.points.push(p0);
                        contour.points.push(p1);
                        point_index += 2;
                    }

                    prev_p = p1;
                }
                PathSegment::CubicTo(p0, p1, p2) => {
                    let prev_d = distance;
                    distance = contour.compute_cubic_segs(
                        prev_p,
                        p0,
                        p1,
                        p2,
                        distance,
                        0,
                        MAX_T_VALUE,
                        point_index,
                        self.tolerance,
                    );

                    if distance > prev_d {
                        contour.points.push(p0);
                        contour.points.push(p1);
                        contour.points.push(p2);
                        point_index += 3;
                    }

                    prev_p = p2;
                }
                PathSegment::Close => {
                    have_seen_close = true;
                }
            }

            // TODO: to contour iter?
            if self.iter.next_verb() == Some(PathVerb::Move) {
                break;
            }
        }

        if !distance.is_finite() {
            return None;
        }

        if have_seen_close {
            let prev_d = distance;
            let first_pt = contour.points[0];
            distance = contour.compute_line_seg(
                contour.points[point_index],
                first_pt,
                distance,
                point_index,
            );

            if distance > prev_d {
                contour.points.push(first_pt);
            }
        }

        contour.length = distance;
        contour.is_closed = have_seen_close;

        if contour.points.is_empty() {
            None
        } else {
            Some(contour)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum SegmentType {
    Line,
    Quad,
    Cubic,
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    distance: f32,      // total distance up to this point
    point_index: usize, // index into the ContourMeasure::points array
    t_value: u32,
    kind: SegmentType,
}

impl Segment {
    fn scalar_t(&self) -> f32 {
        debug_assert!(self.t_value <= MAX_T_VALUE);
        // 1/kMaxTValue can't be represented as a float, but it's close and the limits work fine.
        const MAX_T_RECIPROCAL: f32 = 1.0 / MAX_T_VALUE as f32;
        self.t_value as f32 * MAX_T_RECIPROCAL
    }
}

#[derive(Default, Debug)]
struct ContourMeasure {
    segments: Vec<Segment>,
    points: Vec<Point>,
    length: f32,
    is_closed: bool,
}

impl ContourMeasure {
    fn push_segment(
        &self,
        mut start_d: f32,
        mut stop_d: f32,
        start_with_move_to: bool,
        pb: &mut PathBuilder,
    ) {
        if start_d < 0.0 {
            start_d = 0.0;
        }

        if stop_d > self.length {
            stop_d = self.length;
        }

        if !(start_d <= stop_d) {
            // catch NaN values as well
            return;
        }

        if self.segments.is_empty() {
            return;
        }

        let (seg_index, mut start_t) = match self.distance_to_segment(start_d) {
            Some(v) => v,
            None => return,
        };
        let mut seg = self.segments[seg_index];

        let (stop_seg_index, stop_t) = match self.distance_to_segment(stop_d) {
            Some(v) => v,
            None => return,
        };
        let stop_seg = self.segments[stop_seg_index];

        debug_assert!(stop_seg_index <= stop_seg_index);
        let mut p = Point::zero();
        if start_with_move_to {
            compute_pos_tan(
                &self.points[seg.point_index..],
                seg.kind,
                start_t,
                Some(&mut p),
                None,
            );
            pb.move_to(p.x, p.y);
        }

        if seg.point_index == stop_seg.point_index {
            segment_to(
                &self.points[seg.point_index..],
                seg.kind,
                start_t,
                stop_t,
                pb,
            );
        } else {
            let mut new_seg_index = seg_index;
            loop {
                segment_to(
                    &self.points[seg.point_index..],
                    seg.kind,
                    start_t,
                    NormalizedF32::ONE,
                    pb,
                );

                let old_point_index = seg.point_index;
                loop {
                    new_seg_index += 1;
                    if self.segments[new_seg_index].point_index != old_point_index {
                        break;
                    }
                }
                seg = self.segments[new_seg_index];

                start_t = NormalizedF32::ZERO;

                if seg.point_index >= stop_seg.point_index {
                    break;
                }
            }

            segment_to(
                &self.points[seg.point_index..],
                seg.kind,
                NormalizedF32::ZERO,
                stop_t,
                pb,
            );
        }
    }

    fn distance_to_segment(&self, distance: f32) -> Option<(usize, NormalizedF32)> {
        debug_assert!(distance >= 0.0 && distance <= self.length);

        let mut index = find_segment(&self.segments, distance);
        // don't care if we hit an exact match or not, so we xor index if it is negative
        index ^= index >> 31;
        let index = index as usize;
        let seg = self.segments[index];

        // now interpolate t-values with the prev segment (if possible)
        let mut start_t = 0.0;
        let mut start_d = 0.0;
        // check if the prev segment is legal, and references the same set of points
        if index > 0 {
            start_d = self.segments[index - 1].distance;
            if self.segments[index - 1].point_index == seg.point_index {
                debug_assert!(self.segments[index - 1].kind == seg.kind);
                start_t = self.segments[index - 1].scalar_t();
            }
        }

        debug_assert!(seg.scalar_t() > start_t);
        debug_assert!(distance >= start_d);
        debug_assert!(seg.distance > start_d);

        let t =
            start_t + (seg.scalar_t() - start_t) * (distance - start_d) / (seg.distance - start_d);
        let t = NormalizedF32::new(t)?;
        Some((index, t))
    }

    fn compute_line_seg(
        &mut self,
        p0: Point,
        p1: Point,
        mut distance: f32,
        point_index: usize,
    ) -> f32 {
        let d = p0.distance(p1);
        debug_assert!(d >= 0.0);
        let prev_d = distance;
        distance += d;
        if distance > prev_d {
            debug_assert!(point_index < self.points.len());
            self.segments.push(Segment {
                distance,
                point_index,
                t_value: MAX_T_VALUE,
                kind: SegmentType::Line,
            });
        }

        distance
    }

    fn compute_quad_segs(
        &mut self,
        p0: Point,
        p1: Point,
        p2: Point,
        mut distance: f32,
        min_t: u32,
        max_t: u32,
        point_index: usize,
        tolerance: f32,
    ) -> f32 {
        if t_span_big_enough(max_t - min_t) != 0 && quad_too_curvy(p0, p1, p2, tolerance) {
            let mut tmp = [Point::zero(); 5];
            let half_t = (min_t + max_t) >> 1;

            path_geometry::chop_quad_at(&[p0, p1, p2], NormalizedF32Exclusive::HALF, &mut tmp);
            distance = self.compute_quad_segs(
                tmp[0],
                tmp[1],
                tmp[2],
                distance,
                min_t,
                half_t,
                point_index,
                tolerance,
            );
            distance = self.compute_quad_segs(
                tmp[2],
                tmp[3],
                tmp[4],
                distance,
                half_t,
                max_t,
                point_index,
                tolerance,
            );
        } else {
            let d = p0.distance(p2);
            let prev_d = distance;
            distance += d;
            if distance > prev_d {
                debug_assert!(point_index < self.points.len());
                self.segments.push(Segment {
                    distance,
                    point_index,
                    t_value: max_t,
                    kind: SegmentType::Quad,
                });
            }
        }

        distance
    }

    fn compute_cubic_segs(
        &mut self,
        p0: Point,
        p1: Point,
        p2: Point,
        p3: Point,
        mut distance: f32,
        min_t: u32,
        max_t: u32,
        point_index: usize,
        tolerance: f32,
    ) -> f32 {
        if t_span_big_enough(max_t - min_t) != 0 && cubic_too_curvy(p0, p1, p2, p3, tolerance) {
            let mut tmp = [Point::zero(); 7];
            let half_t = (min_t + max_t) >> 1;

            path_geometry::chop_cubic_at2(
                &[p0, p1, p2, p3],
                NormalizedF32Exclusive::HALF,
                &mut tmp,
            );
            distance = self.compute_cubic_segs(
                tmp[0],
                tmp[1],
                tmp[2],
                tmp[3],
                distance,
                min_t,
                half_t,
                point_index,
                tolerance,
            );
            distance = self.compute_cubic_segs(
                tmp[3],
                tmp[4],
                tmp[5],
                tmp[6],
                distance,
                half_t,
                max_t,
                point_index,
                tolerance,
            );
        } else {
            let d = p0.distance(p3);
            let prev_d = distance;
            distance += d;
            if distance > prev_d {
                debug_assert!(point_index < self.points.len());
                self.segments.push(Segment {
                    distance,
                    point_index,
                    t_value: max_t,
                    kind: SegmentType::Cubic,
                });
            }
        }

        distance
    }
}

fn find_segment(base: &[Segment], key: f32) -> i32 {
    let mut lo = 0u32;
    let mut hi = (base.len() - 1) as u32;

    while lo < hi {
        let mid = (hi + lo) >> 1;
        if base[mid as usize].distance < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    if base[hi as usize].distance < key {
        hi += 1;
        hi = !hi;
    } else if key < base[hi as usize].distance {
        hi = !hi;
    }

    hi as i32
}

fn compute_pos_tan(
    points: &[Point],
    seg_kind: SegmentType,
    t: NormalizedF32,
    pos: Option<&mut Point>,
    tangent: Option<&mut Point>,
) {
    match seg_kind {
        SegmentType::Line => {
            if let Some(pos) = pos {
                *pos = Point::from_xy(
                    interp(points[0].x, points[1].x, t),
                    interp(points[0].y, points[1].y, t),
                );
            }

            if let Some(tangent) = tangent {
                tangent.set_normalize(points[1].x - points[0].x, points[1].y - points[0].y);
            }
        }
        SegmentType::Quad => {
            let src = array_ref![points, 0, 3];
            if let Some(pos) = pos {
                *pos = path_geometry::eval_quad_at(src, t);
            }

            if let Some(tangent) = tangent {
                *tangent = path_geometry::eval_quad_tangent_at(src, t);
                tangent.normalize();
            }
        }
        SegmentType::Cubic => {
            let src = array_ref![points, 0, 4];
            if let Some(pos) = pos {
                *pos = path_geometry::eval_cubic_pos_at(src, t);
            }

            if let Some(tangent) = tangent {
                *tangent = path_geometry::eval_cubic_tangent_at(src, t);
                tangent.normalize();
            }
        }
    }
}

fn segment_to(
    points: &[Point],
    seg_kind: SegmentType,
    start_t: NormalizedF32,
    stop_t: NormalizedF32,
    pb: &mut PathBuilder,
) {
    debug_assert!(start_t <= stop_t);

    if start_t == stop_t {
        if let Some(pt) = pb.last_point() {
            // If the dash as a zero-length on segment, add a corresponding zero-length line.
            // The stroke code will add end caps to zero length lines as appropriate.
            pb.line_to(pt.x, pt.y);
        }

        return;
    }

    match seg_kind {
        SegmentType::Line => {
            if stop_t == NormalizedF32::ONE {
                pb.line_to(points[1].x, points[1].y);
            } else {
                pb.line_to(
                    interp(points[0].x, points[1].x, stop_t),
                    interp(points[0].y, points[1].y, stop_t),
                );
            }
        }
        SegmentType::Quad => {
            let mut tmp0 = [Point::zero(); 5];
            let mut tmp1 = [Point::zero(); 5];
            if start_t == NormalizedF32::ZERO {
                if stop_t == NormalizedF32::ONE {
                    pb.quad_to_pt(points[1], points[2]);
                } else {
                    let stop_t = NormalizedF32Exclusive::new_bounded(stop_t.get());
                    path_geometry::chop_quad_at(points, stop_t, &mut tmp0);
                    pb.quad_to_pt(tmp0[1], tmp0[2]);
                }
            } else {
                let start_tt = NormalizedF32Exclusive::new_bounded(start_t.get());
                path_geometry::chop_quad_at(points, start_tt, &mut tmp0);
                if stop_t == NormalizedF32::ONE {
                    pb.quad_to_pt(tmp0[3], tmp0[4]);
                } else {
                    let new_t = (stop_t.get() - start_t.get()) / (1.0 - start_t.get());
                    let new_t = NormalizedF32Exclusive::new_bounded(new_t);
                    path_geometry::chop_quad_at(&tmp0[2..], new_t, &mut tmp1);
                    pb.quad_to_pt(tmp1[1], tmp1[2]);
                }
            }
        }
        SegmentType::Cubic => {
            let mut tmp0 = [Point::zero(); 7];
            let mut tmp1 = [Point::zero(); 7];
            if start_t == NormalizedF32::ZERO {
                if stop_t == NormalizedF32::ONE {
                    pb.cubic_to_pt(points[1], points[2], points[3]);
                } else {
                    let stop_t = NormalizedF32Exclusive::new_bounded(stop_t.get());
                    path_geometry::chop_cubic_at2(array_ref![points, 0, 4], stop_t, &mut tmp0);
                    pb.cubic_to_pt(tmp0[1], tmp0[2], tmp0[3]);
                }
            } else {
                let start_tt = NormalizedF32Exclusive::new_bounded(start_t.get());
                path_geometry::chop_cubic_at2(array_ref![points, 0, 4], start_tt, &mut tmp0);
                if stop_t == NormalizedF32::ONE {
                    pb.cubic_to_pt(tmp0[4], tmp0[5], tmp0[6]);
                } else {
                    let new_t = (stop_t.get() - start_t.get()) / (1.0 - start_t.get());
                    let new_t = NormalizedF32Exclusive::new_bounded(new_t);
                    path_geometry::chop_cubic_at2(array_ref![tmp0, 3, 4], new_t, &mut tmp1);
                    pb.cubic_to_pt(tmp1[1], tmp1[2], tmp1[3]);
                }
            }
        }
    }
}

fn t_span_big_enough(t_span: u32) -> u32 {
    debug_assert!(t_span <= MAX_T_VALUE);
    t_span >> 10
}

fn quad_too_curvy(p0: Point, p1: Point, p2: Point, tolerance: f32) -> bool {
    // diff = (a/4 + b/2 + c/4) - (a/2 + c/2)
    // diff = -a/4 + b/2 - c/4
    let dx = (p1.x).half() - (p0.x + p2.x).half().half();
    let dy = (p1.y).half() - (p0.y + p2.y).half().half();

    let dist = dx.abs().max(dy.abs());
    dist > tolerance
}

fn cubic_too_curvy(p0: Point, p1: Point, p2: Point, p3: Point, tolerance: f32) -> bool {
    let n0 = cheap_dist_exceeds_limit(
        p1,
        interp_safe(p0.x, p3.x, 1.0 / 3.0),
        interp_safe(p0.y, p3.y, 1.0 / 3.0),
        tolerance,
    );

    let n1 = cheap_dist_exceeds_limit(
        p2,
        interp_safe(p0.x, p3.x, 2.0 / 3.0),
        interp_safe(p0.y, p3.y, 2.0 / 3.0),
        tolerance,
    );

    n0 || n1
}

fn cheap_dist_exceeds_limit(pt: Point, x: f32, y: f32, tolerance: f32) -> bool {
    let dist = (x - pt.x).abs().max((y - pt.y).abs());
    // just made up the 1/2
    dist > tolerance
}

/// Linearly interpolate between A and B, based on t.
///
/// If t is 0, return A. If t is 1, return B else interpolate.
fn interp(a: f32, b: f32, t: NormalizedF32) -> f32 {
    a + (b - a) * t.get()
}

fn interp_safe(a: f32, b: f32, t: f32) -> f32 {
    debug_assert!(t >= 0.0 && t <= 1.0);
    a + (b - a) * t
}
