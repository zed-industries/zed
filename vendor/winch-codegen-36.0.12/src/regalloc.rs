use crate::{
    codegen::CodeGenError,
    isa::reg::{Reg, RegClass},
    regset::{RegBitSet, RegSet},
};

use anyhow::{Result, anyhow};

/// The register allocator.
///
/// The register allocator uses a single-pass algorithm;
/// its implementation uses a bitset as a freelist
/// to track per-class register availability.
///
/// If a particular register is not available upon request
/// the register allocation will perform a "spill", essentially
/// moving Local and Register values in the stack to memory.
/// This process ensures that whenever a register is requested,
/// it is going to be available.
pub(crate) struct RegAlloc {
    /// The register set.
    regset: RegSet,
}

impl RegAlloc {
    /// Create a register allocator from a bit set for each register class.
    pub fn from(gpr: RegBitSet, fpr: RegBitSet) -> Self {
        let rs = RegSet::new(gpr, fpr);
        Self { regset: rs }
    }

    /// Allocate the next available register for the given class,
    /// spilling if not available.
    pub fn reg_for_class<F>(&mut self, class: RegClass, spill: &mut F) -> Result<Reg>
    where
        F: FnMut(&mut RegAlloc) -> Result<()>,
    {
        match self.regset.reg_for_class(class) {
            Some(reg) => Ok(reg),
            None => {
                spill(self)?;
                self.regset
                    .reg_for_class(class)
                    .ok_or_else(|| anyhow!(CodeGenError::expected_register_to_be_available()))
            }
        }
    }

    /// Returns true if the specified register is allocatable.
    pub fn reg_available(&self, reg: Reg) -> bool {
        self.regset.named_reg_available(reg)
    }

    /// Request a specific register, spilling if not available.
    pub fn reg<F>(&mut self, named: Reg, mut spill: F) -> Result<Reg>
    where
        F: FnMut(&mut RegAlloc) -> Result<()>,
    {
        match self.regset.reg(named) {
            Some(reg) => Ok(reg),
            None => {
                spill(self)?;
                self.regset
                    .reg(named)
                    .ok_or_else(|| anyhow!(CodeGenError::expected_register_to_be_available()))
            }
        }
    }

    /// Free the given register.
    pub fn free(&mut self, reg: Reg) {
        self.regset.free(reg);
    }
}
