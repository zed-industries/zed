//! Segment computation and linking.
//!
//! A segment is a series of at least two consecutive points that are
//! appropriately aligned along a coordinate axis.
//!
//! The linking stage associates pairs of segments to form stems and
//! identifies serifs with a post-process pass.

use super::super::{
    derived_constant,
    metrics::fixed_div,
    outline::Outline,
    style::ScriptGroup,
    topo::{Axis, Dimension, Segment},
};
use raw::tables::glyf::PointFlags;

// Bounds for score, position and coordinate values.
// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1598>
const MAX_SCORE: i32 = 32000;
const MIN_SCORE: i32 = -32000;

/// Computes segments for the Latin writing system.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1537>
pub(crate) fn compute_segments(
    outline: &mut Outline,
    axis: &mut Axis,
    _group: ScriptGroup,
) -> bool {
    assign_point_uvs(outline, axis.dim);
    if !build_segments(outline, axis) {
        return false;
    }
    adjust_segment_heights(outline, axis);
    // This is never actually executed due to a bug in FreeType
    // See point 2 at <https://github.com/googlefonts/fontations/issues/1129>
    // if group != ScriptGroup::Default {
    //     _detect_round_segments_cjk(outline, axis);
    // }
    true
}

/// Link segments to form stems and serifs.
///
/// If `max_width` is provided, use it to refine the scoring function.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1990>
pub(crate) fn link_segments(
    outline: &Outline,
    axis: &mut Axis,
    scale: i32,
    group: ScriptGroup,
    max_width: Option<i32>,
) {
    if group == ScriptGroup::Default {
        link_segments_default(outline, axis, max_width);
    } else {
        link_segments_cjk(outline, axis, scale)
    }
}

/// Link segments to form stems and serifs.
///
/// If `max_width` is provided, use it to refine the scoring function.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1990>
fn link_segments_default(outline: &Outline, axis: &mut Axis, max_width: Option<i32>) {
    let max_width = max_width.unwrap_or_default();
    // Heuristic value to set up a minimum for overlapping
    let len_threshold = derived_constant(outline.units_per_em, 8).max(1);
    // Heuristic value to weight lengths
    let len_score = derived_constant(outline.units_per_em, 6000);
    // Heuristic value to weight distances (not a latin constant since
    // it works on multiples of stem width)
    let dist_score = 3000;
    // Compare each segment to the others.. O(n^2)
    let segments = axis.segments.as_mut_slice();
    for ix1 in 0..segments.len() {
        let seg1 = segments[ix1];
        if seg1.dir != axis.major_dir {
            continue;
        }
        let pos1 = seg1.pos as i32;
        // Search for stems having opposite directions with seg1 to the
        // "left" of seg2
        for ix2 in 0..segments.len() {
            let seg1 = segments[ix1];
            let seg2 = segments[ix2];
            let pos2 = seg2.pos as i32;
            if seg1.dir.is_opposite(seg2.dir) && pos2 > pos1 {
                // Compute distance between the segments
                // Note: the min/max functions chosen here are intentional
                let min = seg1.min_coord.max(seg2.min_coord) as i32;
                let max = seg1.max_coord.min(seg2.max_coord) as i32;
                // Compute maximum coordinate difference or how much they
                // overlap
                let len = max - min;
                if len >= len_threshold {
                    // verbatim from FreeType:
                    // "The score is the sum of two demerits indicating the
                    //  `badness' of a fit, measured along the segments' main axis
                    //  and orthogonal to it, respectively.
                    //
                    // - The less overlapping along the main axis, the worse it
                    //   is, causing a larger demerit.
                    //
                    // - The nearer the orthogonal distance to a stem width, the
                    //   better it is, causing a smaller demerit.  For simplicity,
                    //   however, we only increase the demerit for values that
                    //   exceed the largest stem width."
                    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L2054>
                    let dist = pos2 - pos1;
                    let dist_demerit = if max_width != 0 {
                        // Distance demerits are based on multiples of max_width
                        let delta = (dist << 10) / max_width - (1 << 10);
                        if delta > 10_000 {
                            MAX_SCORE
                        } else if delta > 0 {
                            delta * delta / dist_score
                        } else {
                            0
                        }
                    } else {
                        dist
                    };
                    let score = dist_demerit + len_score / len;
                    if score < seg1.score {
                        let seg1 = &mut segments[ix1];
                        seg1.score = score;
                        seg1.link_ix = Some(ix2 as u16);
                    }
                    if score < seg2.score {
                        let seg2 = &mut segments[ix2];
                        seg2.score = score;
                        seg2.link_ix = Some(ix1 as u16);
                    }
                }
            }
        }
    }
    // Now compute "serif" segments
    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L2109>
    for ix1 in 0..segments.len() {
        let Some(ix2) = segments[ix1].link_ix else {
            continue;
        };
        let seg2_link = segments[ix2 as usize].link_ix;
        if seg2_link != Some(ix1 as u16) {
            let seg1 = &mut segments[ix1];
            seg1.link_ix = None;
            seg1.serif_ix = seg2_link;
        }
    }
}

/// Link segments to form stems and serifs for the CJK script group.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afcjk.c#L848>
fn link_segments_cjk(outline: &Outline, axis: &mut Axis, scale: i32) {
    // Heuristic value to set up a minimum for overlapping
    let len_threshold = derived_constant(outline.units_per_em, 8);
    let dist_threshold = fixed_div(64 * 3, scale);
    // Compare each segment to the others.. O(n^2)
    let segments = axis.segments.as_mut_slice();
    for ix1 in 0..segments.len() {
        let seg1 = segments[ix1];
        if seg1.dir != axis.major_dir {
            continue;
        }
        let pos1 = seg1.pos as i32;
        // Search for stems having opposite directions with seg1 to the
        // "left" of seg2
        for ix2 in 0..segments.len() {
            let seg1 = segments[ix1];
            let seg2 = segments[ix2];
            if ix1 == ix2 || !seg1.dir.is_opposite(seg2.dir) {
                continue;
            }
            let pos2 = seg2.pos as i32;
            let dist = pos2 - pos1;
            if dist < 0 {
                continue;
            }
            // Compute distance between the segments
            // Note: the min/max functions chosen here are intentional
            let min = seg1.min_coord.max(seg2.min_coord) as i32;
            let max = seg1.max_coord.min(seg2.max_coord) as i32;
            // Compute maximum coordinate difference or how much they
            // overlap
            let len = max - min;
            if len >= len_threshold {
                let check_seg = |seg: &Segment| {
                    // Some more magic heuristics...
                    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afcjk.c#L896>
                    (dist * 8 < seg.score * 9) && (dist * 8 < seg.score * 7 || seg.len < len)
                };
                if check_seg(&seg1) {
                    let seg = &mut segments[ix1];
                    seg.score = dist;
                    seg.len = len;
                    seg.link_ix = Some(ix2 as _);
                }
                if check_seg(&seg2) {
                    let seg = &mut segments[ix2];
                    seg.score = dist;
                    seg.len = len;
                    seg.link_ix = Some(ix1 as _);
                }
            }
        }
    }
    // Now compute "serif" segments
    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afcjk.c#L917>
    for ix1 in 0..segments.len() {
        let seg1 = segments[ix1];
        if seg1.score >= dist_threshold {
            continue;
        }
        let Some(link1) = seg1.link(segments).copied() else {
            continue;
        };
        // Unwrap is fine because we checked for existence above
        let link1_ix = seg1.link_ix.unwrap() as usize;
        if link1.link_ix != Some(ix1 as u16) || link1.pos <= seg1.pos {
            continue;
        }
        for ix2 in 0..segments.len() {
            let seg2 = segments[ix2];
            if seg2.pos > seg1.pos || ix1 == ix2 {
                continue;
            }
            let Some(link2) = seg2.link(segments).copied() else {
                continue;
            };
            if link2.link_ix != Some(ix2 as u16) || link2.pos < link1.pos {
                continue;
            }
            if seg1.pos == seg2.pos && link1.pos == link2.pos {
                continue;
            }
            if seg2.score <= seg1.score || seg1.score * 4 <= seg2.score {
                continue;
            }
            if seg1.len >= seg2.len * 3 {
                // Again, we definitely have a valid link2
                let link2_ix = seg2.link_ix.unwrap() as usize;
                for seg in segments.iter_mut() {
                    let link_ix = seg.link_ix;
                    if link_ix == Some(ix2 as u16) {
                        seg.link_ix = None;
                        seg.serif_ix = Some(link1_ix as u16);
                    } else if link_ix == Some(link2_ix as u16) {
                        seg.link_ix = None;
                        seg.serif_ix = Some(ix1 as u16);
                    }
                }
            } else {
                segments[ix1].link_ix = None;
                segments[link1_ix].link_ix = None;
                break;
            }
        }
    }
    for ix1 in 0..segments.len() {
        let seg1 = segments[ix1];
        let Some(seg2) = seg1.link(segments).copied() else {
            continue;
        };
        if seg2.link_ix != Some(ix1 as u16) {
            segments[ix1].link_ix = None;
            if seg2.score < dist_threshold || seg1.score < seg2.score * 4 {
                segments[ix1].serif_ix = seg2.link_ix;
            }
        }
    }
}

/// Set the (u, v) values to font unit coords for each point depending
/// on the axis dimension.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1562>
fn assign_point_uvs(outline: &mut Outline, dim: Dimension) {
    if dim == Axis::HORIZONTAL {
        for point in &mut outline.points {
            point.u = point.fx;
            point.v = point.fy;
        }
    } else {
        for point in &mut outline.points {
            point.u = point.fy;
            point.v = point.fx;
        }
    }
}

/// Build the set of segments for each contour.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1588>
fn build_segments(outline: &mut Outline, axis: &mut Axis) -> bool {
    let flat_threshold = outline.units_per_em / 14;
    axis.segments.clear();
    let major_dir = axis.major_dir.normalize();
    let mut segment_dir = major_dir;
    let points = outline.points.as_mut_slice();
    for contour in &outline.contours {
        let is_single_point_contour = contour.range().len() == 1;
        let mut point_ix = contour.first();
        let mut last_ix = contour.prev(point_ix);
        let mut state = State::default();
        let mut prev_state = state;
        let mut prev_segment_ix: Option<usize> = None;
        let mut segment_ix = 0;
        // Check if we're starting on an edge and if so, find
        // the starting point
        if points[point_ix].out_dir.is_same_axis(major_dir)
            && points[last_ix].out_dir.is_same_axis(major_dir)
        {
            last_ix = point_ix;
            loop {
                point_ix = contour.prev(point_ix);
                if !points[point_ix].out_dir.is_same_axis(major_dir) {
                    point_ix = contour.next(point_ix);
                    break;
                }
                if point_ix == last_ix {
                    break;
                }
            }
        }
        last_ix = point_ix;
        let mut on_edge = false;
        let mut passed = false;
        loop {
            if on_edge {
                // Get min and max position
                let point = points[point_ix];
                state.min_pos = state.min_pos.min(point.u);
                state.max_pos = state.max_pos.max(point.u);
                // Get min and max coordinate and flags
                let v = point.v;
                if v < state.min_coord {
                    state.min_coord = v;
                    state.min_flags = point.flags;
                }
                if v > state.max_coord {
                    state.max_coord = v;
                    state.max_flags = point.flags;
                }
                // Get min and max coord of on curve points
                if point.is_on_curve() {
                    state.min_on_coord = state.min_on_coord.min(point.v);
                    state.max_on_coord = state.max_on_coord.max(point.v);
                }
                if point.out_dir != segment_dir || point_ix == last_ix {
                    if prev_segment_ix.is_none()
                        || axis.segments[segment_ix].first_ix
                            != axis.segments[prev_segment_ix.unwrap()].last_ix
                    {
                        // The points are different signifying that we are
                        // leaving an edge, so create a new segment
                        let segment = &mut axis.segments[segment_ix];
                        segment.last_ix = point_ix as u16;
                        state.apply_to_segment(segment, flat_threshold);
                        prev_segment_ix = Some(segment_ix);
                        prev_state = state;
                    } else {
                        // The points are the same, so merge the segments
                        let prev_segment = &mut axis.segments[prev_segment_ix.unwrap()];
                        if prev_segment.last_point(points).in_dir == point.in_dir {
                            // We have identical directions; unify segments
                            // and update constraints
                            state.min_pos = prev_state.min_pos.min(state.min_pos);
                            state.max_pos = prev_state.max_pos.max(state.max_pos);
                            if prev_state.min_coord < state.min_coord {
                                state.min_coord = prev_state.min_coord;
                                state.min_flags = prev_state.min_flags;
                            }
                            if prev_state.max_coord > state.max_coord {
                                state.max_coord = prev_state.max_coord;
                                state.max_flags = prev_state.max_flags;
                            }
                            state.min_on_coord = prev_state.min_on_coord.min(state.min_on_coord);
                            state.max_on_coord = prev_state.max_on_coord.max(state.max_on_coord);
                            prev_segment.last_ix = point_ix as u16;
                            state.apply_to_segment(prev_segment, flat_threshold);
                        } else {
                            // We have different directions; use the
                            // properties of the longer segment
                            if (prev_state.max_coord - prev_state.min_coord).abs()
                                > (state.max_coord - state.min_coord).abs()
                            {
                                // Discard current segment
                                prev_state.min_pos = prev_state.min_pos.min(state.min_pos);
                                prev_state.max_pos = prev_state.max_pos.max(state.max_pos);
                                prev_segment.last_ix = point_ix as u16;
                                prev_segment.pos =
                                    ((prev_state.min_pos + prev_state.max_pos) >> 1) as i16;
                                prev_segment.delta =
                                    ((prev_state.max_pos - prev_state.min_pos) >> 1) as i16;
                            } else {
                                // Discard previous segment
                                state.min_pos = state.min_pos.min(prev_state.min_pos);
                                state.max_pos = state.max_pos.max(prev_state.max_pos);
                                let mut segment = axis.segments[segment_ix];
                                segment.last_ix = point_ix as u16;
                                state.apply_to_segment(&mut segment, flat_threshold);
                                axis.segments[prev_segment_ix.unwrap()] = segment;
                                prev_state = state;
                            }
                        }
                        axis.segments.pop();
                    }
                    on_edge = false;
                }
            }
            if point_ix == last_ix {
                if passed {
                    break;
                }
                passed = true;
            }
            let point = points[point_ix];
            if !on_edge && (point.out_dir.is_same_axis(major_dir) || is_single_point_contour) {
                if axis.segments.len() > 1000 {
                    axis.segments.clear();
                    return false;
                }
                segment_ix = axis.segments.len();
                segment_dir = point.out_dir;
                let mut segment = Segment {
                    dir: segment_dir,
                    first_ix: point_ix as u16,
                    last_ix: point_ix as u16,
                    score: MAX_SCORE,
                    ..Default::default()
                };
                state.min_pos = point.u;
                state.max_pos = point.u;
                state.min_coord = point.v;
                state.max_coord = point.v;
                state.min_flags = point.flags;
                state.max_flags = point.flags;
                if !point.is_on_curve() {
                    state.min_on_coord = MAX_SCORE;
                    state.max_on_coord = MIN_SCORE;
                } else {
                    state.min_on_coord = point.v;
                    state.max_on_coord = point.v;
                }
                on_edge = true;
                if is_single_point_contour {
                    segment.pos = state.min_pos as i16;
                    if !point.is_on_curve() {
                        segment.flags |= Segment::ROUND;
                    }
                    segment.min_coord = point.v as i16;
                    segment.max_coord = point.v as i16;
                    segment.height = 0;
                    on_edge = false;
                }
                axis.segments.push(segment);
            }
            point_ix = contour.next(point_ix);
        }
    }
    true
}

/// Slightly increase the height of segments when it makes sense to better
/// detect and ignore serifs.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1933>
fn adjust_segment_heights(outline: &mut Outline, axis: &mut Axis) {
    let points = outline.points.as_slice();
    for segment in &mut axis.segments {
        let first = segment.first_point(points);
        let last = segment.last_point(points);
        fn adjust_height(segment: &mut Segment, v1: i32, v2: i32) {
            segment.height = (segment.height as i32 + ((v1 - v2) >> 1)) as i16;
        }
        let prev = &points[first.prev()];
        let next = &points[last.next()];
        if first.v < last.v {
            if prev.v < first.v {
                adjust_height(segment, first.v, prev.v);
            }
            if next.v > last.v {
                adjust_height(segment, next.v, last.v);
            }
        } else {
            if prev.v > first.v {
                adjust_height(segment, prev.v, first.v);
            }
            if next.v < last.v {
                adjust_height(segment, last.v, next.v);
            }
        }
    }
}

/// Performs the additional step of detecting round segments for the CJK script
/// group.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afcjk.c#L818>
fn _detect_round_segments_cjk(outline: &mut Outline, axis: &mut Axis) {
    let points = outline.points.as_slice();
    // A segment is considered round if it doesn't have successive on-curve
    // points
    for segment in &mut axis.segments {
        segment.flags &= !Segment::ROUND;
        let mut point_ix = segment.first();
        let last_ix = segment.last();
        let first_point = &points[point_ix];
        let mut is_prev_on_curve = first_point.is_on_curve();
        point_ix = first_point.next();
        loop {
            let point = &points[point_ix];
            let is_on_curve = point.is_on_curve();
            if is_prev_on_curve && is_on_curve {
                // Two on-curves in a row means we're not a round segment
                break;
            }
            is_prev_on_curve = is_on_curve;
            point_ix = point.next();
            if point_ix == last_ix {
                // We've reached the last point without two successive
                // on-curves so we're round
                segment.flags |= Segment::ROUND;
                break;
            }
        }
    }
}

/// Capture current and previous state while computing segments.
///
/// Values measured along a segment (point.v) are called "coordinates" and
/// values orthogonal to it (point.u) are called "positions"
#[derive(Copy, Clone)]
struct State {
    min_pos: i32,
    max_pos: i32,
    min_coord: i32,
    max_coord: i32,
    min_flags: PointFlags,
    max_flags: PointFlags,
    min_on_coord: i32,
    max_on_coord: i32,
}

impl Default for State {
    fn default() -> Self {
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/aflatin.c#L1598>
        Self {
            min_pos: MAX_SCORE,
            max_pos: MIN_SCORE,
            min_coord: MAX_SCORE,
            max_coord: MIN_SCORE,
            min_flags: PointFlags::default(),
            max_flags: PointFlags::default(),
            min_on_coord: MAX_SCORE,
            max_on_coord: MIN_SCORE,
        }
    }
}

impl State {
    fn apply_to_segment(&self, segment: &mut Segment, flat_threshold: i32) {
        segment.pos = ((self.min_pos + self.max_pos) >> 1) as i16;
        segment.delta = ((self.max_pos - self.min_pos) >> 1) as i16;
        // A segment is round if either end point is a
        // control and the length of the on points in
        // between fits within a heuristic limit.
        if (!self.min_flags.is_on_curve() || !self.max_flags.is_on_curve())
            && (self.max_on_coord - self.min_on_coord) < flat_threshold
        {
            segment.flags |= Segment::ROUND;
        }
        segment.min_coord = self.min_coord as i16;
        segment.max_coord = self.max_coord as i16;
        segment.height = segment.max_coord - segment.min_coord;
    }
}

#[cfg(test)]
mod tests {
    use super::{super::super::outline::Direction, *};
    use crate::MetadataProvider;
    use raw::{types::GlyphId, FontRef};

    #[test]
    fn horizontal_segments() {
        let font = FontRef::new(font_test_data::NOTOSERIFHEBREW_AUTOHINT_METRICS).unwrap();
        let glyphs = font.outline_glyphs();
        let glyph = glyphs.get(GlyphId::new(8)).unwrap();
        let mut outline = Outline::default();
        outline.fill(&glyph, Default::default()).unwrap();
        let mut axis = Axis::new(Axis::HORIZONTAL, outline.orientation);
        compute_segments(&mut outline, &mut axis, ScriptGroup::Default);
        link_segments(&outline, &mut axis, 0, ScriptGroup::Default, None);
        let segments = retain_segment_test_fields(&axis.segments);
        let expected = [
            Segment {
                flags: 0,
                dir: Direction::Up,
                pos: 55,
                delta: 0,
                min_coord: 26,
                max_coord: 360,
                height: 372,
                link_ix: Some(3),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Up,
                pos: 112,
                delta: 0,
                min_coord: 481,
                max_coord: 504,
                height: 34,
                link_ix: Some(2),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Down,
                pos: 168,
                delta: 0,
                min_coord: 483,
                max_coord: 504,
                height: 26,
                link_ix: Some(1),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Down,
                pos: 109,
                delta: 0,
                min_coord: 109,
                max_coord: 366,
                height: 288,
                link_ix: Some(0),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Up,
                pos: 453,
                delta: 0,
                min_coord: 169,
                max_coord: 432,
                height: 304,
                link_ix: Some(7),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 1,
                dir: Direction::Up,
                pos: 62,
                delta: 0,
                min_coord: 517,
                max_coord: 566,
                height: 76,
                link_ix: None,
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 1,
                dir: Direction::Down,
                pos: 103,
                delta: 0,
                min_coord: 619,
                max_coord: 647,
                height: 41,
                link_ix: None,
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Down,
                pos: 507,
                delta: 0,
                min_coord: 40,
                max_coord: 485,
                height: 498,
                link_ix: Some(4),
                serif_ix: None,
                ..Default::default()
            },
        ];
        assert_eq!(segments, &expected);
    }

    #[test]
    fn vertical_segments() {
        let font = FontRef::new(font_test_data::NOTOSERIFHEBREW_AUTOHINT_METRICS).unwrap();
        let glyphs = font.outline_glyphs();
        let glyph = glyphs.get(GlyphId::new(8)).unwrap();
        let mut outline = Outline::default();
        outline.fill(&glyph, Default::default()).unwrap();
        let mut axis = Axis::new(Axis::VERTICAL, outline.orientation);
        compute_segments(&mut outline, &mut axis, ScriptGroup::Default);
        link_segments(&outline, &mut axis, 0, ScriptGroup::Default, None);
        let segments = retain_segment_test_fields(&axis.segments);
        let expected = [
            Segment {
                flags: 0,
                dir: Direction::Left,
                pos: 0,
                delta: 0,
                min_coord: 85,
                max_coord: 470,
                height: 418,
                link_ix: Some(2),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Right,
                pos: 504,
                delta: 0,
                min_coord: 112,
                max_coord: 168,
                height: 56,
                link_ix: Some(3),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Right,
                pos: 109,
                delta: 0,
                min_coord: 109,
                max_coord: 427,
                height: 327,
                link_ix: Some(0),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Left,
                pos: 483,
                delta: 0,
                min_coord: 86,
                max_coord: 400,
                height: 352,
                link_ix: Some(1),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Right,
                pos: 647,
                delta: 0,
                min_coord: 76,
                max_coord: 103,
                height: 29,
                link_ix: None,
                serif_ix: Some(1),
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Right,
                pos: 592,
                delta: 0,
                min_coord: 131,
                max_coord: 437,
                height: 346,
                link_ix: None,
                serif_ix: Some(1),
                ..Default::default()
            },
        ];
        assert_eq!(segments, &expected);
    }

    #[test]
    fn cjk_horizontal_segments() {
        let font = FontRef::new(font_test_data::NOTOSERIFTC_AUTOHINT_METRICS).unwrap();
        let glyphs = font.outline_glyphs();
        let glyph = glyphs.get(GlyphId::new(9)).unwrap();
        let mut outline = Outline::default();
        outline.fill(&glyph, Default::default()).unwrap();
        let mut axis = Axis::new(Axis::HORIZONTAL, outline.orientation);
        compute_segments(&mut outline, &mut axis, ScriptGroup::Cjk);
        link_segments(&outline, &mut axis, 67109, ScriptGroup::Cjk, None);
        let segments = retain_segment_test_fields(&axis.segments);
        let expected = [
            Segment {
                flags: 0,
                dir: Direction::Down,
                pos: 731,
                delta: 0,
                min_coord: 155,
                max_coord: 676,
                height: 524,
                link_ix: Some(1),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Up,
                pos: 670,
                delta: 0,
                min_coord: 133,
                max_coord: 712,
                height: 579,
                link_ix: Some(0),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Down,
                pos: 458,
                delta: 0,
                min_coord: 741,
                max_coord: 757,
                height: 88,
                link_ix: None,
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Down,
                pos: 911,
                delta: 0,
                min_coord: -9,
                max_coord: 791,
                height: 821,
                link_ix: Some(5),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Up,
                pos: 693,
                delta: 0,
                min_coord: -7,
                max_coord: 9,
                height: 18,
                link_ix: None,
                serif_ix: Some(5),
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Up,
                pos: 849,
                delta: 0,
                min_coord: 11,
                max_coord: 829,
                height: 823,
                link_ix: Some(3),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Down,
                pos: 569,
                delta: 0,
                min_coord: 547,
                max_coord: 576,
                height: 29,
                link_ix: None,
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Down,
                pos: 201,
                delta: 0,
                min_coord: -57,
                max_coord: 540,
                height: 599,
                link_ix: Some(8),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Up,
                pos: 138,
                delta: 0,
                min_coord: -78,
                max_coord: 543,
                height: 640,
                link_ix: Some(7),
                serif_ix: None,
                ..Default::default()
            },
        ];
        assert_eq!(segments, &expected);
    }

    #[test]
    fn cjk_vertical_segments() {
        let font = FontRef::new(font_test_data::NOTOSERIFTC_AUTOHINT_METRICS).unwrap();
        let glyphs = font.outline_glyphs();
        let glyph = glyphs.get(GlyphId::new(9)).unwrap();
        let mut outline = Outline::default();
        outline.fill(&glyph, Default::default()).unwrap();
        let mut axis = Axis::new(Axis::VERTICAL, outline.orientation);
        compute_segments(&mut outline, &mut axis, ScriptGroup::Cjk);
        link_segments(&outline, &mut axis, 67109, ScriptGroup::Cjk, None);
        let segments = retain_segment_test_fields(&axis.segments);
        let expected = [
            Segment {
                flags: 0,
                dir: Direction::Right,
                pos: 758,
                delta: 0,
                min_coord: 280,
                max_coord: 545,
                height: 288,
                link_ix: Some(1),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Left,
                pos: 729,
                delta: 0,
                min_coord: 288,
                max_coord: 674,
                height: 391,
                link_ix: Some(0),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 1,
                dir: Direction::Left,
                pos: 133,
                delta: 0,
                min_coord: 670,
                max_coord: 693,
                height: 34,
                link_ix: None,
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Right,
                pos: 757,
                delta: 0,
                min_coord: 393,
                max_coord: 458,
                height: 70,
                link_ix: None,
                serif_ix: Some(0),
                ..Default::default()
            },
            Segment {
                flags: 1,
                dir: Direction::Right,
                pos: 3,
                delta: 2,
                min_coord: 727,
                max_coord: 838,
                height: 133,
                link_ix: None,
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Right,
                pos: 576,
                delta: 0,
                min_coord: 397,
                max_coord: 569,
                height: 177,
                link_ix: Some(7),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Left,
                pos: 547,
                delta: 0,
                min_coord: 387,
                max_coord: 569,
                height: 182,
                link_ix: None,
                serif_ix: Some(7),
                ..Default::default()
            },
            Segment {
                flags: 0,
                dir: Direction::Left,
                pos: 576,
                delta: 0,
                min_coord: 536,
                max_coord: 546,
                height: 10,
                link_ix: Some(5),
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 1,
                dir: Direction::Left,
                pos: -78,
                delta: 0,
                min_coord: 138,
                max_coord: 161,
                height: 34,
                link_ix: None,
                serif_ix: None,
                ..Default::default()
            },
            Segment {
                flags: 1,
                dir: Direction::Left,
                pos: 788,
                delta: 0,
                min_coord: 262,
                max_coord: 294,
                height: 46,
                link_ix: None,
                serif_ix: None,
                ..Default::default()
            },
        ];
        assert_eq!(segments, &expected);
    }

    // Retain the fields that are valid and comparable after
    // the segment pass.
    fn retain_segment_test_fields(segments: &[Segment]) -> Vec<Segment> {
        segments
            .iter()
            .map(|segment| Segment {
                flags: segment.flags,
                dir: segment.dir,
                pos: segment.pos,
                delta: segment.delta,
                min_coord: segment.min_coord,
                max_coord: segment.max_coord,
                height: segment.height,
                link_ix: segment.link_ix,
                serif_ix: segment.serif_ix,
                ..Default::default()
            })
            .collect()
    }
}
