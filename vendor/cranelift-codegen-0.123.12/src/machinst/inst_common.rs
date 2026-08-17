//! A place to park MachInst::Inst fragments which are common across multiple architectures.

use crate::ir::Inst as IRInst;

//============================================================================
// Instruction input "slots".
//
// We use these types to refer to operand numbers, and result numbers, together
// with the associated instruction, in a type-safe way.

/// Identifier for a particular input of an instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InsnInput {
    pub(crate) insn: IRInst,
    pub(crate) input: usize,
}
