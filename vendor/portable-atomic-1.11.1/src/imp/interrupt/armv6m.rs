// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Adapted from https://github.com/rust-embedded/cortex-m.

Refs: https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Level-Programmers--Model/Registers/The-special-purpose-mask-register--PRIMASK

Generated asm:
- armv6-m https://godbolt.org/z/1sqKnsY6n
*/

#[cfg(not(portable_atomic_no_asm))]
use core::arch::asm;

pub(super) use core::sync::atomic;

pub(super) type State = u32;

/// Disables interrupts and returns the previous interrupt state.
#[inline(always)]
pub(super) fn disable() -> State {
    let primask: State;
    // SAFETY: reading the priority mask register and disabling interrupts are safe.
    // (see module-level comments of interrupt/mod.rs on the safety of using privileged instructions)
    unsafe {
        // Do not use `nomem` and `readonly` because prevent subsequent memory accesses from being reordered before interrupts are disabled.
        asm!(
            "mrs {primask}, PRIMASK", // primask = PRIMASK
            "cpsid i",                // PRIMASK.PM = 1
            primask = out(reg) primask,
            options(nostack, preserves_flags),
        );
    }
    primask
}

/// Restores the previous interrupt state.
///
/// # Safety
///
/// The state must be the one retrieved by the previous `disable`.
#[inline(always)]
pub(super) unsafe fn restore(prev_primask: State) {
    // SAFETY: the caller must guarantee that the state was retrieved by the previous `disable`,
    // and we've checked that interrupts were enabled before disabling interrupts.
    unsafe {
        // This clobbers the entire PRIMASK register. See msp430.rs to safety on this.
        //
        // Do not use `nomem` and `readonly` because prevent preceding memory accesses from being reordered after interrupts are enabled.
        asm!(
            "msr PRIMASK, {prev_primask}", // PRIMASK = prev_primask
            prev_primask = in(reg) prev_primask,
            options(nostack, preserves_flags),
        );
    }
}
