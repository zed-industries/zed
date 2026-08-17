//! Miscellaneous instructions.
//!
//! Implements 3 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#miscellaneous-instructions>

use super::{Engine, OpResult};

impl Engine<'_> {
    /// Get information.
    ///
    /// GETINFO[] (0x88)
    ///
    /// Pops: selector: integer
    /// Pushes: result: integer
    ///
    /// GETINFO is used to obtain data about the font scaler version and the
    /// characteristics of the current glyph. The instruction pops a selector
    /// used to determine the type of information desired and pushes a result
    /// onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#get-information>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6689>
    pub(super) fn op_getinfo(&mut self) -> OpResult {
        use getinfo::*;
        let selector = self.value_stack.pop()?;
        let mut result = 0;
        // Interpreter version (selector bit: 0, result bits: 0-7)
        if (selector & VERSION_SELECTOR_BIT) != 0 {
            result = 40;
        }
        // Glyph rotated (selector bit: 1, result bit: 8)
        if (selector & GLYPH_ROTATED_SELECTOR_BIT) != 0 && self.graphics.is_rotated {
            result |= GLYPH_ROTATED_RESULT_BIT;
        }
        // Glyph stretched (selector bit: 2, result bit: 9)
        if (selector & GLYPH_STRETCHED_SELECTOR_BIT) != 0 && self.graphics.is_stretched {
            result |= GLYPH_STRETCHED_RESULT_BIT;
        }
        // Font variations (selector bit: 3, result bit: 10)
        if (selector & FONT_VARIATIONS_SELECTOR_BIT) != 0 && self.axis_count != 0 {
            result |= FONT_VARIATIONS_RESULT_BIT;
        }
        // The following only apply for smooth hinting.
        if self.graphics.target.is_smooth() {
            // Subpixel hinting [cleartype enabled] (selector bit: 6, result bit: 13)
            // (always enabled)
            if (selector & SUBPIXEL_HINTING_SELECTOR_BIT) != 0 {
                result |= SUBPIXEL_HINTING_RESULT_BIT;
            }
            // Vertical LCD subpixels? (selector bit: 8, result bit: 15)
            if (selector & VERTICAL_LCD_SELECTOR_BIT) != 0 && self.graphics.target.is_vertical_lcd()
            {
                result |= VERTICAL_LCD_RESULT_BIT;
            }
            // Subpixel positioned? (selector bit: 10, result bit: 17)
            // (always enabled)
            if (selector & SUBPIXEL_POSITIONED_SELECTOR_BIT) != 0 {
                result |= SUBPIXEL_POSITIONED_RESULT_BIT;
            }
            // Symmetrical smoothing (selector bit: 11, result bit: 18)
            // Note: FreeType always enables this but we allow direct control
            // with our own flag.
            // See <https://github.com/googlefonts/fontations/issues/1080>
            if (selector & SYMMETRICAL_SMOOTHING_SELECTOR_BIT) != 0
                && self.graphics.target.symmetric_rendering()
            {
                result |= SYMMETRICAL_SMOOTHING_RESULT_BIT;
            }
            // ClearType hinting and grayscale rendering (selector bit: 12, result bit: 19)
            if (selector & GRAYSCALE_CLEARTYPE_SELECTOR_BIT) != 0
                && self.graphics.target.is_grayscale_cleartype()
            {
                result |= GRAYSCALE_CLEARTYPE_RESULT_BIT;
            }
        }
        self.value_stack.push(result)
    }

    /// Get variation.
    ///
    /// GETVARIATION[] (0x91)
    ///
    /// Pushes: Normalized axes coordinates, one for each axis in the font.
    ///
    /// GETVARIATION is used to obtain the current normalized variation
    /// coordinates for each axis. The coordinate for the first axis, as
    /// defined in the 'fvar' table, is pushed first on the stack, followed
    /// by each consecutive axis until the coordinate for the last axis is
    /// on the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#get-variation>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6813>
    pub(super) fn op_getvariation(&mut self) -> OpResult {
        // For non-variable fonts, this falls back to IDEF resolution.
        let axis_count = self.axis_count as usize;
        if axis_count != 0 {
            // Make sure we push `axis_count` coords regardless of the value
            // provided by the user.
            for coord in self
                .coords
                .iter()
                .copied()
                .chain(std::iter::repeat(Default::default()))
                .take(axis_count)
            {
                self.value_stack.push(coord.to_bits() as i32)?;
            }
            Ok(())
        } else {
            self.op_unknown(0x91)
        }
    }

    /// Get data.
    ///
    /// GETDATA[] (0x92)
    ///
    /// Pushes: 17
    ///
    /// Undocumented and nobody knows what this does. FreeType just
    /// returns 17 for variable fonts and falls back to IDEF lookup
    /// otherwise.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L6851>
    pub(super) fn op_getdata(&mut self) -> OpResult {
        if self.axis_count != 0 {
            self.value_stack.push(17)
        } else {
            self.op_unknown(0x92)
        }
    }
}

/// Constants for the GETINFO instruction. Extracted here
/// to enable access from tests.
mod getinfo {
    // Interpreter version (selector bit: 0, result bits: 0-7)
    pub const VERSION_SELECTOR_BIT: i32 = 1 << 0;

    // Glyph rotated (selector bit: 1, result bit: 8)
    pub const GLYPH_ROTATED_SELECTOR_BIT: i32 = 1 << 1;
    pub const GLYPH_ROTATED_RESULT_BIT: i32 = 1 << 8;

    // Glyph stretched (selector bit: 2, result bit: 9)
    pub const GLYPH_STRETCHED_SELECTOR_BIT: i32 = 1 << 2;
    pub const GLYPH_STRETCHED_RESULT_BIT: i32 = 1 << 9;

    // Font variations (selector bit: 3, result bit: 10)
    pub const FONT_VARIATIONS_SELECTOR_BIT: i32 = 1 << 3;
    pub const FONT_VARIATIONS_RESULT_BIT: i32 = 1 << 10;

    // Subpixel hinting [cleartype enabled] (selector bit: 6, result bit: 13)
    // (always enabled)
    pub const SUBPIXEL_HINTING_SELECTOR_BIT: i32 = 1 << 6;
    pub const SUBPIXEL_HINTING_RESULT_BIT: i32 = 1 << 13;

    // Vertical LCD subpixels? (selector bit: 8, result bit: 15)
    pub const VERTICAL_LCD_SELECTOR_BIT: i32 = 1 << 8;
    pub const VERTICAL_LCD_RESULT_BIT: i32 = 1 << 15;

    // Subpixel positioned? (selector bit: 10, result bit: 17)
    // (always enabled)
    pub const SUBPIXEL_POSITIONED_SELECTOR_BIT: i32 = 1 << 10;
    pub const SUBPIXEL_POSITIONED_RESULT_BIT: i32 = 1 << 17;

    // Symmetrical smoothing (selector bit: 11, result bit: 18)
    // Note: FreeType always enables this but we deviate when our own
    // preserve linear metrics flag is enabled.
    pub const SYMMETRICAL_SMOOTHING_SELECTOR_BIT: i32 = 1 << 11;
    pub const SYMMETRICAL_SMOOTHING_RESULT_BIT: i32 = 1 << 18;

    // ClearType hinting and grayscale rendering (selector bit: 12, result bit: 19)
    pub const GRAYSCALE_CLEARTYPE_SELECTOR_BIT: i32 = 1 << 12;
    pub const GRAYSCALE_CLEARTYPE_RESULT_BIT: i32 = 1 << 19;
}

#[cfg(test)]
mod tests {
    use super::super::{
        super::super::super::{SmoothMode, Target},
        Engine, HintErrorKind, MockEngine,
    };
    use raw::types::F2Dot14;
    use read_fonts::tables::glyf::bytecode::Opcode;

    #[test]
    fn getinfo() {
        use super::getinfo::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // version
        engine.getinfo_test(VERSION_SELECTOR_BIT, 40);
        // not rotated
        engine.getinfo_test(GLYPH_ROTATED_SELECTOR_BIT, 0);
        // rotated
        engine.graphics.is_rotated = true;
        engine.getinfo_test(GLYPH_ROTATED_SELECTOR_BIT, GLYPH_ROTATED_RESULT_BIT);
        // not stretched
        engine.getinfo_test(GLYPH_STRETCHED_SELECTOR_BIT, 0);
        // stretched
        engine.graphics.is_stretched = true;
        engine.getinfo_test(GLYPH_STRETCHED_SELECTOR_BIT, GLYPH_STRETCHED_RESULT_BIT);
        // stretched and rotated
        engine.getinfo_test(
            GLYPH_ROTATED_SELECTOR_BIT | GLYPH_STRETCHED_SELECTOR_BIT,
            GLYPH_ROTATED_RESULT_BIT | GLYPH_STRETCHED_RESULT_BIT,
        );
        // not variable
        engine.getinfo_test(FONT_VARIATIONS_SELECTOR_BIT, 0);
        // variable
        engine.axis_count = 1;
        engine.getinfo_test(FONT_VARIATIONS_SELECTOR_BIT, FONT_VARIATIONS_RESULT_BIT);
        // in strong hinting mode, the following selectors are always disabled
        engine.graphics.target = Target::Mono;
        for selector in [
            SUBPIXEL_HINTING_SELECTOR_BIT,
            VERTICAL_LCD_SELECTOR_BIT,
            SUBPIXEL_POSITIONED_SELECTOR_BIT,
            SYMMETRICAL_SMOOTHING_SELECTOR_BIT,
            GRAYSCALE_CLEARTYPE_SELECTOR_BIT,
        ] {
            engine.getinfo_test(selector, 0);
        }
        // set back to smooth mode
        engine.graphics.target = Target::default();
        for (selector, result) in [
            // default smooth mode is grayscale cleartype
            (
                GRAYSCALE_CLEARTYPE_SELECTOR_BIT,
                GRAYSCALE_CLEARTYPE_RESULT_BIT,
            ),
            // always enabled in smooth mode
            (SUBPIXEL_HINTING_SELECTOR_BIT, SUBPIXEL_HINTING_RESULT_BIT),
            (
                SUBPIXEL_POSITIONED_SELECTOR_BIT,
                SUBPIXEL_POSITIONED_RESULT_BIT,
            ),
        ] {
            engine.getinfo_test(selector, result);
        }
        // vertical lcd
        engine.graphics.target = Target::Smooth {
            mode: SmoothMode::VerticalLcd,
            preserve_linear_metrics: true,
            symmetric_rendering: false,
        };
        engine.getinfo_test(VERTICAL_LCD_SELECTOR_BIT, VERTICAL_LCD_RESULT_BIT);
        // symmetical smoothing is disabled
        engine.getinfo_test(SYMMETRICAL_SMOOTHING_SELECTOR_BIT, 0);
        // grayscale cleartype is disabled when lcd_subpixel is not None
        engine.getinfo_test(GRAYSCALE_CLEARTYPE_SELECTOR_BIT, 0);
        // reset to default to disable preserve linear metrics
        engine.graphics.target = Target::default();
        // now symmetrical smoothing is enabled
        engine.getinfo_test(
            SYMMETRICAL_SMOOTHING_SELECTOR_BIT,
            SYMMETRICAL_SMOOTHING_RESULT_BIT,
        );
    }

    #[test]
    fn getvariation() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // no variations should trigger unknown opcode
        assert!(matches!(
            engine.op_getvariation(),
            Err(HintErrorKind::UnhandledOpcode(Opcode::GETVARIATION))
        ));
        // set the axis count to a non-zero value to enable variations
        engine.axis_count = 2;
        // and creates some coords
        let coords = [
            F2Dot14::from_f32(-1.0),
            F2Dot14::from_f32(0.5),
            F2Dot14::from_f32(1.0),
        ];
        let coords_bits = coords.map(|x| x.to_bits() as i32);
        // too few, pad with zeros
        engine.coords = &coords[0..1];
        engine.op_getvariation().unwrap();
        assert_eq!(engine.value_stack.len(), 2);
        assert_eq!(engine.value_stack.values(), &[coords_bits[0], 0]);
        engine.value_stack.clear();
        // too many, truncate
        engine.coords = &coords[0..3];
        engine.op_getvariation().unwrap();
        assert_eq!(engine.value_stack.len(), 2);
        assert_eq!(engine.value_stack.values(), &coords_bits[0..2]);
        engine.value_stack.clear();
        // just right
        engine.coords = &coords[0..2];
        engine.op_getvariation().unwrap();
        assert_eq!(engine.value_stack.len(), 2);
        assert_eq!(engine.value_stack.values(), &coords_bits[0..2]);
    }

    #[test]
    fn getdata() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // no variations should trigger unknown opcode
        assert!(matches!(
            engine.op_getdata(),
            Err(HintErrorKind::UnhandledOpcode(Opcode::GETDATA))
        ));
        // set the axis count to a non-zero value to enable variations
        engine.axis_count = 1;
        engine.op_getdata().unwrap();
        // :shrug:
        assert_eq!(engine.value_stack.pop().unwrap(), 17);
    }

    impl Engine<'_> {
        fn getinfo_test(&mut self, selector: i32, expected: i32) {
            self.value_stack.push(selector).unwrap();
            self.op_getinfo().unwrap();
            assert_eq!(self.value_stack.pop().unwrap(), expected);
        }
    }
}
