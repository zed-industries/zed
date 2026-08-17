//! Managing outlines.
//!
//! Implements 87 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#managing-outlines>

use super::{
    super::{
        graphics::CoordAxis,
        zone::{PointDisplacement, ZonePointer},
    },
    math, Engine, F26Dot6, HintErrorKind, OpResult,
};

impl Engine<'_> {
    /// Flip point.
    ///
    /// FLIPPT[] (0x80)
    ///
    /// Pops: p: point number (uint32)
    ///
    /// Uses the loop counter.
    ///
    /// Flips points that are off the curve so that they are on the curve and
    /// points that are on the curve so that they are off the curve. The point
    /// is not marked as touched. The result of a FLIPPT instruction is that
    /// the contour describing part of a glyph outline is redefined.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#flip-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5002>
    pub(super) fn op_flippt(&mut self) -> OpResult {
        let count = self.graphics.loop_counter as usize;
        self.graphics.loop_counter = 1;
        // In backward compatibility mode, don't flip points after IUP has
        // been done.
        if self.graphics.backward_compatibility
            && self.graphics.did_iup_x
            && self.graphics.did_iup_y
        {
            for _ in 0..count {
                self.value_stack.pop()?;
            }
            return Ok(());
        }
        let zone = self.graphics.zone_mut(ZonePointer::Glyph);
        for _ in 0..count {
            let p = self.value_stack.pop_usize()?;
            zone.flip_on_curve(p)?;
        }
        Ok(())
    }

    /// Flip range on.
    ///
    /// FLIPRGON[] (0x81)
    ///
    /// Pops: highpoint: highest point number in range of points to be flipped (uint32)
    ///       lowpoint: lowest point number in range of points to be flipped (uint32)
    ///
    /// Flips a range of points beginning with lowpoint and ending with highpoint so that
    /// any off the curve points become on the curve points. The points are not marked as
    /// touched.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#flip-range-on>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5056>
    pub(super) fn op_fliprgon(&mut self) -> OpResult {
        self.set_on_curve_for_range(true)
    }

    /// Flip range off.
    ///
    /// FLIPRGOFF[] (0x82)
    ///
    /// Pops: highpoint: highest point number in range of points to be flipped (uint32)
    ///       lowpoint: lowest point number in range of points to be flipped (uint32)
    ///
    /// Flips a range of points beginning with lowpoint and ending with
    /// highpoint so that any on the curve points become off the curve points.
    /// The points are not marked as touched.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#flip-range-off>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5094>
    pub(super) fn op_fliprgoff(&mut self) -> OpResult {
        self.set_on_curve_for_range(false)
    }

    /// Shift point by the last point.
    ///
    /// SHP\[a\] (0x32 - 0x33)
    ///
    /// a: 0: uses rp2 in the zone pointed to by zp1
    ///    1: uses rp1 in the zone pointed to by zp0
    ///
    /// Pops: p: point to be shifted
    ///
    /// Uses the loop counter.
    ///
    /// Shift point p by the same amount that the reference point has been
    /// shifted. Point p is shifted along the freedom_vector so that the
    /// distance between the new position of point p and the current position
    /// of point p is the same as the distance between the current position
    /// of the reference point and the original position of the reference point.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#shift-point-by-the-last-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5211>
    pub(super) fn op_shp(&mut self, opcode: u8) -> OpResult {
        let gs = &mut self.graphics;
        let PointDisplacement { dx, dy, .. } = gs.point_displacement(opcode)?;
        let count = gs.loop_counter;
        gs.loop_counter = 1;
        for _ in 0..count {
            let p = self.value_stack.pop_usize()?;
            gs.move_zp2_point(p, dx, dy, true)?;
        }
        Ok(())
    }

    /// Shift contour by the last point.
    ///
    /// SHC\[a\] (0x34 - 0x35)
    ///
    /// a: 0: uses rp2 in the zone pointed to by zp1
    ///    1: uses rp1 in the zone pointed to by zp0
    ///
    /// Pops: c: contour to be shifted
    ///
    /// Shifts every point on contour c by the same amount that the reference
    /// point has been shifted. Each point is shifted along the freedom_vector
    /// so that the distance between the new position of the point and the old
    /// position of that point is the same as the distance between the current
    /// position of the reference point and the original position of the
    /// reference point. The distance is measured along the projection_vector.
    /// If the reference point is one of the points defining the contour, the
    /// reference point is not moved by this instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#shift-contour-by-the-last-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5266>
    pub(super) fn op_shc(&mut self, opcode: u8) -> OpResult {
        let gs = &mut self.graphics;
        let contour_ix = self.value_stack.pop_usize()?;
        if !gs.is_pedantic && contour_ix >= gs.zp2().contours.len() {
            return Ok(());
        }
        let point_disp = gs.point_displacement(opcode)?;
        let start = if contour_ix != 0 {
            gs.zp2().contour(contour_ix - 1)? as usize + 1
        } else {
            0
        };
        let end = if gs.zp2.is_twilight() {
            gs.zp2().points.len()
        } else {
            gs.zp2().contour(contour_ix)? as usize + 1
        };
        for i in start..end {
            if point_disp.zone != gs.zp2 || point_disp.point_ix != i {
                gs.move_zp2_point(i, point_disp.dx, point_disp.dy, true)?;
            }
        }
        Ok(())
    }

    /// Shift zone by the last point.
    ///
    /// SHZ\[a\] (0x36 - 0x37)
    ///
    /// a: 0: uses rp2 in the zone pointed to by zp1
    ///    1: uses rp1 in the zone pointed to by zp0
    ///
    /// Pops: e: zone to be shifted
    ///
    /// Shift the points in the specified zone (Z1 or Z0) by the same amount
    /// that the reference point has been shifted. The points in the zone are
    /// shifted along the freedom_vector so that the distance between the new
    /// position of the shifted points and their old position is the same as
    /// the distance between the current position of the reference point and
    /// the original position of the reference point.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#shift-zone-by-the-last-pt>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5318>
    pub(super) fn op_shz(&mut self, opcode: u8) -> OpResult {
        let _e = ZonePointer::try_from(self.value_stack.pop()?)?;
        let gs = &mut self.graphics;
        let point_disp = gs.point_displacement(opcode)?;
        let end = if gs.zp2.is_twilight() {
            gs.zp2().points.len()
        } else if !gs.zp2().contours.is_empty() {
            *gs.zp2()
                .contours
                .last()
                .ok_or(HintErrorKind::InvalidContourIndex(0))? as usize
                + 1
        } else {
            0
        };
        for i in 0..end {
            if point_disp.zone != gs.zp2 || i != point_disp.point_ix {
                gs.move_zp2_point(i, point_disp.dx, point_disp.dy, false)?;
            }
        }
        Ok(())
    }

    /// Shift point by a pixel amount.
    ///
    /// SHPIX (0x38)
    ///
    /// Pops: amount: magnitude of the shift (F26Dot6)
    ///       p1, p2,.. pn: points to be shifted
    ///
    /// Uses the loop counter.
    ///
    /// Shifts the points specified by the amount stated. When the loop
    /// variable is used, the amount to be shifted is put onto the stack
    /// only once. That is, if loop = 3, then the contents of the top of
    /// the stack should be point p1, point p2, point p3, amount. The value
    /// amount is expressed in sixty-fourths of a pixel.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#shift-point-by-a-pixel-amount>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5366>
    pub(super) fn op_shpix(&mut self) -> OpResult {
        let gs = &mut self.graphics;
        let in_twilight = gs.zp0.is_twilight() || gs.zp1.is_twilight() || gs.zp2.is_twilight();
        let amount = self.value_stack.pop()?;
        let dx = F26Dot6::from_bits(math::mul14(amount, gs.freedom_vector.x));
        let dy = F26Dot6::from_bits(math::mul14(amount, gs.freedom_vector.y));
        let count = gs.loop_counter;
        gs.loop_counter = 1;
        let did_iup = gs.did_iup_x && gs.did_iup_y;
        for _ in 0..count {
            let p = self.value_stack.pop_usize()?;
            if gs.backward_compatibility {
                if in_twilight
                    || (!did_iup
                        && ((gs.is_composite && gs.freedom_vector.y != 0)
                            || gs.zp2().is_touched(p, CoordAxis::Y)?))
                {
                    gs.move_zp2_point(p, dx, dy, true)?;
                }
            } else {
                gs.move_zp2_point(p, dx, dy, true)?;
            }
        }
        Ok(())
    }

    /// Move stack indirect relative point.
    ///
    /// MSIRP\[a\] (0x3A - 0x3B)
    ///
    /// a: 0: do not set rp0 to p
    ///    1: set rp0 to p
    ///
    /// Pops: d: distance (F26Dot6)
    ///       p: point number
    ///
    /// Makes the distance between a point p and rp0 equal to the value
    /// specified on the stack. The distance on the stack is in fractional
    /// pixels (F26Dot6). An MSIRP has the same effect as a MIRP instruction
    /// except that it takes its value from the stack rather than the Control
    /// Value Table. As a result, the cut_in does not affect the results of a
    /// MSIRP. Additionally, MSIRP is unaffected by the round_state.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#move-stack-indirect-relative-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5439>
    pub(super) fn op_msirp(&mut self, opcode: u8) -> OpResult {
        let gs = &mut self.graphics;
        let distance = self.value_stack.pop_f26dot6()?;
        let point_ix = self.value_stack.pop_usize()?;
        if !gs.is_pedantic && !gs.in_bounds([(gs.zp1, point_ix), (gs.zp0, gs.rp0)]) {
            return Ok(());
        }
        if gs.zp1.is_twilight() {
            *gs.zp1_mut().point_mut(point_ix)? = gs.zp0().original(gs.rp0)?;
            gs.move_original(gs.zp1, point_ix, distance)?;
            *gs.zp1_mut().point_mut(point_ix)? = gs.zp1().original(point_ix)?;
        }
        let d = gs.project(gs.zp1().point(point_ix)?, gs.zp0().point(gs.rp0)?);
        gs.move_point(gs.zp1, point_ix, distance.wrapping_sub(d))?;
        gs.rp1 = gs.rp0;
        gs.rp2 = point_ix;
        if (opcode & 1) != 0 {
            gs.rp0 = point_ix;
        }
        Ok(())
    }

    /// Move direct absolute point.
    ///
    /// MDAP\[a\] (0x2E - 0x2F)
    ///
    /// a: 0: do not round the value
    ///    1: round the value
    ///
    /// Pops: p: point number
    ///
    /// Sets the reference points rp0 and rp1 equal to point p. If a=1, this
    /// instruction rounds point p to the grid point specified by the state
    /// variable round_state. If a=0, it simply marks the point as touched in
    /// the direction(s) specified by the current freedom_vector. This command
    /// is often used to set points in the twilight zone.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#move-direct-absolute-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5487>
    pub(super) fn op_mdap(&mut self, opcode: u8) -> OpResult {
        let gs = &mut self.graphics;
        let p = self.value_stack.pop_usize()?;
        if !gs.is_pedantic && !gs.in_bounds([(gs.zp0, p)]) {
            gs.rp0 = p;
            gs.rp1 = p;
            return Ok(());
        }
        let distance = if (opcode & 1) != 0 {
            let cur_dist = gs.project(gs.zp0().point(p)?, Default::default());
            gs.round(cur_dist) - cur_dist
        } else {
            F26Dot6::ZERO
        };
        gs.move_point(gs.zp0, p, distance)?;
        gs.rp0 = p;
        gs.rp1 = p;
        Ok(())
    }

    /// Move indirect absolute point.
    ///
    /// MIAP\[a\] (0x3E - 0x3F)
    ///
    /// a: 0: do not round the distance and don't use control value cutin
    ///    1: round the distance and use control value cutin
    ///
    /// Pops: n: CVT entry number
    ///       p: point number
    ///
    /// Moves point p to the absolute coordinate position specified by the nth
    /// Control Value Table entry. The coordinate is measured along the current
    /// projection_vector. If a=1, the position will be rounded as specified by
    /// round_state. If a=1, and if the device space difference between the CVT
    /// value and the original position is greater than the
    /// control_value_cut_in, then the original position will be rounded
    /// (instead of the CVT value.)
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#move-indirect-absolute-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5526>
    pub(super) fn op_miap(&mut self, opcode: u8) -> OpResult {
        let gs = &mut self.graphics;
        let cvt_entry = self.value_stack.pop_usize()?;
        let point_ix = self.value_stack.pop_usize()?;
        let mut distance = self.cvt.get(cvt_entry)?;
        if gs.zp0.is_twilight() {
            // Special behavior for twilight zone.
            // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5548>
            let fv = gs.freedom_vector;
            let z = gs.zp0_mut();
            let original_point = z.original_mut(point_ix)?;
            original_point.x = F26Dot6::from_bits(math::mul14(distance.to_bits(), fv.x));
            original_point.y = F26Dot6::from_bits(math::mul14(distance.to_bits(), fv.y));
            *z.point_mut(point_ix)? = *original_point;
        }
        let original_distance = gs.project(gs.zp0().point(point_ix)?, Default::default());
        if (opcode & 1) != 0 {
            let delta = (distance.wrapping_sub(original_distance)).abs();
            if delta > gs.control_value_cutin {
                distance = original_distance;
            }
            distance = gs.round(distance);
        }
        gs.move_point(gs.zp0, point_ix, distance.wrapping_sub(original_distance))?;
        gs.rp0 = point_ix;
        gs.rp1 = point_ix;
        Ok(())
    }

    /// Move direct relative point.
    ///
    /// MDRP\[abcde\] (0xC0 - 0xDF)
    ///
    /// a: 0: do not set rp0 to point p after move
    ///    1: do set rp0 to point p after move
    /// b: 0: do not keep distance greater than or equal to minimum_distance
    ///    1: keep distance greater than or equal to minimum_distance
    /// c: 0: do not round distance
    ///    1: round the distance
    /// de: distance type for engine characteristic compensation
    ///
    /// Pops: p: point number
    ///       
    /// MDRP moves point p along the freedom_vector so that the distance from
    /// its new position to the current position of rp0 is the same as the
    /// distance between the two points in the original uninstructed outline,
    /// and then adjusts it to be consistent with the Boolean settings. Note
    /// that it is only the original positions of rp0 and point p and the
    /// current position of rp0 that determine the new position of point p
    /// along the freedom_vector.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#move-direct-relative-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5610>
    pub(super) fn op_mdrp(&mut self, opcode: u8) -> OpResult {
        let gs = &mut self.graphics;
        let p = self.value_stack.pop_usize()?;
        if !gs.is_pedantic && !gs.in_bounds([(gs.zp1, p), (gs.zp0, gs.rp0)]) {
            gs.rp1 = gs.rp0;
            gs.rp2 = p;
            if (opcode & 16) != 0 {
                gs.rp0 = p;
            }
            return Ok(());
        }
        let mut original_distance = if gs.zp0.is_twilight() || gs.zp1.is_twilight() {
            gs.dual_project(gs.zp1().original(p)?, gs.zp0().original(gs.rp0)?)
        } else {
            let v1 = gs.zp1().unscaled(p);
            let v2 = gs.zp0().unscaled(gs.rp0);
            let dist = gs.dual_project_unscaled(v1, v2);
            F26Dot6::from_bits(math::mul(dist, gs.unscaled_to_pixels()))
        };
        let cutin = gs.single_width_cutin;
        let value = gs.single_width;
        if cutin > F26Dot6::ZERO
            && original_distance < value + cutin
            && original_distance > value - cutin
        {
            original_distance = if original_distance >= F26Dot6::ZERO {
                value
            } else {
                -value
            };
        }
        // round flag
        let mut distance = if (opcode & 4) != 0 {
            gs.round(original_distance)
        } else {
            original_distance
        };
        // minimum distance flag
        if (opcode & 8) != 0 {
            let min_distance = gs.min_distance;
            if original_distance >= F26Dot6::ZERO {
                if distance < min_distance {
                    distance = min_distance;
                }
            } else if distance > -min_distance {
                distance = -min_distance;
            }
        }
        original_distance = gs.project(gs.zp1().point(p)?, gs.zp0().point(gs.rp0)?);
        gs.move_point(gs.zp1, p, distance.wrapping_sub(original_distance))?;
        gs.rp1 = gs.rp0;
        gs.rp2 = p;
        if (opcode & 16) != 0 {
            gs.rp0 = p;
        }
        Ok(())
    }

    /// Move indirect relative point.
    ///
    /// MIRP\[abcde\] (0xE0 - 0xFF)
    ///
    /// a: 0: do not set rp0 to point p after move
    ///    1: do set rp0 to point p after move
    /// b: 0: do not keep distance greater than or equal to minimum_distance
    ///    1: keep distance greater than or equal to minimum_distance
    /// c: 0: do not round distance and do not look at control_value_cutin
    ///    1: round the distance and look at control_value_cutin
    /// de: distance type for engine characteristic compensation
    ///
    /// Pops: n: CVT entry number
    ///       p: point number
    ///       
    /// A MIRP instruction makes it possible to preserve the distance between
    /// two points subject to a number of qualifications. Depending upon the
    /// setting of Boolean flag b, the distance can be kept greater than or
    /// equal to the value established by the minimum_distance state variable.
    /// Similarly, the instruction can be set to round the distance according
    /// to the round_state graphics state variable. The value of the minimum
    /// distance variable is the smallest possible value the distance between
    /// two points can be rounded to. Additionally, if the c Boolean is set,
    /// the MIRP instruction acts subject to the control_value_cut_in. If the
    /// difference between the actual measurement and the value in the CVT is
    /// sufficiently small (less than the cut_in_value), the CVT value will be
    /// used and not the actual value. If the device space difference between
    /// this distance from the CVT and the single_width_value is smaller than
    /// the single_width_cut_in, then use the single_width_value rather than
    /// the outline or Control Value Table distance.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#move-indirect-relative-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5731>
    pub(super) fn op_mirp(&mut self, opcode: u8) -> OpResult {
        let gs = &mut self.graphics;
        let n = (self.value_stack.pop()? + 1) as usize;
        let p = self.value_stack.pop_usize()?;
        if !gs.is_pedantic
            && (!gs.in_bounds([(gs.zp1, p), (gs.zp0, gs.rp0)]) || (n > self.cvt.len()))
        {
            gs.rp1 = gs.rp0;
            if (opcode & 16) != 0 {
                gs.rp0 = p;
            }
            gs.rp2 = p;
            return Ok(());
        }
        let mut cvt_distance = if n == 0 {
            F26Dot6::ZERO
        } else {
            self.cvt.get(n - 1)?
        };
        // single width test
        let cutin = gs.single_width_cutin;
        let value = gs.single_width;
        let mut delta = cvt_distance.wrapping_sub(value).abs();
        if delta < cutin {
            cvt_distance = if cvt_distance >= F26Dot6::ZERO {
                value
            } else {
                -value
            };
        }
        if gs.zp1.is_twilight() {
            let fv = gs.freedom_vector;
            let point = {
                let d = cvt_distance.to_bits();
                let p2 = gs.zp0().original(gs.rp0)?;
                let p1 = gs.zp1_mut().original_mut(p)?;
                p1.x = p2.x + F26Dot6::from_bits(math::mul(d, fv.x));
                p1.y = p2.y + F26Dot6::from_bits(math::mul(d, fv.y));
                *p1
            };
            *gs.zp1_mut().point_mut(p)? = point;
        }
        let original_distance = gs.dual_project(gs.zp1().original(p)?, gs.zp0().original(gs.rp0)?);
        let current_distance = gs.project(gs.zp1().point(p)?, gs.zp0().point(gs.rp0)?);
        // auto flip test
        if gs.auto_flip && (original_distance.to_bits() ^ cvt_distance.to_bits()) < 0 {
            cvt_distance = -cvt_distance;
        }
        // control value cutin and round
        let mut distance = if (opcode & 4) != 0 {
            if gs.zp0 == gs.zp1 {
                delta = cvt_distance.wrapping_sub(original_distance).abs();
                if delta > gs.control_value_cutin {
                    cvt_distance = original_distance;
                }
            }
            gs.round(cvt_distance)
        } else {
            cvt_distance
        };
        // minimum distance test
        if (opcode & 8) != 0 {
            let min_distance = gs.min_distance;
            if original_distance >= F26Dot6::ZERO {
                if distance < min_distance {
                    distance = min_distance
                };
            } else if distance > -min_distance {
                distance = -min_distance
            }
        }
        gs.move_point(gs.zp1, p, distance.wrapping_sub(current_distance))?;
        gs.rp1 = gs.rp0;
        if (opcode & 16) != 0 {
            gs.rp0 = p;
        }
        gs.rp2 = p;
        Ok(())
    }

    /// Align relative point.
    ///
    /// ALIGNRP[] (0x3C)
    ///
    /// Pops: p: point number (uint32)
    ///
    /// Uses the loop counter.
    ///
    /// Reduces the distance between rp0 and point p to zero. Since distance
    /// is measured along the projection_vector and movement is along the
    /// freedom_vector, the effect of the instruction is to align points.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#align-relative-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5882>
    pub(super) fn op_alignrp(&mut self) -> OpResult {
        let gs = &mut self.graphics;
        let count = gs.loop_counter;
        gs.loop_counter = 1;
        for _ in 0..count {
            let p = self.value_stack.pop_usize()?;
            let distance = gs.project(gs.zp1().point(p)?, gs.zp0().point(gs.rp0)?);
            gs.move_point(gs.zp1, p, -distance)?;
        }
        Ok(())
    }

    /// Move point to intersection of two lines.
    ///
    /// ISECT[] (0x0F)
    ///
    /// Pops: b1: end point of line 2
    ///       b0: start point of line 2
    ///       a1: end point of line 1
    ///       a0: start point of line 1
    ///       p: point to move.
    ///
    /// Puts point p at the intersection of the lines A and B. The points a0
    /// and a1 define line A. Similarly, b0 and b1 define line B. ISECT
    /// ignores the freedom_vector in moving point p.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#moves-point-p-to-the-intersection-of-two-lines>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5934>
    pub(super) fn op_isect(&mut self) -> OpResult {
        let gs = &mut self.graphics;
        let b1 = self.value_stack.pop_usize()?;
        let b0 = self.value_stack.pop_usize()?;
        let a1 = self.value_stack.pop_usize()?;
        let a0 = self.value_stack.pop_usize()?;
        let point_ix = self.value_stack.pop_usize()?;
        // Lots of funky fixed point math so just map these to i32 to avoid
        // a bunch of wrapping/unwrapping.
        // To shreds you say!
        let [pa0, pa1] = {
            let z = gs.zp1();
            [z.point(a0)?, z.point(a1)?].map(|p| p.map(F26Dot6::to_bits))
        };
        let [pb0, pb1] = {
            let z = gs.zp0();
            [z.point(b0)?, z.point(b1)?].map(|p| p.map(F26Dot6::to_bits))
        };
        let dbx = pb1.x - pb0.x;
        let dby = pb1.y - pb0.y;
        let dax = pa1.x - pa0.x;
        let day = pa1.y - pa0.y;
        let dx = pb0.x - pa0.x;
        let dy = pb0.y - pa0.y;
        use math::mul_div;
        let discriminant = mul_div(dax, -dby, 0x40) + mul_div(day, dbx, 0x40);
        let dotproduct = mul_div(dax, dbx, 0x40) + mul_div(day, dby, 0x40);
        // Useful context from FreeType:
        //
        // "The discriminant above is actually a cross product of vectors
        // da and db. Together with the dot product, they can be used as
        // surrogates for sine and cosine of the angle between the vectors.
        // Indeed,
        //       dotproduct   = |da||db|cos(angle)
        //       discriminant = |da||db|sin(angle)
        // We use these equations to reject grazing intersections by
        // thresholding abs(tan(angle)) at 1/19, corresponding to 3 degrees."
        //
        // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L5986>
        if discriminant.wrapping_abs().wrapping_mul(19) > dotproduct.abs() {
            let v = mul_div(dx, -dby, 0x40) + mul_div(dy, dbx, 0x40);
            let x = mul_div(v, dax, discriminant);
            let y = mul_div(v, day, discriminant);
            let point = gs.zp2_mut().point_mut(point_ix)?;
            point.x = F26Dot6::from_bits(pa0.x + x);
            point.y = F26Dot6::from_bits(pa0.y + y);
        } else {
            let point = gs.zp2_mut().point_mut(point_ix)?;
            point.x = F26Dot6::from_bits((pa0.x + pa1.x + pb0.x + pb1.x) / 4);
            point.y = F26Dot6::from_bits((pa0.y + pa1.y + pb0.y + pb1.y) / 4);
        }
        gs.zp2_mut().touch(point_ix, CoordAxis::Both)?;
        Ok(())
    }

    /// Align points.
    ///
    /// ALIGNPTS[] (0x27)
    ///
    /// Pops: p1: point number
    ///       p2: point number
    ///
    /// Makes the distance between point 1 and point 2 zero by moving both
    /// along the freedom_vector to the average of both their projections
    /// along the projection_vector.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#align-points>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6030>
    pub(super) fn op_alignpts(&mut self) -> OpResult {
        let p2 = self.value_stack.pop_usize()?;
        let p1 = self.value_stack.pop_usize()?;
        let gs = &mut self.graphics;
        let distance = F26Dot6::from_bits(
            gs.project(gs.zp0().point(p2)?, gs.zp1().point(p1)?)
                .to_bits()
                / 2,
        );
        gs.move_point(gs.zp1, p1, distance)?;
        gs.move_point(gs.zp0, p2, -distance)?;
        Ok(())
    }

    /// Interpolate point by last relative stretch.
    ///
    /// IP[] (0x39)
    ///
    /// Pops: p: point number
    ///
    /// Uses the loop counter.
    ///
    /// Moves point p so that its relationship to rp1 and rp2 is the same as it
    /// was in the original uninstructed outline. Measurements are made along
    /// the projection_vector, and movement to satisfy the interpolation
    /// relationship is constrained to be along the freedom_vector. This
    /// instruction is not valid if rp1 and rp2 have the same position on the
    /// projection_vector.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#interpolate-point-by-the-last-relative-stretch>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6065>
    pub(super) fn op_ip(&mut self) -> OpResult {
        let gs = &mut self.graphics;
        let count = gs.loop_counter;
        gs.loop_counter = 1;
        if !gs.is_pedantic && !gs.in_bounds([(gs.zp0, gs.rp1), (gs.zp1, gs.rp2)]) {
            return Ok(());
        }
        let in_twilight = gs.zp0.is_twilight() || gs.zp1.is_twilight() || gs.zp2.is_twilight();
        let orus_base = if in_twilight {
            gs.zp0().original(gs.rp1)?
        } else {
            gs.zp0().unscaled(gs.rp1).map(F26Dot6::from_bits)
        };
        let cur_base = gs.zp0().point(gs.rp1)?;
        let old_range = if in_twilight {
            gs.dual_project(gs.zp1().original(gs.rp2)?, orus_base)
        } else {
            gs.dual_project(gs.zp1().unscaled(gs.rp2).map(F26Dot6::from_bits), orus_base)
        };
        let cur_range = gs.project(gs.zp1().point(gs.rp2)?, cur_base);
        for _ in 0..count {
            let point = self.value_stack.pop_usize()?;
            if !gs.is_pedantic && !gs.in_bounds([(gs.zp2, point)]) {
                continue;
            }
            let original_distance = if in_twilight {
                gs.dual_project(gs.zp2().original(point)?, orus_base)
            } else {
                gs.dual_project(gs.zp2().unscaled(point).map(F26Dot6::from_bits), orus_base)
            };
            let cur_distance = gs.project(gs.zp2().point(point)?, cur_base);
            let new_distance = if original_distance != F26Dot6::ZERO {
                if old_range != F26Dot6::ZERO {
                    F26Dot6::from_bits(math::mul_div(
                        original_distance.to_bits(),
                        cur_range.to_bits(),
                        old_range.to_bits(),
                    ))
                } else {
                    original_distance
                }
            } else {
                F26Dot6::ZERO
            };
            gs.move_point(gs.zp2, point, new_distance.wrapping_sub(cur_distance))?;
        }
        Ok(())
    }

    /// Interpolate untouched points through the outline.
    ///
    /// IUP\[a\] (0x30 - 0x31)
    ///
    /// a: 0: interpolate in the y-direction
    ///    1: interpolate in the x-direction
    ///
    /// Considers a glyph contour by contour, moving any untouched points in
    /// each contour that are between a pair of touched points. If the
    /// coordinates of an untouched point were originally between those of
    /// the touched pair, it is linearly interpolated between the new
    /// coordinates, otherwise the untouched point is shifted by the amount
    /// the nearest touched point is shifted.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#interpolate-untouched-points-through-the-outline>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6391>
    pub(super) fn op_iup(&mut self, opcode: u8) -> OpResult {
        let gs = &mut self.graphics;
        let axis = if (opcode & 1) != 0 {
            CoordAxis::X
        } else {
            CoordAxis::Y
        };
        let mut run = true;
        // In backward compatibility mode, allow IUP until it has been done on
        // both axes.
        if gs.backward_compatibility {
            if gs.did_iup_x && gs.did_iup_y {
                run = false;
            }
            if axis == CoordAxis::X {
                gs.did_iup_x = true;
            } else {
                gs.did_iup_y = true;
            }
        }
        if run {
            gs.zone_mut(ZonePointer::Glyph).iup(axis)?;
        }
        Ok(())
    }

    /// Untouch point.
    ///
    /// UTP[] (0x29)
    ///
    /// Pops: p: point number (uint32)
    ///
    /// Marks point p as untouched. A point may be touched in the x direction,
    /// the y direction, both, or neither. This instruction uses the current
    /// freedom_vector to determine whether to untouch the point in the
    /// x-direction, the y direction, or both. Points that are marked as
    /// untouched will be moved by an IUP (interpolate untouched points)
    /// instruction. Using UTP you can ensure that a point will be affected
    /// by IUP even if it was previously touched.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#untouch-point>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6222>
    pub(super) fn op_utp(&mut self) -> OpResult {
        let p = self.value_stack.pop_usize()?;
        let coord_axis = match (
            self.graphics.freedom_vector.x != 0,
            self.graphics.freedom_vector.y != 0,
        ) {
            (true, true) => Some(CoordAxis::Both),
            (true, false) => Some(CoordAxis::X),
            (false, true) => Some(CoordAxis::Y),
            (false, false) => None,
        };
        if let Some(coord_axis) = coord_axis {
            self.graphics.zp0_mut().untouch(p, coord_axis)?;
        }
        Ok(())
    }

    /// Helper for FLIPRGON and FLIPRGOFF.
    fn set_on_curve_for_range(&mut self, on: bool) -> OpResult {
        let high_point = self.value_stack.pop_usize()?;
        let low_point = self.value_stack.pop_usize()?;
        // high_point is inclusive but Zone::set_on_curve takes an exclusive
        // range
        let high_point = high_point
            .checked_add(1)
            .ok_or(HintErrorKind::InvalidPointIndex(high_point))?;
        // In backward compatibility mode, don't flip points after IUP has
        // been done.
        if self.graphics.backward_compatibility
            && self.graphics.did_iup_x
            && self.graphics.did_iup_y
        {
            return Ok(());
        }
        self.graphics
            .zone_mut(ZonePointer::Glyph)
            .set_on_curve(low_point, high_point, on)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::MockEngine, math, CoordAxis, Engine, ZonePointer};
    use raw::{
        tables::glyf::{bytecode::Opcode, PointMarker},
        types::{F26Dot6, Point},
    };

    #[test]
    fn flip_point() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // Points all start as off-curve in the mock engine.
        // Flip every odd point in the first 10
        let count = 5;
        // First, set the loop counter:
        engine.value_stack.push(count).unwrap();
        engine.op_sloop().unwrap();
        // Now push the point indices
        for i in (1..=9).step_by(2) {
            engine.value_stack.push(i).unwrap();
        }
        assert_eq!(engine.value_stack.len(), count as usize);
        // And flip!
        engine.op_flippt().unwrap();
        let flags = &engine.graphics.zones[1].flags;
        for i in 0..10 {
            // Odd points are now on-curve
            assert_eq!(flags[i].is_on_curve(), i & 1 != 0);
        }
    }

    /// Backward compat + IUP state prevents flipping.
    #[test]
    fn state_prevents_flip_point() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // Points all start as off-curve in the mock engine.
        // Flip every odd point in the first 10
        let count = 5;
        // First, set the loop counter:
        engine.value_stack.push(count).unwrap();
        engine.op_sloop().unwrap();
        // Now push the point indices
        for i in (1..=9).step_by(2) {
            engine.value_stack.push(i).unwrap();
        }
        assert_eq!(engine.value_stack.len(), count as usize);
        // Prevent flipping
        engine.graphics.backward_compatibility = true;
        engine.graphics.did_iup_x = true;
        engine.graphics.did_iup_y = true;
        // But try anyway
        engine.op_flippt().unwrap();
        let flags = &engine.graphics.zones[1].flags;
        for i in 0..10 {
            // All points are still off-curve
            assert!(!flags[i].is_on_curve());
        }
    }

    #[test]
    fn flip_range_on_off() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // Points all start as off-curve in the mock engine.
        // Flip 10..=20 on
        engine.value_stack.push(10).unwrap();
        engine.value_stack.push(20).unwrap();
        engine.op_fliprgon().unwrap();
        for (i, flag) in engine.graphics.zones[1].flags.iter().enumerate() {
            assert_eq!(flag.is_on_curve(), (10..=20).contains(&i));
        }
        // Now flip 12..=15 off
        engine.value_stack.push(12).unwrap();
        engine.value_stack.push(15).unwrap();
        engine.op_fliprgoff().unwrap();
        for (i, flag) in engine.graphics.zones[1].flags.iter().enumerate() {
            assert_eq!(
                flag.is_on_curve(),
                (10..=11).contains(&i) || (16..=20).contains(&i)
            );
        }
    }

    /// Backward compat + IUP state prevents flipping.
    #[test]
    fn state_prevents_flip_range_on_off() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // Prevent flipping
        engine.graphics.backward_compatibility = true;
        engine.graphics.did_iup_x = true;
        engine.graphics.did_iup_y = true;
        // Points all start as off-curve in the mock engine.
        // Try to flip 10..=20 on
        engine.value_stack.push(10).unwrap();
        engine.value_stack.push(20).unwrap();
        engine.op_fliprgon().unwrap();
        for flag in engine.graphics.zones[1].flags.iter() {
            assert!(!flag.is_on_curve());
        }
        // Reset all points to on
        for flag in engine.graphics.zones[1].flags.iter_mut() {
            flag.set_on_curve();
        }
        // Now try to flip 12..=15 off
        engine.value_stack.push(12).unwrap();
        engine.value_stack.push(15).unwrap();
        engine.op_fliprgoff().unwrap();
        for flag in engine.graphics.zones[1].flags.iter() {
            assert!(flag.is_on_curve());
        }
    }

    #[test]
    fn untouch_point() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // Touch all points in both axes to start.
        let count = engine.graphics.zones[1].points.len();
        for i in 0..count {
            engine.graphics.zones[1].touch(i, CoordAxis::Both).unwrap();
        }
        let mut untouch = |point_ix: usize, fx, fy, marker| {
            assert!(engine.graphics.zp0().flags[point_ix].has_marker(marker));
            // Untouch axis is based on freedom vector:
            engine.graphics.freedom_vector.x = fx;
            engine.graphics.freedom_vector.y = fy;
            engine.value_stack.push(point_ix as i32).unwrap();
            engine.op_utp().unwrap();
            assert!(!engine.graphics.zp0().flags[point_ix].has_marker(marker));
        };
        // Untouch point 0 in x axis
        untouch(0, 1, 0, PointMarker::TOUCHED_X);
        // Untouch point 1 in y axis
        untouch(1, 0, 1, PointMarker::TOUCHED_Y);
        // untouch point 2 in both axes
        untouch(2, 1, 1, PointMarker::TOUCHED);
    }

    #[test]
    fn shp() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        engine.graphics.zp2 = ZonePointer::Glyph;
        engine.graphics.rp2 = 1;
        let point = engine.graphics.zones[1].point_mut(1).unwrap();
        point.x = F26Dot6::from_bits(132);
        point.y = F26Dot6::from_bits(-256);
        engine.value_stack.push(1).unwrap();
        engine.op_shp(0).unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(136, -254));
    }

    #[test]
    fn shc() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        engine.graphics.zp2 = ZonePointer::Glyph;
        engine.graphics.rp2 = 1;
        let point = engine.graphics.zones[1].point_mut(1).unwrap();
        point.x = F26Dot6::from_bits(132);
        point.y = F26Dot6::from_bits(-256);
        engine.value_stack.push(0).unwrap();
        engine.op_shc(0).unwrap();
        let points = engine.graphics.zones[1]
            .points
            .iter()
            .map(|p| p.map(F26Dot6::to_bits))
            .take(3)
            .collect::<Vec<_>>();
        assert_eq!(
            points,
            &[Point::new(4, 2), Point::new(132, -256), Point::new(4, 2),]
        );
    }

    #[test]
    fn shz() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        engine.graphics.zp2 = ZonePointer::Glyph;
        engine.graphics.rp2 = 1;
        let point = engine.graphics.zones[1].point_mut(1).unwrap();
        point.x = F26Dot6::from_bits(132);
        point.y = F26Dot6::from_bits(-256);
        engine.value_stack.push(0).unwrap();
        engine.op_shz(0).unwrap();
        let points = engine.graphics.zones[1]
            .points
            .iter()
            .map(|p| p.map(F26Dot6::to_bits))
            .take(3)
            .collect::<Vec<_>>();
        assert_eq!(
            points,
            &[Point::new(4, 2), Point::new(132, -256), Point::new(4, 2),]
        );
    }

    #[test]
    fn shpix() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp2 = ZonePointer::Glyph;
        let point = engine.graphics.zones[1].point_mut(1).unwrap();
        point.x = F26Dot6::from_bits(132);
        point.y = F26Dot6::from_bits(-256);
        // point index
        engine.value_stack.push(1).unwrap();
        // amount to move in pixels along freedom vector
        engine.value_stack.push(42).unwrap();
        engine.op_shpix().unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(170, -237));
    }

    #[test]
    fn msirp() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        engine.graphics.zp1 = ZonePointer::Glyph;
        let point = engine.graphics.zones[1].point_mut(1).unwrap();
        point.x = F26Dot6::from_bits(132);
        point.y = F26Dot6::from_bits(-256);
        // point index
        engine.value_stack.push(1).unwrap();
        // amount to move in pixels along freedom vector
        engine.value_stack.push(-42).unwrap();
        engine.op_msirp(0).unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(91, -277));
        assert_eq!(engine.graphics.rp0, 0);
        // opcode with bit 0 set changes rp0 to point_ix
        engine.value_stack.push(4).unwrap();
        engine.value_stack.push(0).unwrap();
        engine.op_msirp(1).unwrap();
        assert_eq!(engine.graphics.rp0, 4);
    }

    #[test]
    fn mdap() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        // with rounding
        engine.set_point_f26dot6(1, 1, (132, -256));
        engine.value_stack.push(1).unwrap();
        engine.op_mdap(1).unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(128, -258));
        // without rounding
        engine.set_point_f26dot6(1, 2, (132, -256));
        engine.value_stack.push(2).unwrap();
        engine.op_mdap(0).unwrap();
        let point = engine.graphics.zones[1].point(2).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(132, -256));
    }

    #[test]
    fn miap() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        // set a CVT distance
        engine.cvt.set(1, F26Dot6::from_f64(0.75)).unwrap();
        // with rounding
        engine.set_point_f26dot6(1, 1, (132, -256));
        engine.value_stack.push(1).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_miap(1).unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(186, -229));
        // without rounding
        engine.set_point_f26dot6(1, 2, (132, -256));
        engine.value_stack.push(2).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_miap(0).unwrap();
        let point = engine.graphics.zones[1].point(2).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(171, -236));
    }

    /// Tests bit 'a' of MDRP which just sets rp0 to the adjusted point
    /// after move.
    #[test]
    fn mdrp_rp0() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        engine.graphics.rp0 = 0;
        // Don't change rp0
        engine.value_stack.push(1).unwrap();
        engine.op_mdrp(Opcode::MDRP00000 as _).unwrap();
        assert_eq!(engine.graphics.rp0, 0);
        // Change rp0
        engine.value_stack.push(1).unwrap();
        engine.op_mdrp(Opcode::MDRP10000 as _).unwrap();
        assert_eq!(engine.graphics.rp0, 1);
    }

    /// Test bit "b" which controls whether distances are adjusted
    /// to the minimum_distance field of GraphicsState.
    #[test]
    fn mdrp_mindist() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        // without min distance check
        engine.set_point_f26dot6(1, 1, (132, -256));
        engine.value_stack.push(1).unwrap();
        engine.op_mdrp(Opcode::MDRP00000 as _).unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(128, -258));
        // with min distance check
        engine.set_point_f26dot6(1, 2, (132, -256));
        engine.value_stack.push(2).unwrap();
        engine.op_mdrp(Opcode::MDRP01000 as _).unwrap();
        let point = engine.graphics.zones[1].point(2).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(186, -229));
    }

    /// Test bit "c" which controls whether distances are rounded.
    #[test]
    fn mdrp_round() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        engine.op_rthg().unwrap();
        // without rounding
        engine.set_point_f26dot6(1, 1, (132, -231));
        engine.value_stack.push(1).unwrap();
        engine.op_mdrp(Opcode::MDRP00000 as _).unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(119, -238));
        // with rounding
        engine.set_point_f26dot6(1, 2, (132, -231));
        engine.value_stack.push(2).unwrap();
        engine.op_mdrp(Opcode::MDRP00100 as _).unwrap();
        let point = engine.graphics.zones[1].point(2).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(147, -223));
    }

    /// Tests bit 'a' of MIRP which just sets rp0 to the adjusted point
    /// after move.
    #[test]
    fn mirp_rp0() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        engine.graphics.rp0 = 0;
        // Don't change rp0
        engine.value_stack.push(1).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_mirp(Opcode::MIRP00000 as _).unwrap();
        assert_eq!(engine.graphics.rp0, 0);
        // Change rp0
        engine.value_stack.push(1).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_mirp(Opcode::MIRP10000 as _).unwrap();
        assert_eq!(engine.graphics.rp0, 1);
    }

    /// Test bit "b" which controls whether distances are adjusted
    /// to the minimum_distance field of GraphicsState.
    #[test]
    fn mirp_mindist() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        // set a CVT distance
        engine.cvt.set(1, F26Dot6::from_f64(0.75)).unwrap();
        // without min distance check
        engine.set_point_f26dot6(1, 1, (132, -256));
        engine.value_stack.push(1).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_mirp(Opcode::MIRP00000 as _).unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(171, -236));
        // with min distance check
        engine.set_point_f26dot6(1, 2, (132, -256));
        engine.value_stack.push(2).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_mirp(Opcode::MIRP01000 as _).unwrap();
        let point = engine.graphics.zones[1].point(2).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(186, -229));
    }

    /// Test bit "c" which controls whether distances are rounded.
    #[test]
    fn mirp_round() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        // set a CVT distance
        engine.cvt.set(1, F26Dot6::from_f64(0.75)).unwrap();
        engine.op_rthg().unwrap();
        // without rounding
        engine.set_point_f26dot6(1, 1, (132, -231));
        engine.value_stack.push(1).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_mirp(Opcode::MIRP00000 as _).unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(162, -216));
        // with rounding
        engine.set_point_f26dot6(1, 2, (132, -231));
        engine.value_stack.push(2).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_mirp(Opcode::MIRP00100 as _).unwrap();
        let point = engine.graphics.zones[1].point(2).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(147, -223));
    }

    #[test]
    fn alignrp() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        engine.graphics.zp1 = ZonePointer::Glyph;
        engine.graphics.rp0 = 0;
        engine.set_point_f26dot6(1, 0, (132, -231));
        engine.set_point_f26dot6(1, 1, (-72, 109));
        engine.value_stack.push(1).unwrap();
        engine.op_alignrp().unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(-45, 122));
    }

    #[test]
    fn isect() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        engine.graphics.zp0 = ZonePointer::Glyph;
        engine.graphics.zp1 = ZonePointer::Glyph;
        engine.graphics.rp0 = 0;
        // Two points for line 1
        engine.set_point_f26dot6(1, 0, (0, 0));
        engine.set_point_f26dot6(1, 1, (100, 100));
        // And two more for line 2
        engine.set_point_f26dot6(1, 2, (0, 100));
        engine.set_point_f26dot6(1, 3, (100, 0));
        // Push point numbers: first is the point where the
        // intersection should be stored.
        for ix in [4, 0, 1, 2, 3] {
            engine.value_stack.push(ix).unwrap();
        }
        engine.op_isect().unwrap();
        let point = engine.graphics.zones[1].point(4).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(50, 50));
    }

    #[test]
    fn alignpts() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        engine.graphics.zp1 = ZonePointer::Glyph;
        engine.set_point_f26dot6(1, 0, (132, -231));
        engine.set_point_f26dot6(1, 1, (-72, 109));
        engine.value_stack.push(0).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_alignpts().unwrap();
        let p1 = engine.graphics.zones[1].point(0).unwrap();
        let p2 = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(p1.map(F26Dot6::to_bits), Point::new(119, -238));
        assert_eq!(p2.map(F26Dot6::to_bits), Point::new(-59, 116));
    }

    #[test]
    fn ip() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        engine.graphics.zp1 = ZonePointer::Glyph;
        engine.graphics.zp2 = ZonePointer::Glyph;
        engine.graphics.rp1 = 2;
        engine.graphics.rp2 = 3;
        engine.set_point_f26dot6(1, 2, (72, -109));
        engine.set_point_f26dot6(1, 1, (132, -231));
        engine.value_stack.push(1).unwrap();
        engine.op_ip().unwrap();
        let point = engine.graphics.zones[1].point(1).unwrap();
        assert_eq!(point.map(F26Dot6::to_bits), Point::new(147, -223));
    }

    #[test]
    fn iup_flags() {
        // IUP shift and interpolate logic is tested in ../zone.rs so just
        // check the flags here.
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        assert!(!engine.graphics.did_iup_x);
        assert!(!engine.graphics.did_iup_y);
        // IUP[y]
        engine.op_iup(0).unwrap();
        assert!(!engine.graphics.did_iup_x);
        assert!(engine.graphics.did_iup_y);
        // IUP[x]
        engine.op_iup(1).unwrap();
        assert!(engine.graphics.did_iup_x);
        assert!(engine.graphics.did_iup_y);
    }

    // Add with overflow caught by fuzzer:
    // https://issues.oss-fuzz.com/issues/377736138
    #[test]
    fn flip_region_avoid_overflow() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        engine.value_stack.push(1).unwrap();
        engine.value_stack.push(-1).unwrap();
        // Just don't panic
        let _ = engine.set_on_curve_for_range(true);
    }

    fn set_test_vectors(engine: &mut Engine) {
        let v = math::normalize14(100, 50);
        engine.graphics.proj_vector = v;
        engine.graphics.dual_proj_vector = v;
        engine.graphics.freedom_vector = v;
        engine.graphics.update_projection_state();
    }

    impl Engine<'_> {
        fn set_point_f26dot6(&mut self, zone_ix: usize, point_ix: usize, xy: (i32, i32)) {
            let p = self.graphics.zones[zone_ix].point_mut(point_ix).unwrap();
            p.x = F26Dot6::from_bits(xy.0);
            p.y = F26Dot6::from_bits(xy.1);
        }
    }
}
