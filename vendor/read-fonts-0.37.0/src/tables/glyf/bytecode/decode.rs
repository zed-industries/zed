//! TrueType bytecode decoder.

use super::{InlineOperands, Instruction, Opcode};

/// An error returned by [`Decoder::decode`] if the end of the bytecode
/// stream is reached unexpectedly.
#[derive(Copy, Clone, Debug)]
pub struct DecodeError;

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unexpected end of bytecode")
    }
}

/// Decodes instructions from TrueType bytecode.
#[derive(Copy, Clone)]
pub struct Decoder<'a> {
    /// The bytecode for the program.
    pub bytecode: &'a [u8],
    /// The "program counter" or current offset into the bytecode.
    pub pc: usize,
}

impl<'a> Decoder<'a> {
    /// Creates a new decoder for the given bytecode and program counter.
    pub fn new(bytecode: &'a [u8], pc: usize) -> Self {
        Self { bytecode, pc }
    }

    /// Decodes the next instruction.
    ///
    /// Returns `None` at the end of the bytecode stream.
    pub fn decode(&mut self) -> Option<Result<Instruction<'a>, DecodeError>> {
        let opcode = Opcode::from_byte(*self.bytecode.get(self.pc)?);
        Some(self.decode_inner(opcode))
    }

    fn decode_inner(&mut self, opcode: Opcode) -> Result<Instruction<'a>, DecodeError> {
        let mut opcode_len = opcode.len();
        let mut count_len = 0;
        // If the opcode length is negative the next byte contains the number
        // of inline operands and |opcode_len| is the size of each operand.
        // <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L7046>
        if opcode_len < 0 {
            let inline_count = *self.bytecode.get(self.pc + 1).ok_or(DecodeError)?;
            opcode_len = opcode_len.abs() * inline_count as i32 + 2;
            count_len = 1;
        }
        let opcode_len = opcode_len as usize;
        let pc = self.pc;
        let next_pc = pc + opcode_len;
        // Skip opcode and potential inline operand count byte.
        let inline_start = pc + 1 + count_len;
        let inline_size = next_pc - inline_start;
        let mut inline_operands = InlineOperands::default();
        if inline_size > 0 {
            inline_operands.bytes = self
                .bytecode
                .get(inline_start..inline_start + inline_size)
                .ok_or(DecodeError)?;
            inline_operands.is_words = opcode.is_push_words();
        }
        self.pc += opcode_len;
        Ok(Instruction {
            opcode,
            inline_operands,
            pc,
        })
    }
}

/// Returns an iterator that yields all instructions in the given bytecode
/// starting at the specified program counter.
pub fn decode_all(
    bytecode: &[u8],
    pc: usize,
) -> impl Iterator<Item = Result<Instruction<'_>, DecodeError>> + '_ + Clone {
    let mut decoder = Decoder::new(bytecode, pc);
    std::iter::from_fn(move || decoder.decode())
}

#[cfg(test)]
mod tests {
    use super::Opcode;

    #[test]
    fn mixed_ops() {
        let mut enc = Encoder::default();
        // intermix push and non-push ops of various sizes to test boundary
        // conditions
        let cases: &[(Opcode, &[i16])] = &[
            (Opcode::PUSHB100, &[1, 2, 3, 255, 5]),
            (Opcode::PUSHW010, &[-1, 4508, -3]),
            (Opcode::IUP0, &[]),
            (Opcode::NPUSHB, &[55; 255]),
            (Opcode::MDRP00110, &[]),
            (Opcode::NPUSHW, &[i16::MIN; 32]),
            (Opcode::LOOPCALL, &[]),
            (Opcode::FLIPOFF, &[]),
            (
                Opcode::PUSHW011,
                &[i16::MIN, i16::MIN / 2, i16::MAX, i16::MAX / 2],
            ),
            (Opcode::GETVARIATION, &[]),
        ];
        for (opcode, values) in cases {
            if !values.is_empty() {
                enc.encode_push(values);
            } else {
                enc.encode(*opcode);
            }
        }
        let all_ins = super::decode_all(&enc.0, 0)
            .map(|ins| ins.unwrap())
            .collect::<Vec<_>>();
        for (ins, (expected_opcode, expected_values)) in all_ins.iter().zip(cases) {
            assert_eq!(ins.opcode, *expected_opcode);
            let values = ins
                .inline_operands
                .values()
                .map(|v| v as i16)
                .collect::<Vec<_>>();
            assert_eq!(&values, expected_values);
        }
    }

    #[test]
    fn non_push_ops() {
        // test decoding of all single byte (non-push) opcodes
        let non_push_ops: Vec<_> = (0..=255)
            .filter(|b| !Opcode::from_byte(*b).is_push())
            .collect();
        let decoded: Vec<_> = super::decode_all(&non_push_ops, 0)
            .map(|ins| ins.unwrap().opcode as u8)
            .collect();
        assert_eq!(non_push_ops, decoded);
    }

    #[test]
    fn real_bytecode() {
        // taken from NotoSerif-Regular, glyph Rturnedsmall, gid 1272
        let bytecode = [
            181, 5, 1, 9, 3, 1, 76, 75, 176, 45, 80, 88, 64, 35, 0, 3, 0, 9, 7, 3, 9, 105, 6, 4, 2,
            1, 1, 2, 97, 5, 1, 2, 2, 109, 77, 11, 8, 2, 7, 7, 0, 95, 10, 1, 0, 0, 107, 0, 78, 27,
            64, 41, 0, 7, 8, 0, 8, 7, 114, 0, 3, 0, 9, 8, 3, 9, 105, 6, 4, 2, 1, 1, 2, 97, 5, 1, 2,
            2, 109, 77, 11, 1, 8, 8, 0, 95, 10, 1, 0, 0, 107, 0, 78, 89, 64, 31, 37, 36, 1, 0, 40,
            38, 36, 44, 37, 44, 34, 32, 27, 25, 24, 23, 22, 20, 17, 16, 12, 10, 9, 8, 0, 35, 1, 35,
            12, 13, 22, 43,
        ];
        // comments below contain the ttx assembly
        let expected = [
            // PUSHB[ ]	/* 6 values pushed */
            // 5 1 9 3 1 76
            "PUSHB[5] 5 1 9 3 1 76",
            // MPPEM[ ]	/* MeasurePixelPerEm */
            "MPPEM",
            // PUSHB[ ]	/* 1 value pushed */
            // 45
            "PUSHB[0] 45",
            // LT[ ]	/* LessThan */
            "LT",
            // IF[ ]	/* If */
            "IF",
            //   NPUSHB[ ]	/* 35 values pushed */
            //   0 3 0 9 7 3 9 105 6 4 2 1 1 2 97 5 1 2 2 109 77 11 8 2 7
            //   7 0 95 10 1 0 0 107 0 78
            "NPUSHB 0 3 0 9 7 3 9 105 6 4 2 1 1 2 97 5 1 2 2 109 77 11 8 2 7 7 0 95 10 1 0 0 107 0 78",
            // ELSE[ ]	/* Else */
            "ELSE",
            //   NPUSHB[ ]	/* 41 values pushed */
            //   0 7 8 0 8 7 114 0 3 0 9 8 3 9 105 6 4 2 1 1 2 97 5 1 2
            //   2 109 77 11 1 8 8 0 95 10 1 0 0 107 0 78
            "NPUSHB 0 7 8 0 8 7 114 0 3 0 9 8 3 9 105 6 4 2 1 1 2 97 5 1 2 2 109 77 11 1 8 8 0 95 10 1 0 0 107 0 78",
            // EIF[ ]	/* EndIf */
            "EIF",
            // NPUSHB[ ]	/* 31 values pushed */
            // 37 36 1 0 40 38 36 44 37 44 34 32 27 25 24 23 22 20 17 16 12 10 9 8 0
            // 35 1 35 12 13 22
            "NPUSHB 37 36 1 0 40 38 36 44 37 44 34 32 27 25 24 23 22 20 17 16 12 10 9 8 0 35 1 35 12 13 22",
            // CALL[ ]	/* CallFunction */
            "CALL",
        ];
        let decoded: Vec<_> = super::decode_all(&bytecode, 0)
            .map(|ins| ins.unwrap())
            .collect();
        let decoded_asm: Vec<_> = decoded.iter().map(|ins| ins.to_string()).collect();
        assert_eq!(decoded_asm, expected);
    }

    /// Simple encoder used for testing.
    #[derive(Default)]
    struct Encoder(Vec<u8>);

    impl Encoder {
        pub fn encode(&mut self, opcode: Opcode) {
            assert!(!opcode.is_push(), "use the encode_push method instead");
            self.0.push(opcode as u8);
        }

        pub fn encode_push(&mut self, values: &[i16]) {
            if values.is_empty() {
                return;
            }
            let is_bytes = values.iter().all(|&x| x >= 0 && x <= u8::MAX as _);
            if values.len() < 256 {
                if is_bytes {
                    if values.len() <= 8 {
                        let opcode =
                            Opcode::from_byte(Opcode::PUSHB000 as u8 + values.len() as u8 - 1);
                        self.0.push(opcode as u8);
                    } else {
                        self.0.push(Opcode::NPUSHB as _);
                        self.0.push(values.len() as _);
                    }
                    self.0.extend(values.iter().map(|&x| x as u8));
                } else {
                    if values.len() <= 8 {
                        let opcode =
                            Opcode::from_byte(Opcode::PUSHW000 as u8 + values.len() as u8 - 1);
                        self.0.push(opcode as u8);
                    } else {
                        self.0.push(Opcode::NPUSHW as _);
                        self.0.push(values.len() as _)
                    }
                    for &value in values {
                        let value = value as u16;
                        self.0.push((value >> 8) as _);
                        self.0.push((value & 0xFF) as _);
                    }
                }
            } else {
                panic!("too many values to push in a single instruction");
            }
        }
    }
}
