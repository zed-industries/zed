//! Glyph zones.

use read_fonts::{
    tables::glyf::{PointFlags, PointMarker},
    types::{F26Dot6, Point},
};

use super::{
    error::HintErrorKind,
    graphics::{CoordAxis, GraphicsState},
    math,
};

use HintErrorKind::{InvalidPointIndex, InvalidPointRange};

/// Reference to either the twilight or glyph zone.
///
/// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructing_glyphs#zones>
#[derive(Copy, Clone, PartialEq, Default, Debug)]
#[repr(u8)]
pub enum ZonePointer {
    Twilight = 0,
    #[default]
    Glyph = 1,
}

impl ZonePointer {
    pub fn is_twilight(self) -> bool {
        self == Self::Twilight
    }
}

impl TryFrom<i32> for ZonePointer {
    type Error = HintErrorKind;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Twilight),
            1 => Ok(Self::Glyph),
            _ => Err(HintErrorKind::InvalidZoneIndex(value)),
        }
    }
}

/// Glyph zone for TrueType hinting.
///
/// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructing_glyphs#zones>
#[derive(Default, Debug)]
pub struct Zone<'a> {
    /// Outline points prior to applying scale.
    pub unscaled: &'a [Point<i32>],
    /// Copy of the outline points after applying scale.
    pub original: &'a mut [Point<F26Dot6>],
    /// Scaled outline points.
    pub points: &'a mut [Point<F26Dot6>],
    pub flags: &'a mut [PointFlags],
    pub contours: &'a [u16],
}

impl<'a> Zone<'a> {
    /// Creates a new hinting zone.
    pub fn new(
        unscaled: &'a [Point<i32>],
        original: &'a mut [Point<F26Dot6>],
        points: &'a mut [Point<F26Dot6>],
        flags: &'a mut [PointFlags],
        contours: &'a [u16],
    ) -> Self {
        Self {
            unscaled,
            original,
            points,
            flags,
            contours,
        }
    }

    pub fn point(&self, index: usize) -> Result<Point<F26Dot6>, HintErrorKind> {
        self.points
            .get(index)
            .copied()
            .ok_or(InvalidPointIndex(index))
    }

    pub fn point_mut(&mut self, index: usize) -> Result<&mut Point<F26Dot6>, HintErrorKind> {
        self.points.get_mut(index).ok_or(InvalidPointIndex(index))
    }

    pub fn original(&self, index: usize) -> Result<Point<F26Dot6>, HintErrorKind> {
        self.original
            .get(index)
            .copied()
            .ok_or(InvalidPointIndex(index))
    }

    pub fn original_mut(&mut self, index: usize) -> Result<&mut Point<F26Dot6>, HintErrorKind> {
        self.original.get_mut(index).ok_or(InvalidPointIndex(index))
    }

    pub fn unscaled(&self, index: usize) -> Point<i32> {
        // Unscaled points in the twilight zone are always (0, 0). This allows
        // us to avoid the allocation for that zone and back it with an empty
        // slice.
        self.unscaled.get(index).copied().unwrap_or_default()
    }

    pub fn contour(&self, index: usize) -> Result<u16, HintErrorKind> {
        self.contours
            .get(index)
            .copied()
            .ok_or(HintErrorKind::InvalidContourIndex(index))
    }

    pub fn touch(&mut self, index: usize, axis: CoordAxis) -> Result<(), HintErrorKind> {
        let flag = self.flags.get_mut(index).ok_or(InvalidPointIndex(index))?;
        flag.set_marker(axis.touched_marker());
        Ok(())
    }

    pub fn untouch(&mut self, index: usize, axis: CoordAxis) -> Result<(), HintErrorKind> {
        let flag = self.flags.get_mut(index).ok_or(InvalidPointIndex(index))?;
        flag.clear_marker(axis.touched_marker());
        Ok(())
    }

    pub fn is_touched(&self, index: usize, axis: CoordAxis) -> Result<bool, HintErrorKind> {
        let flag = self.flags.get(index).ok_or(InvalidPointIndex(index))?;
        Ok(flag.has_marker(axis.touched_marker()))
    }

    pub fn flip_on_curve(&mut self, index: usize) -> Result<(), HintErrorKind> {
        let flag = self.flags.get_mut(index).ok_or(InvalidPointIndex(index))?;
        flag.flip_on_curve();
        Ok(())
    }

    pub fn set_on_curve(
        &mut self,
        start: usize,
        end: usize,
        on: bool,
    ) -> Result<(), HintErrorKind> {
        let flags = self
            .flags
            .get_mut(start..end)
            .ok_or(InvalidPointRange(start, end))?;
        if on {
            for flag in flags {
                flag.set_on_curve();
            }
        } else {
            for flag in flags {
                flag.clear_on_curve();
            }
        }
        Ok(())
    }

    /// Interpolate untouched points.
    ///
    /// Based on <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6391>
    pub fn iup(&mut self, axis: CoordAxis) -> Result<(), HintErrorKind> {
        let mut point = 0;
        for i in 0..self.contours.len() {
            let mut end_point = self.contour(i)? as usize;
            let first_point = point;
            if end_point >= self.points.len() {
                end_point = self.points.len() - 1;
            }
            while point <= end_point && !self.is_touched(point, axis)? {
                point += 1;
            }
            if point <= end_point {
                let first_touched = point;
                let mut cur_touched = point;
                point += 1;
                while point <= end_point {
                    if self.is_touched(point, axis)? {
                        self.iup_interpolate(axis, cur_touched + 1, point - 1, cur_touched, point)?;
                        cur_touched = point;
                    }
                    point += 1;
                }
                if cur_touched == first_touched {
                    self.iup_shift(axis, first_point, end_point, cur_touched)?;
                } else {
                    self.iup_interpolate(
                        axis,
                        cur_touched + 1,
                        end_point,
                        cur_touched,
                        first_touched,
                    )?;
                    if first_touched > 0 {
                        self.iup_interpolate(
                            axis,
                            first_point,
                            first_touched - 1,
                            cur_touched,
                            first_touched,
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Shift the range of points p1..=p2 based on the delta given by the
    /// reference point p.
    ///
    /// Based on <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6262>
    fn iup_shift(
        &mut self,
        axis: CoordAxis,
        p1: usize,
        p2: usize,
        p: usize,
    ) -> Result<(), HintErrorKind> {
        if p1 > p2 || p1 > p || p > p2 {
            return Ok(());
        }
        macro_rules! shift_coord {
            ($coord:ident) => {
                let delta = self.point(p)?.$coord - self.original(p)?.$coord;
                if delta != F26Dot6::ZERO {
                    let (first, second) = self
                        .points
                        .get_mut(p1..=p2)
                        .ok_or(InvalidPointRange(p1, p2 + 1))?
                        .split_at_mut(p - p1);
                    for point in first
                        .iter_mut()
                        .chain(second.get_mut(1..).ok_or(InvalidPointIndex(p - p1))?)
                    {
                        point.$coord += delta;
                    }
                }
            };
        }
        if axis == CoordAxis::X {
            shift_coord!(x);
        } else {
            shift_coord!(y);
        }
        Ok(())
    }

    /// Interpolate the range of points p1..=p2 based on the deltas
    /// given by the two reference points.
    ///
    /// Based on <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6284>
    fn iup_interpolate(
        &mut self,
        axis: CoordAxis,
        p1: usize,
        p2: usize,
        mut ref1: usize,
        mut ref2: usize,
    ) -> Result<(), HintErrorKind> {
        if p1 > p2 {
            return Ok(());
        }
        let max_points = self.points.len();
        if ref1 >= max_points || ref2 >= max_points {
            return Ok(());
        }
        macro_rules! interpolate_coord {
            ($coord:ident) => {
                let mut orus1 = self.unscaled(ref1).$coord;
                let mut orus2 = self.unscaled(ref2).$coord;
                if orus1 > orus2 {
                    use core::mem::swap;
                    swap(&mut orus1, &mut orus2);
                    swap(&mut ref1, &mut ref2);
                }
                let org1 = self.original(ref1)?.$coord;
                let org2 = self.original(ref2)?.$coord;
                let cur1 = self.point(ref1)?.$coord;
                let cur2 = self.point(ref2)?.$coord;
                let delta1 = cur1 - org1;
                let delta2 = cur2 - org2;
                let iter = self
                    .original
                    .get(p1..=p2)
                    .ok_or(InvalidPointRange(p1, p2 + 1))?
                    .iter()
                    .zip(
                        self.unscaled
                            .get(p1..=p2)
                            .ok_or(InvalidPointRange(p1, p2 + 1))?,
                    )
                    .zip(
                        self.points
                            .get_mut(p1..=p2)
                            .ok_or(InvalidPointRange(p1, p2 + 1))?,
                    );
                if cur1 == cur2 || orus1 == orus2 {
                    for ((orig, _unscaled), point) in iter {
                        let a = orig.$coord;
                        point.$coord = if a <= org1 {
                            a + delta1
                        } else if a >= org2 {
                            a + delta2
                        } else {
                            cur1
                        };
                    }
                } else {
                    let scale = math::div((cur2 - cur1).to_bits(), orus2 - orus1);
                    for ((orig, unscaled), point) in iter {
                        let a = orig.$coord;
                        point.$coord = if a <= org1 {
                            a + delta1
                        } else if a >= org2 {
                            a + delta2
                        } else {
                            cur1 + F26Dot6::from_bits(math::mul(unscaled.$coord - orus1, scale))
                        };
                    }
                }
            };
        }
        if axis == CoordAxis::X {
            interpolate_coord!(x);
        } else {
            interpolate_coord!(y);
        }
        Ok(())
    }
}

impl<'a> GraphicsState<'a> {
    /// Takes an array of (zone pointer, point index) pairs and returns true if
    /// all accesses would be valid.
    pub fn in_bounds<const N: usize>(&self, pairs: [(ZonePointer, usize); N]) -> bool {
        for (zp, index) in pairs {
            if index > self.zone(zp).points.len() {
                return false;
            }
        }
        true
    }

    #[inline(always)]
    pub fn zone(&self, pointer: ZonePointer) -> &Zone<'a> {
        &self.zones[pointer as usize]
    }

    #[inline(always)]
    pub fn zone_mut(&mut self, pointer: ZonePointer) -> &mut Zone<'a> {
        &mut self.zones[pointer as usize]
    }

    #[inline(always)]
    pub fn zp0(&self) -> &Zone<'a> {
        self.zone(self.zp0)
    }

    #[inline(always)]
    pub fn zp0_mut(&mut self) -> &mut Zone<'a> {
        self.zone_mut(self.zp0)
    }

    #[inline(always)]
    pub fn zp1(&self) -> &Zone<'_> {
        self.zone(self.zp1)
    }

    #[inline(always)]
    pub fn zp1_mut(&mut self) -> &mut Zone<'a> {
        self.zone_mut(self.zp1)
    }

    #[inline(always)]
    pub fn zp2(&self) -> &Zone<'_> {
        self.zone(self.zp2)
    }

    #[inline(always)]
    pub fn zp2_mut(&mut self) -> &mut Zone<'a> {
        self.zone_mut(self.zp2)
    }
}

impl GraphicsState<'_> {
    /// Moves the requested original point by the given distance.
    // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L1743>
    pub(crate) fn move_original(
        &mut self,
        zone: ZonePointer,
        point_ix: usize,
        distance: F26Dot6,
    ) -> Result<(), HintErrorKind> {
        let fv = self.freedom_vector;
        let fdotp = self.fdotp;
        let axis = self.freedom_axis;
        let point = self.zone_mut(zone).original_mut(point_ix)?;
        match axis {
            CoordAxis::X => point.x += distance,
            CoordAxis::Y => point.y += distance,
            CoordAxis::Both => {
                let distance = distance.to_bits();
                if fv.x != 0 {
                    point.x += F26Dot6::from_bits(math::mul_div(distance, fv.x, fdotp));
                }
                if fv.y != 0 {
                    point.y += F26Dot6::from_bits(math::mul_div(distance, fv.y, fdotp));
                }
            }
        }
        Ok(())
    }

    /// Moves the requested scaled point by the given distance.
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L1771>
    pub(crate) fn move_point(
        &mut self,
        zone: ZonePointer,
        point_ix: usize,
        distance: F26Dot6,
    ) -> Result<(), HintErrorKind> {
        // Note: we never adjust x in backward compatibility mode and we never
        // adjust y in backward compatibility mode after IUP has been done in
        // both directions.
        //
        // The primary motivation is to avoid horizontal adjustments in cases
        // where subpixel rendering provides better fidelity.
        //
        // For more detail, see <https://learn.microsoft.com/en-us/typography/cleartype/truetypecleartype>
        let back_compat = self.backward_compatibility;
        let back_compat_and_did_iup = back_compat && self.did_iup_x && self.did_iup_y;
        let zone = &mut self.zones[zone as usize];
        let point = zone.point_mut(point_ix)?;
        match self.freedom_axis {
            CoordAxis::X => {
                if !back_compat {
                    point.x += distance;
                }
                zone.touch(point_ix, CoordAxis::X)?;
            }
            CoordAxis::Y => {
                if !back_compat_and_did_iup {
                    point.y += distance;
                }
                zone.touch(point_ix, CoordAxis::Y)?;
            }
            CoordAxis::Both => {
                // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L1669>
                let fv = self.freedom_vector;
                let distance = distance.to_bits();
                if fv.x != 0 {
                    if !back_compat {
                        point.x += F26Dot6::from_bits(math::mul_div(distance, fv.x, self.fdotp));
                    }
                    zone.touch(point_ix, CoordAxis::X)?;
                }
                if fv.y != 0 {
                    if !back_compat_and_did_iup {
                        zone.point_mut(point_ix)?.y +=
                            F26Dot6::from_bits(math::mul_div(distance, fv.y, self.fdotp));
                    }
                    zone.touch(point_ix, CoordAxis::Y)?;
                }
            }
        }
        Ok(())
    }

    /// Moves the requested scaled point in the zone referenced by zp2 by the
    /// given delta.
    ///
    /// This is a helper function for SHP, SHC, SHZ, and SHPIX instructions.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5170>
    pub(crate) fn move_zp2_point(
        &mut self,
        point_ix: usize,
        dx: F26Dot6,
        dy: F26Dot6,
        do_touch: bool,
    ) -> Result<(), HintErrorKind> {
        // See notes above in move_point() about how this is used.
        let back_compat = self.backward_compatibility;
        let back_compat_and_did_iup = back_compat && self.did_iup_x && self.did_iup_y;
        let fv = self.freedom_vector;
        let zone = self.zp2_mut();
        if fv.x != 0 {
            if !back_compat {
                zone.point_mut(point_ix)?.x += dx;
            }
            if do_touch {
                zone.touch(point_ix, CoordAxis::X)?;
            }
        }
        if fv.y != 0 {
            if !back_compat_and_did_iup {
                zone.point_mut(point_ix)?.y += dy;
            }
            if do_touch {
                zone.touch(point_ix, CoordAxis::Y)?;
            }
        }
        Ok(())
    }

    /// Computes the adjustment made to a point along the current freedom vector.
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5126>
    pub(crate) fn point_displacement(
        &mut self,
        opcode: u8,
    ) -> Result<PointDisplacement, HintErrorKind> {
        let (zone, point_ix) = if (opcode & 1) != 0 {
            (self.zp0, self.rp1)
        } else {
            (self.zp1, self.rp2)
        };
        let zone_data = self.zone(zone);
        let point = zone_data.point(point_ix)?;
        let original_point = zone_data.original(point_ix)?;
        let distance = self.project(point, original_point);
        let fv = self.freedom_vector;
        let dx = F26Dot6::from_bits(math::mul_div(distance.to_bits(), fv.x, self.fdotp));
        let dy = F26Dot6::from_bits(math::mul_div(distance.to_bits(), fv.y, self.fdotp));
        Ok(PointDisplacement {
            zone,
            point_ix,
            dx,
            dy,
        })
    }
}

#[derive(PartialEq, Debug)]
pub(crate) struct PointDisplacement {
    pub zone: ZonePointer,
    pub point_ix: usize,
    pub dx: F26Dot6,
    pub dy: F26Dot6,
}

impl CoordAxis {
    fn touched_marker(self) -> PointMarker {
        match self {
            CoordAxis::Both => PointMarker::TOUCHED,
            CoordAxis::X => PointMarker::TOUCHED_X,
            CoordAxis::Y => PointMarker::TOUCHED_Y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{math, CoordAxis, GraphicsState, PointDisplacement, Zone, ZonePointer};
    use raw::{
        tables::glyf::{PointFlags, PointMarker},
        types::{F26Dot6, Point},
    };

    #[test]
    fn flip_on_curve_point() {
        let on_curve = PointFlags::on_curve();
        let off_curve = PointFlags::off_curve_quad();
        let mut zone = Zone {
            unscaled: &mut [],
            original: &mut [],
            points: &mut [],
            contours: &[],
            flags: &mut [on_curve, off_curve, off_curve, on_curve],
        };
        for i in 0..4 {
            zone.flip_on_curve(i).unwrap();
        }
        assert_eq!(zone.flags, &[off_curve, on_curve, on_curve, off_curve]);
    }

    #[test]
    fn set_on_curve_regions() {
        let on_curve = PointFlags::on_curve();
        let off_curve = PointFlags::off_curve_quad();
        let mut zone = Zone {
            unscaled: &mut [],
            original: &mut [],
            points: &mut [],
            contours: &[],
            flags: &mut [on_curve, off_curve, off_curve, on_curve],
        };
        zone.set_on_curve(0, 2, true).unwrap();
        zone.set_on_curve(2, 4, false).unwrap();
        assert_eq!(zone.flags, &[on_curve, on_curve, off_curve, off_curve]);
    }

    #[test]
    fn iup_shift() {
        let [untouched, touched] = point_markers();
        // A single touched point shifts the whole contour
        let mut original = f26dot6_points([(0, 0), (10, 10), (20, 20)]);
        let mut points = f26dot6_points([(-5, -20), (10, 10), (20, 20)]);
        let mut zone = Zone {
            unscaled: &mut [],
            original: &mut original,
            points: &mut points,
            contours: &[3],
            flags: &mut [touched, untouched, untouched],
        };
        zone.iup(CoordAxis::X).unwrap();
        assert_eq!(zone.points, &f26dot6_points([(-5, -20), (5, 10), (15, 20)]),);
        zone.iup(CoordAxis::Y).unwrap();
        assert_eq!(zone.points, &f26dot6_points([(-5, -20), (5, -10), (15, 0)]),);
    }

    #[test]
    fn iup_interpolate() {
        let [untouched, touched] = point_markers();
        // Two touched points interpolates the intermediate point(s)
        let mut original = f26dot6_points([(0, 0), (10, 10), (20, 20)]);
        let mut points = f26dot6_points([(-5, -20), (10, 10), (27, 56)]);
        let mut zone = Zone {
            unscaled: &mut [
                Point::new(0, 0),
                Point::new(500, 500),
                Point::new(1000, 1000),
            ],
            original: &mut original,
            points: &mut points,
            contours: &[3],
            flags: &mut [touched, untouched, touched],
        };
        zone.iup(CoordAxis::X).unwrap();
        assert_eq!(
            zone.points,
            &f26dot6_points([(-5, -20), (11, 10), (27, 56)]),
        );
        zone.iup(CoordAxis::Y).unwrap();
        assert_eq!(
            zone.points,
            &f26dot6_points([(-5, -20), (11, 18), (27, 56)]),
        );
    }

    #[test]
    fn move_point_x() {
        let mut mock = MockGraphicsState::new();
        let mut gs = mock.graphics_state(100, 0);
        let point_ix = 0;
        let orig_x = gs.zones[1].point(point_ix).unwrap().x;
        let dx = F26Dot6::from_bits(10);
        // backward compatibility is on by default and we don't move x coord
        gs.move_point(ZonePointer::Glyph, 0, dx).unwrap();
        assert_eq!(orig_x, gs.zones[1].point(point_ix).unwrap().x);
        // disable so we actually move
        gs.backward_compatibility = false;
        gs.move_point(ZonePointer::Glyph, 0, dx).unwrap();
        let new_x = gs.zones[1].point(point_ix).unwrap().x;
        assert_ne!(orig_x, new_x);
        assert_eq!(new_x, orig_x + dx)
    }

    #[test]
    fn move_point_y() {
        let mut mock = MockGraphicsState::new();
        let mut gs = mock.graphics_state(0, 100);
        let point_ix = 0;
        let orig_y = gs.zones[1].point(point_ix).unwrap().y;
        let dy = F26Dot6::from_bits(10);
        // movement in y is prevented post-iup when backward
        // compatibility is enabled
        gs.did_iup_x = true;
        gs.did_iup_y = true;
        gs.move_point(ZonePointer::Glyph, 0, dy).unwrap();
        assert_eq!(orig_y, gs.zones[1].point(point_ix).unwrap().y);
        // allow movement
        gs.did_iup_x = false;
        gs.did_iup_y = false;
        gs.move_point(ZonePointer::Glyph, 0, dy).unwrap();
        let new_y = gs.zones[1].point(point_ix).unwrap().y;
        assert_ne!(orig_y, new_y);
        assert_eq!(new_y, orig_y + dy)
    }

    #[test]
    fn move_point_x_and_y() {
        let mut mock = MockGraphicsState::new();
        let mut gs = mock.graphics_state(100, 50);
        let point_ix = 0;
        let orig_point = gs.zones[1].point(point_ix).unwrap();
        let dist = F26Dot6::from_bits(10);
        // prevent movement in x and y
        gs.did_iup_x = true;
        gs.did_iup_y = true;
        gs.move_point(ZonePointer::Glyph, 0, dist).unwrap();
        assert_eq!(orig_point, gs.zones[1].point(point_ix).unwrap());
        // allow movement
        gs.backward_compatibility = false;
        gs.did_iup_x = false;
        gs.did_iup_y = false;
        gs.move_point(ZonePointer::Glyph, 0, dist).unwrap();
        let point = gs.zones[1].point(point_ix).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(4, -16));
    }

    #[test]
    fn move_original_x() {
        let mut mock = MockGraphicsState::new();
        let mut gs = mock.graphics_state(100, 0);
        let point_ix = 0;
        let orig_x = gs.zones[1].original(point_ix).unwrap().x;
        let dx = F26Dot6::from_bits(10);
        gs.move_original(ZonePointer::Glyph, 0, dx).unwrap();
        let new_x = gs.zones[1].original(point_ix).unwrap().x;
        assert_eq!(new_x, orig_x + dx)
    }

    #[test]
    fn move_original_y() {
        let mut mock = MockGraphicsState::new();
        let mut gs = mock.graphics_state(0, 100);
        let point_ix = 0;
        let orig_y = gs.zones[1].original(point_ix).unwrap().y;
        let dy = F26Dot6::from_bits(10);
        gs.move_original(ZonePointer::Glyph, 0, dy).unwrap();
        let new_y = gs.zones[1].original(point_ix).unwrap().y;
        assert_eq!(new_y, orig_y + dy)
    }

    #[test]
    fn move_original_x_and_y() {
        let mut mock = MockGraphicsState::new();
        let mut gs = mock.graphics_state(100, 50);
        let point_ix = 0;
        let dist = F26Dot6::from_bits(10);
        gs.move_original(ZonePointer::Glyph, 0, dist).unwrap();
        let point = gs.zones[1].original(point_ix).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(9, 4));
    }

    #[test]
    fn move_zp2_point() {
        let mut mock = MockGraphicsState::new();
        let mut gs = mock.graphics_state(100, 50);
        gs.zp2 = ZonePointer::Glyph;
        let point_ix = 0;
        let orig_point = gs.zones[1].point(point_ix).unwrap();
        let dx = F26Dot6::from_bits(10);
        let dy = F26Dot6::from_bits(-10);
        // prevent movement in x and y
        gs.did_iup_x = true;
        gs.did_iup_y = true;
        gs.move_zp2_point(point_ix, dx, dy, false).unwrap();
        assert_eq!(orig_point, gs.zones[1].point(point_ix).unwrap());
        // allow movement
        gs.backward_compatibility = false;
        gs.did_iup_x = false;
        gs.did_iup_y = false;
        gs.move_zp2_point(point_ix, dx, dy, false).unwrap();
        let point = gs.zones[1].point(point_ix).unwrap();
        assert_eq!(point, orig_point + Point::new(dx, dy));
    }

    #[test]
    fn point_displacement() {
        let mut mock = MockGraphicsState::new();
        let mut gs = mock.graphics_state(100, 50);
        gs.zp0 = ZonePointer::Glyph;
        gs.rp1 = 0;
        assert_eq!(
            gs.point_displacement(1).unwrap(),
            PointDisplacement {
                zone: ZonePointer::Glyph,
                point_ix: 0,
                dx: F26Dot6::from_f64(-0.1875),
                dy: F26Dot6::from_f64(-0.09375),
            }
        );
        gs.rp2 = 2;
        assert_eq!(
            gs.point_displacement(0).unwrap(),
            PointDisplacement {
                zone: ZonePointer::Glyph,
                point_ix: 2,
                dx: F26Dot6::from_f64(0.390625),
                dy: F26Dot6::from_f64(0.203125),
            }
        );
    }

    struct MockGraphicsState {
        points: [Point<F26Dot6>; 3],
        original: [Point<F26Dot6>; 3],
        contours: [u16; 1],
        flags: [PointFlags; 3],
    }

    impl MockGraphicsState {
        fn new() -> Self {
            Self {
                points: f26dot6_points([(-5, -20), (10, 10), (20, 20)]),
                original: f26dot6_points([(0, 0), (10, 10), (20, -42)]),
                flags: [PointFlags::default(); 3],
                contours: [3],
            }
        }

        fn graphics_state(&mut self, fv_x: i32, fv_y: i32) -> GraphicsState {
            let glyph = Zone {
                unscaled: &mut [],
                original: &mut self.original,
                points: &mut self.points,
                contours: &self.contours,
                flags: &mut self.flags,
            };
            let v = math::normalize14(fv_x, fv_y);
            let mut gs = GraphicsState {
                zones: [Zone::default(), glyph],
                freedom_vector: v,
                proj_vector: v,
                zp0: ZonePointer::Glyph,
                ..Default::default()
            };
            gs.update_projection_state();
            gs
        }
    }

    fn point_markers() -> [PointFlags; 2] {
        let untouched = PointFlags::default();
        let mut touched = untouched;
        touched.set_marker(PointMarker::TOUCHED);
        [untouched, touched]
    }

    fn f26dot6_points<const N: usize>(points: [(i32, i32); N]) -> [Point<F26Dot6>; N] {
        points.map(|point| Point::new(F26Dot6::from_bits(point.0), F26Dot6::from_bits(point.1)))
    }
}
