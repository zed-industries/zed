//! Logical functions.
//!
//! Implements 11 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#logical-functions>

use super::{Engine, F26Dot6, OpResult};

impl Engine<'_> {
    /// Less than.
    ///
    /// LT[] (0x50)
    ///
    /// Pops: e1, e2
    /// Pushes: Boolean value
    ///
    /// First pops e2, then pops e1 off the stack and compares them: if e1 is
    /// less than e2, 1, signifying TRUE, is pushed onto the stack. If e1 is
    /// not less than e2, 0, signifying FALSE, is placed onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#less-than>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2721>
    pub(super) fn op_lt(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok((a < b) as i32))
    }

    /// Less than or equal.
    ///
    /// LTEQ[] (0x51)
    ///
    /// Pops: e1, e2
    /// Pushes: Boolean value
    ///
    /// Pops e2 and e1 off the stack and compares them. If e1 is less than or
    /// equal to e2, 1, signifying TRUE, is pushed onto the stack. If e1 is
    /// not less than or equal to e2, 0, signifying FALSE, is placed onto the
    /// stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#less-than-or-equal>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2734>
    pub(super) fn op_lteq(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok((a <= b) as i32))
    }

    /// Greater than.
    ///
    /// GT[] (0x52)
    ///
    /// Pops: e1, e2
    /// Pushes: Boolean value
    ///
    /// First pops e2 then pops e1 off the stack and compares them. If e1 is
    /// greater than e2, 1, signifying TRUE, is pushed onto the stack. If e1
    /// is not greater than e2, 0, signifying FALSE, is placed onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#greater-than>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2747>
    pub(super) fn op_gt(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok((a > b) as i32))
    }

    /// Greater than or equal.
    ///
    /// GTEQ[] (0x53)
    ///
    /// Pops: e1, e2
    /// Pushes: Boolean value
    ///
    /// Pops e1 and e2 off the stack and compares them. If e1 is greater than
    /// or equal to e2, 1, signifying TRUE, is pushed onto the stack. If e1
    /// is not greater than or equal to e2, 0, signifying FALSE, is placed
    /// onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#greater-than-or-equal>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2760>
    pub(super) fn op_gteq(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok((a >= b) as i32))
    }

    /// Equal.
    ///
    /// EQ[] (0x54)
    ///
    /// Pops: e1, e2
    /// Pushes: Boolean value
    ///
    /// Pops e1 and e2 off the stack and compares them. If they are equal, 1,
    /// signifying TRUE is pushed onto the stack. If they are not equal, 0,
    /// signifying FALSE is placed onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#equal>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2773>
    pub(super) fn op_eq(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok((a == b) as i32))
    }

    /// Not equal.
    ///
    /// NEQ[] (0x55)
    ///
    /// Pops: e1, e2
    /// Pushes: Boolean value
    ///
    /// Pops e1 and e2 from the stack and compares them. If they are not equal,
    /// 1, signifying TRUE, is pushed onto the stack. If they are equal, 0,
    /// signifying FALSE, is placed on the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#not-equal>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2786>
    pub(super) fn op_neq(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok((a != b) as i32))
    }

    /// Odd.
    ///
    /// ODD[] (0x56)
    ///
    /// Pops: e1
    /// Pushes: Boolean value
    ///
    /// Tests whether the number at the top of the stack is odd. Pops e1 from
    /// the stack and rounds it as specified by the round_state before testing
    /// it. After the value is rounded, it is shifted from a fixed point value
    /// to an integer value (any fractional values are ignored). If the integer
    /// value is odd, one, signifying TRUE, is pushed onto the stack. If it is
    /// even, zero, signifying FALSE is placed onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#odd>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2799>
    pub(super) fn op_odd(&mut self) -> OpResult {
        let round_state = self.graphics.round_state;
        self.value_stack.apply_unary(|e1| {
            Ok((round_state.round(F26Dot6::from_bits(e1)).to_bits() & 127 == 64) as i32)
        })
    }

    /// Even.
    ///
    /// EVEN[] (0x57)
    ///
    /// Pops: e1
    /// Pushes: Boolean value
    ///
    /// Tests whether the number at the top of the stack is even. Pops e1 off
    /// the stack and rounds it as specified by the round_state before testing
    /// it. If the rounded number is even, one, signifying TRUE, is pushed onto
    /// the stack if it is odd, zero, signifying FALSE, is placed onto the
    /// stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#even>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2813>
    pub(super) fn op_even(&mut self) -> OpResult {
        let round_state = self.graphics.round_state;
        self.value_stack.apply_unary(|e1| {
            Ok((round_state.round(F26Dot6::from_bits(e1)).to_bits() & 127 == 0) as i32)
        })
    }

    /// Logical and.
    ///
    /// AND[] (0x5A)
    ///
    /// Pops: e1, e2
    /// Pushes: Boolean value
    ///
    /// Pops e1 and e2 off the stack and pushes onto the stack the result of a
    /// logical and of the two elements. Zero is returned if either or both of
    /// the elements are FALSE (have the value zero). One is returned if both
    /// elements are TRUE (have a non zero value).
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#logical-and>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2827>
    pub(super) fn op_and(&mut self) -> OpResult {
        self.value_stack
            .apply_binary(|a, b| Ok((a != 0 && b != 0) as i32))
    }

    /// Logical or.
    ///
    /// OR[] (0x5B)
    ///
    /// Pops: e1, e2
    /// Pushes: Boolean value
    ///
    /// Pops e1 and e2 off the stack and pushes onto the stack the result of a
    /// logical or operation between the two elements. Zero is returned if both
    /// of the elements are FALSE. One is returned if either one or both of the
    /// elements are TRUE (has a nonzero value).
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#logical-or>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2840>
    pub(super) fn op_or(&mut self) -> OpResult {
        self.value_stack
            .apply_binary(|a, b| Ok((a != 0 || b != 0) as i32))
    }

    /// Logical not.
    ///
    /// NOT[] (0x5C)
    ///
    /// Pops: e
    /// Pushes: (not e): logical negation of e
    ///
    /// Pops e off the stack and returns the result of a logical NOT operation
    /// performed on e. If originally zero, one is pushed onto the stack if
    /// originally nonzero, zero is pushed onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#logical-not>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2853>
    pub(super) fn op_not(&mut self) -> OpResult {
        self.value_stack.apply_unary(|e| Ok((e == 0) as i32))
    }
}

#[cfg(test)]
mod tests {
    use super::super::MockEngine;

    #[test]
    fn compare_ops() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        for a in -10..=10 {
            for b in -10..=10 {
                let input = &[a, b];
                engine.test_exec(input, a < b, |engine| {
                    engine.op_lt().unwrap();
                });
                engine.test_exec(input, a <= b, |engine| {
                    engine.op_lteq().unwrap();
                });
                engine.test_exec(input, a > b, |engine| {
                    engine.op_gt().unwrap();
                });
                engine.test_exec(input, a >= b, |engine| {
                    engine.op_gteq().unwrap();
                });
                engine.test_exec(input, a == b, |engine| {
                    engine.op_eq().unwrap();
                });
                engine.test_exec(input, a != b, |engine| {
                    engine.op_neq().unwrap();
                });
            }
        }
    }

    #[test]
    fn parity_ops() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // These operate on 26.6 so values are multiple of 64
        let cases = [
            // (input, is_even)
            (0, true),
            (64, false),
            (128, true),
            (192, false),
            (256, true),
            (57, false),
            (-128, true),
        ];
        for (input, is_even) in cases {
            engine.test_exec(&[input], is_even, |engine| {
                engine.op_even().unwrap();
            });
        }
        for (input, is_even) in cases {
            engine.test_exec(&[input], !is_even, |engine| {
                engine.op_odd().unwrap();
            });
        }
    }

    #[test]
    fn not_op() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        engine.test_exec(&[0], 1, |engine| {
            engine.op_not().unwrap();
        });
        engine.test_exec(&[234234], 0, |engine| {
            engine.op_not().unwrap();
        });
    }

    #[test]
    fn and_or_ops() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        for a in -10..=10 {
            for b in -10..=10 {
                let input = &[a, b];
                let a = a != 0;
                let b = b != 0;
                engine.test_exec(input, a && b, |engine| {
                    engine.op_and().unwrap();
                });
                engine.test_exec(input, a || b, |engine| {
                    engine.op_or().unwrap();
                });
            }
        }
    }
}
