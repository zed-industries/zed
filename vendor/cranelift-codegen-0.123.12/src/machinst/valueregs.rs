//! Data structure for tracking the (possibly multiple) registers that hold one
//! SSA `Value`.

use regalloc2::{PReg, VReg};

use super::{RealReg, Reg, VirtualReg, Writable};
use std::fmt::Debug;

const VALUE_REGS_PARTS: usize = 2;

/// Location at which a `Value` is stored in register(s): the value is located
/// in one or more registers, depending on its width. A value may be stored in
/// more than one register if the machine has no registers wide enough
/// otherwise: for example, on a 32-bit architecture, we may store `I64` values
/// in two registers, and `I128` values in four.
///
/// By convention, the register parts are kept in machine-endian order here.
///
/// N.B.: we cap the capacity of this at four (when any 32-bit target is
/// enabled) or two (otherwise), and we use special in-band sentinel `Reg`
/// values (`Reg::invalid()`) to avoid the need to carry a separate length. This
/// allows the struct to be `Copy` (no heap or drop overhead) and be only 16 or
/// 8 bytes, which is important for compiler performance.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ValueRegs<R: Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel> {
    parts: [R; VALUE_REGS_PARTS],
}

impl<R: Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel> Debug for ValueRegs<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut f = f.debug_tuple("ValueRegs");
        let mut last_valid = true;
        for part in self.parts {
            if part.is_invalid_sentinel() {
                last_valid = false;
            } else {
                debug_assert!(last_valid);
                f.field(&part);
            }
        }
        f.finish()
    }
}

/// A type with an "invalid" sentinel value.
pub trait InvalidSentinel: Copy + Eq {
    /// The invalid sentinel value.
    fn invalid_sentinel() -> Self;
    /// Is this the invalid sentinel?
    fn is_invalid_sentinel(self) -> bool {
        self == Self::invalid_sentinel()
    }
}
impl InvalidSentinel for Reg {
    fn invalid_sentinel() -> Self {
        Reg::from(VReg::invalid())
    }
}
impl InvalidSentinel for VirtualReg {
    fn invalid_sentinel() -> Self {
        VirtualReg::from(VReg::invalid())
    }
}
impl InvalidSentinel for RealReg {
    fn invalid_sentinel() -> Self {
        RealReg::from(PReg::invalid())
    }
}
impl InvalidSentinel for Writable<Reg> {
    fn invalid_sentinel() -> Self {
        Writable::from_reg(Reg::invalid_sentinel())
    }
}

impl<R: Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel> ValueRegs<R> {
    /// Create an invalid Value-in-Reg.
    pub fn invalid() -> Self {
        ValueRegs {
            parts: [R::invalid_sentinel(); VALUE_REGS_PARTS],
        }
    }

    /// Is this Value-to-Reg mapping valid?
    pub fn is_valid(self) -> bool {
        !self.parts[0].is_invalid_sentinel()
    }
    /// Is this Value-to-Reg mapping invalid?
    pub fn is_invalid(self) -> bool {
        self.parts[0].is_invalid_sentinel()
    }

    /// Return the single register used for this value, if any.
    pub fn only_reg(self) -> Option<R> {
        if self.len() == 1 {
            Some(self.parts[0])
        } else {
            None
        }
    }

    /// Return a slice of the registers storing this value.
    pub fn regs(&self) -> &[R] {
        &self.parts[0..self.len()]
    }

    /// Return a mutable slice of the registers storing this value.
    pub fn regs_mut(&mut self) -> &mut [R] {
        let len = self.len();
        &mut self.parts[0..len]
    }
}

impl<R: Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel> ValueRegs<R> {
    /// Create a Value-in-R location for a value stored in one register.
    pub fn one(reg: R) -> Self {
        ValueRegs {
            parts: [reg, R::invalid_sentinel()],
        }
    }
    /// Create a Value-in-R location for a value stored in two registers.
    pub fn two(r1: R, r2: R) -> Self {
        ValueRegs { parts: [r1, r2] }
    }

    /// Return the number of registers used.
    pub fn len(self) -> usize {
        // If rustc/LLVM is smart enough, this might even be vectorized...
        (self.parts[0] != R::invalid_sentinel()) as usize
            + (self.parts[1] != R::invalid_sentinel()) as usize
    }

    /// Map individual registers via a map function.
    pub fn map<NewR, F>(self, f: F) -> ValueRegs<NewR>
    where
        NewR: Clone + Copy + Debug + PartialEq + Eq + InvalidSentinel,
        F: Fn(R) -> NewR,
    {
        ValueRegs {
            parts: [f(self.parts[0]), f(self.parts[1])],
        }
    }
}

/// Create a writable ValueRegs.
pub(crate) fn writable_value_regs(regs: ValueRegs<Reg>) -> ValueRegs<Writable<Reg>> {
    regs.map(|r| Writable::from_reg(r))
}

/// Strip a writable ValueRegs down to a readonly ValueRegs.
pub(crate) fn non_writable_value_regs(regs: ValueRegs<Writable<Reg>>) -> ValueRegs<Reg> {
    regs.map(|r| r.to_reg())
}
