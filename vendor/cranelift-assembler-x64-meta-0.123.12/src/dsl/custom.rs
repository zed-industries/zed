//! Define customizations available for aspects of the assembler.
//!
//! When a customization is applied to an instruction, the generated code will
//! call the corresponding function in a `custom` module
//! (`custom::<customization>::<inst>`). E.g., to modify the display of a `NOP`
//! instruction with format `M`, the generated assembler will call:
//! `custom::display::nop_m(...)`.

use core::fmt;
use std::ops::BitOr;

#[derive(PartialEq, Debug)]
pub enum Customization {
    /// Modify the disassembly of an instruction.
    Display,
    /// Modify how an instruction is emitted into the code buffer.
    Encode,
    /// Modify the instruction mnemonic (see [`crate::dsl::Inst::mnemonic`]);
    /// this customization is irrelevant if [`CustomOperation::Display`] is also
    /// specified.
    Mnemonic,
    /// Modify how a register allocator visits the operands of an instruction.
    Visit,
}

impl fmt::Display for Customization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl BitOr for Customization {
    type Output = Custom;
    fn bitor(self, rhs: Self) -> Self::Output {
        assert_ne!(self, rhs, "duplicate custom operation: {self:?}");
        Custom(vec![self, rhs])
    }
}

impl BitOr<Customization> for Custom {
    type Output = Custom;
    fn bitor(mut self, rhs: Customization) -> Self::Output {
        assert!(
            !self.0.contains(&rhs),
            "duplicate custom operation: {rhs:?}"
        );
        self.0.push(rhs);
        self
    }
}

#[derive(PartialEq, Default)]
pub struct Custom(Vec<Customization>);

impl Custom {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Customization> {
        self.0.iter()
    }

    pub fn contains(&self, operation: Customization) -> bool {
        self.0.contains(&operation)
    }
}

impl fmt::Display for Custom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" | ")
        )
    }
}

impl From<Customization> for Custom {
    fn from(operation: Customization) -> Self {
        Custom(vec![operation])
    }
}
