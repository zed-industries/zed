//! Compensating for the engine characteristics (rounding).
//!
//! Implements 4 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#compensating-for-the-engine-characteristics>

use super::{Engine, OpResult};

impl Engine<'_> {
    /// Round value.
    ///
    /// ROUND\[ab\] (0x68 - 0x6B)
    ///
    /// Pops: n1
    /// Pushes: n2
    ///
    /// Rounds a value according to the state variable round_state while
    /// compensating for the engine. n1 is popped off the stack and,
    /// depending on the engine characteristics, is increased or decreased
    /// by a set amount. The number obtained is then rounded and pushed
    /// back onto the stack as n2.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#round-value>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3143>
    pub(super) fn op_round(&mut self) -> OpResult {
        let n1 = self.value_stack.pop_f26dot6()?;
        let n2 = self.graphics.round(n1);
        self.value_stack.push(n2.to_bits())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{super::round::RoundMode, MockEngine};

    #[test]
    fn round() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        use RoundMode::*;
        let cases = [
            (Grid, &[(0, 0), (32, 64), (-32, -64), (64, 64), (50, 64)]),
            (
                HalfGrid,
                &[(0, 32), (32, 32), (-32, -32), (64, 96), (50, 32)],
            ),
            (
                DoubleGrid,
                &[(0, 0), (32, 32), (-32, -32), (64, 64), (50, 64)],
            ),
            (DownToGrid, &[(0, 0), (32, 0), (-32, 0), (64, 64), (50, 0)]),
            (
                UpToGrid,
                &[(0, 0), (32, 64), (-32, -64), (64, 64), (50, 64)],
            ),
            (Off, &[(0, 0), (32, 32), (-32, -32), (64, 64), (50, 50)]),
        ];
        for (mode, values) in cases {
            match mode {
                Grid => engine.op_rtg().unwrap(),
                HalfGrid => engine.op_rthg().unwrap(),
                DoubleGrid => engine.op_rtdg().unwrap(),
                DownToGrid => engine.op_rdtg().unwrap(),
                UpToGrid => engine.op_rutg().unwrap(),
                Off => engine.op_roff().unwrap(),
                _ => unreachable!(),
            }
            for (input, expected) in values {
                engine.value_stack.push(*input).unwrap();
                engine.op_round().unwrap();
                let result = engine.value_stack.pop().unwrap();
                assert_eq!(*expected, result);
            }
        }
    }
}
