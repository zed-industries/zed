// Copyright 2009 The Android Open Source Project
// Copyright 2020 Yevhenii Reizner
//
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use arrayvec::ArrayVec;

use tiny_skia_path::{NormalizedF32Exclusive, SCALAR_MAX};

use crate::{Path, Point, Rect};

use crate::edge_builder::{edge_iter, PathEdge, PathEdgeIter};
use crate::line_clipper;
use crate::path_geometry;

#[cfg(all(not(feature = "std"), feature = "no-std-float"))]
use tiny_skia_path::NoStdFloat;

// This is a fail-safe `arr[n..n+3].try_into().unwrap()` alternative.
// Everything is checked at compile-time so there is no bound checking and panics.
macro_rules! copy_3_points {
    ($arr:expr, $i:expr) => {
        [$arr[$i], $arr[$i + 1], $arr[$i + 2]]
    };
}

macro_rules! copy_4_points {
    ($arr:expr, $i:expr) => {
        [$arr[$i], $arr[$i + 1], $arr[$i + 2], $arr[$i + 3]]
    };
}

/// Max curvature in X and Y split cubic into 9 pieces, * (line + cubic).
const MAX_VERBS: usize = 18;

pub type ClippedEdges = ArrayVec<PathEdge, MAX_VERBS>;

pub struct EdgeClipper {
    clip: Rect,
    can_cull_to_the_right: bool,
    edges: ClippedEdges,
}

impl EdgeClipper {
    fn new(clip: Rect, can_cull_to_the_right: bool) -> Self {
        EdgeClipper {
            clip,
            can_cull_to_the_right,
            edges: ArrayVec::new(),
        }
    }

    fn clip_line(mut self, p0: Point, p1: Point) -> Option<ClippedEdges> {
        let mut points = [Point::zero(); line_clipper::MAX_POINTS];
        let points = line_clipper::clip(
            &[p0, p1],
            &self.clip,
            self.can_cull_to_the_right,
            &mut points,
        );
        if !points.is_empty() {
            for i in 0..points.len() - 1 {
                self.push_line(points[i], points[i + 1]);
            }
        }

        if self.edges.is_empty() {
            None
        } else {
            Some(self.edges)
        }
    }

    fn push_line(&mut self, p0: Point, p1: Point) {
        self.edges.push(PathEdge::LineTo(p0, p1));
    }

    fn push_vline(&mut self, x: f32, mut y0: f32, mut y1: f32, reverse: bool) {
        if reverse {
            core::mem::swap(&mut y0, &mut y1);
        }

        self.edges.push(PathEdge::LineTo(
            Point::from_xy(x, y0),
            Point::from_xy(x, y1),
        ));
    }

    fn clip_quad(mut self, p0: Point, p1: Point, p2: Point) -> Option<ClippedEdges> {
        let pts = [p0, p1, p2];
        let bounds = Rect::from_points(&pts)?;

        if !quick_reject(&bounds, &self.clip) {
            let mut mono_y = [Point::zero(); 5];
            let count_y = path_geometry::chop_quad_at_y_extrema(&pts, &mut mono_y);
            for y in 0..=count_y {
                let mut mono_x = [Point::zero(); 5];
                let y_points: [Point; 3] = copy_3_points!(mono_y, y * 2);
                let count_x = path_geometry::chop_quad_at_x_extrema(&y_points, &mut mono_x);
                for x in 0..=count_x {
                    let x_points: [Point; 3] = copy_3_points!(mono_x, x * 2);
                    self.clip_mono_quad(&x_points);
                }
            }
        }

        if self.edges.is_empty() {
            None
        } else {
            Some(self.edges)
        }
    }

    // src[] must be monotonic in X and Y
    fn clip_mono_quad(&mut self, src: &[Point; 3]) {
        let mut pts = [Point::zero(); 3];
        let mut reverse = sort_increasing_y(src, &mut pts);

        // are we completely above or below
        if pts[2].y <= self.clip.top() || pts[0].y >= self.clip.bottom() {
            return;
        }

        // Now chop so that pts is contained within clip in Y
        chop_quad_in_y(&self.clip, &mut pts);

        if pts[0].x > pts[2].x {
            pts.swap(0, 2);
            reverse = !reverse;
        }
        debug_assert!(pts[0].x <= pts[1].x);
        debug_assert!(pts[1].x <= pts[2].x);

        // Now chop in X has needed, and record the segments

        if pts[2].x <= self.clip.left() {
            // wholly to the left
            self.push_vline(self.clip.left(), pts[0].y, pts[2].y, reverse);
            return;
        }

        if pts[0].x >= self.clip.right() {
            // wholly to the right
            if !self.can_cull_to_the_right {
                self.push_vline(self.clip.right(), pts[0].y, pts[2].y, reverse);
            }

            return;
        }

        let mut t = NormalizedF32Exclusive::ANY;
        let mut tmp = [Point::zero(); 5];

        // are we partially to the left
        if pts[0].x < self.clip.left() {
            if chop_mono_quad_at_x(&pts, self.clip.left(), &mut t) {
                path_geometry::chop_quad_at(&pts, t, &mut tmp);
                self.push_vline(self.clip.left(), tmp[0].y, tmp[2].y, reverse);
                // clamp to clean up imprecise numerics in the chop
                tmp[2].x = self.clip.left();
                tmp[3].x = tmp[3].x.max(self.clip.left());

                pts[0] = tmp[2];
                pts[1] = tmp[3];
            } else {
                // if chopMonoQuadAtY failed, then we may have hit inexact numerics
                // so we just clamp against the left
                self.push_vline(self.clip.left(), pts[0].y, pts[2].y, reverse);
                return;
            }
        }

        // are we partially to the right
        if pts[2].x > self.clip.right() {
            if chop_mono_quad_at_x(&pts, self.clip.right(), &mut t) {
                path_geometry::chop_quad_at(&pts, t, &mut tmp);
                // clamp to clean up imprecise numerics in the chop
                tmp[1].x = tmp[1].x.min(self.clip.right());
                tmp[2].x = self.clip.right();

                self.push_quad(&copy_3_points!(tmp, 0), reverse);
                self.push_vline(self.clip.right(), tmp[2].y, tmp[4].y, reverse);
            } else {
                // if chopMonoQuadAtY failed, then we may have hit inexact numerics
                // so we just clamp against the right
                pts[1].x = pts[1].x.min(self.clip.right());
                pts[2].x = pts[2].x.min(self.clip.right());
                self.push_quad(&pts, reverse);
            }
        } else {
            // wholly inside the clip
            self.push_quad(&pts, reverse);
        }
    }

    fn push_quad(&mut self, pts: &[Point; 3], reverse: bool) {
        if reverse {
            self.edges.push(PathEdge::QuadTo(pts[2], pts[1], pts[0]));
        } else {
            self.edges.push(PathEdge::QuadTo(pts[0], pts[1], pts[2]));
        }
    }

    fn clip_cubic(mut self, p0: Point, p1: Point, p2: Point, p3: Point) -> Option<ClippedEdges> {
        let pts = [p0, p1, p2, p3];
        let bounds = Rect::from_points(&pts)?;

        // check if we're clipped out vertically
        if bounds.bottom() > self.clip.top() && bounds.top() < self.clip.bottom() {
            if too_big_for_reliable_float_math(&bounds) {
                // can't safely clip the cubic, so we give up and draw a line (which we can safely clip)
                //
                // If we rewrote chopcubicat*extrema and chopmonocubic using doubles, we could very
                // likely always handle the cubic safely, but (it seems) at a big loss in speed, so
                // we'd only want to take that alternate impl if needed.
                return self.clip_line(p0, p3);
            } else {
                let mut mono_y = [Point::zero(); 10];
                let count_y = path_geometry::chop_cubic_at_y_extrema(&pts, &mut mono_y);
                for y in 0..=count_y {
                    let mut mono_x = [Point::zero(); 10];
                    let y_points: [Point; 4] = copy_4_points!(mono_y, y * 3);
                    let count_x = path_geometry::chop_cubic_at_x_extrema(&y_points, &mut mono_x);
                    for x in 0..=count_x {
                        let x_points: [Point; 4] = copy_4_points!(mono_x, x * 3);
                        self.clip_mono_cubic(&x_points);
                    }
                }
            }
        }

        if self.edges.is_empty() {
            None
        } else {
            Some(self.edges)
        }
    }

    // src[] must be monotonic in X and Y
    fn clip_mono_cubic(&mut self, src: &[Point; 4]) {
        let mut pts = [Point::zero(); 4];
        let mut reverse = sort_increasing_y(src, &mut pts);

        // are we completely above or below
        if pts[3].y <= self.clip.top() || pts[0].y >= self.clip.bottom() {
            return;
        }

        // Now chop so that pts is contained within clip in Y
        chop_cubic_in_y(&self.clip, &mut pts);

        if pts[0].x > pts[3].x {
            pts.swap(0, 3);
            pts.swap(1, 2);
            reverse = !reverse;
        }

        // Now chop in X has needed, and record the segments

        if pts[3].x <= self.clip.left() {
            // wholly to the left
            self.push_vline(self.clip.left(), pts[0].y, pts[3].y, reverse);
            return;
        }

        if pts[0].x >= self.clip.right() {
            // wholly to the right
            if !self.can_cull_to_the_right {
                self.push_vline(self.clip.right(), pts[0].y, pts[3].y, reverse);
            }

            return;
        }

        // are we partially to the left
        if pts[0].x < self.clip.left() {
            let mut tmp = [Point::zero(); 7];
            chop_mono_cubic_at_x(&pts, self.clip.left(), &mut tmp);
            self.push_vline(self.clip.left(), tmp[0].y, tmp[3].y, reverse);

            // tmp[3, 4].fX should all be to the right of clip.left().
            // Since we can't trust the numerics of
            // the chopper, we force those conditions now
            tmp[3].x = self.clip.left();
            tmp[4].x = tmp[4].x.max(self.clip.left());

            pts[0] = tmp[3];
            pts[1] = tmp[4];
            pts[2] = tmp[5];
        }

        // are we partially to the right
        if pts[3].x > self.clip.right() {
            let mut tmp = [Point::zero(); 7];
            chop_mono_cubic_at_x(&pts, self.clip.right(), &mut tmp);
            tmp[3].x = self.clip.right();
            tmp[2].x = tmp[2].x.min(self.clip.right());

            self.push_cubic(&copy_4_points!(tmp, 0), reverse);
            self.push_vline(self.clip.right(), tmp[3].y, tmp[6].y, reverse);
        } else {
            // wholly inside the clip
            self.push_cubic(&pts, reverse);
        }
    }

    fn push_cubic(&mut self, pts: &[Point; 4], reverse: bool) {
        if reverse {
            self.edges
                .push(PathEdge::CubicTo(pts[3], pts[2], pts[1], pts[0]));
        } else {
            self.edges
                .push(PathEdge::CubicTo(pts[0], pts[1], pts[2], pts[3]));
        }
    }
}

pub struct EdgeClipperIter<'a> {
    edge_iter: PathEdgeIter<'a>,
    clip: Rect,
    can_cull_to_the_right: bool,
}

impl<'a> EdgeClipperIter<'a> {
    pub fn new(path: &'a Path, clip: Rect, can_cull_to_the_right: bool) -> Self {
        EdgeClipperIter {
            edge_iter: edge_iter(path),
            clip,
            can_cull_to_the_right,
        }
    }
}

impl Iterator for EdgeClipperIter<'_> {
    type Item = ClippedEdges;

    fn next(&mut self) -> Option<Self::Item> {
        for edge in &mut self.edge_iter {
            let clipper = EdgeClipper::new(self.clip, self.can_cull_to_the_right);

            match edge {
                PathEdge::LineTo(p0, p1) => {
                    if let Some(edges) = clipper.clip_line(p0, p1) {
                        return Some(edges);
                    }
                }
                PathEdge::QuadTo(p0, p1, p2) => {
                    if let Some(edges) = clipper.clip_quad(p0, p1, p2) {
                        return Some(edges);
                    }
                }
                PathEdge::CubicTo(p0, p1, p2, p3) => {
                    if let Some(edges) = clipper.clip_cubic(p0, p1, p2, p3) {
                        return Some(edges);
                    }
                }
            }
        }

        None
    }
}

fn quick_reject(bounds: &Rect, clip: &Rect) -> bool {
    bounds.top() >= clip.bottom() || bounds.bottom() <= clip.top()
}

// src[] must be monotonic in Y. This routine copies src into dst, and sorts
// it to be increasing in Y. If it had to reverse the order of the points,
// it returns true, otherwise it returns false
fn sort_increasing_y(src: &[Point], dst: &mut [Point]) -> bool {
    // We need the data to be monotonically increasing in Y.
    // Never fails, because src is always non-empty.
    if src[0].y > src.last().unwrap().y {
        for (i, p) in src.iter().rev().enumerate() {
            dst[i] = *p;
        }

        true
    } else {
        dst[0..src.len()].copy_from_slice(src);
        false
    }
}

/// Modifies pts[] in place so that it is clipped in Y to the clip rect.
fn chop_quad_in_y(clip: &Rect, pts: &mut [Point; 3]) {
    let mut t = NormalizedF32Exclusive::ANY;
    let mut tmp = [Point::zero(); 5];

    // are we partially above
    if pts[0].y < clip.top() {
        if chop_mono_quad_at_y(pts, clip.top(), &mut t) {
            // take the 2nd chopped quad
            path_geometry::chop_quad_at(pts, t, &mut tmp);
            // clamp to clean up imprecise numerics in the chop
            tmp[2].y = clip.top();
            tmp[3].y = tmp[3].y.max(clip.top());

            pts[0] = tmp[2];
            pts[1] = tmp[3];
        } else {
            // if chop_mono_quad_at_y failed, then we may have hit inexact numerics
            // so we just clamp against the top
            for p in pts.iter_mut() {
                if p.y < clip.top() {
                    p.y = clip.top();
                }
            }
        }
    }

    // are we partially below
    if pts[2].y > clip.bottom() {
        if chop_mono_quad_at_y(pts, clip.bottom(), &mut t) {
            path_geometry::chop_quad_at(pts, t, &mut tmp);
            // clamp to clean up imprecise numerics in the chop
            tmp[1].y = tmp[1].y.min(clip.bottom());
            tmp[2].y = clip.bottom();

            pts[1] = tmp[1];
            pts[2] = tmp[2];
        } else {
            // if chop_mono_quad_at_y failed, then we may have hit inexact numerics
            // so we just clamp against the bottom
            for p in pts.iter_mut() {
                if p.y > clip.bottom() {
                    p.y = clip.bottom();
                }
            }
        }
    }
}

fn chop_mono_quad_at_x(pts: &[Point; 3], x: f32, t: &mut NormalizedF32Exclusive) -> bool {
    chop_mono_quad_at(pts[0].x, pts[1].x, pts[2].x, x, t)
}

fn chop_mono_quad_at_y(pts: &[Point; 3], y: f32, t: &mut NormalizedF32Exclusive) -> bool {
    chop_mono_quad_at(pts[0].y, pts[1].y, pts[2].y, y, t)
}

fn chop_mono_quad_at(
    c0: f32,
    c1: f32,
    c2: f32,
    target: f32,
    t: &mut NormalizedF32Exclusive,
) -> bool {
    // Solve F(t) = y where F(t) := [0](1-t)^2 + 2[1]t(1-t) + [2]t^2
    // We solve for t, using quadratic equation, hence we have to rearrange
    // our coefficients to look like At^2 + Bt + C
    let a = c0 - c1 - c1 + c2;
    let b = 2.0 * (c1 - c0);
    let c = c0 - target;

    let mut roots = path_geometry::new_t_values();
    let count = path_geometry::find_unit_quad_roots(a, b, c, &mut roots);
    if count != 0 {
        *t = roots[0];
        true
    } else {
        false
    }
}

fn too_big_for_reliable_float_math(r: &Rect) -> bool {
    // limit set as the largest float value for which we can still reliably compute things like
    // - chopping at XY extrema
    // - chopping at Y or X values for clipping
    //
    // Current value chosen just by experiment. Larger (and still succeeds) is always better.

    let limit = (1 << 22) as f32;
    r.left() < -limit || r.top() < -limit || r.right() > limit || r.bottom() > limit
}

/// Modifies pts[] in place so that it is clipped in Y to the clip rect.
fn chop_cubic_in_y(clip: &Rect, pts: &mut [Point; 4]) {
    // are we partially above
    if pts[0].y < clip.top() {
        let mut tmp = [Point::zero(); 7];
        chop_mono_cubic_at_y(pts, clip.top(), &mut tmp);

        // For a large range in the points, we can do a poor job of chopping, such that the t
        // we computed resulted in the lower cubic still being partly above the clip.
        //
        // If just the first or first 2 Y values are above the fTop, we can just smash them
        // down. If the first 3 Ys are above fTop, we can't smash all 3, as that can really
        // distort the cubic. In this case, we take the first output (tmp[3..6] and treat it as
        // a guess, and re-chop against fTop. Then we fall through to checking if we need to
        // smash the first 1 or 2 Y values.
        if tmp[3].y < clip.top() && tmp[4].y < clip.top() && tmp[5].y < clip.top() {
            let tmp2: [Point; 4] = copy_4_points!(tmp, 3);
            chop_mono_cubic_at_y(&tmp2, clip.top(), &mut tmp);
        }

        // tmp[3, 4].y should all be to the below clip.fTop.
        // Since we can't trust the numerics of the chopper, we force those conditions now
        tmp[3].y = clip.top();
        tmp[4].y = tmp[4].y.max(clip.top());

        pts[0] = tmp[3];
        pts[1] = tmp[4];
        pts[2] = tmp[5];
    }

    // are we partially below
    if pts[3].y > clip.bottom() {
        let mut tmp = [Point::zero(); 7];
        chop_mono_cubic_at_y(pts, clip.bottom(), &mut tmp);
        tmp[3].y = clip.bottom();
        tmp[2].y = tmp[2].y.min(clip.bottom());

        pts[1] = tmp[1];
        pts[2] = tmp[2];
        pts[3] = tmp[3];
    }
}

fn chop_mono_cubic_at_x(src: &[Point; 4], x: f32, dst: &mut [Point; 7]) {
    if path_geometry::chop_mono_cubic_at_x(src, x, dst) {
        return;
    }

    let src_values = [src[0].x, src[1].x, src[2].x, src[3].x];
    path_geometry::chop_cubic_at2(src, mono_cubic_closest_t(&src_values, x), dst);
}

fn chop_mono_cubic_at_y(src: &[Point; 4], y: f32, dst: &mut [Point; 7]) {
    if path_geometry::chop_mono_cubic_at_y(src, y, dst) {
        return;
    }

    let src_values = [src[0].y, src[1].y, src[2].y, src[3].y];
    path_geometry::chop_cubic_at2(src, mono_cubic_closest_t(&src_values, y), dst);
}

fn mono_cubic_closest_t(src: &[f32; 4], mut x: f32) -> NormalizedF32Exclusive {
    let mut t = 0.5;
    let mut last_t;
    let mut best_t = t;
    let mut step = 0.25;
    let d = src[0];
    let a = src[3] + 3.0 * (src[1] - src[2]) - d;
    let b = 3.0 * (src[2] - src[1] - src[1] + d);
    let c = 3.0 * (src[1] - d);
    x -= d;
    let mut closest = SCALAR_MAX;
    loop {
        let loc = ((a * t + b) * t + c) * t;
        let dist = (loc - x).abs();
        if closest > dist {
            closest = dist;
            best_t = t;
        }

        last_t = t;
        t += if loc < x { step } else { -step };
        step *= 0.5;

        if !(closest > 0.25 && last_t != t) {
            break;
        }
    }

    NormalizedF32Exclusive::new(best_t).unwrap()
}
