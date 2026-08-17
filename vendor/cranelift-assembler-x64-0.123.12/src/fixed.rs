//! Operands with fixed register encodings.

use crate::{AsReg, Size};

/// A _fixed_ register.
///
/// Some operands are implicit to the instruction and thus use a fixed register
/// for execution. Because this assembler is generic over any register type
/// (`R`), this wrapper provides a way to record the fixed register encoding we
/// expect to use (`E`).
///
/// ```
/// # use cranelift_assembler_x64::{AsReg, Fixed, gpr};
/// # let valid_reg = 0;
/// let fixed = Fixed::<u8, { gpr::enc::RAX }>(valid_reg);
/// assert_eq!(fixed.enc(), gpr::enc::RAX);
/// ```
///
/// ```should_panic
/// # use cranelift_assembler_x64::{AsReg, Fixed, gpr};
/// # let invalid_reg = 42;
/// let fixed = Fixed::<u8, { gpr::enc::RAX }>(invalid_reg);
/// fixed.enc(); // Will panic because `invalid_reg` does not match `RAX`.
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Fixed<R, const E: u8>(pub R);

impl<R, const E: u8> Fixed<R, E> {
    /// Return the fixed register encoding.
    ///
    /// Regardless of what `R` is (e.g., pre-register allocation), we want to be
    /// able to know what this register should encode as.
    pub fn expected_enc(&self) -> u8 {
        E
    }

    /// Return the register name at the given `size`.
    pub fn to_string(&self, size: Option<Size>) -> String
    where
        R: AsReg,
    {
        self.0.to_string(size)
    }
}

impl<R: AsReg, const E: u8> AsReg for Fixed<R, E> {
    fn new(reg: u8) -> Self {
        assert!(reg == E);
        Self(R::new(reg))
    }

    fn enc(&self) -> u8 {
        assert!(self.0.enc() == E);
        self.0.enc()
    }
}

impl<R, const E: u8> AsRef<R> for Fixed<R, E> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

impl<R, const E: u8> From<R> for Fixed<R, E> {
    fn from(reg: R) -> Fixed<R, E> {
        Fixed(reg)
    }
}
