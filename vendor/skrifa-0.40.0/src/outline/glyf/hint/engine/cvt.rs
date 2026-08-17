//! Managing the control value table.
//!
//! Implements 3 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#managing-the-control-value-table>

use super::{super::math::mul, Engine, F26Dot6, OpResult};

impl Engine<'_> {
    /// Write control value table in pixel units.
    ///
    /// WCVTP[] (0x44)
    ///
    /// Pops: value: number in pixels (F26Dot6 fixed point number),
    ///       location: Control Value Table location (uint32)
    ///
    /// Pops a location and a value from the stack and puts that value in the
    /// specified location in the Control Value Table. This instruction assumes
    /// the value is in pixels and not in FUnits.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#write-control-value-table-in-pixel-units>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3044>
    pub(super) fn op_wcvtp(&mut self) -> OpResult {
        let value = self.value_stack.pop_f26dot6()?;
        let location = self.value_stack.pop_usize()?;
        let result = self.cvt.set(location, value);
        if self.graphics.is_pedantic {
            result
        } else {
            Ok(())
        }
    }

    /// Write control value table in font units.
    ///
    /// WCVTF[] (0x70)
    ///
    /// Pops: value: number in pixels (F26Dot6 fixed point number),
    ///       location: Control Value Table location (uint32)
    ///
    /// Pops a location and a value from the stack and puts the specified
    /// value in the specified address in the Control Value Table. This
    /// instruction assumes the value is expressed in FUnits and not pixels.
    /// The value is scaled before being written to the table.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#write-control-value-table-in-funits>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3067>
    pub(super) fn op_wcvtf(&mut self) -> OpResult {
        let value = self.value_stack.pop()?;
        let location = self.value_stack.pop_usize()?;
        let result = self.cvt.set(
            location,
            F26Dot6::from_bits(mul(value, self.graphics.scale)),
        );
        if self.graphics.is_pedantic {
            result
        } else {
            Ok(())
        }
    }

    /// Read control value table.
    ///
    /// RCVT[] (0x45)
    ///
    /// Pops: location: CVT entry number
    /// Pushes: value: CVT value (F26Dot6)
    ///
    /// Pops a location from the stack and pushes the value in the location
    /// specified in the Control Value Table onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#read-control-value-table>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3090>
    pub(super) fn op_rcvt(&mut self) -> OpResult {
        let location = self.value_stack.pop()? as usize;
        let maybe_value = self.cvt.get(location);
        let value = if self.graphics.is_pedantic {
            maybe_value?
        } else {
            maybe_value.unwrap_or_default()
        };
        self.value_stack.push(value.to_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{super::math, HintErrorKind, MockEngine};

    #[test]
    fn write_read() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        for i in 0..8 {
            engine.value_stack.push(i).unwrap();
            engine.value_stack.push(i * 2).unwrap();
            engine.op_wcvtp().unwrap();
        }
        for i in 0..8 {
            engine.value_stack.push(i).unwrap();
            engine.op_rcvt().unwrap();
            assert_eq!(engine.value_stack.pop().unwrap(), i * 2);
        }
    }

    #[test]
    fn write_scaled_read() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        let scale = 64;
        engine.graphics.scale = scale;
        for i in 0..8 {
            engine.value_stack.push(i).unwrap();
            engine.value_stack.push(i * 2).unwrap();
            // WCVTF takes a value in font units and converts to pixels
            // with the current scale
            engine.op_wcvtf().unwrap();
        }
        for i in 0..8 {
            engine.value_stack.push(i).unwrap();
            engine.op_rcvt().unwrap();
            let value = engine.value_stack.pop().unwrap();
            assert_eq!(value, math::mul(i * 2, scale));
        }
    }

    #[test]
    fn pedantry() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        let oob_index = 1000;
        // Disable pedantic mode: OOB writes are ignored, OOB reads
        // push 0
        engine.graphics.is_pedantic = false;
        engine.value_stack.push(oob_index).unwrap();
        engine.value_stack.push(0).unwrap();
        engine.op_wcvtp().unwrap();
        engine.value_stack.push(oob_index).unwrap();
        engine.value_stack.push(0).unwrap();
        engine.op_wcvtf().unwrap();
        engine.value_stack.push(oob_index).unwrap();
        engine.op_rcvt().unwrap();
        // Enable pedantic mode: OOB reads/writes error
        engine.graphics.is_pedantic = true;
        engine.value_stack.push(oob_index).unwrap();
        engine.value_stack.push(0).unwrap();
        assert_eq!(
            engine.op_wcvtp(),
            Err(HintErrorKind::InvalidCvtIndex(oob_index as _))
        );
        engine.value_stack.push(oob_index).unwrap();
        engine.value_stack.push(0).unwrap();
        assert_eq!(
            engine.op_wcvtf(),
            Err(HintErrorKind::InvalidCvtIndex(oob_index as _))
        );
        engine.value_stack.push(oob_index).unwrap();
        assert_eq!(
            engine.op_rcvt(),
            Err(HintErrorKind::InvalidCvtIndex(oob_index as _))
        );
    }
}
