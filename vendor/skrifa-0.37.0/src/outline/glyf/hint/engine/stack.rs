//! Managing the stack and pushing data onto the interpreter stack.
//!
//! Implements 26 instructions.
//!
//! See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#managing-the-stack>
//! and <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#pushing-data-onto-the-interpreter-stack>

use read_fonts::tables::glyf::bytecode::InlineOperands;

use super::{Engine, OpResult};

impl Engine<'_> {
    /// Duplicate top stack element.
    ///
    /// DUP[] (0x20)
    ///
    /// Pops: e
    /// Pushes: e, e
    ///
    /// Duplicates the element at the top of the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#duplicate-top-stack-element>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2650>
    pub(super) fn op_dup(&mut self) -> OpResult {
        self.value_stack.dup()
    }

    /// Pop top stack element.
    ///
    /// POP[] (0x21)
    ///
    /// Pops: e
    ///
    /// Pops the top element of the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#pop-top-stack-element>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2663>
    pub(super) fn op_pop(&mut self) -> OpResult {
        self.value_stack.pop()?;
        Ok(())
    }

    /// Clear the entire stack.
    ///
    /// CLEAR[] (0x22)
    ///
    /// Pops: all the items on the stack
    ///
    /// Clears all elements from the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#clear-the-entire-stack>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2676>
    pub(super) fn op_clear(&mut self) -> OpResult {
        self.value_stack.clear();
        Ok(())
    }

    /// Swap the top two elements on the stack.
    ///
    /// SWAP[] (0x23)
    ///
    /// Pops: e2, e1
    /// Pushes: e1, e2
    ///
    /// Swaps the top two elements of the stack making the old top element the
    /// second from the top and the old second element the top element.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#swap-the-top-two-elements-on-the-stack>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2689>
    pub(super) fn op_swap(&mut self) -> OpResult {
        self.value_stack.swap()
    }

    /// Returns the depth of the stack.
    ///
    /// DEPTH[] (0x24)
    ///
    /// Pushes: n; number of elements
    ///
    /// Pushes n, the number of elements currently in the stack onto the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#returns-the-depth-of-the-stack>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L2707>
    pub(super) fn op_depth(&mut self) -> OpResult {
        let n = self.value_stack.len();
        self.value_stack.push(n as i32)
    }

    /// Copy the indexed element to the top of the stack.
    ///
    /// CINDEX[] (0x25)
    ///
    /// Pops: k: stack element number
    /// Pushes: ek: indexed element
    ///
    /// Puts a copy of the kth stack element on the top of the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#copy-the-indexed-element-to-the-top-of-the-stack>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3232>
    pub(super) fn op_cindex(&mut self) -> OpResult {
        self.value_stack.copy_index()
    }

    /// Move the indexed element to the top of the stack.
    ///
    /// MINDEX[] (0x26)
    ///
    /// Pops: k: stack element number
    /// Pushes: ek: indexed element
    ///
    /// Moves the indexed element to the top of the stack.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#move-the-indexed-element-to-the-top-of-the-stack>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3199>
    pub(super) fn op_mindex(&mut self) -> OpResult {
        self.value_stack.move_index()
    }

    /// Roll the top three stack elements.
    ///
    /// ROLL[] (0x8a)
    ///
    /// Pops: a, b, c (top three stack elements)
    /// Pushes: b, a, c (elements reordered)
    ///
    /// Performs a circular shift of the top three objects on the stack with
    /// the effect being to move the third element to the top of the stack
    /// and to move the first two elements down one position. ROLL is
    /// equivalent to MINDEX[] 3.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#roll-the-top-three-stack-elements>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3258>
    pub(super) fn op_roll(&mut self) -> OpResult {
        self.value_stack.roll()
    }

    /// Push data onto the interpreter stack.
    ///
    /// NPUSHB[] (0x8a)
    ///
    /// Takes n unsigned bytes from the instruction stream, where n is an
    /// unsigned integer in the range (0..255), and pushes them onto the stack.
    /// n itself is not pushed onto the stack.
    ///
    /// NPUSHW[] (0x41)
    ///
    /// Takes n 16-bit signed words from the instruction stream, where n is an
    /// unsigned integer in the range (0..255), and pushes them onto the stack.
    /// n itself is not pushed onto the stack.
    ///
    /// PUSHB\[abc\] (0xB0 - 0xB7)
    ///
    /// Takes the specified number of bytes from the instruction stream and
    /// pushes them onto the interpreter stack.
    /// The variables a, b, and c are binary digits representing numbers from
    /// 000 to 111 (0-7 in binary). Because the actual number of bytes (n) is
    /// from 1 to 8, 1 is automatically added to the ABC figure to obtain the
    /// actual number of bytes pushed.
    ///
    /// PUSHW\[abc\] (0xB8 - 0xBF)
    ///
    /// Takes the specified number of words from the instruction stream and
    /// pushes them onto the interpreter stack.
    /// The variables a, b, and c are binary digits representing numbers from
    /// 000 to 111 (0-7 binary). Because the actual number of bytes (n) is from
    /// 1 to 8, 1 is automatically added to the abc figure to obtain the actual
    /// number of bytes pushed.
    ///
    /// See <https://learn.microsoft.com/en-us/typography/opentype/spec/tt_instructions#pushing-data-onto-the-interpreter-stack>
    /// and <https://gitlab.freedesktop.org/freetype/freetype/-/blob/57617782464411201ce7bbc93b086c1b4d7d84a5/src/truetype/ttinterp.c#L3858>
    pub(super) fn op_push(&mut self, operands: &InlineOperands) -> OpResult {
        self.value_stack.push_inline_operands(operands)
    }
}

#[cfg(test)]
mod tests {
    use super::super::MockEngine;
    use read_fonts::tables::glyf::bytecode::MockInlineOperands;

    #[test]
    fn stack_ops() {
        let mut mock = MockEngine::new();
        let mut engine = mock.engine();
        let byte_args = MockInlineOperands::from_bytes(&[2, 4, 6, 8]);
        let word_args = MockInlineOperands::from_words(&[-2000, 4000, -6000, 8000]);
        let initial_stack = byte_args
            .operands()
            .values()
            .chain(word_args.operands().values())
            .collect::<Vec<_>>();
        // Push instructions
        engine.op_push(&byte_args.operands()).unwrap();
        engine.op_push(&word_args.operands()).unwrap();
        assert_eq!(engine.value_stack.values(), initial_stack);
        // DEPTH[]
        engine.op_depth().unwrap();
        assert_eq!(
            engine.value_stack.pop().ok(),
            Some(initial_stack.len() as i32)
        );
        // POP[]
        engine.op_pop().unwrap();
        engine.op_pop().unwrap();
        assert_eq!(
            engine.value_stack.values(),
            &initial_stack[..initial_stack.len() - 2]
        );
        // SWAP[]
        engine.op_swap().unwrap();
        assert_eq!(&engine.value_stack.values()[4..], &[4000, -2000]);
        // ROLL[]
        engine.op_roll().unwrap();
        assert_eq!(&engine.value_stack.values()[3..], &[4000, -2000, 8]);
        // CINDEX[]
        engine.value_stack.push(4).unwrap();
        engine.op_cindex().unwrap();
        assert_eq!(engine.value_stack.peek(), Some(6));
        // MINDEX[]
        engine.value_stack.push(3).unwrap();
        engine.op_mindex().unwrap();
        assert_eq!(engine.value_stack.peek(), Some(-2000));
    }
}
