// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Refs: https://developer.arm.com/documentation/ddi0406/cb/System-Level-Architecture/The-System-Level-Programmers--Model/ARM-processor-modes-and-ARM-core-registers/Program-Status-Registers--PSRs-

Generated asm:
- armv5te https://godbolt.org/z/fhaW3d9Kv
*/

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;

// - 0x80 - I (IRQ mask) bit (1 << 7)
// - 0x40 - F (FIQ mask) bit (1 << 6)
// We disable only IRQs by default. See also https://github.com/taiki-e/portable-atomic/pull/28#issuecomment-1214146912.
#[cfg(not(portable_atomic_disable_fiq))]
macro_rules! mask {
    () => {
        "0x80"
    };
}
#[cfg(portable_atomic_disable_fiq)]
macro_rules! mask {
    () => {
        "0xC0" // 0x80 | 0x40
    };
}

pub(super) type State = u32;

/// Disables interrupts and returns the previous interrupt state.
#[inline]
#[instruction_set(arm::a32)]
pub(super) fn disable() -> State {
    let cpsr: State;
    // SAFETY: reading CPSR and disabling interrupts are safe.
    // (see module-level comments of interrupt/mod.rs on the safety of using privileged instructions)
    unsafe {
        asm!(
            "mrs {prev}, cpsr",                      // prev = CPSR
            concat!("orr {new}, {prev}, ", mask!()), // new = prev | mask
            "msr cpsr_c, {new}",                     // CPSR.{I,F,T,M} = new.{I,F,T,M}
            prev = out(reg) cpsr,
            new = out(reg) _,
            // Do not use `nomem` and `readonly` because prevent subsequent memory accesses from being reordered before interrupts are disabled.
            options(nostack, preserves_flags),
        );
    }
    cpsr
}

/// Restores the previous interrupt state.
///
/// # Safety
///
/// The state must be the one retrieved by the previous `disable`.
#[inline]
#[instruction_set(arm::a32)]
pub(super) unsafe fn restore(prev_cpsr: State) {
    // SAFETY: the caller must guarantee that the state was retrieved by the previous `disable`,
    //
    // This clobbers the control field mask byte of CPSR. See msp430.rs to safety on this.
    // (preserves_flags is fine because we can clobber only the I, F, T, and M bits of CPSR.)
    //
    // Refs: https://developer.arm.com/documentation/dui0473/m/arm-and-thumb-instructions/msr--general-purpose-register-to-psr-
    unsafe {
        // Do not use `nomem` and `readonly` because prevent preceding memory accesses from being reordered after interrupts are enabled.
        asm!(
            "msr cpsr_c, {prev_cpsr}", // CPSR.{I,F,T,M} = prev_cpsr.{I,F,T,M}
            prev_cpsr = in(reg) prev_cpsr,
            options(nostack, preserves_flags),
        );
    }
}

// On pre-v6 Arm, we cannot use core::sync::atomic here because they call the
// `__sync_*` builtins for non-relaxed load/store (because pre-v6 Arm doesn't
// have Data Memory Barrier).
//
// Generated asm:
// - armv5te https://godbolt.org/z/deqTqPzqz
pub(crate) mod atomic {
    #[cfg(not(portable_atomic_no_asm))]
    use core::arch::asm;
    use core::{cell::UnsafeCell, sync::atomic::Ordering};

    macro_rules! atomic {
        ($([$($generics:tt)*])? $atomic_type:ident, $value_type:ty $(as $cast:ty)?, $suffix:tt) => {
            #[repr(transparent)]
            pub(crate) struct $atomic_type $(<$($generics)*>)? {
                v: UnsafeCell<$value_type>,
            }

            // Send is implicitly implemented for atomic integers, but not for atomic pointers.
            // SAFETY: any data races are prevented by atomic operations.
            unsafe impl $(<$($generics)*>)? Send for $atomic_type $(<$($generics)*>)? {}
            // SAFETY: any data races are prevented by atomic operations.
            unsafe impl $(<$($generics)*>)? Sync for $atomic_type $(<$($generics)*>)? {}

            impl $(<$($generics)*>)? $atomic_type $(<$($generics)*>)? {
                #[inline]
                pub(crate) fn load(&self, _order: Ordering) -> $value_type {
                    let src = self.v.get();
                    // SAFETY: any data races are prevented by atomic intrinsics and the raw
                    // pointer passed in is valid because we got it from a reference.
                    unsafe {
                        let out $(: $cast)?;
                        // inline asm without nomem/readonly implies compiler fence.
                        // And compiler fence is fine because the user explicitly declares that
                        // the system is single-core by using an unsafe cfg.
                        asm!(
                            concat!("ldr", $suffix, " {out}, [{src}]"), // atomic { out = *src }
                            src = in(reg) src,
                            out = lateout(reg) out,
                            options(nostack, preserves_flags),
                        );
                        out $(as $cast as $value_type)?
                    }
                }

                #[inline]
                pub(crate) fn store(&self, val: $value_type, _order: Ordering) {
                    let dst = self.v.get();
                    // SAFETY: any data races are prevented by atomic intrinsics and the raw
                    // pointer passed in is valid because we got it from a reference.
                    unsafe {
                        // inline asm without nomem/readonly implies compiler fence.
                        // And compiler fence is fine because the user explicitly declares that
                        // the system is single-core by using an unsafe cfg.
                        asm!(
                            concat!("str", $suffix, " {val}, [{dst}]"), // atomic { *dst = val }
                            dst = in(reg) dst,
                            val = in(reg) val $(as $cast)?,
                            options(nostack, preserves_flags),
                        );
                    }
                }
            }
        };
    }

    atomic!(AtomicI8, i8, "b");
    atomic!(AtomicU8, u8, "b");
    atomic!(AtomicI16, i16, "h");
    atomic!(AtomicU16, u16, "h");
    atomic!(AtomicI32, i32, "");
    atomic!(AtomicU32, u32, "");
    atomic!(AtomicIsize, isize, "");
    atomic!(AtomicUsize, usize, "");
    atomic!([T] AtomicPtr, *mut T as *mut u8, "");
}
