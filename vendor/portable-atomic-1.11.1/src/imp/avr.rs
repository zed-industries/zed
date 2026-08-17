// SPDX-License-Identifier: Apache-2.0 OR MIT

/*
Atomic load/store implementation on AVR.

Operations not supported here are provided by disabling interrupts.
See also src/imp/interrupt/avr.rs.

See "Atomic operation overview by architecture" in atomic-maybe-uninit for a more comprehensive and
detailed description of the atomic and synchronize instructions in this architecture:
https://github.com/taiki-e/atomic-maybe-uninit/blob/HEAD/src/arch/README.md#avr

Note: Ordering is always SeqCst.

Refs:
- AVR® Instruction Set Manual, Rev. DS40002198B
  https://ww1.microchip.com/downloads/en/DeviceDoc/AVR-InstructionSet-Manual-DS40002198.pdf
- atomic-maybe-uninit
  https://github.com/taiki-e/atomic-maybe-uninit

Generated asm:
- avr https://godbolt.org/z/j49rYbj4d
*/

use core::{arch::asm, cell::UnsafeCell, sync::atomic::Ordering};

macro_rules! atomic8 {
    ($atomic_type:ident, $value_type:ty) => {
        #[repr(transparent)]
        pub(crate) struct $atomic_type {
            v: UnsafeCell<$value_type>,
        }

        // Send is implicitly implemented for atomic integers, but not for atomic pointers.
        // SAFETY: any data races are prevented by atomic operations.
        unsafe impl Send for $atomic_type {}
        // SAFETY: any data races are prevented by atomic operations.
        unsafe impl Sync for $atomic_type {}

        impl $atomic_type {
            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn load(&self, order: Ordering) -> $value_type {
                crate::utils::assert_load_ordering(order);
                let src = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    let out;
                    asm!(
                        "ld {out}, Z", // atomic { out = *Z }
                        in("Z") src,
                        out = out(reg) out,
                        options(nostack, preserves_flags),
                    );
                    out
                }
            }

            #[inline]
            #[cfg_attr(all(debug_assertions, not(portable_atomic_no_track_caller)), track_caller)]
            pub(crate) fn store(&self, val: $value_type, order: Ordering) {
                crate::utils::assert_store_ordering(order);
                let dst = self.v.get();
                // SAFETY: any data races are prevented by atomic intrinsics and the raw
                // pointer passed in is valid because we got it from a reference.
                unsafe {
                    asm!(
                        "st Z, {val}", // atomic { *Z = val }
                        in("Z") dst,
                        val = in(reg) val,
                        options(nostack, preserves_flags),
                    );
                }
            }
        }
    };
}

atomic8!(AtomicI8, i8);
atomic8!(AtomicU8, u8);
