//! Managing delta exceptions.
//!
//! Implements 6 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#managing-exceptions>

use super::{super::graphics::CoordAxis, Engine, F26Dot6, OpResult};
use read_fonts::tables::glyf::bytecode::Opcode;

impl Engine<'_> {
    /// Delta exception P1, P2 and P3.
    ///
    /// DELTAP1[] (0x5D)
    /// DELTAP2[] (0x71)
    /// DELTAP3[] (0x72)
    ///
    /// Pops: n: number of pairs of exception specifications and points (uint32)
    ///       p1, arg1, p2, arg2, ..., pnn argn: n pairs of exception specifications
    ///             and points (pairs of uint32s)
    ///
    /// DELTAP moves the specified points at the size and by the
    /// amount specified in the paired argument. An arbitrary number of points
    /// and arguments can be specified.
    ///
    /// The only difference between the instructions is the bias added to the
    /// point adjustment.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#delta-exception-p1>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6509>
    pub(super) fn op_deltap(&mut self, opcode: Opcode) -> OpResult {
        let gs = &mut self.graphics;
        let ppem = gs.ppem as u32;
        let point_count = gs.zp0().points.len();
        let n = self.value_stack.pop_count_checked()?;
        // Each exception requires two values on the stack so limit our
        // count to prevent looping in non-pedantic mode (where the stack ops
        // will produce 0 instead of an underflow error)
        let n = n.min(self.value_stack.len() / 2);
        let bias = match opcode {
            Opcode::DELTAP2 => 16,
            Opcode::DELTAP3 => 32,
            _ => 0,
        } + gs.delta_base as u32;
        let back_compat = gs.backward_compatibility;
        let did_iup = gs.did_iup_x && gs.did_iup_y;
        for _ in 0..n {
            let point_ix = self.value_stack.pop_usize()?;
            let mut b = self.value_stack.pop()?;
            // FreeType notes that some popular fonts contain invalid DELTAP
            // instructions so out of bounds points are ignored.
            // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6537>
            if point_ix >= point_count {
                continue;
            }
            let mut c = (b as u32 & 0xF0) >> 4;
            c += bias;
            if ppem == c {
                // Blindly copying FreeType here
                // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6565>
                b = (b & 0xF) - 8;
                if b >= 0 {
                    b += 1;
                }
                b *= 1 << (6 - gs.delta_shift as i32);
                let distance = F26Dot6::from_bits(b);
                if back_compat {
                    if !did_iup
                        && ((gs.is_composite && gs.freedom_vector.y != 0)
                            || gs.zp0().is_touched(point_ix, CoordAxis::Y)?)
                    {
                        gs.move_point(gs.zp0, point_ix, distance)?;
                    }
                } else {
                    gs.move_point(gs.zp0, point_ix, distance)?;
                }
            }
        }
        Ok(())
    }

    /// Delta exception C1, C2 and C3.
    ///
    /// DELTAC1[] (0x73)
    /// DELTAC2[] (0x74)
    /// DELTAC3[] (0x75)
    ///
    /// Pops: n: number of pairs of exception specifications and CVT entry numbers (uint32)
    ///       c1, arg1, c2, arg2,..., cn, argn: (pairs of uint32s)
    ///
    /// DELTAC changes the value in each CVT entry specified at the size and
    /// by the amount specified in its paired argument.
    ///
    /// The only difference between the instructions is the bias added to the
    /// adjustment.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#delta-exception-c1>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6604>
    pub(super) fn op_deltac(&mut self, opcode: Opcode) -> OpResult {
        let gs = &mut self.graphics;
        let ppem = gs.ppem as u32;
        let n = self.value_stack.pop_count_checked()?;
        // Each exception requires two values on the stack so limit our
        // count to prevent looping in non-pedantic mode (where the stack ops
        // will produce 0 instead of an underflow error)
        let n = n.min(self.value_stack.len() / 2);
        let bias = match opcode {
            Opcode::DELTAC2 => 16,
            Opcode::DELTAC3 => 32,
            _ => 0,
        } + gs.delta_base as u32;
        for _ in 0..n {
            let cvt_ix = self.value_stack.pop_usize()?;
            let mut b = self.value_stack.pop()?;
            let mut c = (b as u32 & 0xF0) >> 4;
            c += bias;
            if ppem == c {
                // See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6660>
                b = (b & 0xF) - 8;
                if b >= 0 {
                    b += 1;
                }
                b *= 1 << (6 - gs.delta_shift as i32);
                let cvt_val = self.cvt.get(cvt_ix)?;
                self.cvt.set(cvt_ix, cvt_val + F26Dot6::from_bits(b))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{super::zone::ZonePointer, HintErrorKind, MockEngine};
    use raw::{
        tables::glyf::bytecode::Opcode,
        types::{F26Dot6, Point},
    };

    #[test]
    fn deltap() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        engine.graphics.backward_compatibility = false;
        engine.graphics.zp0 = ZonePointer::Glyph;
        let raw_ppem = 16;
        let raw_adjustment = 7;
        for (point_ix, (ppem_bias, opcode)) in [
            (0, Opcode::DELTAP1),
            (16, Opcode::DELTAP2),
            (32, Opcode::DELTAP3),
        ]
        .iter()
        .enumerate()
        {
            let ppem = raw_ppem + ppem_bias;
            engine.graphics.ppem = ppem;
            // packed ppem + adjustment entry
            let packed_ppem = raw_ppem - engine.graphics.delta_base as i32;
            engine
                .value_stack
                .push((packed_ppem << 4) | raw_adjustment)
                .unwrap();
            // point index
            engine.value_stack.push(point_ix as _).unwrap();
            // exception count
            engine.value_stack.push(1).unwrap();
            engine.op_deltap(*opcode).unwrap();
            let point = engine.graphics.zones[1].point(point_ix).unwrap();
            assert_eq!(point.map(F26Dot6::to_bits), Point::new(-8, 0));
        }
    }

    #[test]
    fn deltac() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        let raw_ppem = 16;
        let raw_adjustment = 7;
        for (cvt_ix, (ppem_bias, opcode)) in [
            (0, Opcode::DELTAC1),
            (16, Opcode::DELTAC2),
            (32, Opcode::DELTAC3),
        ]
        .iter()
        .enumerate()
        {
            let ppem = raw_ppem + ppem_bias;
            engine.graphics.ppem = ppem;
            // packed ppem + adjustment entry
            let packed_ppem = raw_ppem - engine.graphics.delta_base as i32;
            engine
                .value_stack
                .push((packed_ppem << 4) | raw_adjustment)
                .unwrap();
            // cvt index
            engine.value_stack.push(cvt_ix as _).unwrap();
            // exception count
            engine.value_stack.push(1).unwrap();
            engine.op_deltac(*opcode).unwrap();
            let value = engine.cvt.get(cvt_ix).unwrap();
            assert_eq!(value.to_bits(), -8);
        }
    }

    /// Fuzzer detected timeout when the count supplied for deltap was
    /// negative. Converting to unsigned resulted in an absurdly high
    /// number leading to timeout.
    /// See <https://issues.oss-fuzz.com/issues/42538387>
    /// and <https://github.com/googlefonts/fontations/issues/1290>
    #[test]
    fn deltap_negative_count() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // We don't care about the parameters to the instruction except
        // for the count which is set to -1
        let stack = [0, 0, -1];
        // Non-pedantic mode: we end up with a count of 0 so do nothing
        for value in &stack {
            engine.value_stack.push(*value).unwrap();
        }
        // This just shouldn't hang the tests
        engine.op_deltap(Opcode::DELTAP3).unwrap();
        // Pedantic mode: raise an error
        engine.value_stack.is_pedantic = true;
        for value in &stack {
            engine.value_stack.push(*value).unwrap();
        }
        assert!(matches!(
            engine.op_deltap(Opcode::DELTAP3),
            Err(HintErrorKind::InvalidStackValue(-1))
        ));
    }

    /// Copy of the above test for DELTAC
    #[test]
    fn deltac_negative_count() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // We don't care about the parameters to the instruction except
        // for the count which is set to -1
        let stack = [0, 0, -1];
        // Non-pedantic mode: we end up with a count of 0 so do nothing
        for value in &stack {
            engine.value_stack.push(*value).unwrap();
        }
        // This just shouldn't hang the tests
        engine.op_deltac(Opcode::DELTAC3).unwrap();
        // Pedantic mode: raise an error
        engine.value_stack.is_pedantic = true;
        for value in &stack {
            engine.value_stack.push(*value).unwrap();
        }
        assert!(matches!(
            engine.op_deltac(Opcode::DELTAC3),
            Err(HintErrorKind::InvalidStackValue(-1))
        ));
    }
}
