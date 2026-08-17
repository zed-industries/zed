//! Expose the `pkey_*` Linux system calls. See the kernel documentation for
//! more information:
//! - [`pkeys`] overview
//! - [`pkey_alloc`] (with `pkey_free`)
//! - [`pkey_mprotect`]
//! - `pkey_set` is implemented directly in assembly.
//!
//! [`pkey_alloc`]: https://man7.org/linux/man-pages/man2/pkey_alloc.2.html
//! [`pkey_mprotect`]: https://man7.org/linux/man-pages/man2/pkey_mprotect.2.html
//! [`pkeys`]: https://man7.org/linux/man-pages/man7/pkeys.7.html

use crate::prelude::*;
use crate::runtime::vm::host_page_size;
use std::io::Error;

/// Protection mask disallowing reads and writes of pkey-protected memory (see
/// `prot` in [`pkey_mprotect`]); in Wasmtime we expect all MPK-protected memory
/// to start as `PROT_NONE`.
pub const PROT_NONE: u32 = libc::PROT_NONE as u32; // == 0b0000;

/// Allocate a new protection key in the Linux kernel ([docs]); returns the
/// key ID.
///
/// [docs]: https://man7.org/linux/man-pages/man2/pkey_alloc.2.html
///
/// Each process has its own separate pkey index; e.g., if process `m`
/// allocates key 1, process `n` can as well.
pub fn pkey_alloc(flags: u32, access_rights: u32) -> Result<u32> {
    assert_eq!(flags, 0); // reserved for future use--must be 0.
    let result = unsafe { libc::syscall(libc::SYS_pkey_alloc, flags, access_rights) };
    if result >= 0 {
        Ok(result
            .try_into()
            .expect("only pkey IDs between 0 and 15 are expected"))
    } else {
        debug_assert_eq!(result, -1); // only this error result is expected.
        Err(Error::last_os_error().into())
    }
}

/// Free a kernel protection key ([docs]).
///
/// [docs]: https://man7.org/linux/man-pages/man2/pkey_alloc.2.html
#[cfg(test)]
pub fn pkey_free(key: u32) -> Result<()> {
    let result = unsafe { libc::syscall(libc::SYS_pkey_free, key) };
    if result == 0 {
        Ok(())
    } else {
        debug_assert_eq!(result, -1); // only this error result is expected.
        Err(Error::last_os_error().into())
    }
}

/// Change the access protections for a page-aligned memory region ([docs]).
///
/// [docs]: https://man7.org/linux/man-pages/man2/pkey_mprotect.2.html
pub fn pkey_mprotect(addr: usize, len: usize, prot: u32, key: u32) -> Result<()> {
    let page_size = host_page_size();
    if addr % page_size != 0 {
        log::warn!(
            "memory must be page-aligned for MPK (addr = {addr:#x}, page size = {page_size}"
        );
    }
    let result = unsafe { libc::syscall(libc::SYS_pkey_mprotect, addr, len, prot, key) };
    if result == 0 {
        Ok(())
    } else {
        debug_assert_eq!(result, -1); // only this error result is expected.
        Err(Error::last_os_error().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "cannot be run when keys() has already allocated all keys"]
    #[test]
    fn check_allocate_and_free() {
        let key = pkey_alloc(0, 0).unwrap();
        assert_eq!(key, 1);
        // It may seem strange to assert the key ID here, but we already
        // make some assumptions:
        //  1. we are running on Linux with `pku` enabled
        //  2. Linux will allocate key 0 for itself
        //  3. we are running this test in non-MPK mode and no one else is
        //     using pkeys
        // If these assumptions are incorrect, this test can be removed.
        pkey_free(key).unwrap()
    }

    #[test]
    fn check_invalid_free() {
        let result = pkey_free(42);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid argument (os error 22)"
        );
    }

    #[test]
    #[should_panic]
    fn check_invalid_alloc_flags() {
        let _ = pkey_alloc(42, 0);
    }

    #[test]
    fn check_invalid_alloc_rights() {
        assert!(pkey_alloc(0, 42).is_err());
    }
}
