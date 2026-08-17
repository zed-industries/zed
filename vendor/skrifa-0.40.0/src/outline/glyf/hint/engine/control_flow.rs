//! Managing the flow of control.
//!
//! Implements 6 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#managing-the-flow-of-control>

use read_fonts::tables::glyf::bytecode::Opcode;

use super::{Engine, HintErrorKind, OpResult};

impl Engine<'_> {
    /// If test.
    ///
    /// IF[] (0x58)
    ///
    /// Pops: e: stack element
    ///
    /// Tests the element popped off the stack: if it is zero (FALSE), the
    /// instruction pointer is jumped to the next ELSE or EIF instruction
    /// in the instruction stream. If the element at the top of the stack is
    /// nonzero (TRUE), the next instruction in the instruction stream is
    /// executed. Execution continues until an ELSE instruction is encountered
    /// or an EIF instruction ends the IF. If an else statement is found before
    /// the EIF, the instruction pointer is moved to the EIF statement.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#if-test>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3334>
    pub(super) fn op_if(&mut self) -> OpResult {
        if self.value_stack.pop()? == 0 {
            // The condition variable is false so we jump to the next
            // ELSE or EIF but we have to skip intermediate IF/ELSE/EIF
            // instructions.
            let mut nest_depth = 1;
            let mut out = false;
            while !out {
                let opcode = self.decode_next_opcode()?;
                match opcode {
                    Opcode::IF => nest_depth += 1,
                    Opcode::ELSE => out = nest_depth == 1,
                    Opcode::EIF => {
                        nest_depth -= 1;
                        out = nest_depth == 0;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// Else.
    ///
    /// ELSE[] (0x1B)
    ///
    /// Marks the start of the sequence of instructions that are to be executed
    /// if an IF instruction encounters a FALSE value on the stack. This
    /// sequence of instructions is terminated with an EIF instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#else>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3378>
    pub(super) fn op_else(&mut self) -> OpResult {
        let mut nest_depth = 1;
        while nest_depth != 0 {
            let opcode = self.decode_next_opcode()?;
            match opcode {
                Opcode::IF => nest_depth += 1,
                Opcode::EIF => nest_depth -= 1,
                _ => {}
            }
        }
        Ok(())
    }

    /// End if.
    ///
    /// EIF[] (0x59)
    ///
    /// Marks the end of an IF[] instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#end-if>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3411>
    pub(super) fn op_eif(&mut self) -> OpResult {
        // Nothing
        Ok(())
    }

    /// Jump relative on true.
    ///
    /// JROT[] (0x78)
    ///
    /// Pops: e: stack element
    ///       offset: number of bytes to move the instruction pointer
    ///
    /// Pops and tests the element value, and then pops the offset. If the
    /// element value is non-zero (TRUE), the signed offset will be added
    /// to the instruction pointer and execution will be resumed at the address
    /// obtained. Otherwise, the jump is not taken and the next instruction in
    /// the instruction stream is executed. The jump is relative to the position
    /// of the instruction itself. That is, the instruction pointer is still
    /// pointing at the JROT[ ] instruction when offset is added to obtain the
    /// new address.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#jump-relative-on-true>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3459>
    pub(super) fn op_jrot(&mut self) -> OpResult {
        let e = self.value_stack.pop()?;
        self.do_jump(e != 0)
    }

    /// Jump.
    ///
    /// JMPR[] (0x1C)
    ///
    /// Pops: offset: number of bytes to move the instruction pointer
    ///
    /// The signed offset is added to the instruction pointer and execution
    /// is resumed at the new location in the instruction steam. The jump is
    /// relative to the position of the instruction itself. That is, the
    /// instruction pointer is still pointing at the JROT[] instruction when
    /// offset is added to obtain the new address.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#jump>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3424>
    pub(super) fn op_jmpr(&mut self) -> OpResult {
        self.do_jump(true)
    }

    /// Jump relative on false.
    ///
    /// JROF[] (0x78)
    ///
    /// Pops: e: stack element
    ///       offset: number of bytes to move the instruction pointer
    ///
    /// Pops and tests the element value, and then pops the offset. If the
    /// element value is non-zero (TRUE), the signed offset will be added
    /// to the instruction pointer and execution will be resumed at the address
    /// obtained. Otherwise, the jump is not taken and the next instruction in
    /// the instruction stream is executed. The jump is relative to the position
    /// of the instruction itself. That is, the instruction pointer is still
    /// pointing at the JROT[ ] instruction when offset is added to obtain the
    /// new address.
    ///
    /// Pops and tests the element value, and then pops the offset. If the
    /// element value is zero (FALSE), the signed offset will be added to the
    /// nstruction pointer and execution will be resumed at the address
    /// obtainted. Otherwise, the jump is not taken and the next instruction
    /// in the instruction stream is executed. The jump is relative to the
    /// position of the instruction itself. That is, the instruction pointer is
    /// still pointing at the JROT[ ] instruction when the offset is added to
    /// obtain the new address.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#jump-relative-on-false>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3474>
    pub(super) fn op_jrof(&mut self) -> OpResult {
        let e = self.value_stack.pop()?;
        self.do_jump(e == 0)
    }

    /// Common code for jump instructions.
    ///
    /// See <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3424>
    fn do_jump(&mut self, test: bool) -> OpResult {
        // Offset is relative to previous jump instruction and decoder is
        // already pointing to next instruction, so subtract one
        let jump_offset = self.value_stack.pop()?.wrapping_sub(1);
        if test {
            if jump_offset < 0 {
                if jump_offset == -1 {
                    // If the offset is -1, we'll just loop in place... forever
                    return Err(HintErrorKind::InvalidJump);
                }
                self.loop_budget.doing_backward_jump()?;
            }
            self.program.decoder.pc = self
                .program
                .decoder
                .pc
                .wrapping_add_signed(jump_offset as isize);
        }
        Ok(())
    }

    fn decode_next_opcode(&mut self) -> Result<Opcode, HintErrorKind> {
        Ok(self
            .program
            .decoder
            .decode()
            .ok_or(HintErrorKind::UnexpectedEndOfBytecode)??
            .opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::{super::MockEngine, HintErrorKind, Opcode};

    #[test]
    fn if_else() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // Some code with nested ifs
        #[rustfmt::skip]
        let ops = [
            IF,
                ADD, // 1
                SUB,
                IF,
                    MUL, // 4
                    DIV,
                ELSE, // 8
                    IUP0, // 7
                    IUP1,
                EIF,
            ELSE, // 10
                RUTG, // 11
                IF,
                EIF,
            EIF // 14
        ];
        let bytecode = ops.map(|op| op as u8);
        engine.program.decoder.bytecode = bytecode.as_slice();
        // Outer if
        {
            // push a true value to enter the first branch
            engine.program.decoder.pc = 1;
            engine.value_stack.push(1).unwrap();
            engine.op_if().unwrap();
            assert_eq!(engine.program.decoder.pc, 1);
            // false enters the else branch
            engine.program.decoder.pc = 1;
            engine.value_stack.push(0).unwrap();
            engine.op_if().unwrap();
            assert_eq!(engine.program.decoder.pc, 11);
        }
        // Inner if
        {
            // push a true value to enter the first branch
            engine.program.decoder.pc = 4;
            engine.value_stack.push(1).unwrap();
            engine.op_if().unwrap();
            assert_eq!(engine.program.decoder.pc, 4);
            // false enters the else branch
            engine.program.decoder.pc = 4;
            engine.value_stack.push(0).unwrap();
            engine.op_if().unwrap();
            assert_eq!(engine.program.decoder.pc, 7);
        }
        // Else with nested if
        {
            // This jumps to the instruction after the next EIF, skipping any
            // nested conditional blocks
            engine.program.decoder.pc = 10;
            engine.op_else().unwrap();
            assert_eq!(engine.program.decoder.pc, 15);
            engine.program.decoder.pc = 8;
            engine.op_else().unwrap();
            assert_eq!(engine.program.decoder.pc, 10);
        }
    }

    #[test]
    fn jumps() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        // Unconditional jump
        {
            engine.program.decoder.pc = 1000;
            engine.value_stack.push(100).unwrap();
            engine.op_jmpr().unwrap();
            assert_eq!(engine.program.decoder.pc, 1099);
        }
        // Jump if true
        {
            engine.program.decoder.pc = 1000;
            // first test false condition, pc shouldn't change
            engine.value_stack.push(100).unwrap();
            engine.value_stack.push(0).unwrap();
            engine.op_jrot().unwrap();
            assert_eq!(engine.program.decoder.pc, 1000);
            // then true condition
            engine.value_stack.push(100).unwrap();
            engine.value_stack.push(1).unwrap();
            engine.op_jrot().unwrap();
            assert_eq!(engine.program.decoder.pc, 1099);
        }
        // Jump if false
        {
            engine.program.decoder.pc = 1000;
            // first test true condition, pc shouldn't change
            engine.value_stack.push(-100).unwrap();
            engine.value_stack.push(1).unwrap();
            engine.op_jrof().unwrap();
            assert_eq!(engine.program.decoder.pc, 1000);
            // then false condition
            engine.value_stack.push(-100).unwrap();
            engine.value_stack.push(0).unwrap();
            engine.op_jrof().unwrap();
            assert_eq!(engine.program.decoder.pc, 899);
        }
        // Exhaust backward jump loop budget
        {
            engine.loop_budget.limit = 40;
            for i in 0..45 {
                engine.value_stack.push(-5).unwrap();
                let result = engine.op_jmpr();
                if i < 39 {
                    result.unwrap();
                } else {
                    assert!(matches!(
                        result,
                        Err(HintErrorKind::ExceededExecutionBudget)
                    ));
                }
            }
        }
    }
}
