//! TrueType hinting bytecode.

mod decode;
mod instruction;
mod opcode;

pub use decode::{decode_all, DecodeError, Decoder};
pub use instruction::{InlineOperands, Instruction};
pub use opcode::Opcode;

// Exported publicly for use by skrifa when the scaler_test feature is
// enabled.
#[cfg(any(test, feature = "scaler_test"))]
pub use instruction::MockInlineOperands;
