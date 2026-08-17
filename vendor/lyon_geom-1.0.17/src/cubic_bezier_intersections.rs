//! Computes intersection parameters for two cubic bézier curves using bézier clipping, also known
//! as fat line clipping.
//!
//! The implementation here was originally ported from that of paper.js:
//! https://github.com/paperjs/paper.js/blob/0deddebb2c83ea2a0c848f7c8ba5e22fa7562a4e/src/path/Curve.js#L2008
//! See "bézier Clipping method" in
//! https://scholarsarchive.byu.edu/facpub/1/
//! for motivation and details of how the process works.

use crate::scalar::Scalar;
use crate::CubicBezierSegment;
use crate::{point, Box2D, Point};
use arrayvec::ArrayVec;
use core::mem;
use core::ops::Range;

// Computes the intersections (if any) between two cubic bézier curves in the form of the `t`
// parameters of each intersection point along the curves.
//
// Returns endpoint intersections where an endpoint intersects the interior of the other curve,
// but not endpoint/endpoint intersections.
//
// Returns no intersections if either curve is a point or if the curves are parallel lines.
pub fn cubic_bezier_intersections_t<S: Scalar>(
    curve1: &CubicBezierSegment<S>,
    curve2: &CubicBezierSegment<S>,
) -> ArrayVec<(S, S), 9> {
    if !curve1
        .fast_bounding_box()
        .intersects(&curve2.fast_bounding_box())
        || curve1 == curve2
        || (curve1.from == curve2.to
            && curve1.ctrl1 == curve2.ctrl2
            && curve1.ctrl2 == curve2.ctrl1
            && curve1.to == curve2.from)
    {
        return ArrayVec::new();
    }

    let mut result = ArrayVec::new();

    #[inline]
    fn midpoint<S: Scalar>(point1: &Point<S>, point2: &Point<S>) -> Point<S> {
        point(
            (point1.x + point2.x) * S::HALF,
            (point1.y + point2.y) * S::HALF,
        )
    }

    let curve1_is_a_point = curve1.is_a_point(S::EPSILON);
    let curve2_is_a_point = curve2.is_a_point(S::EPSILON);
    if curve1_is_a_point && !curve2_is_a_point {
        let point1 = midpoint(&curve1.from, &curve1.to);
        let curve_params = point_curve_intersections(&point1, curve2, S::EPSILON);
        for t in curve_params {
            if t > S::EPSILON && t < S::ONE - S::EPSILON {
                result.push((S::ZERO, t));
            }
        }
    } else if !curve1_is_a_point && curve2_is_a_point {
        let point2 = midpoint(&curve2.from, &curve2.to);
        let curve_params = point_curve_intersections(&point2, curve1, S::EPSILON);
        for t in curve_params {
            if t > S::EPSILON && t < S::ONE - S::EPSILON {
                result.push((t, S::ZERO));
            }
        }
    }
    if curve1_is_a_point || curve2_is_a_point {
        // Caller is always responsible for checking endpoints and overlaps, in the case that both
        // curves were points.
        return result;
    }

    let linear1 = curve1.is_linear(S::EPSILON);
    let linear2 = curve2.is_linear(S::EPSILON);
    if linear1 && !linear2 {
        result = line_curve_intersections(curve1, curve2, /* flip */ false);
    } else if !linear1 && linear2 {
        result = line_curve_intersections(curve2, curve1, /* flip */ true);
    } else if linear1 && linear2 {
        result = line_line_intersections(curve1, curve2);
    } else {
        add_curve_intersections(
            curve1,
            curve2,
            &(S::ZERO..S::ONE),
            &(S::ZERO..S::ONE),
            &mut result,
            /* flip */ false,
            /* recursion_count */ 0,
            /* call_count */ 0,
            /* original curve1 */ curve1,
            /* original curve2 */ curve2,
        );
    }

    result
}

fn point_curve_intersections<S: Scalar>(
    pt: &Point<S>,
    curve: &CubicBezierSegment<S>,
    epsilon: S,
) -> ArrayVec<S, 9> {
    let mut result = ArrayVec::new();

    // (If both endpoints are epsilon close, we only return S::ZERO.)
    if (*pt - curve.from).square_length() < epsilon {
        result.push(S::ZERO);
        return result;
    }
    if (*pt - curve.to).square_length() < epsilon {
        result.push(S::ONE);
        return result;
    }

    let curve_x_t_params = curve.solve_t_for_x(pt.x);
    let curve_y_t_params = curve.solve_t_for_y(pt.y);
    // We want to coalesce parameters representing the same intersection from the x and y
    // directions, but the parameter calculations aren't very accurate, so give a little more
    // leeway there (TODO: this isn't perfect, as you might expect - the dupes that pass here are
    // currently being detected in add_intersection).
    let param_eps = S::TEN * epsilon;
    for params in [curve_x_t_params, curve_y_t_params].iter() {
        for t in params {
            let t = *t;
            if (*pt - curve.sample(t)).square_length() > epsilon {
                continue;
            }
            let mut already_found_t = false;
            for u in &result {
                if S::abs(t - *u) < param_eps {
                    already_found_t = true;
                    break;
                }
            }
            if !already_found_t {
                result.push(t);
            }
        }
    }

    if !result.is_empty() {
        return result;
    }

    // The remaining case is if pt is within epsilon of an interior point of curve, but not within
    // the x-range or y-range of the curve (which we already checked) - for example if curve is a
    // horizontal line that extends beyond its endpoints, and pt is just outside an end of the line;
    // or if the curve has a cusp in one of the corners of its convex hull and pt is
    // diagonally just outside the hull.  This is a rare case (could we even ignore it?).
    #[inline]
    fn maybe_add<S: Scalar>(
        t: S,
        pt: &Point<S>,
        curve: &CubicBezierSegment<S>,
        epsilon: S,
        result: &mut ArrayVec<S, 9>,
    ) -> bool {
        if (curve.sample(t) - *pt).square_length() < epsilon {
            result.push(t);
            return true;
        }
        false
    }

    let _ = maybe_add(curve.x_minimum_t(), pt, curve, epsilon, &mut result)
        || maybe_add(curve.x_maximum_t(), pt, curve, epsilon, &mut result)
        || maybe_add(curve.y_minimum_t(), pt, curve, epsilon, &mut result)
        || maybe_add(curve.y_maximum_t(), pt, curve, epsilon, &mut result);

    result
}

fn line_curve_intersections<S: Scalar>(
    line_as_curve: &CubicBezierSegment<S>,
    curve: &CubicBezierSegment<S>,
    flip: bool,
) -> ArrayVec<(S, S), 9> {
    let mut result = ArrayVec::new();
    let baseline = line_as_curve.baseline();
    let curve_intersections = curve.line_intersections_t(&baseline.to_line());
    let line_is_mostly_vertical =
        S::abs(baseline.from.y - baseline.to.y) >= S::abs(baseline.from.x - baseline.to.x);
    for curve_t in curve_intersections {
        let line_intersections = if line_is_mostly_vertical {
            let intersection_y = curve.y(curve_t);
            line_as_curve.solve_t_for_y(intersection_y)
        } else {
            let intersection_x = curve.x(curve_t);
            line_as_curve.solve_t_for_x(intersection_x)
        };

        for line_t in line_intersections {
            add_intersection(line_t, line_as_curve, curve_t, curve, flip, &mut result);
        }
    }

    result
}

fn line_line_intersections<S: Scalar>(
    curve1: &CubicBezierSegment<S>,
    curve2: &CubicBezierSegment<S>,
) -> ArrayVec<(S, S), 9> {
    let mut result = ArrayVec::new();

    let intersection = curve1
        .baseline()
        .to_line()
        .intersection(&curve2.baseline().to_line());
    if intersection.is_none() {
        return result;
    }

    let intersection = intersection.unwrap();

    #[inline]
    fn parameters_for_line_point<S: Scalar>(
        curve: &CubicBezierSegment<S>,
        pt: &Point<S>,
    ) -> ArrayVec<S, 3> {
        let line_is_mostly_vertical =
            S::abs(curve.from.y - curve.to.y) >= S::abs(curve.from.x - curve.to.x);
        if line_is_mostly_vertical {
            curve.solve_t_for_y(pt.y)
        } else {
            curve.solve_t_for_x(pt.x)
        }
    }

    let line1_params = parameters_for_line_point(curve1, &intersection);
    if line1_params.is_empty() {
        return result;
    }

    let line2_params = parameters_for_line_point(curve2, &intersection);
    if line2_params.is_empty() {
        return result;
    }

    for t1 in &line1_params {
        for t2 in &line2_params {
            // It could be argued that an endpoint intersection located in the interior of one
            // or both curves should be returned here; we currently don't.
            add_intersection(*t1, curve1, *t2, curve2, /* flip */ false, &mut result);
        }
    }

    result
}

// This function implements the main bézier clipping algorithm by recursively subdividing curve1 and
// curve2 in to smaller and smaller portions of the original curves with the property that one of
// the curves intersects the fat line of the other curve at each stage.
//
// curve1 and curve2 at each stage are sub-bézier curves of the original curves; flip tells us
// whether curve1 at a given stage is a sub-curve of the original curve1 or the original curve2;
// similarly for curve2.  domain1 and domain2 shrink (or stay the same) at each stage and describe
// which subdomain of an original curve the current curve1 and curve2 correspond to. (The domains of
// curve1 and curve2 are 0..1 at every stage.)
#[allow(clippy::too_many_arguments)]
fn add_curve_intersections<S: Scalar>(
    curve1: &CubicBezierSegment<S>,
    curve2: &CubicBezierSegment<S>,
    domain1: &Range<S>,
    domain2: &Range<S>,
    intersections: &mut ArrayVec<(S, S), 9>,
    flip: bool,
    mut recursion_count: u32,
    mut call_count: u32,
    orig_curve1: &CubicBezierSegment<S>,
    orig_curve2: &CubicBezierSegment<S>,
) -> u32 {
    call_count += 1;
    recursion_count += 1;
    if call_count >= 4096 || recursion_count >= 60 {
        return call_count;
    }

    let epsilon = if inputs_are_f32::<S>() {
        S::value(5e-6)
    } else {
        S::value(1e-9)
    };

    if domain2.start == domain2.end || curve2.is_a_point(S::ZERO) {
        add_point_curve_intersection(
            curve2,
            /* point is curve1 */ false,
            curve1,
            domain2,
            domain1,
            intersections,
            flip,
        );
        return call_count;
    } else if curve2.from == curve2.to {
        // There's no curve2 baseline to fat-line against (and we'll (debug) crash if we try with
        // the current implementation), so split curve2 and try again.
        let new_2_curves = orig_curve2.split_range(domain2.clone()).split(S::HALF);
        let domain2_mid = (domain2.start + domain2.end) * S::HALF;
        call_count = add_curve_intersections(
            curve1,
            &new_2_curves.0,
            domain1,
            &(domain2.start..domain2_mid),
            intersections,
            flip,
            recursion_count,
            call_count,
            orig_curve1,
            orig_curve2,
        );
        call_count = add_curve_intersections(
            curve1,
            &new_2_curves.1,
            domain1,
            &(domain2_mid..domain2.end),
            intersections,
            flip,
            recursion_count,
            call_count,
            orig_curve1,
            orig_curve2,
        );
        return call_count;
    }

    // (Don't call this before checking for point curves: points are inexact and can lead to false
    // negatives here.)
    if !rectangles_overlap(&curve1.fast_bounding_box(), &curve2.fast_bounding_box()) {
        return call_count;
    }

    let (t_min_clip, t_max_clip) = match restrict_curve_to_fat_line(curve1, curve2) {
        Some((min, max)) => (min, max),
        None => return call_count,
    };

    // t_min_clip and t_max_clip are (0, 1)-based, so project them back to get the new restricted
    // range:
    let new_domain1 =
        &(domain_value_at_t(domain1, t_min_clip)..domain_value_at_t(domain1, t_max_clip));

    if S::max(
        domain2.end - domain2.start,
        new_domain1.end - new_domain1.start,
    ) < epsilon
    {
        let t1 = (new_domain1.start + new_domain1.end) * S::HALF;
        let t2 = (domain2.start + domain2.end) * S::HALF;
        if inputs_are_f32::<S>() {
            // There's an unfortunate tendency for curve2 endpoints that end near (but not all
            // that near) to the interior of curve1 to register as intersections, so try to avoid
            // that. (We could be discarding a legitimate intersection here.)
            let end_eps = S::value(1e-3);
            if (t2 < end_eps || t2 > S::ONE - end_eps)
                && (orig_curve1.sample(t1) - orig_curve2.sample(t2)).length() > S::FIVE
            {
                return call_count;
            }
        }
        add_intersection(t1, orig_curve1, t2, orig_curve2, flip, intersections);
        return call_count;
    }

    // Reduce curve1 to the part that might intersect curve2.
    let curve1 = &orig_curve1.split_range(new_domain1.clone());

    // (Note: it's possible for new_domain1 to have become a point, even if
    // t_min_clip < t_max_clip. It's also possible for curve1 to not be a point even if new_domain1
    // is a point (but then curve1 will be very small).)
    if new_domain1.start == new_domain1.end || curve1.is_a_point(S::ZERO) {
        add_point_curve_intersection(
            curve1,
            /* point is curve1 */ true,
            curve2,
            new_domain1,
            domain2,
            intersections,
            flip,
        );
        return call_count;
    }

    // If the new range is still 80% or more of the old range, subdivide and try again.
    if t_max_clip - t_min_clip > S::EIGHT / S::TEN {
        // Subdivide the curve which has converged the least.
        if new_domain1.end - new_domain1.start > domain2.end - domain2.start {
            let new_1_curves = curve1.split(S::HALF);
            let new_domain1_mid = (new_domain1.start + new_domain1.end) * S::HALF;
            call_count = add_curve_intersections(
                curve2,
                &new_1_curves.0,
                domain2,
                &(new_domain1.start..new_domain1_mid),
                intersections,
                !flip,
                recursion_count,
                call_count,
                orig_curve2,
                orig_curve1,
            );
            call_count = add_curve_intersections(
                curve2,
                &new_1_curves.1,
                domain2,
                &(new_domain1_mid..new_domain1.end),
                intersections,
                !flip,
                recursion_count,
                call_count,
                orig_curve2,
                orig_curve1,
            );
        } else {
            let new_2_curves = orig_curve2.split_range(domain2.clone()).split(S::HALF);
            let domain2_mid = (domain2.start + domain2.end) * S::HALF;
            call_count = add_curve_intersections(
                &new_2_curves.0,
                curve1,
                &(domain2.start..domain2_mid),
                new_domain1,
                intersections,
                !flip,
                recursion_count,
                call_count,
                orig_curve2,
                orig_curve1,
            );
            call_count = add_curve_intersections(
                &new_2_curves.1,
                curve1,
                &(domain2_mid..domain2.end),
                new_domain1,
                intersections,
                !flip,
                recursion_count,
                call_count,
                orig_curve2,
                orig_curve1,
            );
        }
    } else {
        // Iterate.
        if domain2.end - domain2.start >= epsilon {
            call_count = add_curve_intersections(
                curve2,
                curve1,
                domain2,
                new_domain1,
                intersections,
                !flip,
                recursion_count,
                call_count,
                orig_curve2,
                orig_curve1,
            );
        } else {
            // The interval on curve2 is already tight enough, so just continue iterating on curve1.
            call_count = add_curve_intersections(
                curve1,
                curve2,
                new_domain1,
                domain2,
                intersections,
                flip,
                recursion_count,
                call_count,
                orig_curve1,
                orig_curve2,
            );
        }
    }

    call_count
}

fn add_point_curve_intersection<S: Scalar>(
    pt_curve: &CubicBezierSegment<S>,
    pt_curve_is_curve1: bool,
    curve: &CubicBezierSegment<S>,
    pt_domain: &Range<S>,
    curve_domain: &Range<S>,
    intersections: &mut ArrayVec<(S, S), 9>,
    flip: bool,
) {
    let pt = pt_curve.from;
    // We assume pt is curve1 when we add intersections below.
    let flip = if pt_curve_is_curve1 { flip } else { !flip };

    // Generally speaking |curve| will be quite small at this point, so see if we can get away with
    // just sampling here.

    let epsilon = epsilon_for_point(&pt);
    let pt_t = (pt_domain.start + pt_domain.end) * S::HALF;

    let curve_t = {
        let mut t_for_min = S::ZERO;
        let mut min_dist_sq = epsilon;
        let tenths = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        for &t in tenths.iter() {
            let t = S::value(t);
            let d = (pt - curve.sample(t)).square_length();
            if d < min_dist_sq {
                t_for_min = t;
                min_dist_sq = d;
            }
        }

        if min_dist_sq == epsilon {
            -S::ONE
        } else {
            let curve_t = domain_value_at_t(curve_domain, t_for_min);
            curve_t
        }
    };

    if curve_t != -S::ONE {
        add_intersection(pt_t, pt_curve, curve_t, curve, flip, intersections);
        return;
    }

    // If sampling didn't work, try a different approach.
    let results = point_curve_intersections(&pt, curve, epsilon);
    for t in results {
        let curve_t = domain_value_at_t(curve_domain, t);
        add_intersection(pt_t, pt_curve, curve_t, curve, flip, intersections);
    }
}

// TODO: replace with Scalar::epsilon_for?
// If we're comparing distances between samples of curves, our epsilon should depend on how big the
// points we're comparing are. This function returns an epsilon appropriate for the size of pt.
fn epsilon_for_point<S: Scalar>(pt: &Point<S>) -> S {
    let max = S::max(S::abs(pt.x), S::abs(pt.y));
    let epsilon = if inputs_are_f32::<S>() {
        match max.to_i32().unwrap() {
            0..=9 => S::value(0.001),
            10..=99 => S::value(0.01),
            100..=999 => S::value(0.1),
            1_000..=9_999 => S::value(0.25),
            10_000..=999_999 => S::HALF,
            _ => S::ONE,
        }
    } else {
        match max.to_i64().unwrap() {
            0..=99_999 => S::EPSILON,
            100_000..=99_999_999 => S::value(1e-5),
            100_000_000..=9_999_999_999 => S::value(1e-3),
            _ => S::value(1e-1),
        }
    };

    epsilon
}

fn add_intersection<S: Scalar>(
    t1: S,
    orig_curve1: &CubicBezierSegment<S>,
    t2: S,
    orig_curve2: &CubicBezierSegment<S>,
    flip: bool,
    intersections: &mut ArrayVec<(S, S), 9>,
) {
    let (t1, t2) = if flip { (t2, t1) } else { (t1, t2) };
    // (This should probably depend in some way on how large our input coefficients are.)
    let epsilon = if inputs_are_f32::<S>() {
        S::value(1e-3)
    } else {
        S::EPSILON
    };
    // Discard endpoint/endpoint intersections.
    let t1_is_an_endpoint = t1 < epsilon || t1 > S::ONE - epsilon;
    let t2_is_an_endpoint = t2 < epsilon || t2 > S::ONE - epsilon;
    if t1_is_an_endpoint && t2_is_an_endpoint {
        return;
    }

    // We can get repeated intersections when we split a curve at an intersection point, or when
    // two curves intersect at a point where the curves are very close together, or when the fat
    // line process breaks down.
    for i in 0..intersections.len() {
        let (old_t1, old_t2) = intersections[i];
        // f32 errors can be particularly bad (over a hundred) if we wind up keeping the "wrong"
        // duplicate intersection, so always keep the one that minimizes sample distance.
        if S::abs(t1 - old_t1) < epsilon && S::abs(t2 - old_t2) < epsilon {
            let cur_dist =
                (orig_curve1.sample(old_t1) - orig_curve2.sample(old_t2)).square_length();
            let new_dist = (orig_curve1.sample(t1) - orig_curve2.sample(t2)).square_length();
            if new_dist < cur_dist {
                intersections[i] = (t1, t2);
            }
            return;
        }
    }

    if intersections.len() < 9 {
        intersections.push((t1, t2));
    }
}

// Returns an interval (t_min, t_max) with the property that for parameter values outside that
// interval, curve1 is guaranteed to not intersect curve2; uses the fat line of curve2 as its basis
// for the guarantee. (See the Sederberg document for what's going on here.)
fn restrict_curve_to_fat_line<S: Scalar>(
    curve1: &CubicBezierSegment<S>,
    curve2: &CubicBezierSegment<S>,
) -> Option<(S, S)> {
    // TODO: Consider clipping against the perpendicular fat line as well (recommended by
    // Sederberg).
    // TODO: The current algorithm doesn't handle the (rare) case where curve1 and curve2 are
    // overlapping lines.

    let baseline2 = curve2.baseline().to_line().equation();

    let d_0 = baseline2.signed_distance_to_point(&curve1.from);
    let d_1 = baseline2.signed_distance_to_point(&curve1.ctrl1);
    let d_2 = baseline2.signed_distance_to_point(&curve1.ctrl2);
    let d_3 = baseline2.signed_distance_to_point(&curve1.to);

    let mut top = ArrayVec::new();
    let mut bottom = ArrayVec::new();
    convex_hull_of_distance_curve(d_0, d_1, d_2, d_3, &mut top, &mut bottom);
    let (d_min, d_max) = curve2.fat_line_min_max();

    clip_convex_hull_to_fat_line(&mut top, &mut bottom, d_min, d_max)
}

// Returns the convex hull of the curve that's the graph of the function
// t -> d(curve1(t), baseline(curve2)). The convex hull is described as a top and a bottom, where
// each of top and bottom is described by the list of its vertices from left to right (the number of
// vertices for each is variable).
fn convex_hull_of_distance_curve<S: Scalar>(
    d0: S,
    d1: S,
    d2: S,
    d3: S,
    top: &mut ArrayVec<Point<S>, 4>,
    bottom: &mut ArrayVec<Point<S>, 4>,
) {
    debug_assert!(top.is_empty());
    debug_assert!(bottom.is_empty());

    let p0 = point(S::ZERO, d0);
    let p1 = point(S::ONE / S::THREE, d1);
    let p2 = point(S::TWO / S::THREE, d2);
    let p3 = point(S::ONE, d3);
    // Compute the vertical signed distance of p1 and p2 from [p0, p3].
    let dist1 = d1 - (S::TWO * d0 + d3) / S::THREE;
    let dist2 = d2 - (d0 + S::TWO * d3) / S::THREE;

    // Compute the hull assuming p1 is on top - we'll switch later if needed.
    if dist1 * dist2 < S::ZERO {
        // p1 and p2 lie on opposite sides of [p0, p3], so the hull is a quadrilateral:
        let _ = top.try_extend_from_slice(&[p0, p1, p3]);
        let _ = bottom.try_extend_from_slice(&[p0, p2, p3]);
    } else {
        // p1 and p2 lie on the same side of [p0, p3]. The hull can be a triangle or a
        // quadrilateral, and [p0, p3] is part of the hull. The hull is a triangle if the vertical
        // distance of one of the middle points p1, p2 is <= half the vertical distance of the
        // other middle point.
        let dist1 = S::abs(dist1);
        let dist2 = S::abs(dist2);
        if dist1 >= S::TWO * dist2 {
            let _ = top.try_extend_from_slice(&[p0, p1, p3]);
            let _ = bottom.try_extend_from_slice(&[p0, p3]);
        } else if dist2 >= S::TWO * dist1 {
            let _ = top.try_extend_from_slice(&[p0, p2, p3]);
            let _ = bottom.try_extend_from_slice(&[p0, p3]);
        } else {
            let _ = top.try_extend_from_slice(&[p0, p1, p2, p3]);
            let _ = bottom.try_extend_from_slice(&[p0, p3]);
        }
    }

    // Flip the hull if needed:
    if dist1 < S::ZERO || (dist1 == S::ZERO && dist2 < S::ZERO) {
        mem::swap(top, bottom);
    }
}

// Returns the min and max values at which the convex hull enters the fat line min/max offset lines.
fn clip_convex_hull_to_fat_line<S: Scalar>(
    hull_top: &mut [Point<S>],
    hull_bottom: &mut [Point<S>],
    d_min: S,
    d_max: S,
) -> Option<(S, S)> {
    // Walk from the left corner of the convex hull until we enter the fat line limits:
    let t_clip_min = walk_convex_hull_start_to_fat_line(hull_top, hull_bottom, d_min, d_max)?;

    // Now walk from the right corner of the convex hull until we enter the fat line limits - to
    // walk right to left we just reverse the order of the hull vertices, so that hull_top and
    // hull_bottom start at the right corner now:
    hull_top.reverse();
    hull_bottom.reverse();
    let t_clip_max = walk_convex_hull_start_to_fat_line(hull_top, hull_bottom, d_min, d_max)?;

    Some((t_clip_min, t_clip_max))
}

// Walk the edges of the convex hull until you hit a fat line offset value, starting from the
// (first vertex in hull_top_vertices == first vertex in hull_bottom_vertices).
fn walk_convex_hull_start_to_fat_line<S: Scalar>(
    hull_top_vertices: &[Point<S>],
    hull_bottom_vertices: &[Point<S>],
    d_min: S,
    d_max: S,
) -> Option<S> {
    let start_corner = hull_top_vertices[0];

    if start_corner.y < d_min {
        walk_convex_hull_edges_to_fat_line(hull_top_vertices, true, d_min)
    } else if start_corner.y > d_max {
        walk_convex_hull_edges_to_fat_line(hull_bottom_vertices, false, d_max)
    } else {
        Some(start_corner.x)
    }
}

// Do the actual walking, starting from the first vertex of hull_vertices.
fn walk_convex_hull_edges_to_fat_line<S: Scalar>(
    hull_vertices: &[Point<S>],
    vertices_are_for_top: bool,
    threshold: S,
) -> Option<S> {
    for i in 0..hull_vertices.len() - 1 {
        let p = hull_vertices[i];
        let q = hull_vertices[i + 1];
        if (vertices_are_for_top && q.y >= threshold) || (!vertices_are_for_top && q.y <= threshold)
        {
            return if q.y == threshold {
                Some(q.x)
            } else {
                Some(p.x + (threshold - p.y) * (q.x - p.x) / (q.y - p.y))
            };
        }
    }
    // All points of the hull are outside the threshold:
    None
}

#[inline]
fn inputs_are_f32<S: Scalar>() -> bool {
    S::EPSILON > S::value(1e-6)
}

#[inline]
// Return the point of domain corresponding to the point t, 0 <= t <= 1.
fn domain_value_at_t<S: Scalar>(domain: &Range<S>, t: S) -> S {
    domain.start + (domain.end - domain.start) * t
}

#[inline]
// Box2D::intersects doesn't count edge/corner intersections, this version does.
fn rectangles_overlap<S: Scalar>(r1: &Box2D<S>, r2: &Box2D<S>) -> bool {
    r1.min.x <= r2.max.x && r2.min.x <= r1.max.x && r1.min.y <= r2.max.y && r2.min.y <= r1.max.y
}

#[cfg(test)]
fn do_test<S: Scalar>(
    curve1: &CubicBezierSegment<S>,
    curve2: &CubicBezierSegment<S>,
    intersection_count: i32,
) {
    do_test_once(curve1, curve2, intersection_count);
    do_test_once(curve2, curve1, intersection_count);
}

#[cfg(test)]
fn do_test_once<S: Scalar>(
    curve1: &CubicBezierSegment<S>,
    curve2: &CubicBezierSegment<S>,
    intersection_count: i32,
) {
    let intersections = cubic_bezier_intersections_t(curve1, curve2);
    for intersection in &intersections {
        let p1 = curve1.sample(intersection.0);
        let p2 = curve2.sample(intersection.1);
        check_dist(&p1, &p2);
    }

    assert_eq!(intersections.len() as i32, intersection_count);
}

#[cfg(test)]
fn check_dist<S: Scalar>(p1: &Point<S>, p2: &Point<S>) {
    let dist = S::sqrt((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y));
    if dist > S::HALF {
        assert!(false, "Intersection points too far apart.");
    }
}

#[test]
fn test_cubic_curve_curve_intersections() {
    do_test(
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(0.0, 1.0),
            ctrl2: point(0.0, 1.0),
            to: point(1.0, 1.0),
        },
        &CubicBezierSegment {
            from: point(0.0, 1.0),
            ctrl1: point(1.0, 1.0),
            ctrl2: point(1.0, 1.0),
            to: point(1.0, 0.0),
        },
        1,
    );
    do_test(
        &CubicBezierSegment {
            from: point(48.0f32, 84.0),
            ctrl1: point(104.0, 176.0),
            ctrl2: point(190.0, 37.0),
            to: point(121.0, 75.0),
        },
        &CubicBezierSegment {
            from: point(68.0, 145.0),
            ctrl1: point(74.0, 6.0),
            ctrl2: point(143.0, 197.0),
            to: point(138.0, 55.0),
        },
        4,
    );
    do_test(
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(0.5, 1.0),
            ctrl2: point(0.5, 1.0),
            to: point(1.0, 0.0),
        },
        &CubicBezierSegment {
            from: point(0.0, 1.0),
            ctrl1: point(0.5, 0.0),
            ctrl2: point(0.5, 0.0),
            to: point(1.0, 1.0),
        },
        2,
    );
    do_test(
        &CubicBezierSegment {
            from: point(0.2, 0.0),
            ctrl1: point(0.5, 3.0),
            ctrl2: point(0.5, -2.0),
            to: point(0.8, 1.0),
        },
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(2.5, 0.5),
            ctrl2: point(-1.5, 0.5),
            to: point(1.0, 0.0),
        },
        9,
    );

    // (A previous version of the code was returning two practically identical
    // intersection points here.)
    do_test(
        &CubicBezierSegment {
            from: point(718133.1363092018, 673674.987999388),
            ctrl1: point(-53014.13135835016, 286988.87959900266),
            ctrl2: point(-900630.1880107201, -7527.6889376943),
            to: point(417822.48349384824, -149039.14932848653),
        },
        &CubicBezierSegment {
            from: point(924715.3309247112, 719414.5221912428),
            ctrl1: point(965365.9679664494, -563421.3040676294),
            ctrl2: point(273552.85484064696, 643090.0890117711),
            to: point(-113963.134524995, 732017.9466050486),
        },
        1,
    );

    // On these curves the algorithm runs to a state at which the new clipped domain1 becomes a
    // point even though t_min_clip < t_max_clip (because domain1 was small enough to begin with
    // relative to the small distance between t_min_clip and t_max_clip), and the new curve1 is not
    // a point (it's split off the old curve1 using t_min_clip < t_max_clip).
    do_test(
        &CubicBezierSegment {
            from: point(423394.5967598548, -91342.7434613118),
            ctrl1: point(333212.450870987, 225564.45711810607),
            ctrl2: point(668108.668469816, -626100.8367380127),
            to: point(-481885.0610437216, 893767.5320803947),
        },
        &CubicBezierSegment {
            from: point(-484505.2601961801, -222621.44229855016),
            ctrl1: point(22432.829984141514, -944727.7102144773),
            ctrl2: point(-433294.66549074976, -168018.60431004688),
            to: point(567688.5977972192, 13975.09633399453),
        },
        3,
    );
}

#[test]
fn test_cubic_control_point_touching() {
    // After splitting the curve2 loop in half, curve1.ctrl1 (and only that
    // point) touches the curve2 fat line - make sure we don't accept that as an
    // intersection. [We're really only interested in the right half of the loop - the rest of the
    // loop is there just to get past an initial fast_bounding_box check.]
    do_test(
        &CubicBezierSegment {
            from: point(-1.0, 0.0),
            ctrl1: point(0.0, 0.0),
            ctrl2: point(-1.0, -0.1),
            to: point(-1.0, -0.1),
        },
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(5.0, -5.0),
            ctrl2: point(-5.0, -5.0),
            to: point(0.0, 0.0),
        },
        0,
    );
}

#[test]
fn test_cubic_self_intersections() {
    // Two self-intersecting curves intersecting at their self-intersections (the origin).
    do_test(
        &CubicBezierSegment {
            from: point(-10.0, -13.636363636363636),
            ctrl1: point(15.0, 11.363636363636363),
            ctrl2: point(-15.0, 11.363636363636363),
            to: point(10.0, -13.636363636363636),
        },
        &CubicBezierSegment {
            from: point(13.636363636363636, -10.0),
            ctrl1: point(-11.363636363636363, 15.0),
            ctrl2: point(-11.363636363636363, -15.0),
            to: point(13.636363636363636, 10.0),
        },
        4,
    );
}

#[test]
fn test_cubic_loops() {
    // This gets up to a recursion count of 53 trying to find (0.0, 0.0) and (1.0, 1.0) (which
    // aren't actually needed) - with the curves in the opposite order it gets up to 81!
    do_test(
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(-10.0, 10.0),
            ctrl2: point(10.0, 10.0),
            to: point(0.0, 0.0),
        },
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(-1.0, 1.0),
            ctrl2: point(1.0, 1.0),
            to: point(0.0, 0.0),
        },
        0,
    );

    do_test(
        &CubicBezierSegment {
            from: point(0.0f32, 0.0),
            ctrl1: point(-100.0, 0.0),
            ctrl2: point(-500.0, 500.0),
            to: point(0.0, 0.0),
        },
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(-800.0, 100.0),
            ctrl2: point(500.0, 500.0),
            to: point(0.0, 0.0),
        },
        3,
    );
}

#[test]
fn test_cubic_line_curve_intersections() {
    do_test(
        &CubicBezierSegment {
            /* line */
            from: point(1.0, 2.0),
            ctrl1: point(20.0, 1.0),
            ctrl2: point(1.0, 2.0),
            to: point(20.0, 1.0),
        },
        &CubicBezierSegment {
            from: point(1.0, 0.0),
            ctrl1: point(1.0, 5.0),
            ctrl2: point(20.0, 25.0),
            to: point(20.0, 0.0),
        },
        2,
    );

    do_test(
        &CubicBezierSegment {
            /* line */
            from: point(0.0f32, 0.0),
            ctrl1: point(-10.0, 0.0),
            ctrl2: point(20.0, 0.0),
            to: point(10.0, 0.0),
        },
        &CubicBezierSegment {
            from: point(-1.0, -1.0),
            ctrl1: point(0.0, 4.0),
            ctrl2: point(10.0, -4.0),
            to: point(11.0, 1.0),
        },
        5,
    );

    do_test(
        &CubicBezierSegment {
            from: point(-1.0, -2.0),
            ctrl1: point(-1.0, 8.0),
            ctrl2: point(1.0, -8.0),
            to: point(1.0, 2.0),
        },
        &CubicBezierSegment {
            /* line */
            from: point(-10.0, -10.0),
            ctrl1: point(20.0, 20.0),
            ctrl2: point(-20.0, -20.0),
            to: point(10.0, 10.0),
        },
        9,
    );
}

#[test]
fn test_cubic_line_line_intersections() {
    do_test(
        &CubicBezierSegment {
            from: point(-10.0, -10.0),
            ctrl1: point(20.0, 20.0),
            ctrl2: point(-20.0, -20.0),
            to: point(10.0, 10.0),
        },
        &CubicBezierSegment {
            from: point(-10.0, 10.0),
            ctrl1: point(20.0, -20.0),
            ctrl2: point(-20.0, 20.0),
            to: point(10.0, -10.0),
        },
        9,
    );

    // Overlapping line segments should return 0 intersections.
    do_test(
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(0.0, 0.0),
            ctrl2: point(10.0, 0.0),
            to: point(10.0, 0.0),
        },
        &CubicBezierSegment {
            from: point(5.0, 0.0),
            ctrl1: point(5.0, 0.0),
            ctrl2: point(15.0, 0.0),
            to: point(15.0, 0.0),
        },
        0,
    );
}

#[test]
// (This test used to fail on a previous version of the algorithm by returning an intersection close
// to (1.0, 0.0), but not close enough to consider it the same as (1.0, 0.0) - the curves are quite
// close at that endpoint.)
fn test_cubic_similar_loops() {
    do_test(
        &CubicBezierSegment {
            from: point(-0.281604145719379, -0.3129629924180608),
            ctrl1: point(-0.04393998118946163, 0.13714701102906668),
            ctrl2: point(0.4472584256288119, 0.2876115686206546),
            to: point(-0.281604145719379, -0.3129629924180608),
        },
        &CubicBezierSegment {
            from: point(-0.281604145719379, -0.3129629924180608),
            ctrl1: point(-0.1560415754252551, -0.22924729391648402),
            ctrl2: point(-0.9224550447067958, 0.19110227764357646),
            to: point(-0.281604145719379, -0.3129629924180608),
        },
        2,
    );
}

#[test]
// (A previous version of the algorithm returned an intersection close to (0.5, 0.5), but not close
// enough to be considered the same as (0.5, 0.5).)
fn test_cubic_no_duplicated_root() {
    do_test(
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(-10.0, 1.0),
            ctrl2: point(10.0, 1.0),
            to: point(0.0, 0.0),
        },
        &CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(-1.0, 1.0),
            ctrl2: point(1.0, 1.0),
            to: point(0.0, 0.0),
        },
        1,
    );
}

#[test]
fn test_cubic_glancing_intersection() {
    use std::panic;
    // The f64 version currently fails on a very close fat line miss after 57 recursions.
    let result = panic::catch_unwind(|| {
        do_test(
            &CubicBezierSegment {
                from: point(0.0, 0.0),
                ctrl1: point(0.0, 8.0),
                ctrl2: point(10.0, 8.0),
                to: point(10.0, 0.0),
            },
            &CubicBezierSegment {
                from: point(0.0, 12.0),
                ctrl1: point(0.0, 4.0),
                ctrl2: point(10.0, 4.0),
                to: point(10.0, 12.0),
            },
            1,
        );
    });
    assert!(result.is_err());

    let result = panic::catch_unwind(|| {
        do_test(
            &CubicBezierSegment {
                from: point(0.0f32, 0.0),
                ctrl1: point(0.0, 8.0),
                ctrl2: point(10.0, 8.0),
                to: point(10.0, 0.0),
            },
            &CubicBezierSegment {
                from: point(0.0, 12.0),
                ctrl1: point(0.0, 4.0),
                ctrl2: point(10.0, 4.0),
                to: point(10.0, 12.0),
            },
            1,
        );
    });
    assert!(result.is_err());
}

#[test]
fn test_cubic_duplicated_intersections() {
    use std::panic;
    let result = panic::catch_unwind(|| {
        // This finds an extra intersection (0.49530116, 0.74361485) - the actual, also found, is
        // (0.49633604, 0.74361396). Their difference is (−0.00103488, 0.00000089) - we consider
        // the two to be the same if both difference values are < 1e-3.
        do_test(
            &CubicBezierSegment {
                from: point(-33307.36f32, -1804.0625),
                ctrl1: point(-59259.727, 70098.31),
                ctrl2: point(98661.78, 48235.703),
                to: point(28422.234, 31845.219),
            },
            &CubicBezierSegment {
                from: point(-21501.133, 51935.344),
                ctrl1: point(-95301.96, 95031.45),
                ctrl2: point(-25882.242, -12896.75),
                to: point(94618.97, 94288.66),
            },
            2,
        );
    });
    assert!(result.is_err());
}

#[test]
fn test_cubic_endpoint_not_an_intersection() {
    // f32 curves seem to be somewhat prone to picking up not-an-intersections where an endpoint of
    // one curve is close to and points into the interior of the other curve, and both curves are
    // "mostly linear" on some level.
    use std::panic;
    let result = panic::catch_unwind(|| {
        do_test(
            &CubicBezierSegment {
                from: point(76868.875f32, 47679.28),
                ctrl1: point(65326.86, 856.21094),
                ctrl2: point(-85621.64, -80823.375),
                to: point(-56517.53, 28062.227),
            },
            &CubicBezierSegment {
                from: point(-67977.72, 77673.53),
                ctrl1: point(-59829.57, -41917.87),
                ctrl2: point(57.4375, 52822.97),
                to: point(51075.86, 85772.84),
            },
            0,
        );
    });
    assert!(result.is_err());
}

#[test]
// The endpoints of curve2 intersect the interior of curve1.
fn test_cubic_interior_endpoint() {
    do_test(
        &CubicBezierSegment {
            // Has its apex at 6.0.
            from: point(-5.0, 0.0),
            ctrl1: point(-5.0, 8.0),
            ctrl2: point(5.0, 8.0),
            to: point(5.0, 0.0),
        },
        &CubicBezierSegment {
            from: point(0.0, 6.0),
            ctrl1: point(-5.0, 0.0),
            ctrl2: point(5.0, 0.0),
            to: point(0.0, 6.0),
        },
        2,
    );
}

#[test]
fn test_cubic_point_curve_intersections() {
    let epsilon = 1e-5;
    {
        let curve1 = CubicBezierSegment {
            from: point(0.0, 0.0),
            ctrl1: point(0.0, 1.0),
            ctrl2: point(0.0, 1.0),
            to: point(1.0, 1.0),
        };
        let sample_t = 0.123456789;
        let pt = curve1.sample(sample_t);
        let curve2 = CubicBezierSegment {
            from: pt,
            ctrl1: pt,
            ctrl2: pt,
            to: pt,
        };
        let intersections = cubic_bezier_intersections_t(&curve1, &curve2);
        assert_eq!(intersections.len(), 1);
        let intersection_t = intersections[0].0;
        assert!(f64::abs(intersection_t - sample_t) < epsilon);
    }
    {
        let curve1 = CubicBezierSegment {
            from: point(-10.0, -13.636363636363636),
            ctrl1: point(15.0, 11.363636363636363),
            ctrl2: point(-15.0, 11.363636363636363),
            to: point(10.0, -13.636363636363636),
        };
        // curve1 has a self intersection at the following parameter values:
        let parameter1 = 0.7611164839335472;
        let parameter2 = 0.23888351606645375;
        let pt = curve1.sample(parameter1);
        let curve2 = CubicBezierSegment {
            from: pt,
            ctrl1: pt,
            ctrl2: pt,
            to: pt,
        };
        let intersections = cubic_bezier_intersections_t(&curve1, &curve2);
        assert_eq!(intersections.len(), 2);
        let intersection_t1 = intersections[0].0;
        let intersection_t2 = intersections[1].0;
        assert!(f64::abs(intersection_t1 - parameter1) < epsilon);
        assert!(f64::abs(intersection_t2 - parameter2) < epsilon);
    }
    {
        let epsilon = epsilon as f32;
        let curve1 = CubicBezierSegment {
            from: point(0.0f32, 0.0),
            ctrl1: point(50.0, 50.0),
            ctrl2: point(-50.0, -50.0),
            to: point(10.0, 10.0),
        };
        // curve1 is a line that passes through (5.0, 5.0) three times:
        let parameter1 = 0.96984464;
        let parameter2 = 0.037427425;
        let parameter3 = 0.44434106;
        let pt = curve1.sample(parameter1);
        let curve2 = CubicBezierSegment {
            from: pt,
            ctrl1: pt,
            ctrl2: pt,
            to: pt,
        };
        let intersections = cubic_bezier_intersections_t(&curve1, &curve2);
        assert_eq!(intersections.len(), 3);
        let intersection_t1 = intersections[0].0;
        let intersection_t2 = intersections[1].0;
        let intersection_t3 = intersections[2].0;
        assert!(f32::abs(intersection_t1 - parameter1) < epsilon);
        assert!(f32::abs(intersection_t2 - parameter2) < epsilon);
        assert!(f32::abs(intersection_t3 - parameter3) < epsilon);
    }
}

#[test]
fn test_cubic_subcurve_intersections() {
    let curve1 = CubicBezierSegment {
        from: point(0.0, 0.0),
        ctrl1: point(0.0, 1.0),
        ctrl2: point(0.0, 1.0),
        to: point(1.0, 1.0),
    };
    let curve2 = curve1.split_range(0.25..0.75);
    // The algorithm will find as many intersections as you let it, basically - make sure we're
    // not allowing too many intersections to be added, and not crashing on out of resources.
    do_test(&curve1, &curve2, 9);
}

#[test]
fn test_cubic_result_distance() {
    // In a previous version this used to return an intersection pair (0.17933762, 0.23783168),
    // close to an actual intersection, where the sampled intersection points on respective curves
    // were at distance 160.08488. The point here is that the old extra intersection was the result
    // of an anomalous fat line calculation, in other words an actual error, not just a "not quite
    // computationally close enough" error.
    do_test(
        &CubicBezierSegment {
            from: point(5893.133f32, -51377.152),
            ctrl1: point(-94403.984, 37668.156),
            ctrl2: point(-58914.684, 30339.195),
            to: point(4895.875, 83473.3),
        },
        &CubicBezierSegment {
            from: point(-51523.734, 75047.05),
            ctrl1: point(-58162.76, -91093.875),
            ctrl2: point(82137.516, -59844.35),
            to: point(46856.406, 40479.64),
        },
        3,
    );
}
