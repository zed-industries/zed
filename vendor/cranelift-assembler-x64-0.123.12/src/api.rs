//! Contains traits that a user of this assembler must implement.

use crate::gpr;
use crate::xmm;
use crate::{Amode, DeferredTarget, GprMem, XmmMem};
use std::fmt;
use std::{num::NonZeroU8, vec::Vec};

/// Describe how an instruction is emitted into a code buffer.
pub trait CodeSink {
    /// Add 1 byte to the code section.
    fn put1(&mut self, _: u8);

    /// Add 2 bytes to the code section.
    fn put2(&mut self, _: u16);

    /// Add 4 bytes to the code section.
    fn put4(&mut self, _: u32);

    /// Add 8 bytes to the code section.
    fn put8(&mut self, _: u64);

    /// Inform the code buffer of a possible trap at the current location;
    /// required for assembling memory accesses.
    fn add_trap(&mut self, code: TrapCode);

    /// Inform the code buffer that a use of `target` is about to happen at the
    /// current offset.
    ///
    /// After this method is called the bytes of the target are then expected to
    /// be placed using one of the above `put*` methods.
    fn use_target(&mut self, target: DeferredTarget);

    /// Resolves a `KnownOffset` value to the actual signed offset.
    fn known_offset(&self, offset: KnownOffset) -> i32;
}

/// Provide a convenient implementation for testing.
impl CodeSink for Vec<u8> {
    fn put1(&mut self, v: u8) {
        self.extend_from_slice(&[v]);
    }

    fn put2(&mut self, v: u16) {
        self.extend_from_slice(&v.to_le_bytes());
    }

    fn put4(&mut self, v: u32) {
        self.extend_from_slice(&v.to_le_bytes());
    }

    fn put8(&mut self, v: u64) {
        self.extend_from_slice(&v.to_le_bytes());
    }

    fn add_trap(&mut self, _: TrapCode) {}

    fn use_target(&mut self, _: DeferredTarget) {}

    fn known_offset(&self, offset: KnownOffset) -> i32 {
        panic!("unknown offset {offset:?}")
    }
}

/// Wrap [`CodeSink`]-specific labels.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct Label(pub u32);

/// Wrap [`CodeSink`]-specific constant keys.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct Constant(pub u32);

/// Wrap [`CodeSink`]-specific trap codes.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(any(test, feature = "fuzz"), derive(arbitrary::Arbitrary))]
pub struct TrapCode(pub NonZeroU8);

impl fmt::Display for TrapCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "trap={}", self.0)
    }
}

/// A `KnownOffset` is a unique identifier for a specific offset known only at
/// emission time.
pub type KnownOffset = u8;

/// A type set fixing the register types used in the assembler.
///
/// This assembler is parameterizable over register types; this allows the
/// assembler users (e.g., Cranelift) to define their own register types
/// independent of this crate.
pub trait Registers {
    /// An x64 general purpose register that may be read.
    type ReadGpr: AsReg;

    /// An x64 general purpose register that may be read and written.
    type ReadWriteGpr: AsReg;

    /// An x64 general purpose register that may be written.
    type WriteGpr: AsReg;

    /// An x64 SSE register that may be read.
    type ReadXmm: AsReg;

    /// An x64 SSE register that may be read and written.
    type ReadWriteXmm: AsReg;

    /// An x64 SSE register that may be written.
    type WriteXmm: AsReg;
}

/// Describe how to interact with an external register type.
pub trait AsReg: Copy + Clone + std::fmt::Debug + PartialEq {
    /// Create a register from its hardware encoding.
    ///
    /// This is primarily useful for fuzzing, though it is also useful for
    /// generating fixed registers.
    fn new(enc: u8) -> Self;

    /// Return the register's hardware encoding; e.g., `0` for `%rax`.
    fn enc(&self) -> u8;

    /// Return the register name.
    fn to_string(&self, size: Option<gpr::Size>) -> String {
        match size {
            Some(size) => gpr::enc::to_string(self.enc(), size).into(),
            None => xmm::enc::to_string(self.enc()).into(),
        }
    }
}

/// Provide a convenient implementation for testing.
impl AsReg for u8 {
    fn new(enc: u8) -> Self {
        enc
    }
    fn enc(&self) -> u8 {
        *self
    }
}

/// Describe a visitor for the register operands of an instruction.
///
/// Due to how Cranelift's register allocation works, we allow the visitor to
/// modify the register operands in place. This allows Cranelift to convert
/// virtual registers (`[128..N)`) to physical registers (`[0..16)`) without
/// re-allocating the entire instruction object.
pub trait RegisterVisitor<R: Registers> {
    /// Visit a read-only register.
    fn read_gpr(&mut self, reg: &mut R::ReadGpr);
    /// Visit a read-write register.
    fn read_write_gpr(&mut self, reg: &mut R::ReadWriteGpr);
    /// Visit a write-only register.
    fn write_gpr(&mut self, reg: &mut R::WriteGpr);

    /// Visit a read-only fixed register; this register can be modified in-place
    /// but must emit as the hardware encoding `enc`.
    fn fixed_read_gpr(&mut self, reg: &mut R::ReadGpr, enc: u8);
    /// Visit a read-write fixed register; this register can be modified
    /// in-place but must emit as the hardware encoding `enc`.
    fn fixed_read_write_gpr(&mut self, reg: &mut R::ReadWriteGpr, enc: u8);
    /// Visit a write-only fixed register; this register can be modified
    /// in-place but must emit as the hardware encoding `enc`.
    fn fixed_write_gpr(&mut self, reg: &mut R::WriteGpr, enc: u8);

    /// Visit a read-only SSE register.
    fn read_xmm(&mut self, reg: &mut R::ReadXmm);
    /// Visit a read-write SSE register.
    fn read_write_xmm(&mut self, reg: &mut R::ReadWriteXmm);
    /// Visit a write-only SSE register.
    fn write_xmm(&mut self, reg: &mut R::WriteXmm);

    /// Visit a read-only fixed SSE register; this register can be modified
    /// in-place but must emit as the hardware encoding `enc`.
    fn fixed_read_xmm(&mut self, reg: &mut R::ReadXmm, enc: u8);
    /// Visit a read-write fixed SSE register; this register can be modified
    /// in-place but must emit as the hardware encoding `enc`.
    fn fixed_read_write_xmm(&mut self, reg: &mut R::ReadWriteXmm, enc: u8);
    /// Visit a read-only fixed SSE register; this register can be modified
    /// in-place but must emit as the hardware encoding `enc`.
    fn fixed_write_xmm(&mut self, reg: &mut R::WriteXmm, enc: u8);

    /// Visit the registers in an [`Amode`].
    ///
    /// This is helpful for generated code: it allows capturing the `R::ReadGpr`
    /// type (which an `Amode` method cannot) and simplifies the code to be
    /// generated.
    fn read_amode(&mut self, amode: &mut Amode<R::ReadGpr>) {
        match amode {
            Amode::ImmReg { base, .. } => {
                self.read_gpr(base);
            }
            Amode::ImmRegRegShift { base, index, .. } => {
                self.read_gpr(base);
                self.read_gpr(index.as_mut());
            }
            Amode::RipRelative { .. } => {}
        }
    }

    /// Helper method to handle a read/write [`GprMem`] operand.
    fn read_write_gpr_mem(&mut self, op: &mut GprMem<R::ReadWriteGpr, R::ReadGpr>) {
        match op {
            GprMem::Gpr(r) => self.read_write_gpr(r),
            GprMem::Mem(m) => self.read_amode(m),
        }
    }

    /// Helper method to handle a write [`GprMem`] operand.
    fn write_gpr_mem(&mut self, op: &mut GprMem<R::WriteGpr, R::ReadGpr>) {
        match op {
            GprMem::Gpr(r) => self.write_gpr(r),
            GprMem::Mem(m) => self.read_amode(m),
        }
    }

    /// Helper method to handle a read-only [`GprMem`] operand.
    fn read_gpr_mem(&mut self, op: &mut GprMem<R::ReadGpr, R::ReadGpr>) {
        match op {
            GprMem::Gpr(r) => self.read_gpr(r),
            GprMem::Mem(m) => self.read_amode(m),
        }
    }

    /// Helper method to handle a read-only [`XmmMem`] operand.
    fn read_xmm_mem(&mut self, op: &mut XmmMem<R::ReadXmm, R::ReadGpr>) {
        match op {
            XmmMem::Xmm(r) => self.read_xmm(r),
            XmmMem::Mem(m) => self.read_amode(m),
        }
    }

    /// Helper method to handle a write [`XmmMem`] operand.
    fn write_xmm_mem(&mut self, op: &mut XmmMem<R::WriteXmm, R::ReadGpr>) {
        match op {
            XmmMem::Xmm(r) => self.write_xmm(r),
            XmmMem::Mem(m) => self.read_amode(m),
        }
    }
}
