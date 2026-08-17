//! Arithmetic and math instructions.
//!
//! Implements 10 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#arithmetic-and-math-instructions>

use super::{super::math, Engine, HintErrorKind, OpResult};

impl Engine<'_> {
    /// ADD[] (0x60)
    ///
    /// Pops: n1, n2 (F26Dot6)
    /// Pushes: (n2 + n1)
    ///
    /// Pops n1 and n2 off the stack and pushes the sum of the two elements
    /// onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#add>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2866>
    pub(super) fn op_add(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok(a.wrapping_add(b)))
    }

    /// SUB[] (0x61)
    ///
    /// Pops: n1, n2 (F26Dot6)
    /// Pushes: (n2 - n1)
    ///
    /// Pops n1 and n2 off the stack and pushes the difference of the two
    /// elements onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#subtract>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2879>
    pub(super) fn op_sub(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok(a.wrapping_sub(b)))
    }

    /// DIV[] (0x62)
    ///
    /// Pops: n1, n2 (F26Dot6)
    /// Pushes: (n2 / n1)
    ///
    /// Pops n1 and n2 off the stack and pushes onto the stack the quotient
    /// obtained by dividing n2 by n1. Note that this truncates rather than
    /// rounds the value.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#divide>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2892>
    pub(super) fn op_div(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| {
            if b == 0 {
                Err(HintErrorKind::DivideByZero)
            } else {
                Ok(math::mul_div_no_round(a, 64, b))
            }
        })
    }

    /// MUL[] (0x63)
    ///
    /// Pops: n1, n2 (F26Dot6)
    /// Pushes: (n2 * n1)
    ///
    /// Pops n1 and n2 off the stack and pushes onto the stack the product of
    /// the two elements.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#multiply>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2909>
    pub(super) fn op_mul(&mut self) -> OpResult {
        self.value_stack
            .apply_binary(|a, b| Ok(math::mul_div(a, b, 64)))
    }

    /// ABS[] (0x64)
    ///
    /// Pops: n
    /// Pushes: |n|: absolute value of n (F26Dot6)
    ///
    /// Pops n off the stack and pushes onto the stack the absolute value of n.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#absolute-value>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2922>
    pub(super) fn op_abs(&mut self) -> OpResult {
        self.value_stack.apply_unary(|n| Ok(n.wrapping_abs()))
    }

    /// NEG[] (0x65)
    ///
    /// Pops: n1
    /// Pushes: -n1: negation of n1 (F26Dot6)
    ///
    /// This instruction pops n1 off the stack and pushes onto the stack the
    /// negated value of n1.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#negate>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2936>
    pub(super) fn op_neg(&mut self) -> OpResult {
        self.value_stack.apply_unary(|n1| Ok(n1.wrapping_neg()))
    }

    /// FLOOR[] (0x66)
    ///
    /// Pops: n1: number whose floor is desired (F26Dot6)
    /// Pushes: n: floor of n1 (F26Dot6)
    ///
    /// Pops n1 and returns n, the greatest integer value less than or equal to n1.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#floor>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2949>
    pub(super) fn op_floor(&mut self) -> OpResult {
        self.value_stack.apply_unary(|n1| Ok(math::floor(n1)))
    }

    /// CEILING[] (0x67)
    ///
    /// Pops: n1: number whose ceiling is desired (F26Dot6)
    /// Pushes: n: ceiling of n1 (F26Dot6)
    ///
    /// Pops n1 and returns n, the least integer value greater than or equal to n1.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#ceiling>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2962>
    pub(super) fn op_ceiling(&mut self) -> OpResult {
        self.value_stack.apply_unary(|n1| Ok(math::ceil(n1)))
    }

    /// MAX[] (0x8B)
    ///
    /// Pops: e1, e2
    /// Pushes: maximum of e1 and e2
    ///
    /// Pops two elements, e1 and e2, from the stack and pushes the larger of
    /// these two quantities onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#maximum-of-top-two-stack-elements>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3171>
    pub(super) fn op_max(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok(a.max(b)))
    }

    /// MIN[] (0x8C)
    ///
    /// Pops: e1, e2
    /// Pushes: minimum of e1 and e2
    ///
    /// Pops two elements, e1 and e2, from the stack and pushes the smaller
    /// of these two quantities onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#minimum-of-top-two-stack-elements>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3185>
    pub(super) fn op_min(&mut self) -> OpResult {
        self.value_stack.apply_binary(|a, b| Ok(a.min(b)))
    }
}

#[cfg(test)]
mod tests {
    use super::{super::MockEngine, math, HintErrorKind};

    /// Test the binary operations that don't require fixed point
    /// arithmetic.
    #[test]
    fn simple_binops() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        for a in -10..=10 {
            for b in -10..=10 {
                let input = &[a, b];
                engine.test_exec(input, a + b, |engine| {
                    engine.op_add().unwrap();
                });
                engine.test_exec(input, a - b, |engine| {
                    engine.op_sub().unwrap();
                });
                engine.test_exec(input, a.max(b), |engine| {
                    engine.op_max().unwrap();
                });
                engine.test_exec(input, a.min(b), |engine| {
                    engine.op_min().unwrap();
                });
            }
        }
    }

    /// Test the unary operations that don't require fixed point
    /// arithmetic.
    #[test]
    fn simple_unops() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        for a in -10..=10 {
            let input = &[a];
            engine.test_exec(input, -a, |engine| {
                engine.op_neg().unwrap();
            });
            engine.test_exec(input, a.abs(), |engine| {
                engine.op_abs().unwrap();
            });
        }
    }

    #[test]
    fn f26dot6_binops() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        for a in -10..=10 {
            for b in -10..=10 {
                let a = a * 64 + 30;
                let b = b * 64 - 30;
                let input = &[a, b];
                engine.test_exec(input, math::mul_div(a, b, 64), |engine| {
                    engine.op_mul().unwrap();
                });
                if b != 0 {
                    engine.test_exec(input, math::mul_div_no_round(a, 64, b), |engine| {
                        engine.op_div().unwrap();
                    });
                } else {
                    engine.value_stack.push(a).unwrap();
                    engine.value_stack.push(b).unwrap();
                    assert!(matches!(engine.op_div(), Err(HintErrorKind::DivideByZero)));
                }
            }
        }
    }

    #[test]
    fn f26dot6_unops() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        for a in -10..=10 {
            for b in -10..=10 {
                let a = a * 64 + b;
                let input = &[a];
                engine.test_exec(input, math::floor(a), |engine| {
                    engine.op_floor().unwrap();
                });
                engine.test_exec(input, math::ceil(a), |engine| {
                    engine.op_ceiling().unwrap();
                });
            }
        }
    }
}
