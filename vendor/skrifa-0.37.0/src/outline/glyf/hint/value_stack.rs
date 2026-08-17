//! Value stack for TrueType interpreter.
//!
use raw::types::F26Dot6;
use read_fonts::tables::glyf::bytecode::InlineOperands;

use super::error::HintErrorKind;

use HintErrorKind::{ValueStackOverflow, ValueStackUnderflow};

/// Value stack for the TrueType interpreter.
///
/// This uses a slice as the backing store rather than a `Vec` to enable
/// support for allocation from user buffers.
///
/// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#managing-the-stack>
pub struct ValueStack<'a> {
    values: &'a mut [i32],
    len: usize,
    pub(super) is_pedantic: bool,
}

impl<'a> ValueStack<'a> {
    pub fn new(values: &'a mut [i32], is_pedantic: bool) -> Self {
        Self {
            values,
            len: 0,
            is_pedantic,
        }
    }

    /// Returns the depth of the stack
    /// <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#returns-the-depth-of-the-stack>
    pub fn len(&self) -> usize {
        self.len
    }

    #[cfg(test)]
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    // This is used in tests and also useful for tracing.
    #[allow(dead_code)]
    pub fn values(&self) -> &[i32] {
        &self.values[..self.len]
    }

    pub fn push(&mut self, value: i32) -> Result<(), HintErrorKind> {
        let ptr = self
            .values
            .get_mut(self.len)
            .ok_or(HintErrorKind::ValueStackOverflow)?;
        *ptr = value;
        self.len += 1;
        Ok(())
    }

    /// Pushes values that have been decoded from the instruction stream
    /// onto the stack.
    ///
    /// Implements the PUSHB[], PUSHW[], NPUSHB[] and NPUSHW[] instructions.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#pushing-data-onto-the-interpreter-stack>
    pub fn push_inline_operands(&mut self, operands: &InlineOperands) -> Result<(), HintErrorKind> {
        let push_count = operands.len();
        let stack_base = self.len;
        for (stack_value, value) in self
            .values
            .get_mut(stack_base..stack_base + push_count)
            .ok_or(ValueStackOverflow)?
            .iter_mut()
            .zip(operands.values())
        {
            *stack_value = value;
        }
        self.len += push_count;
        Ok(())
    }

    pub fn peek(&mut self) -> Option<i32> {
        if self.len > 0 {
            self.values.get(self.len - 1).copied()
        } else {
            None
        }
    }

    /// Pops a value from the stack.
    ///
    /// Implements the POP[] instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#pop-top-stack-element>
    pub fn pop(&mut self) -> Result<i32, HintErrorKind> {
        if let Some(value) = self.peek() {
            self.len -= 1;
            Ok(value)
        } else if self.is_pedantic {
            Err(ValueStackUnderflow)
        } else {
            Ok(0)
        }
    }

    /// Convenience method for instructions that expect values in 26.6 format.
    pub fn pop_f26dot6(&mut self) -> Result<F26Dot6, HintErrorKind> {
        Ok(F26Dot6::from_bits(self.pop()?))
    }

    /// Convenience method for instructions that pop values that are used as an
    /// index.
    pub fn pop_usize(&mut self) -> Result<usize, HintErrorKind> {
        Ok(self.pop()? as usize)
    }

    /// Convenience method for popping a value intended as a count.
    ///
    /// When a negative value is encountered, returns an error in pedantic mode
    /// or 0 otherwise.
    pub fn pop_count_checked(&mut self) -> Result<usize, HintErrorKind> {
        let value = self.pop()?;
        if value < 0 && self.is_pedantic {
            Err(HintErrorKind::InvalidStackValue(value))
        } else {
            Ok(value.max(0) as usize)
        }
    }

    /// Applies a unary operation.
    ///
    /// Pops `a` from the stack and pushes `op(a)`.
    pub fn apply_unary(
        &mut self,
        mut op: impl FnMut(i32) -> Result<i32, HintErrorKind>,
    ) -> Result<(), HintErrorKind> {
        let a = self.pop()?;
        self.push(op(a)?)
    }

    /// Applies a binary operation.
    ///
    /// Pops `b` and `a` from the stack and pushes `op(a, b)`.
    pub fn apply_binary(
        &mut self,
        mut op: impl FnMut(i32, i32) -> Result<i32, HintErrorKind>,
    ) -> Result<(), HintErrorKind> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(op(a, b)?)
    }

    /// Clear the entire stack.
    ///
    /// Implements the CLEAR[] instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#clear-the-entire-stack>
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Duplicate top stack element.
    ///
    /// Implements the DUP[] instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#duplicate-top-stack-element>
    pub fn dup(&mut self) -> Result<(), HintErrorKind> {
        if let Some(value) = self.peek() {
            self.push(value)
        } else if self.is_pedantic {
            Err(ValueStackUnderflow)
        } else {
            self.push(0)
        }
    }

    /// Swap the top two elements on the stack.
    ///
    /// Implements the SWAP[] instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#swap-the-top-two-elements-on-the-stack>
    pub fn swap(&mut self) -> Result<(), HintErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(a)?;
        self.push(b)
    }

    /// Copy the indexed element to the top of the stack.
    ///
    /// Implements the CINDEX[] instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#copy-the-indexed-element-to-the-top-of-the-stack>
    pub fn copy_index(&mut self) -> Result<(), HintErrorKind> {
        let top_ix = self.len.checked_sub(1).ok_or(ValueStackUnderflow)?;
        let index = *self.values.get(top_ix).ok_or(ValueStackUnderflow)? as usize;
        let element_ix = top_ix.checked_sub(index).ok_or(ValueStackUnderflow)?;
        self.values[top_ix] = self.values[element_ix];
        Ok(())
    }

    /// Moves the indexed element to the top of the stack.
    ///
    /// Implements the MINDEX[] instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#move-the-indexed-element-to-the-top-of-the-stack>
    pub fn move_index(&mut self) -> Result<(), HintErrorKind> {
        let top_ix = self.len.checked_sub(1).ok_or(ValueStackUnderflow)?;
        let index = *self.values.get(top_ix).ok_or(ValueStackUnderflow)? as usize;
        let element_ix = top_ix.checked_sub(index).ok_or(ValueStackUnderflow)?;
        let new_top_ix = top_ix.checked_sub(1).ok_or(ValueStackUnderflow)?;
        let value = self.values[element_ix];
        self.values
            .copy_within(element_ix + 1..self.len, element_ix);
        self.values[new_top_ix] = value;
        self.len -= 1;
        Ok(())
    }

    /// Roll the top three stack elements.
    ///
    /// Implements the ROLL[] instruction.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#roll-the-top-three-stack-elements>
    pub fn roll(&mut self) -> Result<(), HintErrorKind> {
        let a = self.pop()?;
        let b = self.pop()?;
        let c = self.pop()?;
        self.push(b)?;
        self.push(a)?;
        self.push(c)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{HintErrorKind, ValueStack};
    use read_fonts::tables::glyf::bytecode::MockInlineOperands;

    // The following are macros because functions can't return a new ValueStack
    // with a borrowed parameter.
    macro_rules! make_stack {
        ($values:expr) => {
            ValueStack {
                values: $values,
                len: $values.len(),
                is_pedantic: true,
            }
        };
    }
    macro_rules! make_empty_stack {
        ($values:expr) => {
            ValueStack {
                values: $values,
                len: 0,
                is_pedantic: true,
            }
        };
    }

    #[test]
    fn push() {
        let mut stack = make_empty_stack!(&mut [0; 4]);
        for i in 0..4 {
            stack.push(i).unwrap();
            assert_eq!(stack.peek(), Some(i));
        }
        assert!(matches!(
            stack.push(0),
            Err(HintErrorKind::ValueStackOverflow)
        ));
    }

    #[test]
    fn push_args() {
        let mut stack = make_empty_stack!(&mut [0; 32]);
        let values = [-5, 2, 2845, 92, -26, 42, i16::MIN, i16::MAX];
        let mock_args = MockInlineOperands::from_words(&values);
        stack.push_inline_operands(&mock_args.operands()).unwrap();
        let mut popped = vec![];
        while !stack.is_empty() {
            popped.push(stack.pop().unwrap());
        }
        assert!(values
            .iter()
            .rev()
            .map(|x| *x as i32)
            .eq(popped.iter().copied()));
    }

    #[test]
    fn pop() {
        let mut stack = make_stack!(&mut [0, 1, 2, 3]);
        for i in (0..4).rev() {
            assert_eq!(stack.pop().ok(), Some(i));
        }
        assert!(matches!(
            stack.pop(),
            Err(HintErrorKind::ValueStackUnderflow)
        ));
    }

    #[test]
    fn dup() {
        let mut stack = make_stack!(&mut [1, 2, 3, 0]);
        // pop extra element so we have room for dup
        stack.pop().unwrap();
        stack.dup().unwrap();
        assert_eq!(stack.values(), &[1, 2, 3, 3]);
    }

    #[test]
    fn swap() {
        let mut stack = make_stack!(&mut [1, 2, 3]);
        stack.swap().unwrap();
        assert_eq!(stack.values(), &[1, 3, 2]);
    }

    #[test]
    fn copy_index() {
        let mut stack = make_stack!(&mut [4, 10, 2, 1, 3]);
        stack.copy_index().unwrap();
        assert_eq!(stack.values(), &[4, 10, 2, 1, 10]);
    }

    #[test]
    fn move_index() {
        let mut stack = make_stack!(&mut [4, 10, 2, 1, 3]);
        stack.move_index().unwrap();
        assert_eq!(stack.values(), &[4, 2, 1, 10]);
    }

    #[test]
    fn roll() {
        let mut stack = make_stack!(&mut [1, 2, 3]);
        stack.roll().unwrap();
        assert_eq!(stack.values(), &[2, 3, 1]);
    }

    #[test]
    fn unnop() {
        let mut stack = make_stack!(&mut [42]);
        stack.apply_unary(|a| Ok(-a)).unwrap();
        assert_eq!(stack.peek(), Some(-42));
        stack.apply_unary(|a| Ok(!a)).unwrap();
        assert_eq!(stack.peek(), Some(!-42));
    }

    #[test]
    fn binop() {
        let mut stack = make_empty_stack!(&mut [0; 32]);
        for value in 1..=5 {
            stack.push(value).unwrap();
        }
        stack.apply_binary(|a, b| Ok(a + b)).unwrap();
        assert_eq!(stack.peek(), Some(9));
        stack.apply_binary(|a, b| Ok(a * b)).unwrap();
        assert_eq!(stack.peek(), Some(27));
        stack.apply_binary(|a, b| Ok(a - b)).unwrap();
        assert_eq!(stack.peek(), Some(-25));
        stack.apply_binary(|a, b| Ok(a / b)).unwrap();
        assert_eq!(stack.peek(), Some(0));
    }

    // Subtract with overflow when stack size is 1 and element index is 0
    // https://oss-fuzz.com/testcase-detail/5765856825507840
    #[test]
    fn move_index_avoid_overflow() {
        let mut stack = make_stack!(&mut [0]);
        // Don't panic
        let _ = stack.move_index();
    }

    #[test]
    fn pop_count_checked() {
        let mut stack = make_stack!(&mut [-1, 128, -1, 128]);
        stack.is_pedantic = true;
        assert_eq!(stack.pop_count_checked(), Ok(128));
        // In pedantic mode, return an error for negative values
        assert!(matches!(
            stack.pop_count_checked(),
            Err(HintErrorKind::InvalidStackValue(-1))
        ));
        stack.is_pedantic = false;
        assert_eq!(stack.pop_count_checked(), Ok(128));
        // In non-pedantic mode, return 0 instead of error
        assert_eq!(stack.pop_count_checked(), Ok(0));
    }
}
