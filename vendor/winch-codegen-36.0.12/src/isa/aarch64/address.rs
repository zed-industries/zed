//! Aarch64 addressing mode.

use anyhow::{Context, Result, anyhow};
use cranelift_codegen::VCodeConstant;
use cranelift_codegen::{
    ir::types,
    isa::aarch64::inst::{AMode, PairAMode, SImm7Scaled, SImm9},
};

use super::regs;
use crate::reg::Reg;

/// Aarch64 indexing mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum Indexing {
    /// Pre-indexed.
    Pre,
    /// Post-indexed.
    Post,
}

/// Memory address representation.
#[derive(Debug, Copy, Clone)]
pub(crate) enum Address {
    /// Base register with an arbitrary offset.  Potentially gets
    /// lowered into multiple instructions during code emission
    /// depending on the offset.
    Offset {
        /// Base register.
        base: Reg,
        /// Offset.
        offset: i64,
    },
    /// Specialized indexed register and offset variant using
    /// the stack pointer.
    IndexedSPOffset {
        /// Offset.
        offset: i64,
        /// Indexing mode.
        indexing: Indexing,
    },
    /// Address of a constant in the constant pool.
    Const(VCodeConstant),
}

impl Address {
    /// Create a pre-indexed addressing mode from the stack pointer.
    pub fn pre_indexed_from_sp(offset: i64) -> Self {
        Self::IndexedSPOffset {
            offset,
            indexing: Indexing::Pre,
        }
    }

    /// Create a post-indexed addressing mode from the stack pointer.
    pub fn post_indexed_from_sp(offset: i64) -> Self {
        Self::IndexedSPOffset {
            offset,
            indexing: Indexing::Post,
        }
    }

    /// Create an offset addressing mode with
    /// the shadow stack pointer register
    /// as a base.
    pub fn from_shadow_sp(offset: i64) -> Self {
        Self::Offset {
            base: regs::shadow_sp(),
            offset,
        }
    }

    /// Create register and arbitrary offset addressing mode.
    pub fn offset(base: Reg, offset: i64) -> Self {
        // This exists to enforce the sp vs shadow_sp invariant, the
        // sp generally should not be used as a base register in an
        // address. In the cases where its usage is required and where
        // we are sure that it's 16-byte aligned, the address should
        // be constructed via the `Self::pre_indexed_sp` and
        // Self::post_indexed_sp functions.
        // For more details around the stack pointer and shadow stack
        // pointer see the docs at regs::shadow_sp().
        assert!(
            base != regs::sp(),
            "stack pointer not allowed in arbitrary offset addressing mode"
        );
        Self::Offset { base, offset }
    }

    /// Create an address for a constant.
    pub fn constant(data: VCodeConstant) -> Self {
        Self::Const(data)
    }

    /// Returns the register base and immediate offset of the given [`Address`].
    ///
    /// # Panics
    /// This function panics if the [`Address`] is not [`Address::Offset`].
    pub fn unwrap_offset(&self) -> (Reg, i64) {
        match self {
            Self::Offset { base, offset } => (*base, *offset),
            _ => panic!("Expected register and offset addressing mode"),
        }
    }
}

// Conversions between `winch-codegen`'s addressing mode representation
// and `cranelift-codegen`s addressing mode representation for aarch64.

impl TryFrom<Address> for PairAMode {
    type Error = anyhow::Error;

    fn try_from(addr: Address) -> Result<Self> {
        use Address::*;
        use Indexing::*;

        match addr {
            IndexedSPOffset { offset, indexing } => {
                let simm7 = SImm7Scaled::maybe_from_i64(offset, types::I64).with_context(|| {
                    format!("Failed to convert {offset} to signed scaled 7 bit offset")
                })?;

                if indexing == Pre {
                    Ok(PairAMode::SPPreIndexed { simm7 })
                } else {
                    Ok(PairAMode::SPPostIndexed { simm7 })
                }
            }
            other => Err(anyhow!(
                "Could not convert {:?} to addressing mode for register pairs",
                other
            )),
        }
    }
}

impl TryFrom<Address> for AMode {
    type Error = anyhow::Error;

    fn try_from(addr: Address) -> Result<Self> {
        use Address::*;
        use Indexing::*;

        match addr {
            IndexedSPOffset { offset, indexing } => {
                let simm9 = SImm9::maybe_from_i64(offset).ok_or_else(|| {
                    // TODO: non-string error
                    anyhow!("Failed to convert {} to signed 9-bit offset", offset)
                })?;

                if indexing == Pre {
                    Ok(AMode::SPPreIndexed { simm9 })
                } else {
                    Ok(AMode::SPPostIndexed { simm9 })
                }
            }
            Offset { base, offset } => Ok(AMode::RegOffset {
                rn: base.into(),
                off: offset,
            }),
            Const(data) => Ok(AMode::Const { addr: data }),
        }
    }
}
