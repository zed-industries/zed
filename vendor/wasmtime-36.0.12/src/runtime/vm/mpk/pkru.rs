//! Control access to the x86 `PKRU` register.
//!
//! As documented in the Intel Software Development Manual, vol 3a, section 2.7,
//! the 32 bits of the `PKRU` register laid out as follows (note the
//! little-endianness):
//!
//! ```text
//! ┌───┬───┬───┬───┬───┬───┐
//! │...│AD2│WD1│AD1│WD0│AD0│
//! └───┴───┴───┴───┴───┴───┘
//! ```
//!
//! - `ADn = 1` means "access disable key `n`"--no reads or writes allowed to
//!   pages marked with key `n`.
//! - `WDn = 1` means "write disable key `n`"--only reads are prevented to pages
//!   marked with key `n`
//! - it is unclear what it means to have both `ADn` and `WDn` set
//!
//! Note that this only handles the user-mode `PKRU` register; there is an
//! equivalent supervisor-mode MSR, `IA32_PKRS`.

use core::arch::asm;

/// This `PKRU` register mask allows access to any pages marked with any
/// key--in other words, reading and writing is permitted to all pages.
pub const ALLOW_ACCESS: u32 = 0;

/// This `PKRU` register mask disables access to any page marked with any
/// key--in other words, no reading or writing to all pages.
pub const DISABLE_ACCESS: u32 = 0b11111111_11111111_11111111_11111111;

/// Read the value of the `PKRU` register.
#[inline]
pub fn read() -> u32 {
    // ECX must be 0 to prevent a general protection exception (#GP).
    let ecx: u32 = 0;
    let pkru: u32;
    unsafe {
        asm!("rdpkru", in("ecx") ecx, out("eax") pkru, out("edx") _,
            options(nomem, nostack, preserves_flags));
    }
    return pkru;
}

/// Write a value to the `PKRU` register.
#[inline]
pub fn write(pkru: u32) {
    // Both ECX and EDX must be 0 to prevent a general protection exception
    // (#GP).
    let ecx: u32 = 0;
    let edx: u32 = 0;
    unsafe {
        asm!("wrpkru", in("eax") pkru, in("ecx") ecx, in("edx") edx,
            options(nomem, nostack, preserves_flags));
    }
}

/// Check the `ECX.PKU` flag (bit 3, zero-based) of the `07h` `CPUID` leaf; see
/// the Intel Software Development Manual, vol 3a, section 2.7. This flag is
/// only set on Intel CPUs, so this function also checks the `CPUID` vendor
/// string.
pub fn has_cpuid_bit_set() -> bool {
    let result = unsafe { core::arch::x86_64::__cpuid(0x07) };
    is_intel_cpu() && (result.ecx & 0b1000) != 0
}

/// Check the `CPUID` vendor string for `GenuineIntel`; see the Intel Software
/// Development Manual, vol 2a, `CPUID` description.
pub fn is_intel_cpu() -> bool {
    // To read the CPU vendor string, we pass 0 in EAX and read 12 ASCII bytes
    // from EBX, EDX, and ECX (in that order).
    let result = unsafe { core::arch::x86_64::__cpuid(0) };
    // Then we check if the vendor string matches "GenuineIntel".
    result.ebx == u32::from_le_bytes(*b"Genu")
        && result.edx == u32::from_le_bytes(*b"ineI")
        && result.ecx == u32::from_le_bytes(*b"ntel")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::vm::mpk::enabled::skip_if_mpk_unavailable;

    #[test]
    #[ignore = "cannot be run with other tests that munge the PKRU register"]
    fn check_read() {
        skip_if_mpk_unavailable!();
        assert_eq!(read(), DISABLE_ACCESS ^ 1);
        // By default, the Linux kernel only allows a process to access key 0,
        // the default kernel key.
    }

    #[test]
    fn check_roundtrip() {
        skip_if_mpk_unavailable!();
        let pkru = read();
        // Allow access to pages marked with any key.
        write(ALLOW_ACCESS);
        assert_eq!(read(), ALLOW_ACCESS);
        // Restore the original value.
        write(pkru);
        assert_eq!(read(), pkru);
    }
}
