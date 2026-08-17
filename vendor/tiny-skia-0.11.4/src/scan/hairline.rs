// Copyright 2006 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use core::convert::TryInto;

use tiny_skia_path::{f32x2, PathVerb, SaturateCast, Scalar};

use crate::{IntRect, LineCap, Path, PathSegment, Point, Rect};

use crate::blitter::Blitter;
use crate::fixed_point::{fdot16, fdot6};
use crate::geom::ScreenIntRect;
use crate::line_clipper;
use crate::math::LENGTH_U32_ONE;
use crate::path_geometry;

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use tiny_skia_path::NoStdFloat;

const FLOAT_PI: f32 = 3.14159265;

pub type LineProc = fn(&[Point], Option<&ScreenIntRect>, &mut dyn Blitter);

const MAX_CUBIC_SUBDIVIDE_LEVEL: u8 = 9;
const MAX_QUAD_SUBDIVIDE_LEVEL: u8 = 5;

pub fn stroke_path(
    path: &Path,
    line_cap: LineCap,
    clip: &ScreenIntRect,
    blitter: &mut dyn Blitter,
) {
    super::hairline::stroke_path_impl(path, line_cap, clip, hair_line_rgn, blitter)
}

fn hair_line_rgn(points: &[Point], clip: Option<&ScreenIntRect>, blitter: &mut dyn Blitter) {
    let max = 32767.0;
    let fixed_bounds = Rect::from_ltrb(-max, -max, max, max).unwrap();

    let clip_bounds = clip.map(|c| c.to_rect());

    for i in 0..points.len() - 1 {
        let mut pts = [Point::zero(); 2];

        // We have to pre-clip the line to fit in a Fixed, so we just chop the line.
        if !line_clipper::intersect(&[points[i], points[i + 1]], &fixed_bounds, &mut pts) {
            continue;
        }

        if let Some(clip_bounds) = clip_bounds {
            let tmp = pts.clone();
            // Perform a clip in scalar space, so we catch huge values which might
            // be missed after we convert to FDot6 (overflow).
            if !line_clipper::intersect(&tmp, &clip_bounds, &mut pts) {
                continue;
            }
        }

        let mut x0 = fdot6::from_f32(pts[0].x);
        let mut y0 = fdot6::from_f32(pts[0].y);
        let mut x1 = fdot6::from_f32(pts[1].x);
        let mut y1 = fdot6::from_f32(pts[1].y);

        debug_assert!(fdot6::can_convert_to_fdot16(x0));
        debug_assert!(fdot6::can_convert_to_fdot16(y0));
        debug_assert!(fdot6::can_convert_to_fdot16(x1));
        debug_assert!(fdot6::can_convert_to_fdot16(y1));

        let dx = x1 - x0;
        let dy = y1 - y0;

        if dx.abs() > dy.abs() {
            // mostly horizontal

            if x0 > x1 {
                // we want to go left-to-right
                core::mem::swap(&mut x0, &mut x1);
                core::mem::swap(&mut y0, &mut y1);
            }

            let mut ix0 = fdot6::round(x0);
            let ix1 = fdot6::round(x1);
            if ix0 == ix1 {
                // too short to draw
                continue;
            }

            let slope = fdot16::div(dy, dx);
            #[allow(clippy::precedence)]
            let mut start_y = fdot6::to_fdot16(y0) + (slope * ((32 - x0) & 63) >> 6);

            // In some cases, probably due to precision/rounding issues,
            // `start_y` can become equal to the image height,
            // which will lead to panic, because we would be accessing pixels outside
            // the current memory buffer.
            // This is tiny-skia specific issue. Skia handles this part differently.
            let max_y = if let Some(clip_bounds) = clip_bounds {
                fdot16::from_f32(clip_bounds.bottom())
            } else {
                i32::MAX
            };

            debug_assert!(ix0 < ix1);
            loop {
                if ix0 >= 0 && start_y >= 0 && start_y < max_y {
                    blitter.blit_h(ix0 as u32, (start_y >> 16) as u32, LENGTH_U32_ONE);
                }

                start_y += slope;
                ix0 += 1;
                if ix0 >= ix1 {
                    break;
                }
            }
        } else {
            // mostly vertical

            if y0 > y1 {
                // we want to go top-to-bottom
                core::mem::swap(&mut x0, &mut x1);
                core::mem::swap(&mut y0, &mut y1);
            }

            let mut iy0 = fdot6::round(y0);
            let iy1 = fdot6::round(y1);
            if iy0 == iy1 {
                // too short to draw
                continue;
            }

            let slope = fdot16::div(dx, dy);
            #[allow(clippy::precedence)]
            let mut start_x = fdot6::to_fdot16(x0) + (slope * ((32 - y0) & 63) >> 6);

            debug_assert!(iy0 < iy1);
            loop {
                if start_x >= 0 && iy0 >= 0 {
                    blitter.blit_h((start_x >> 16) as u32, iy0 as u32, LENGTH_U32_ONE);
                }

                start_x += slope;
                iy0 += 1;
                if iy0 >= iy1 {
                    break;
                }
            }
        }
    }
}

pub fn stroke_path_impl(
    path: &Path,
    line_cap: LineCap,
    clip: &ScreenIntRect,
    line_proc: LineProc,
    blitter: &mut dyn Blitter,
) {
    let mut inset_clip = None;
    let mut outset_clip = None;

    {
        let cap_out = if line_cap == LineCap::Butt { 1.0 } else { 2.0 };
        let ibounds = match path
            .bounds()
            .outset(cap_out, cap_out)
            .and_then(|r| r.round_out())
        {
            Some(v) => v,
            None => return,
        };
        if clip.to_int_rect().intersect(&ibounds).is_none() {
            return;
        }

        if !clip.to_int_rect().contains(&ibounds) {
            // We now cache two scalar rects, to use for culling per-segment (e.g. cubic).
            // Since we're hairlining, the "bounds" of the control points isn't necessairly the
            // limit of where a segment can draw (it might draw up to 1 pixel beyond in aa-hairs).
            //
            // Compute the pt-bounds per segment is easy, so we do that, and then inversely adjust
            // the culling bounds so we can just do a straight compare per segment.
            //
            // insetClip is use for quick-accept (i.e. the segment is not clipped), so we inset
            // it from the clip-bounds (since segment bounds can be off by 1).
            //
            // outsetClip is used for quick-reject (i.e. the segment is entirely outside), so we
            // outset it from the clip-bounds.
            match clip.to_int_rect().make_outset(1, 1) {
                Some(v) => outset_clip = Some(v),
                None => return,
            }
            match clip.to_int_rect().inset(1, 1) {
                Some(v) => inset_clip = Some(v),
                None => return,
            }
        }
    }

    let clip = Some(clip);
    let mut prev_verb = PathVerb::Move;
    let mut first_pt = Point::zero();
    let mut last_pt = Point::zero();

    let mut iter = path.segments();
    while let Some(segment) = iter.next() {
        let verb = iter.curr_verb();
        let next_verb = iter.next_verb();
        let last_pt2;
        match segment {
            PathSegment::MoveTo(p) => {
                first_pt = p;
                last_pt = p;
                last_pt2 = p;
            }
            PathSegment::LineTo(p) => {
                let mut points = [last_pt, p];
                if line_cap != LineCap::Butt {
                    extend_pts(line_cap, prev_verb, next_verb, &mut points);
                }

                line_proc(&points, clip, blitter);
                last_pt = p;
                last_pt2 = points[0];
            }
            PathSegment::QuadTo(p0, p1) => {
                let mut points = [last_pt, p0, p1];
                if line_cap != LineCap::Butt {
                    extend_pts(line_cap, prev_verb, next_verb, &mut points);
                }

                hair_quad(
                    &points,
                    clip,
                    inset_clip.as_ref(),
                    outset_clip.as_ref(),
                    compute_quad_level(&points),
                    line_proc,
                    blitter,
                );

                last_pt = p1;
                last_pt2 = points[0];
            }
            PathSegment::CubicTo(p0, p1, p2) => {
                let mut points = [last_pt, p0, p1, p2];
                if line_cap != LineCap::Butt {
                    extend_pts(line_cap, prev_verb, next_verb, &mut points);
                }

                hair_cubic(
                    &points,
                    clip,
                    inset_clip.as_ref(),
                    outset_clip.as_ref(),
                    line_proc,
                    blitter,
                );

                last_pt = p2;
                last_pt2 = points[0];
            }
            PathSegment::Close => {
                let mut points = [last_pt, first_pt];
                if line_cap != LineCap::Butt && prev_verb == PathVerb::Move {
                    // cap moveTo/close to match svg expectations for degenerate segments
                    extend_pts(line_cap, prev_verb, next_verb, &mut points);
                }
                line_proc(&points, clip, blitter);
                last_pt2 = points[0];
            }
        }

        if line_cap != LineCap::Butt {
            if prev_verb == PathVerb::Move
                && matches!(verb, PathVerb::Line | PathVerb::Quad | PathVerb::Cubic)
            {
                first_pt = last_pt2; // the curve moved the initial point, so close to it instead
            }

            prev_verb = verb;
        }
    }
}

/// Extend the points in the direction of the starting or ending tangent by 1/2 unit to
/// account for a round or square cap.
///
/// If there's no distance between the end point and
/// the control point, use the next control point to create a tangent. If the curve
/// is degenerate, move the cap out 1/2 unit horizontally.
fn extend_pts(
    line_cap: LineCap,
    prev_verb: PathVerb,
    next_verb: Option<PathVerb>,
    points: &mut [Point],
) {
    debug_assert!(!points.is_empty()); // TODO: use non-zero slice
    debug_assert!(line_cap != LineCap::Butt);

    // The area of a circle is PI*R*R. For a unit circle, R=1/2, and the cap covers half of that.
    let cap_outset = if line_cap == LineCap::Square {
        0.5
    } else {
        FLOAT_PI / 8.0
    };
    if prev_verb == PathVerb::Move {
        let first = points[0];
        let mut offset = 0;
        let mut controls = points.len() - 1;
        let mut tangent;
        loop {
            offset += 1;
            tangent = first - points[offset];

            if !tangent.is_zero() {
                break;
            }

            controls -= 1;
            if controls == 0 {
                break;
            }
        }

        if tangent.is_zero() {
            tangent = Point::from_xy(1.0, 0.0);
            controls = points.len() - 1; // If all points are equal, move all but one.
        } else {
            tangent.normalize();
        }

        offset = 0;
        loop {
            // If the end point and control points are equal, loop to move them in tandem.
            points[offset].x += tangent.x * cap_outset;
            points[offset].y += tangent.y * cap_outset;

            offset += 1;
            controls += 1;
            if controls >= points.len() {
                break;
            }
        }
    }

    if matches!(
        next_verb,
        Some(PathVerb::Move) | Some(PathVerb::Close) | None
    ) {
        let last = points.last().unwrap().clone();
        let mut offset = points.len() - 1;
        let mut controls = points.len() - 1;
        let mut tangent;
        loop {
            offset -= 1;
            tangent = last - points[offset];

            if !tangent.is_zero() {
                break;
            }

            controls -= 1;
            if controls == 0 {
                break;
            }
        }

        if tangent.is_zero() {
            tangent = Point::from_xy(-1.0, 0.0);
            controls = points.len() - 1;
        } else {
            tangent.normalize();
        }

        offset = points.len() - 1;
        loop {
            points[offset].x += tangent.x * cap_outset;
            points[offset].y += tangent.y * cap_outset;

            offset -= 1;
            controls += 1;
            if controls >= points.len() {
                break;
            }
        }
    }
}

fn hair_quad(
    points: &[Point; 3],
    mut clip: Option<&ScreenIntRect>,
    inset_clip: Option<&IntRect>,
    outset_clip: Option<&IntRect>,
    level: u8,
    line_proc: LineProc,
    blitter: &mut dyn Blitter,
) {
    if let Some(inset_clip) = inset_clip {
        debug_assert!(outset_clip.is_some());
        let inset_clip = inset_clip.to_rect();
        let outset_clip = match outset_clip {
            Some(v) => v.to_rect(),
            None => return,
        };

        let bounds = match compute_nocheck_quad_bounds(points) {
            Some(v) => v,
            None => return,
        };
        if !geometric_overlap(&outset_clip, &bounds) {
            return; // nothing to do
        } else if geometric_contains(&inset_clip, &bounds) {
            clip = None;
        }
    }

    hair_quad2(points, clip, level, line_proc, blitter);
}

fn compute_nocheck_quad_bounds(points: &[Point; 3]) -> Option<Rect> {
    debug_assert!(points[0].is_finite());
    debug_assert!(points[1].is_finite());
    debug_assert!(points[2].is_finite());

    let mut min = points[0].to_f32x2();
    let mut max = min;
    for i in 1..3 {
        let pair = points[i].to_f32x2();
        min = min.min(pair);
        max = max.max(pair);
    }

    Rect::from_ltrb(min.x(), min.y(), max.x(), max.y())
}

fn geometric_overlap(a: &Rect, b: &Rect) -> bool {
    a.left() < b.right() && b.left() < a.right() && a.top() < b.bottom() && b.top() < a.bottom()
}

fn geometric_contains(outer: &Rect, inner: &Rect) -> bool {
    inner.right() <= outer.right()
        && inner.left() >= outer.left()
        && inner.bottom() <= outer.bottom()
        && inner.top() >= outer.top()
}

fn hair_quad2(
    points: &[Point; 3],
    clip: Option<&ScreenIntRect>,
    level: u8,
    line_proc: LineProc,
    blitter: &mut dyn Blitter,
) {
    debug_assert!(level <= MAX_QUAD_SUBDIVIDE_LEVEL); // TODO: to type

    let coeff = path_geometry::QuadCoeff::from_points(points);

    const MAX_POINTS: usize = (1 << MAX_QUAD_SUBDIVIDE_LEVEL) + 1;
    let lines = 1 << level;
    debug_assert!(lines < MAX_POINTS);

    let mut tmp = [Point::zero(); MAX_POINTS];
    tmp[0] = points[0];

    let mut t = f32x2::default();
    let dt = f32x2::splat(1.0 / lines as f32);
    for i in 1..lines {
        t = t + dt;
        let v = (coeff.a * t + coeff.b) * t + coeff.c;
        tmp[i] = Point::from_xy(v.x(), v.y());
    }

    tmp[lines] = points[2];
    line_proc(&tmp[0..lines + 1], clip, blitter);
}

fn compute_quad_level(points: &[Point; 3]) -> u8 {
    let d = compute_int_quad_dist(points);
    // Quadratics approach the line connecting their start and end points
    // 4x closer with each subdivision, so we compute the number of
    // subdivisions to be the minimum need to get that distance to be less
    // than a pixel.
    let mut level = (33 - d.leading_zeros()) >> 1;
    // sanity check on level (from the previous version)
    if level > MAX_QUAD_SUBDIVIDE_LEVEL as u32 {
        level = MAX_QUAD_SUBDIVIDE_LEVEL as u32;
    }

    level as u8
}

fn compute_int_quad_dist(points: &[Point; 3]) -> u32 {
    // compute the vector between the control point ([1]) and the middle of the
    // line connecting the start and end ([0] and [2])
    let dx = ((points[0].x + points[2].x).half() - points[1].x).abs();
    let dy = ((points[0].y + points[2].y).half() - points[1].y).abs();

    // convert to whole pixel values (use ceiling to be conservative).
    // assign to unsigned so we can safely add 1/2 of the smaller and still fit in
    // u32, since T::saturate_from() returns 31 bits at most.
    let idx = i32::saturate_from(dx.ceil()) as u32;
    let idy = i32::saturate_from(dy.ceil()) as u32;

    // use the cheap approx for distance
    if idx > idy {
        idx + (idy >> 1)
    } else {
        idy + (idx >> 1)
    }
}

fn hair_cubic(
    points: &[Point; 4],
    mut clip: Option<&ScreenIntRect>,
    inset_clip: Option<&IntRect>,
    outset_clip: Option<&IntRect>,
    line_proc: LineProc,
    blitter: &mut dyn Blitter,
) {
    if let Some(inset_clip) = inset_clip {
        debug_assert!(outset_clip.is_some());
        let inset_clip = inset_clip.to_rect();
        let outset_clip = match outset_clip {
            Some(v) => v.to_rect(),
            None => return,
        };

        let bounds = match compute_nocheck_cubic_bounds(points) {
            Some(v) => v,
            None => return,
        };
        if !geometric_overlap(&outset_clip, &bounds) {
            return; // noting to do
        } else if geometric_contains(&inset_clip, &bounds) {
            clip = None;
        }
    }

    if quick_cubic_niceness_check(points) {
        hair_cubic2(points, clip, line_proc, blitter);
    } else {
        let mut tmp = [Point::zero(); 13];
        let mut t_values = path_geometry::new_t_values();

        let count = path_geometry::chop_cubic_at_max_curvature(points, &mut t_values, &mut tmp);
        for i in 0..count {
            let offset = i * 3;
            let new_points: [Point; 4] = tmp[offset..offset + 4].try_into().unwrap();
            hair_cubic2(&new_points, clip, line_proc, blitter);
        }
    }
}

fn compute_nocheck_cubic_bounds(points: &[Point; 4]) -> Option<Rect> {
    debug_assert!(points[0].is_finite());
    debug_assert!(points[1].is_finite());
    debug_assert!(points[2].is_finite());
    debug_assert!(points[3].is_finite());

    let mut min = points[0].to_f32x2();
    let mut max = min;
    for i in 1..4 {
        let pair = points[i].to_f32x2();
        min = min.min(pair);
        max = max.max(pair);
    }

    Rect::from_ltrb(min.x(), min.y(), max.x(), max.y())
}

// The off-curve points are "inside" the limits of the on-curve points.
fn quick_cubic_niceness_check(points: &[Point; 4]) -> bool {
    lt_90(points[1], points[0], points[3])
        && lt_90(points[2], points[0], points[3])
        && lt_90(points[1], points[3], points[0])
        && lt_90(points[2], points[3], points[0])
}

fn lt_90(p0: Point, pivot: Point, p2: Point) -> bool {
    (p0 - pivot).dot(p2 - pivot) >= 0.0
}

fn hair_cubic2(
    points: &[Point; 4],
    clip: Option<&ScreenIntRect>,
    line_proc: LineProc,
    blitter: &mut dyn Blitter,
) {
    let lines = compute_cubic_segments(points);
    debug_assert!(lines > 0);
    if lines == 1 {
        line_proc(&[points[0], points[3]], clip, blitter);
        return;
    }

    let coeff = path_geometry::CubicCoeff::from_points(points);

    const MAX_POINTS: usize = (1 << MAX_CUBIC_SUBDIVIDE_LEVEL) + 1;
    debug_assert!(lines < MAX_POINTS);
    let mut tmp = [Point::zero(); MAX_POINTS];

    let dt = f32x2::splat(1.0 / lines as f32);
    let mut t = f32x2::default();

    tmp[0] = points[0];
    for i in 1..lines {
        t = t + dt;
        tmp[i] = Point::from_f32x2(((coeff.a * t + coeff.b) * t + coeff.c) * t + coeff.d);
    }

    if tmp.iter().all(|p| p.is_finite()) {
        tmp[lines] = points[3];
        line_proc(&tmp[0..lines + 1], clip, blitter);
    } else {
        // else some point(s) are non-finite, so don't draw
    }
}

fn compute_cubic_segments(points: &[Point; 4]) -> usize {
    let p0 = points[0].to_f32x2();
    let p1 = points[1].to_f32x2();
    let p2 = points[2].to_f32x2();
    let p3 = points[3].to_f32x2();

    let one_third = f32x2::splat(1.0 / 3.0);
    let two_third = f32x2::splat(2.0 / 3.0);

    let p13 = one_third * p3 + two_third * p0;
    let p23 = one_third * p0 + two_third * p3;

    let diff = (p1 - p13).abs().max((p2 - p23).abs()).max_component();
    let mut tol = 1.0 / 8.0;

    for i in 0..MAX_CUBIC_SUBDIVIDE_LEVEL {
        if diff < tol {
            return 1 << i;
        }

        tol *= 4.0;
    }

    1 << MAX_CUBIC_SUBDIVIDE_LEVEL
}
