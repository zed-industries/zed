//! x64 addressing mode.

use crate::reg::Reg;
use cranelift_codegen::VCodeConstant;

/// Memory address representation.
#[derive(Debug, Copy, Clone)]
pub(crate) enum Address {
    /// Base register with an immediate offset.
    Offset { base: Reg, offset: u32 },
    /// Address to identify a constant.
    Const(VCodeConstant),
    /// Address at `(base + index * 2^shift) + simm32`
    ImmRegRegShift {
        simm32: i32,
        base: Reg,
        index: Reg,
        shift: u8,
    },
}

impl Address {
    /// Create an offset.
    pub fn offset(base: Reg, offset: u32) -> Self {
        Self::Offset { base, offset }
    }

    /// Create an address for a constant.
    pub fn constant(data: VCodeConstant) -> Self {
        Self::Const(data)
    }

    /// Check if the address is a made of a base and offset.
    pub fn is_offset(&self) -> bool {
        match self {
            Self::Offset { .. } => true,
            _ => false,
        }
    }
}
