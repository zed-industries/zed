#![no_std]
//! This crate provides an easy, fast, cross-platform way to retrieve the
//! memory page size of the current system.
//!
//! Modern hardware and software tend to load data into RAM (and transfer data
//! from RAM to disk) in discrete chunk called pages. This crate provides a
//! helper method to retrieve the size in bytes of these pages. Since the page
//! size *should not* change during execution, this crate will cache the result
//! after it has been called once.
//!
//! To make this crate useful for writing memory allocators, it does not require
//! (but can use) the Rust standard library.
//!
//! Since Windows addresses sometimes have to correspond with an allocation
//! granularity that does not always match the size of the page, I have included
//! a method to retrieve that as well.
//!
//! # Example
//!
//! ```rust
//! extern crate page_size;
//! println!("{}", page_size::get());
//! ```

#[cfg(feature = "no_std")]
extern crate spin;
#[cfg(feature = "no_std")]
use spin::Once;

#[cfg(not(feature = "no_std"))]
extern crate std;
#[cfg(not(feature = "no_std"))]
use std::sync::Once;

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate winapi;

/// This function retrieves the system's memory page size.
///
/// # Example
///
/// ```rust
/// extern crate page_size;
/// println!("{}", page_size::get());
/// ```
pub fn get() -> usize {
    get_helper()
}

/// This function retrieves the system's memory allocation granularity.
///
/// # Example
///
/// ```rust
/// extern crate page_size;
/// println!("{}", page_size::get_granularity());
/// ```
pub fn get_granularity() -> usize {
    get_granularity_helper()
}

// Unix Section

#[cfg(all(unix, feature = "no_std"))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once<usize> = Once::new();

    *INIT.call_once(unix::get)
}

#[cfg(all(unix, not(feature = "no_std")))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once = Once::new();
    static mut PAGE_SIZE: usize = 0;

    unsafe {
        INIT.call_once(|| PAGE_SIZE = unix::get());
        PAGE_SIZE
    }
}

// Unix does not have a specific allocation granularity.
// The page size works well.
#[cfg(unix)]
#[inline]
fn get_granularity_helper() -> usize {
    get_helper()
}

#[cfg(unix)]
mod unix {
    use libc::{sysconf, _SC_PAGESIZE};

    #[inline]
    pub fn get() -> usize {
        unsafe { sysconf(_SC_PAGESIZE) as usize }
    }
}

// WebAssembly section

// WebAssembly does not have a specific allocation granularity.
// The page size works well.
#[cfg(all(not(target_os = "emscripten"), any(target_arch = "wasm32", target_arch = "wasm64")))]
#[inline]
fn get_granularity_helper() -> usize {
    // <https://webassembly.github.io/spec/core/exec/runtime.html#page-size>
    65536
}

// Windows Section

#[cfg(all(windows, feature = "no_std"))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once<usize> = Once::new();

    *INIT.call_once(windows::get)
}

#[cfg(all(windows, not(feature = "no_std")))]
#[inline]
fn get_helper() -> usize {
    static INIT: Once = Once::new();
    static mut PAGE_SIZE: usize = 0;

    unsafe {
        INIT.call_once(|| PAGE_SIZE = windows::get());
        PAGE_SIZE
    }
}

#[cfg(all(windows, feature = "no_std"))]
#[inline]
fn get_granularity_helper() -> usize {
    static GRINIT: Once<usize> = Once::new();

    *GRINIT.call_once(windows::get_granularity)
}

#[cfg(all(windows, not(feature = "no_std")))]
#[inline]
fn get_granularity_helper() -> usize {
    static GRINIT: Once = Once::new();
    static mut GRANULARITY: usize = 0;

    unsafe {
        GRINIT.call_once(|| GRANULARITY = windows::get_granularity());
        GRANULARITY
    }
}

#[cfg(windows)]
mod windows {
    #[cfg(feature = "no_std")]
    use core::mem;
    #[cfg(not(feature = "no_std"))]
    use std::mem;

    use winapi::um::sysinfoapi::GetSystemInfo;
    use winapi::um::sysinfoapi::{LPSYSTEM_INFO, SYSTEM_INFO};

    #[inline]
    pub fn get() -> usize {
        unsafe {
            let mut info: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut info as LPSYSTEM_INFO);

            info.dwPageSize as usize
        }
    }

    #[inline]
    pub fn get_granularity() -> usize {
        unsafe {
            let mut info: SYSTEM_INFO = mem::zeroed();
            GetSystemInfo(&mut info as LPSYSTEM_INFO);

            info.dwAllocationGranularity as usize
        }
    }
}

// Stub Section

#[cfg(not(any(unix, windows)))]
#[inline]
fn get_helper() -> usize {
    4096 // 4k is the default on many systems
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        #[allow(unused_variables)]
        let page_size = get();
    }

    #[test]
    fn test_get_granularity() {
        #[allow(unused_variables)]
        let granularity = get_granularity();
    }
}
