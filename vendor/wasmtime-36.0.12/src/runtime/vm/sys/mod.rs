//! OS-related abstractions required by Wasmtime.
//!
//! This module is intended to house all logic that's specific to either Unix
//! or Windows, for example. The goal of this module is to be the "single
//! module" to edit if Wasmtime is ported to a new platform. Ideally all that's
//! needed is an extra block below and a new platform should be good to go after
//! filling out the implementation.

#![allow(
    clippy::cast_sign_loss,
    reason = "platforms too fiddly to worry about this"
)]

use crate::runtime::vm::SendSyncPtr;
use core::ptr::{self, NonNull};

/// What happens to a mapping after it is decommitted?
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DecommitBehavior {
    /// The mapping is zeroed.
    Zero,
    /// The original mapping is restored. If it was zero, then it is zero again;
    /// if it was a CoW mapping, then the original CoW mapping is restored;
    /// etc...
    RestoreOriginalMapping,
}

fn empty_mmap() -> SendSyncPtr<[u8]> {
    // Callers of this API assume that `.as_ptr()` below returns something
    // page-aligned and non-null. This is because the pointer returned from
    // that location is casted to other types which reside at a higher
    // alignment than a byte for example. Despite the length being zero we
    // still need to ensure that the pointer is suitably aligned.
    //
    // To handle that do a bit of trickery here to get the compiler to
    // generate an empty array to a high-alignment type (here 4k which is
    // the min page size we work with today). Then use this empty array as
    // the source pointer for an empty byte slice. It's a bit wonky but this
    // makes it such that the returned length is always zero (so this is
    // safe) but the pointer is always 4096 or suitably aligned.
    #[repr(C, align(4096))]
    struct PageAligned;
    let empty_page_alloc: &mut [PageAligned] = &mut [];
    let empty = NonNull::new(ptr::slice_from_raw_parts_mut(
        empty_page_alloc.as_mut_ptr().cast(),
        0,
    ))
    .unwrap();
    SendSyncPtr::from(empty)
}

cfg_if::cfg_if! {
    if #[cfg(miri)] {
        mod miri;
        pub use miri::*;
    } else if #[cfg(not(feature = "std"))] {
        mod custom;
        pub use custom::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::*;
    } else if #[cfg(unix)] {
        mod unix;
        pub use unix::*;
    } else {
        mod custom;
        pub use custom::*;
    }
}
