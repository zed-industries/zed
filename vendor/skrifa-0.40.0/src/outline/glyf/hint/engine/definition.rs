//! Defining and using functions and instructions.
//!
//! Implements 5 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#defining-and-using-functions-and-instructions>

use read_fonts::tables::glyf::bytecode::Opcode;

use super::{
    super::{definition::Definition, program::Program},
    Engine, HintErrorKind, OpResult,
};

/// [Functions|Instructions] may not exceed 64K in size.
/// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#function-definition>
/// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#instruction-definition>
const MAX_DEFINITION_SIZE: usize = u16::MAX as usize;

impl Engine<'_> {
    /// Function definition.
    ///
    /// FDEF[] (0x2C)
    ///
    /// Pops: f: function identifier number
    ///
    /// Marks the start of a function definition. The argument f is a number
    /// that uniquely identifies this function. A function definition can
    /// appear only in the Font Program or the CVT program; attempts to invoke
    /// the FDEF instruction within a glyph program will result in an error.
    /// Functions may not exceed 64K in size.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#function-definition>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3496>
    pub(super) fn op_fdef(&mut self) -> OpResult {
        let f = self.value_stack.pop()?;
        self.do_def(DefKind::Function, f)
    }

    /// End function definition.
    ///
    /// ENDF[] (0x2D)
    ///
    /// Marks the end of a function definition or an instruction definition.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#end-function-definition>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3578>
    pub(super) fn op_endf(&mut self) -> OpResult {
        self.program.leave()
    }

    /// Call function.
    ///
    /// CALL[] (0x2B)
    ///
    /// Pops: f: function identifier number
    ///
    /// Calls the function identified by the number f.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#call-function>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3623>
    pub(super) fn op_call(&mut self) -> OpResult {
        let f = self.value_stack.pop()?;
        self.do_call(DefKind::Function, 1, f)
    }

    /// Loop and call function.
    ///
    /// LOOPCALL[] (0x2a)
    ///
    /// Pops: f: function identifier number
    ///       count: number of times to call the function
    ///
    /// Calls the function f, count number of times.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#loop-and-call-function>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3704>
    pub(super) fn op_loopcall(&mut self) -> OpResult {
        let f = self.value_stack.pop()?;
        let count = self.value_stack.pop()?;
        if count > 0 {
            self.loop_budget.doing_loop_call(count as usize)?;
            self.do_call(DefKind::Function, count as u32, f)
        } else {
            Ok(())
        }
    }

    /// Instruction definition.
    ///
    /// IDEF[] (0x89)
    ///
    /// Pops: opcode
    ///
    /// Begins the definition of an instruction. The instruction definition
    /// terminates when at ENDF, which is encountered in the instruction
    /// stream. Subsequent executions of the opcode popped will be directed
    /// to the contents of this instruction definition (IDEF). IDEFs must be
    /// defined in the Font Program or the CVT Program; attempts to invoke the
    /// IDEF instruction within a glyph program will result in an error. An
    /// IDEF affects only undefined opcodes. If the opcode in question is
    /// already defined, the interpreter will ignore the IDEF. This is to be
    /// used as a patching mechanism for future instructions. Instructions
    /// may not exceed 64K in size.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#instruction-definition>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3788>
    pub(super) fn op_idef(&mut self) -> OpResult {
        let opcode = self.value_stack.pop()?;
        self.do_def(DefKind::Instruction, opcode)
    }

    /// Catch all for unhandled opcodes which will attempt to dispatch to a
    /// user defined instruction.
    pub(super) fn op_unknown(&mut self, opcode: u8) -> OpResult {
        self.do_call(DefKind::Instruction, 1, opcode as i32)
    }

    /// Common code for FDEF and IDEF.
    fn do_def(&mut self, kind: DefKind, key: i32) -> OpResult {
        if self.program.initial == Program::Glyph {
            return Err(HintErrorKind::DefinitionInGlyphProgram);
        }
        let defs = match kind {
            DefKind::Function => &mut self.definitions.functions,
            DefKind::Instruction => &mut self.definitions.instructions,
        };
        let def = defs.allocate(key)?;
        let start = self.program.decoder.pc;
        while let Some(ins) = self.program.decoder.decode() {
            let ins = ins?;
            match ins.opcode {
                Opcode::FDEF | Opcode::IDEF => return Err(HintErrorKind::NestedDefinition),
                Opcode::ENDF => {
                    let range = start..ins.pc + 1;
                    if self.graphics.is_pedantic && range.len() > MAX_DEFINITION_SIZE {
                        *def = Default::default();
                        return Err(HintErrorKind::DefinitionTooLarge);
                    }
                    *def = Definition::new(self.program.current, range, key);
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(HintErrorKind::UnexpectedEndOfBytecode)
    }

    /// Common code for CALL, LOOPCALL and unknown opcode handling.
    fn do_call(&mut self, kind: DefKind, count: u32, key: i32) -> OpResult {
        if count == 0 {
            return Ok(());
        }
        let def = match kind {
            DefKind::Function => self.definitions.functions.get(key),
            DefKind::Instruction => match self.definitions.instructions.get(key) {
                // Remap an invalid definition error to unhandled opcode
                Err(HintErrorKind::InvalidDefinition(opcode)) => Err(
                    HintErrorKind::UnhandledOpcode(Opcode::from_byte(opcode as u8)),
                ),
                result => result,
            },
        };
        self.program.enter(*def?, count)
    }
}

enum DefKind {
    Function,
    Instruction,
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            super::program::{Program, ProgramState},
            Engine, MockEngine,
        },
        HintErrorKind, Opcode, MAX_DEFINITION_SIZE,
    };

    /// Define two functions, one of which calls the other with
    /// both CALL and LOOPCALL.
    #[test]
    fn define_function_call_loopcall() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            op(PUSHB001), 1, 0,
            // FDEF 0: adds 2 to top stack value
            op(FDEF),
                op(PUSHB000), 2,
                op(ADD),
            op(ENDF),
            // FDEF 1: calls FDEF 0 once, loop calls 5 times, then
            // negates the result
            op(FDEF),
                op(PUSHB000), 0,
                op(CALL),
                op(PUSHB001), 5, 0,
                op(LOOPCALL),
                op(NEG),
            op(ENDF),
        ];
        // Execute this code to define our functions
        engine.set_font_code(&font_code);
        engine.run().unwrap();
        // Call FDEF 1 with value of 10 on the stack:
        // * calls FDEF 0 which adds 2
        // * loop calls FDEF 0 an additional 5 times which adds a total of 10
        // * then negates the result
        // leaving -22 on the stack
        engine.value_stack.push(10).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_call().unwrap();
        engine.run().unwrap();
        assert_eq!(engine.value_stack.pop().ok(), Some(-22));
    }

    /// Control value programs can override functions defined in the font
    /// program based on instance state.
    #[test]
    fn override_function() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            op(PUSHB001), 0, 0,
            // FDEF 0: adds 2 to top stack value
            op(FDEF),
                op(PUSHB000), 2,
                op(ADD),
            op(ENDF),
            // Redefine FDEF 0: subtract 2 instead
            op(FDEF),
                op(PUSHB000), 2,
                op(SUB),                
            op(ENDF),
        ];
        // Execute this code to define our functions
        engine.set_font_code(&font_code);
        engine.run().unwrap();
        // Call FDEF 0 with value of 10 on the stack:
        // * should subtract 2 rather than add
        // leaving 8 on the stack
        engine.value_stack.push(10).unwrap();
        engine.value_stack.push(0).unwrap();
        engine.op_call().unwrap();
        engine.run().unwrap();
        assert_eq!(engine.value_stack.pop().ok(), Some(8));
    }

    /// Executes a call from a CV program into a font program.
    ///
    /// Tests ProgramState bytecode/decoder management.
    #[test]
    fn call_different_program() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            op(PUSHB000), 0,
            // FDEF 0: adds 2 to top stack value
            op(FDEF),
                op(PUSHB000), 2,
                op(ADD),
            op(ENDF),
        ];
        #[rustfmt::skip]
        let cv_code = [
            // Call function defined in font program and negate result
            op(PUSHB001), 40, 0,
            op(CALL),
            op(NEG)
        ];
        let glyph_code = &[];
        // Run font program first to define the function
        engine.program = ProgramState::new(&font_code, &cv_code, glyph_code, Program::Font);
        engine.run().unwrap();
        // Now run CV program which calls into the font program
        engine.program = ProgramState::new(&font_code, &cv_code, glyph_code, Program::ControlValue);
        engine.run().unwrap();
        // Executing CV program:
        // * pushes 40 to the stack
        // * calls FDEF 0 in font program which adds 2
        // * returns to CV program
        // * negates the value
        // leaving -42 on the stack
        assert_eq!(engine.value_stack.pop().ok(), Some(-42));
    }

    /// Fail when we exceed loop call budget.
    #[test]
    fn loopcall_budget() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        let limit = engine.loop_budget.limit;
        #[rustfmt::skip]
        let font_code = [
            op(PUSHB001), 1, 0,
            // FDEF 0: does nothing
            op(FDEF),
            op(ENDF),
            // FDEF 1: loop calls FDEF 0 twice, exceeding the budget on the
            // second attempt
            op(FDEF),
                op(PUSHB001), limit as u8, 0,
                op(LOOPCALL),
                op(PUSHB001), 1, 0,
                op(LOOPCALL), // pc = 13
            op(ENDF),
        ];
        // Execute this code to define our functions
        engine.set_font_code(&font_code);
        engine.run().unwrap();
        // Call FDEF 1 which attempts to loop call FDEF 0 (limit + 1) times
        engine.value_stack.push(10).unwrap();
        engine.value_stack.push(1).unwrap();
        engine.op_call().unwrap();
        let err = engine.run().unwrap_err();
        assert!(matches!(err.kind, HintErrorKind::ExceededExecutionBudget));
        assert_eq!(err.pc, 13);
    }

    /// Defines an instruction using an available opcode and executes it.
    #[test]
    fn define_instruction_and_use() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            // IDEF 0x93: adds 2 to top stack value
            op(PUSHB000), op(INS93),
            op(IDEF),
                op(PUSHB000), 2,
                op(ADD),
            op(ENDF),
            // FDEF 0: uses defined instruction 0x93 and negates the result 
            op(PUSHB000), 0,
            op(FDEF),
                op(INS93),
                op(NEG),
            op(ENDF),
        ];
        // Execute this code to define our functions
        engine.set_font_code(&font_code);
        engine.run().unwrap();
        // Call FDEF 0 with value of 10 on the stack:
        // * executes defined instruction 0x93
        // * then negates the result
        // leaving -12 on the stack
        engine.value_stack.push(10).unwrap();
        engine.value_stack.push(0).unwrap();
        engine.op_call().unwrap();
        engine.run().unwrap();
        assert_eq!(engine.value_stack.pop().ok(), Some(-12));
    }

    // Invalid to nest definitions.
    #[test]
    fn nested_definition() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            op(PUSHB001), 1, 0,
            op(FDEF), // pc = 3
                op(FDEF),
                op(ENDF),
            op(ENDF),
        ];
        // Execute this code to define our functions
        engine.set_font_code(&font_code);
        let err = engine.run().unwrap_err();
        assert!(matches!(err.kind, HintErrorKind::NestedDefinition));
        assert_eq!(err.pc, 3);
    }

    // Invalid to modify definitions from the glyph program.
    #[test]
    fn definition_in_glyph_program() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            op(PUSHB000), 0,
            op(FDEF), // pc = 2
            op(ENDF),
        ];
        engine.set_font_code(&font_code);
        engine.program.initial = Program::Glyph;
        let err = engine.run().unwrap_err();
        assert!(matches!(err.kind, HintErrorKind::DefinitionInGlyphProgram));
        assert_eq!(err.pc, 2);
    }

    #[test]
    fn undefined_function() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        engine.value_stack.push(111).unwrap();
        assert!(matches!(
            engine.op_call(),
            Err(HintErrorKind::InvalidDefinition(111))
        ));
    }

    /// Fun function that just calls itself :)
    #[test]
    fn infinite_recursion() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            // FDEF 0: call FDEF 0
            op(PUSHB000), 0,
            op(FDEF),
                op(PUSHB000), 0,
                op(CALL), // pc = 5
            op(ENDF),
        ];
        engine.set_font_code(&font_code);
        engine.run().unwrap();
        // Call stack overflow
        engine.value_stack.push(0).unwrap();
        engine.op_call().unwrap();
        let err = engine.run().unwrap_err();
        assert!(matches!(err.kind, HintErrorKind::CallStackOverflow));
        assert_eq!(err.pc, 5);
    }

    #[test]
    fn call_stack_underflow() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            op(ENDF)
        ];
        engine.set_font_code(&font_code);
        let err = engine.run().unwrap_err();
        assert!(matches!(err.kind, HintErrorKind::CallStackUnderflow));
        assert_eq!(err.pc, 0);
    }

    #[test]
    fn unhandled_opcode() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            op(Opcode::INS28),
        ];
        engine.set_font_code(&font_code);
        let err = engine.run().unwrap_err();
        assert!(matches!(
            err.kind,
            HintErrorKind::UnhandledOpcode(Opcode::INS28)
        ));
        assert_eq!(err.pc, 0);
    }

    #[test]
    fn too_many_definitions() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        #[rustfmt::skip]
        let font_code = [
            op(PUSHB101), 0, 1, 2, 3, 4, 5,
            op(FDEF), op(ENDF),
            op(FDEF), op(ENDF),
            op(FDEF), op(ENDF),
            op(FDEF), op(ENDF),
            op(FDEF), op(ENDF),
            op(FDEF), op(ENDF),
        ];
        engine.set_font_code(&font_code);
        let err = engine.run().unwrap_err();
        assert!(matches!(err.kind, HintErrorKind::TooManyDefinitions));
        assert_eq!(err.pc, 17);
    }

    #[test]
    fn big_definition() {
        use Opcode::*;
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        let mut font_code = vec![];
        font_code.extend_from_slice(&[op(PUSHB000), 0, op(FDEF)]);
        font_code.extend(core::iter::repeat(op(NEG)).take(MAX_DEFINITION_SIZE + 1));
        font_code.push(op(ENDF));
        engine.set_font_code(&font_code);
        engine.graphics.is_pedantic = true;
        engine.value_stack.push(1).unwrap();
        let err = engine.run().unwrap_err();
        assert!(matches!(err.kind, HintErrorKind::DefinitionTooLarge));
        assert_eq!(err.pc, 2);
    }

    fn op(opcode: Opcode) -> u8 {
        opcode as u8
    }

    impl<'a> Engine<'a> {
        fn set_font_code(&mut self, code: &'a [u8]) {
            self.program.bytecode[0] = code;
            self.program.decoder.bytecode = code;
            self.program.current = Program::Font;
        }
    }
}
