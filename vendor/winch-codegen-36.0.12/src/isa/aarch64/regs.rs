//! AArch64 register definition.

use crate::isa::reg::Reg;
use crate::regset::RegBitSet;
use regalloc2::{PReg, RegClass};

/// FPR index bound.
const MAX_FPR: u32 = 32;
/// FPR index bound.
const MAX_GPR: u32 = 32;

/// Construct a X-register from an index.
pub(crate) const fn xreg(num: u8) -> Reg {
    assert!((num as u32) < MAX_GPR);
    Reg::new(PReg::new(num as usize, RegClass::Int))
}

/// Construct a V-register from an index.
pub(crate) const fn vreg(num: u8) -> Reg {
    assert!((num as u32) < MAX_FPR);
    Reg::new(PReg::new(num as usize, RegClass::Float))
}

/// Scratch register.
/// Intra-procedure-call corruptible register.
pub(crate) const fn ip0() -> Reg {
    xreg(16)
}

// Alias to register v31.
const fn float_scratch() -> Reg {
    vreg(31)
}

/// Scratch register.
/// Intra-procedure-call corruptible register.
pub(crate) const fn ip1() -> Reg {
    xreg(17)
}

/// Register used to carry platform state.
const fn platform() -> Reg {
    xreg(18)
}

/// Frame pointer register.
pub(crate) const fn fp() -> Reg {
    xreg(29)
}

/// Link register for function calls.
pub(crate) const fn lr() -> Reg {
    xreg(30)
}

/// Zero register.
pub(crate) const fn zero() -> Reg {
    xreg(31)
}

/// The VM context register.
pub(crate) const fn vmctx() -> Reg {
    xreg(9)
}

/// Stack pointer register.
///
/// In aarch64 the zero and stack pointer registers are contextually
/// different but have the same hardware encoding; to differentiate
/// them, we are following Cranelift's encoding and representing it as
/// 31 + 32.  Ref:
/// https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/codegen/src/isa/aarch64/inst/regs.rs#L70
pub(crate) const fn sp() -> Reg {
    Reg::new(PReg::new(31 + 32, RegClass::Int))
}

/// Shadow stack pointer register.
///
/// The shadow stack pointer (SSP) is used as the base for memory addressing
/// to workaround Aarch64's constraint on the stack pointer 16-byte
/// alignment for memory addressing. This allows word-size loads and
/// stores.  It's always assumed that the real stack pointer (SP) is
/// 16-byte unaligned; the only exceptions to this assumption are:
///
/// * The function prologue and epilogue in which we use SP
///   for addressing, assuming that the 16-byte alignment is respected.
/// * Call sites, in which the code generation process explicitly ensures that
///   the stack pointer is 16-byte aligned.
/// * Code that could result in signal handling, like loads/stores.
///
/// SSP is utilized for space allocation. After each allocation, its value is
/// copied to SP, ensuring that SP accurately reflects the allocated space.
/// Accessing memory below SP may lead to undefined behavior, as this memory can
/// be overwritten by interrupts and signal handlers.
///
/// This approach requires copying the value of SSP into SP every time SSP
/// changes, more explicitly, this happens at three main locations:
///
/// 1. After space is allocated, to respect the requirement of avoiding
///    addressing space below SP.
/// 2. At function epilogue.
/// 3. After explicit SP is emitted (code that could result in signal handling).
///
/// +-----------+      Prologue:
/// |           |      * Save SSP (callee-saved)
/// +-----------+----- * SP at function entry (after prologue, slots for FP and LR)
/// |           |      * Copy the value of SP to SSP
/// |           |
/// +-----------+----- SSP after reserving stack space for locals and arguments
/// |           |      Copy the value of SSP to SP
/// |           |
/// +-----------+----- SSP after a push
/// |           |      Copy the value of SSP to SP
/// |           |      
/// |           |       
/// |           |
/// |           |      Epilogue:
/// |           |      * Copy SSP to SP
/// +-----------+----- * Restore SSP (callee-saved)
/// +-----------+      
///
/// In summary, the following invariants must be respected:
///
/// * SSP is considered primary, and must be used to allocate and deallocate
///   stack space(e.g. push, pop). This operation must always be followed by
///   a copy of SSP to SP.
/// * SP must never be used to address memory except when we are certain that
///   the required alignment is respected (e.g.  during the prologue and epilogue)
/// * SP must be explicitly aligned when code could result in signal handling.
/// * The value of SP is copied to SSP when entering a function.
/// * The value of SSP doesn't change between
///   function calls (as it's callee saved), compliant with
///   Aarch64's ABI.
/// * SSP is not available during register allocation.
pub(crate) const fn shadow_sp() -> Reg {
    xreg(28)
}

/// Bitmask for non-allocatable GPR.
const NON_ALLOCATABLE_GPR: u32 = (1 << ip0().hw_enc())
    | (1 << ip1().hw_enc())
    | (1 << platform().hw_enc())
    | (1 << fp().hw_enc())
    | (1 << lr().hw_enc())
    | (1 << zero().hw_enc())
    | (1 << shadow_sp().hw_enc())
    | (1 << vmctx().hw_enc());
/// Bitmask to represent the available general purpose registers.
const ALLOCATABLE_GPR: u32 = u32::MAX & !NON_ALLOCATABLE_GPR;

/// Bitmask for non-allocatable FPR.
/// All FPRs are allocatable, v0..=v7 are generally used for params and results.

const NON_ALLOCATABLE_FPR: u32 = 1 << float_scratch().hw_enc();
/// Bitmask to represent the available floating point registers.
const ALLOCATABLE_FPR: u32 = u32::MAX & !NON_ALLOCATABLE_FPR;

/// Allocatable scratch general purpose registers.
const ALLOCATABLE_SCRATCH_GPR: u32 = (1 << ip0().hw_enc()) | (1 << ip1().hw_enc());
/// Non-allocatable scratch general purpose registers.
const NON_ALLOCATABLE_SCRATCH_GPR: u32 = u32::MAX & !ALLOCATABLE_SCRATCH_GPR;

const ALLOCATABLE_SCRATCH_FPR: u32 = 1 << float_scratch().hw_enc();
/// Non-allocatable scratch general purpose registers.
const NON_ALLOCATABLE_SCRATCH_FPR: u32 = u32::MAX & !ALLOCATABLE_SCRATCH_FPR;

/// Bitset for allocatable general purpose registers.
pub fn gpr_bit_set() -> RegBitSet {
    RegBitSet::int(
        ALLOCATABLE_GPR.into(),
        NON_ALLOCATABLE_GPR.into(),
        usize::try_from(MAX_GPR).unwrap(),
    )
}

/// Bitset for allocatable floating point registers.
pub fn fpr_bit_set() -> RegBitSet {
    RegBitSet::float(
        ALLOCATABLE_FPR.into(),
        NON_ALLOCATABLE_FPR.into(),
        usize::try_from(MAX_FPR).unwrap(),
    )
}

/// Bitset for allocatable scratch general purpose registers.
pub fn scratch_gpr_bitset() -> RegBitSet {
    RegBitSet::int(
        ALLOCATABLE_SCRATCH_GPR.into(),
        NON_ALLOCATABLE_SCRATCH_GPR.into(),
        usize::try_from(MAX_GPR).unwrap(),
    )
}

/// Bitset for allocatable scratch floating point registers.
pub fn scratch_fpr_bitset() -> RegBitSet {
    RegBitSet::float(
        ALLOCATABLE_SCRATCH_FPR.into(),
        NON_ALLOCATABLE_SCRATCH_FPR.into(),
        usize::try_from(MAX_FPR).unwrap(),
    )
}
