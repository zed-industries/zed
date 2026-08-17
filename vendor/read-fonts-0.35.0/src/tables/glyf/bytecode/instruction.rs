/// Decoded representation of a TrueType instruction.
use super::Opcode;

/// Decoded TrueType instruction.
#[derive(Copy, Clone, Debug)]
pub struct Instruction<'a> {
    /// Operation code.
    pub opcode: Opcode,
    /// Instruction operands that were decoded from the bytecode.
    pub inline_operands: InlineOperands<'a>,
    /// Program counter -- offset into the bytecode where this
    /// instruction was decoded.
    pub pc: usize,
}

impl std::fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.opcode.name())?;
        for value in self.inline_operands.values() {
            write!(f, " {value}")?;
        }
        Ok(())
    }
}

/// Sequence of instruction operands that are encoded directly in the bytecode.
///
/// This is only used for push instructions.
#[derive(Copy, Clone, Default, Debug)]
pub struct InlineOperands<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) is_words: bool,
}

impl<'a> InlineOperands<'a> {
    /// Returns the number of operands.
    #[inline]
    pub fn len(&self) -> usize {
        if self.is_words {
            self.bytes.len() / 2
        } else {
            self.bytes.len()
        }
    }

    /// Returns true if there are no operands.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns an iterator over the operand values.
    #[inline]
    pub fn values(&self) -> impl Iterator<Item = i32> + 'a + Clone {
        let (bytes, words) = if self.is_words {
            (&[][..], self.bytes)
        } else {
            (self.bytes, &[][..])
        };
        bytes
            .iter()
            .map(|byte| *byte as u32 as i32)
            .chain(words.chunks_exact(2).map(|chunk| {
                let word = ((chunk[0] as u16) << 8) | chunk[1] as u16;
                // Double cast to ensure sign extension
                word as i16 as i32
            }))
    }
}

/// Mock for testing inline operands.
#[cfg(any(test, feature = "scaler_test"))]
pub struct MockInlineOperands {
    bytes: Vec<u8>,
    is_words: bool,
}

#[cfg(any(test, feature = "scaler_test"))]
impl MockInlineOperands {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.into(),
            is_words: false,
        }
    }

    pub fn from_words(words: &[i16]) -> Self {
        Self {
            bytes: words
                .iter()
                .map(|word| *word as u16)
                .flat_map(|word| vec![(word >> 8) as u8, word as u8])
                .collect(),
            is_words: true,
        }
    }

    pub fn operands(&self) -> InlineOperands {
        InlineOperands {
            bytes: &self.bytes,
            is_words: self.is_words,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MockInlineOperands;

    #[test]
    fn byte_operands() {
        let values = [5, 2, 85, 92, 26, 42, u8::MIN, u8::MAX];
        let mock = MockInlineOperands::from_bytes(&values);
        let decoded = mock.operands().values().collect::<Vec<_>>();
        assert!(values.iter().map(|x| *x as i32).eq(decoded.iter().copied()));
    }

    #[test]
    fn word_operands() {
        let values = [-5, 2, 2845, 92, -26, 42, i16::MIN, i16::MAX];
        let mock = MockInlineOperands::from_words(&values);
        let decoded = mock.operands().values().collect::<Vec<_>>();
        assert!(values.iter().map(|x| *x as i32).eq(decoded.iter().copied()));
    }
}
