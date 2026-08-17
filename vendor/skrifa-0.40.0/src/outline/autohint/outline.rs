//! Outline representation and helpers for autohinting.

use super::{
    super::{
        path,
        pen::PathStyle,
        unscaled::{UnscaledOutlineSink, UnscaledPoint},
        DrawError, LocationRef, OutlineGlyph, OutlinePen,
    },
    metrics::Scale,
};
use crate::collections::SmallVec;
use core::ops::Range;
use raw::{
    tables::glyf::{PointFlags, PointMarker},
    types::{F26Dot6, F2Dot14},
};

/// Hinting directions.
///
/// The values are such that `dir1 + dir2 == 0` when the directions are
/// opposite.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.h#L45>
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i8)]
pub(crate) enum Direction {
    #[default]
    None = 4,
    Right = 1,
    Left = -1,
    Up = 2,
    Down = -2,
}

impl Direction {
    /// Computes a direction from a vector.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.c#L751>
    pub fn new(dx: i32, dy: i32) -> Self {
        let (dir, long_arm, short_arm) = if dy >= dx {
            if dy >= -dx {
                (Direction::Up, dy, dx)
            } else {
                (Direction::Left, -dx, dy)
            }
        } else if dy >= -dx {
            (Direction::Right, dx, dy)
        } else {
            (Direction::Down, -dy, dx)
        };
        // Return no direction if arm lengths do not differ enough.
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.c#L789>
        if long_arm <= 14 * short_arm.abs() {
            Direction::None
        } else {
            dir
        }
    }

    pub fn is_opposite(self, other: Self) -> bool {
        self as i8 + other as i8 == 0
    }

    pub fn is_same_axis(self, other: Self) -> bool {
        (self as i8).abs() == (other as i8).abs()
    }

    pub fn normalize(self) -> Self {
        // FreeType uses absolute value for this.
        match self {
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            _ => self,
        }
    }
}

/// The overall orientation of an outline.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum Orientation {
    Clockwise,
    CounterClockwise,
}

/// Outline point with a lot of context for hinting.
///
/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.h#L239>
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub(crate) struct Point {
    /// Describes the type and hinting state of the point.
    pub flags: PointFlags,
    /// X coordinate in font units.
    pub fx: i32,
    /// Y coordinate in font units.
    pub fy: i32,
    /// Scaled X coordinate.
    pub ox: i32,
    /// Scaled Y coordinate.
    pub oy: i32,
    /// Hinted X coordinate.
    pub x: i32,
    /// Hinted Y coordinate.
    pub y: i32,
    /// Direction of inwards vector.
    pub in_dir: Direction,
    /// Direction of outwards vector.
    pub out_dir: Direction,
    /// Context dependent coordinate.
    pub u: i32,
    /// Context dependent coordinate.
    pub v: i32,
    /// Index of next point in contour.
    pub next_ix: u16,
    /// Index of previous point in contour.
    pub prev_ix: u16,
}

impl Point {
    pub fn is_on_curve(&self) -> bool {
        self.flags.is_on_curve()
    }

    /// Returns the index of the next point in the contour.
    pub fn next(&self) -> usize {
        self.next_ix as usize
    }

    /// Returns the index of the previous point in the contour.
    pub fn prev(&self) -> usize {
        self.prev_ix as usize
    }

    #[inline(always)]
    fn as_contour_point(&self) -> path::ContourPoint<F26Dot6> {
        path::ContourPoint {
            x: F26Dot6::from_bits(self.x),
            y: F26Dot6::from_bits(self.y),
            flags: self.flags,
        }
    }
}

// Matches FreeType's inline usage
//
// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.h#L332>
const MAX_INLINE_POINTS: usize = 96;
const MAX_INLINE_CONTOURS: usize = 8;

#[derive(Default)]
pub(crate) struct Outline {
    pub units_per_em: i32,
    pub orientation: Option<Orientation>,
    pub points: SmallVec<Point, MAX_INLINE_POINTS>,
    pub contours: SmallVec<Contour, MAX_INLINE_CONTOURS>,
    pub advance: i32,
}

impl Outline {
    /// Fills the outline from the given glyph.
    pub fn fill(&mut self, glyph: &OutlineGlyph, coords: &[F2Dot14]) -> Result<(), DrawError> {
        self.clear();
        let advance = glyph.draw_unscaled(LocationRef::new(coords), None, self)?;
        self.advance = advance;
        self.units_per_em = glyph.units_per_em() as i32;
        // Heuristic value
        let near_limit = 20 * self.units_per_em / 2048;
        self.link_points();
        self.mark_near_points(near_limit);
        self.compute_directions(near_limit);
        self.simplify_topology();
        self.check_remaining_weak_points();
        self.compute_orientation();
        Ok(())
    }

    /// Applies dimension specific scaling factors and deltas to each
    /// point in the outline.
    pub fn scale(&mut self, scale: &Scale) {
        use super::metrics::fixed_mul;
        for point in &mut self.points {
            let x = fixed_mul(point.fx, scale.x_scale) + scale.x_delta;
            let y = fixed_mul(point.fy, scale.y_scale) + scale.y_delta;
            point.ox = x;
            point.x = x;
            point.oy = y;
            point.y = y;
        }
    }

    pub fn clear(&mut self) {
        self.units_per_em = 0;
        self.points.clear();
        self.contours.clear();
        self.advance = 0;
    }

    pub fn to_path(
        &self,
        style: PathStyle,
        pen: &mut impl OutlinePen,
    ) -> Result<(), path::ToPathError> {
        for contour in &self.contours {
            let Some(points) = self.points.get(contour.range()) else {
                continue;
            };
            if let (Some(first_point), Some(last_point)) = (
                points.first().map(Point::as_contour_point),
                points.last().map(Point::as_contour_point),
            ) {
                path::contour_to_path(
                    points.iter().map(Point::as_contour_point),
                    first_point,
                    last_point,
                    style,
                    pen,
                )?;
            }
        }
        Ok(())
    }
}

impl Outline {
    /// Sets next and previous indices for each point.
    fn link_points(&mut self) {
        let points = self.points.as_mut_slice();
        for contour in &self.contours {
            let Some(points) = points.get_mut(contour.range()) else {
                continue;
            };
            let first_ix = contour.first() as u16;
            let mut prev_ix = contour.last() as u16;
            for (ix, point) in points.iter_mut().enumerate() {
                let ix = ix as u16 + first_ix;
                point.prev_ix = prev_ix;
                prev_ix = ix;
                point.next_ix = ix + 1;
            }
            points.last_mut().unwrap().next_ix = first_ix;
        }
    }

    /// Computes the near flag for each contour.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.c#L1017>
    fn mark_near_points(&mut self, near_limit: i32) {
        let points = self.points.as_mut_slice();
        for contour in &self.contours {
            let mut prev_ix = contour.last();
            for ix in contour.range() {
                let point = points[ix];
                let prev = &mut points[prev_ix];
                // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.c#L1017>
                let out_x = point.fx - prev.fx;
                let out_y = point.fy - prev.fy;
                if out_x.abs() + out_y.abs() < near_limit {
                    prev.flags.set_marker(PointMarker::NEAR);
                }
                prev_ix = ix;
            }
        }
    }

    /// Compute directions of in and out vectors.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.c#L1064>
    fn compute_directions(&mut self, near_limit: i32) {
        let near_limit2 = 2 * near_limit - 1;
        let points = self.points.as_mut_slice();
        for contour in &self.contours {
            // Walk backward to find the first non-near point.
            let mut first_ix = contour.first();
            let mut ix = first_ix;
            let mut prev_ix = contour.prev(first_ix);
            let mut point = points[first_ix];
            while prev_ix != first_ix {
                let prev = points[prev_ix];
                let out_x = point.fx - prev.fx;
                let out_y = point.fy - prev.fy;
                // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.c#L1102>
                if out_x.abs() + out_y.abs() >= near_limit2 {
                    break;
                }
                point = prev;
                ix = prev_ix;
                prev_ix = contour.prev(prev_ix);
            }
            first_ix = ix;
            // Abuse u and v fields to store deltas to the next and previous
            // non-near points, respectively.
            let first = &mut points[first_ix];
            first.u = first_ix as _;
            first.v = first_ix as _;
            let mut next_ix = first_ix;
            let mut ix = first_ix;
            // Now loop over all points in the contour to compute in and
            // out directions
            let mut out_x = 0;
            let mut out_y = 0;
            loop {
                let point_ix = next_ix;
                next_ix = contour.next(point_ix);
                let point = points[point_ix];
                let next = &mut points[next_ix];
                // Accumulate the deltas until we surpass near_limit
                out_x += next.fx - point.fx;
                out_y += next.fy - point.fy;
                if out_x.abs() + out_y.abs() < near_limit {
                    next.flags.set_marker(PointMarker::WEAK_INTERPOLATION);
                    // The original code is a do-while loop, so make
                    // sure we keep this condition before the continue
                    if next_ix == first_ix {
                        break;
                    }
                    continue;
                }
                let out_dir = Direction::new(out_x, out_y);
                next.in_dir = out_dir;
                next.v = ix as _;
                let cur = &mut points[ix];
                cur.u = next_ix as _;
                cur.out_dir = out_dir;
                // Adjust directions for all intermediate points
                let mut inter_ix = contour.next(ix);
                while inter_ix != next_ix {
                    let point = &mut points[inter_ix];
                    point.in_dir = out_dir;
                    point.out_dir = out_dir;
                    inter_ix = contour.next(inter_ix);
                }
                ix = next_ix;
                points[ix].u = first_ix as _;
                points[first_ix].v = ix as _;
                out_x = 0;
                out_y = 0;
                if next_ix == first_ix {
                    break;
                }
            }
        }
    }

    /// Simplify so that we can identify local extrema more reliably.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.c#L1181>
    fn simplify_topology(&mut self) {
        let points = self.points.as_mut_slice();
        for i in 0..points.len() {
            let point = points[i];
            if point.flags.has_marker(PointMarker::WEAK_INTERPOLATION) {
                continue;
            }
            if point.in_dir == Direction::None && point.out_dir == Direction::None {
                let u_index = point.u as usize;
                let v_index = point.v as usize;
                let next_u = points[u_index];
                let prev_v = points[v_index];
                let in_x = point.fx - prev_v.fx;
                let in_y = point.fy - prev_v.fy;
                let out_x = next_u.fx - point.fx;
                let out_y = next_u.fy - point.fy;
                if (in_x ^ out_x) >= 0 && (in_y ^ out_y) >= 0 {
                    // Both vectors point into the same quadrant
                    points[i].flags.set_marker(PointMarker::WEAK_INTERPOLATION);
                    points[v_index].u = u_index as _;
                    points[u_index].v = v_index as _;
                }
            }
        }
    }

    /// Check for remaining weak points.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/autofit/afhints.c#L1226>
    fn check_remaining_weak_points(&mut self) {
        let points = self.points.as_mut_slice();
        for i in 0..points.len() {
            let point = points[i];
            let mut make_weak = false;
            if point.flags.has_marker(PointMarker::WEAK_INTERPOLATION) {
                // Already weak
                continue;
            }
            if !point.flags.is_on_curve() {
                // Control points are always weak
                make_weak = true;
            } else if point.out_dir == point.in_dir {
                if point.out_dir != Direction::None {
                    // Point lies on a vertical or horizontal segment but
                    // not at start or end
                    make_weak = true;
                } else {
                    let u_index = point.u as usize;
                    let v_index = point.v as usize;
                    let next_u = points[u_index];
                    let prev_v = points[v_index];
                    if is_corner_flat(
                        point.fx - prev_v.fx,
                        point.fy - prev_v.fy,
                        next_u.fx - point.fx,
                        next_u.fy - point.fy,
                    ) {
                        // One of the vectors is more dominant
                        make_weak = true;
                        points[v_index].u = u_index as _;
                        points[u_index].v = v_index as _;
                    }
                }
            } else if point.in_dir.is_opposite(point.out_dir) {
                // Point forms a "spike"
                make_weak = true;
            }
            if make_weak {
                points[i].flags.set_marker(PointMarker::WEAK_INTERPOLATION);
            }
        }
    }

    /// Computes the overall winding order of the outline.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/base/ftoutln.c#L1049>
    fn compute_orientation(&mut self) {
        self.orientation = None;
        let points = self.points.as_slice();
        if points.is_empty() {
            return;
        }
        fn point_to_i64(point: &Point) -> (i64, i64) {
            (point.fx as i64, point.fy as i64)
        }
        let mut area = 0i64;
        for contour in &self.contours {
            let last_ix = contour.last();
            let first_ix = contour.first();
            let (mut prev_x, mut prev_y) = point_to_i64(&points[last_ix]);
            for point in &points[first_ix..=last_ix] {
                let (x, y) = point_to_i64(point);
                area += (y - prev_y) * (x + prev_x);
                (prev_x, prev_y) = (x, y);
            }
        }
        use core::cmp::Ordering;
        self.orientation = match area.cmp(&0) {
            Ordering::Less => Some(Orientation::CounterClockwise),
            Ordering::Greater => Some(Orientation::Clockwise),
            Ordering::Equal => None,
        };
    }
}

/// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/base/ftcalc.c#L1026>
fn is_corner_flat(in_x: i32, in_y: i32, out_x: i32, out_y: i32) -> bool {
    let ax = in_x + out_x;
    let ay = in_y + out_y;
    fn hypot(x: i32, y: i32) -> i32 {
        let x = x.abs();
        let y = y.abs();
        if x > y {
            x + ((3 * y) >> 3)
        } else {
            y + ((3 * x) >> 3)
        }
    }
    let d_in = hypot(in_x, in_y);
    let d_out = hypot(out_x, out_y);
    let d_hypot = hypot(ax, ay);
    (d_in + d_out - d_hypot) < (d_hypot >> 4)
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Contour {
    first_ix: u16,
    last_ix: u16,
}

impl Contour {
    pub fn first(self) -> usize {
        self.first_ix as usize
    }

    pub fn last(self) -> usize {
        self.last_ix as usize
    }

    pub fn next(self, index: usize) -> usize {
        if index >= self.last_ix as usize {
            self.first_ix as usize
        } else {
            index + 1
        }
    }

    pub fn prev(self, index: usize) -> usize {
        if index <= self.first_ix as usize {
            self.last_ix as usize
        } else {
            index - 1
        }
    }

    pub fn range(self) -> Range<usize> {
        self.first()..self.last() + 1
    }
}

impl UnscaledOutlineSink for Outline {
    fn try_reserve(&mut self, additional: usize) -> Result<(), DrawError> {
        if self.points.try_reserve(additional) {
            Ok(())
        } else {
            Err(DrawError::InsufficientMemory)
        }
    }

    fn push(&mut self, point: UnscaledPoint) -> Result<(), DrawError> {
        let new_point = Point {
            flags: point.flags,
            fx: point.x as i32,
            fy: point.y as i32,
            ..Default::default()
        };
        let new_point_ix: u16 = self
            .points
            .len()
            .try_into()
            .map_err(|_| DrawError::InsufficientMemory)?;
        if point.is_contour_start {
            self.contours.push(Contour {
                first_ix: new_point_ix,
                last_ix: new_point_ix,
            });
        } else if let Some(last_contour) = self.contours.last_mut() {
            last_contour.last_ix += 1;
        } else {
            // If our first point is not marked as contour start, just
            // create a new contour.
            self.contours.push(Contour {
                first_ix: new_point_ix,
                last_ix: new_point_ix,
            });
        }
        self.points.push(new_point);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{pen::SvgPen, DrawSettings};
    use super::*;
    use crate::{prelude::Size, MetadataProvider};
    use raw::{types::GlyphId, FontRef, TableProvider};

    #[test]
    fn direction_from_vectors() {
        assert_eq!(Direction::new(-100, 0), Direction::Left);
        assert_eq!(Direction::new(100, 0), Direction::Right);
        assert_eq!(Direction::new(0, -100), Direction::Down);
        assert_eq!(Direction::new(0, 100), Direction::Up);
        assert_eq!(Direction::new(7, 100), Direction::Up);
        // This triggers the too close heuristic
        assert_eq!(Direction::new(8, 100), Direction::None);
    }

    #[test]
    fn direction_axes() {
        use Direction::*;
        let hori = [Left, Right];
        let vert = [Up, Down];
        for h in hori {
            for h2 in hori {
                assert!(h.is_same_axis(h2));
                if h != h2 {
                    assert!(h.is_opposite(h2));
                } else {
                    assert!(!h.is_opposite(h2));
                }
            }
            for v in vert {
                assert!(!h.is_same_axis(v));
                assert!(!h.is_opposite(v));
            }
        }
        for v in vert {
            for v2 in vert {
                assert!(v.is_same_axis(v2));
                if v != v2 {
                    assert!(v.is_opposite(v2));
                } else {
                    assert!(!v.is_opposite(v2));
                }
            }
        }
    }

    #[test]
    fn fill_outline() {
        let outline = make_outline(font_test_data::NOTOSERIFHEBREW_AUTOHINT_METRICS, 8);
        use Direction::*;
        let expected = &[
            // (x, y, in_dir, out_dir, flags)
            (107, 0, Left, Left, 3),
            (85, 0, Left, None, 2),
            (55, 26, None, Up, 2),
            (55, 71, Up, Up, 3),
            (55, 332, Up, Up, 3),
            (55, 360, Up, None, 2),
            (67, 411, None, None, 2),
            (93, 459, None, None, 2),
            (112, 481, None, Up, 1),
            (112, 504, Up, Right, 1),
            (168, 504, Right, Down, 1),
            (168, 483, Down, None, 1),
            (153, 473, None, None, 2),
            (126, 428, None, None, 2),
            (109, 366, None, Down, 2),
            (109, 332, Down, Down, 3),
            (109, 109, Down, Right, 1),
            (407, 109, Right, Right, 3),
            (427, 109, Right, None, 2),
            (446, 136, None, None, 2),
            (453, 169, None, Up, 2),
            (453, 178, Up, Up, 3),
            (453, 374, Up, Up, 3),
            (453, 432, Up, None, 2),
            (400, 483, None, Left, 2),
            (362, 483, Left, Left, 3),
            (109, 483, Left, Left, 3),
            (86, 483, Left, None, 2),
            (62, 517, None, Up, 2),
            (62, 555, Up, Up, 3),
            (62, 566, Up, None, 2),
            (64, 587, None, None, 2),
            (71, 619, None, None, 2),
            (76, 647, None, Right, 1),
            (103, 647, Right, Down, 9),
            (103, 644, Down, Down, 3),
            (103, 619, Down, None, 2),
            (131, 592, None, Right, 2),
            (155, 592, Right, Right, 3),
            (386, 592, Right, Right, 3),
            (437, 592, Right, None, 2),
            (489, 552, None, None, 2),
            (507, 485, None, Down, 2),
            (507, 443, Down, Down, 3),
            (507, 75, Down, Down, 3),
            (507, 40, Down, None, 2),
            (470, 0, None, Left, 2),
            (436, 0, Left, Left, 3),
        ];
        let points = outline
            .points
            .iter()
            .map(|point| {
                (
                    point.fx,
                    point.fy,
                    point.in_dir,
                    point.out_dir,
                    point.flags.to_bits(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(&points, expected);
    }

    #[test]
    fn orientation() {
        let tt_outline = make_outline(font_test_data::NOTOSERIFHEBREW_AUTOHINT_METRICS, 8);
        // TrueType outlines are counter clockwise
        assert_eq!(tt_outline.orientation, Some(Orientation::CounterClockwise));
        let ps_outline = make_outline(font_test_data::CANTARELL_VF_TRIMMED, 4);
        // PostScript outlines are clockwise
        assert_eq!(ps_outline.orientation, Some(Orientation::Clockwise));
    }

    fn make_outline(font_data: &[u8], glyph_id: u32) -> Outline {
        let font = FontRef::new(font_data).unwrap();
        let glyphs = font.outline_glyphs();
        let glyph = glyphs.get(GlyphId::from(glyph_id)).unwrap();
        let mut outline = Outline::default();
        outline.fill(&glyph, Default::default()).unwrap();
        outline
    }

    #[test]
    fn mostly_off_curve_to_path_scan_backward() {
        compare_path_conversion(font_test_data::MOSTLY_OFF_CURVE, PathStyle::FreeType);
    }

    #[test]
    fn mostly_off_curve_to_path_scan_forward() {
        compare_path_conversion(font_test_data::MOSTLY_OFF_CURVE, PathStyle::HarfBuzz);
    }

    #[test]
    fn starting_off_curve_to_path_scan_backward() {
        compare_path_conversion(font_test_data::STARTING_OFF_CURVE, PathStyle::FreeType);
    }

    #[test]
    fn starting_off_curve_to_path_scan_forward() {
        compare_path_conversion(font_test_data::STARTING_OFF_CURVE, PathStyle::HarfBuzz);
    }

    #[test]
    fn cubic_to_path_scan_backward() {
        compare_path_conversion(font_test_data::CUBIC_GLYF, PathStyle::FreeType);
    }

    #[test]
    fn cubic_to_path_scan_forward() {
        compare_path_conversion(font_test_data::CUBIC_GLYF, PathStyle::HarfBuzz);
    }

    #[test]
    fn cff_to_path_scan_backward() {
        compare_path_conversion(font_test_data::CANTARELL_VF_TRIMMED, PathStyle::FreeType);
    }

    #[test]
    fn cff_to_path_scan_forward() {
        compare_path_conversion(font_test_data::CANTARELL_VF_TRIMMED, PathStyle::HarfBuzz);
    }

    /// Ensures autohint path conversion matches the base scaler path
    /// conversion for all glyphs in the given font with a certain
    /// path style.
    fn compare_path_conversion(font_data: &[u8], path_style: PathStyle) {
        let font = FontRef::new(font_data).unwrap();
        let glyph_count = font.maxp().unwrap().num_glyphs();
        let glyphs = font.outline_glyphs();
        let mut results = Vec::new();
        // And all glyphs
        for gid in 0..glyph_count {
            let glyph = glyphs.get(GlyphId::from(gid)).unwrap();
            // Unscaled, unhinted code path
            let mut base_svg = SvgPen::default();
            let settings = DrawSettings::unhinted(Size::unscaled(), LocationRef::default())
                .with_path_style(path_style);
            glyph.draw(settings, &mut base_svg).unwrap();
            let base_svg = base_svg.to_string();
            // Autohinter outline code path
            let mut outline = Outline::default();
            outline.fill(&glyph, Default::default()).unwrap();
            // The to_path method uses the (x, y) coords which aren't filled
            // until we scale (and we aren't doing that here) so update
            // them with 26.6 values manually
            for point in &mut outline.points {
                point.x = point.fx << 6;
                point.y = point.fy << 6;
            }
            let mut autohint_svg = SvgPen::default();
            outline.to_path(path_style, &mut autohint_svg).unwrap();
            let autohint_svg = autohint_svg.to_string();
            if base_svg != autohint_svg {
                results.push((gid, base_svg, autohint_svg));
            }
        }
        if !results.is_empty() {
            let report: String = results
                .into_iter()
                .map(|(gid, expected, got)| {
                    format!("[glyph {gid}]\nexpected: {expected}\n     got: {got}")
                })
                .collect::<Vec<_>>()
                .join("\n");
            panic!("outline to path comparison failed:\n{report}");
        }
    }
}
