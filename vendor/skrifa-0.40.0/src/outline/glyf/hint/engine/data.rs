//! Reading and writing data.
//!
//! Implements 7 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#reading-and-writing-data>

use super::{
    super::{math, zone::ZonePointer},
    Engine, OpResult,
};

impl Engine<'_> {
    /// Get coordinate project in to the projection vector.
    ///
    /// GC\[a\] (0x46 - 0x47)
    ///
    /// a: 0: use current position of point p
    ///    1: use the position of point p in the original outline
    ///
    /// Pops: p: point number
    /// Pushes: value: coordinate location (F26Dot6)
    ///
    /// Measures the coordinate value of point p on the current
    /// projection_vector and pushes the value onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#get-coordinate-projected-onto-the-projection_vector>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4512>
    pub(super) fn op_gc(&mut self, opcode: u8) -> OpResult {
        let p = self.value_stack.pop_usize()?;
        let gs = &mut self.graphics;
        if !gs.is_pedantic && !gs.in_bounds([(gs.zp2, p)]) {
            self.value_stack.push(0)?;
            return Ok(());
        }
        let value = if (opcode & 1) != 0 {
            gs.dual_project(gs.zp2().original(p)?, Default::default())
        } else {
            gs.project(gs.zp2().point(p)?, Default::default())
        };
        self.value_stack.push(value.to_bits())?;
        Ok(())
    }

    /// Set coordinate from the stack using projection vector and freedom
    /// vector.
    ///
    /// SCFS[] (0x48)
    ///
    /// Pops: value: distance from origin to move point (F26Dot6)
    ///       p: point number
    ///
    /// Moves point p from its current position along the freedom_vector so
    /// that its component along the projection_vector becomes the value popped
    /// off the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#sets-coordinate-from-the-stack-using-projection_vector-and-freedom_vector>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4550>
    pub(super) fn op_scfs(&mut self) -> OpResult {
        let value = self.value_stack.pop_f26dot6()?;
        let p = self.value_stack.pop_usize()?;
        let gs = &mut self.graphics;
        let projection = gs.project(gs.zp2().point(p)?, Default::default());
        gs.move_point(gs.zp2, p, value.wrapping_sub(projection))?;
        if gs.zp2.is_twilight() {
            let twilight = gs.zone_mut(ZonePointer::Twilight);
            *twilight.original_mut(p)? = twilight.point(p)?;
        }
        Ok(())
    }

    /// Measure distance.
    ///
    /// MD\[a\] (0x46 - 0x47)
    ///
    /// a: 0: measure distance in grid-fitted outline
    ///    1: measure distance in original outline
    ///
    /// Pops: p1: point number
    ///       p2: point number
    /// Pushes: distance (F26Dot6)
    ///
    /// Measures the distance between outline point p1 and outline point p2.
    /// The value returned is in pixels (F26Dot6) If distance is negative, it
    /// was measured against the projection vector. Reversing the order in
    /// which the points are listed will change the sign of the result.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#measure-distance>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L4593>
    pub(super) fn op_md(&mut self, opcode: u8) -> OpResult {
        let p1 = self.value_stack.pop_usize()?;
        let p2 = self.value_stack.pop_usize()?;
        let gs = &self.graphics;
        if !gs.is_pedantic && !gs.in_bounds([(gs.zp0, p2), (gs.zp1, p1)]) {
            self.value_stack.push(0)?;
            return Ok(());
        }
        let distance = if (opcode & 1) != 0 {
            // measure in grid fitted outline
            gs.project(gs.zp0().point(p2)?, gs.zp1().point(p1)?)
                .to_bits()
        } else if gs.zp0.is_twilight() || gs.zp1.is_twilight() {
            // special case for twilight zone
            gs.dual_project(gs.zp0().original(p2)?, gs.zp1().original(p1)?)
                .to_bits()
        } else {
            // measure in original unscaled outline
            math::mul(
                gs.dual_project_unscaled(gs.zp0().unscaled(p2), gs.zp1().unscaled(p1)),
                gs.unscaled_to_pixels(),
            )
        };
        self.value_stack.push(distance)
    }

    /// Measure pixels per em.
    ///
    /// MPPEM[] (0x4B)
    ///
    /// Pushes: ppem: pixels per em (uint32)
    ///
    /// This instruction pushes the number of pixels per em onto the stack.
    /// Pixels per em is a function of the resolution of the rendering device
    /// and the current point size and the current transformation matrix.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#measure-pixels-per-em>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2609>
    pub(super) fn op_mppem(&mut self) -> OpResult {
        self.value_stack.push(self.graphics.ppem)
    }

    /// Measure point size.
    ///
    /// MPS[] (0x4C)
    ///
    /// Pushes: pointSize: the size in points of the current glyph (F26Dot6)
    ///
    /// Measure point size can be used to obtain a value which serves as the
    /// basis for choosing whether to branch to an alternative path through the
    /// instruction stream. It makes it possible to treat point sizes below or
    /// above a certain threshold differently.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#measure-point-size>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2623>
    pub(super) fn op_mps(&mut self) -> OpResult {
        // Note: FreeType computes this at
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttdriver.c#L392>
        // which is mul_div(ppem, 64 * 72, resolution) where resolution
        // is always 72 for our purposes (Skia), resulting in ppem * 64.
        self.value_stack.push(self.graphics.ppem * 64)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{super::zone::ZonePointer, math, Engine, MockEngine};
    use raw::types::F26Dot6;

    #[test]
    fn measure_ppem_and_point_size() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        let ppem = 20;
        engine.graphics.ppem = ppem;
        engine.op_mppem().unwrap();
        assert_eq!(engine.value_stack.pop().unwrap(), ppem);
        engine.op_mps().unwrap();
        assert_eq!(engine.value_stack.pop().unwrap(), ppem * 64);
    }

    #[test]
    fn gc() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        // current point projected coord
        let point = engine.graphics.zones[1].point_mut(1).unwrap();
        point.x = F26Dot6::from_bits(132);
        point.y = F26Dot6::from_bits(-256);
        engine.value_stack.push(1).unwrap();
        engine.op_gc(0).unwrap();
        assert_eq!(engine.value_stack.pop().unwrap(), 4);
        // original point projected coord
        let point = engine.graphics.zones[1].original_mut(1).unwrap();
        point.x = F26Dot6::from_bits(-64);
        point.y = F26Dot6::from_bits(521);
        engine.value_stack.push(1).unwrap();
        engine.op_gc(1).unwrap();
        assert_eq!(engine.value_stack.pop().unwrap(), 176);
    }

    #[test]
    fn scfs() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        // This instruction is a nop in backward compatibility mode
        // and before IUP.
        engine.graphics.backward_compatibility = false;
        engine.graphics.did_iup_x = true;
        engine.graphics.did_iup_y = true;
        // use the twilight zone to test the optional code path
        engine.graphics.zp2 = ZonePointer::Twilight;
        let point = engine.graphics.zones[0].point_mut(1).unwrap();
        point.x = F26Dot6::from_bits(132);
        point.y = F26Dot6::from_bits(-256);
        // assert we're not currently the same
        assert_ne!(
            engine.graphics.zones[0].point(1).unwrap(),
            engine.graphics.zones[0].original(1).unwrap()
        );
        // push point number
        engine.value_stack.push(1).unwrap();
        // push value to match
        engine.value_stack.push(42).unwrap();
        // set coordinate from stack!
        engine.op_scfs().unwrap();
        let point = engine.graphics.zones[0].point(1).unwrap();
        assert_eq!(point.x.to_bits(), 166);
        assert_eq!(point.y.to_bits(), -239);
        // ensure that we set original = point
        assert_eq!(point, engine.graphics.zones[0].original(1).unwrap());
    }

    #[test]
    fn md_scaled() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        // first path, measure in grid fitted outline
        let zone = engine.graphics.zone_mut(ZonePointer::Glyph);
        let point1 = zone.point_mut(1).unwrap();
        point1.x = F26Dot6::from_bits(132);
        point1.y = F26Dot6::from_bits(-256);
        let point2 = zone.point_mut(3).unwrap();
        point2.x = F26Dot6::from_bits(-64);
        point2.y = F26Dot6::from_bits(100);
        // now measure
        engine.value_stack.push(1).unwrap();
        engine.value_stack.push(3).unwrap();
        engine.op_md(1).unwrap();
        assert_eq!(engine.value_stack.pop().unwrap(), 16);
    }

    #[test]
    fn md_unscaled() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        // second path, measure in original unscaled outline.
        // unscaled points are set in mock engine but we need a scale
        engine.graphics.scale = 375912;
        engine.value_stack.push(1).unwrap();
        engine.value_stack.push(3).unwrap();
        engine.op_md(0).unwrap();
        assert_eq!(engine.value_stack.pop().unwrap(), 11);
    }

    #[test]
    fn md_twilight() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        set_test_vectors(&mut engine);
        // final path, measure in original outline, in twilight zone
        engine.graphics.zp0 = ZonePointer::Twilight;
        engine.graphics.zp1 = ZonePointer::Twilight;
        // set some points
        let zone = engine.graphics.zone_mut(ZonePointer::Twilight);
        let point1 = zone.original_mut(1).unwrap();
        point1.x = F26Dot6::from_bits(132);
        point1.y = F26Dot6::from_bits(-256);
        let point2 = zone.original_mut(3).unwrap();
        point2.x = F26Dot6::from_bits(-64);
        point2.y = F26Dot6::from_bits(100);
        // now measure
        engine.value_stack.push(1).unwrap();
        engine.value_stack.push(3).unwrap();
        engine.op_md(0).unwrap();
        assert_eq!(engine.value_stack.pop().unwrap(), 16);
    }

    fn set_test_vectors(engine: &mut Engine) {
        let v = math::normalize14(100, 50);
        engine.graphics.proj_vector = v;
        engine.graphics.dual_proj_vector = v;
        engine.graphics.freedom_vector = v;
        engine.graphics.update_projection_state();
    }
}
