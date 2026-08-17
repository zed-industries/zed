//! Implementation for WASIp2 and WASIp3.
use crate::Error;
use core::{mem::MaybeUninit, ptr::copy_nonoverlapping};

#[cfg(target_env = "p2")]
use wasip2 as wasi;

// Workaround to silence `unexpected_cfgs` warning
// on Rust version between 1.85 and 1.91
#[cfg(not(target_env = "p2"))]
#[cfg(target_env = "p3")]
use wasip3 as wasi;

#[cfg(not(target_env = "p2"))]
#[cfg(not(target_env = "p3"))]
compile_error!("Unknown version of WASI (only previews 1, 2 and 3 are supported)");

use wasi::random::random::get_random_u64;

#[inline]
pub fn inner_u32() -> Result<u32, Error> {
    let val = get_random_u64();
    Ok(crate::util::truncate(val))
}

#[inline]
pub fn inner_u64() -> Result<u64, Error> {
    Ok(get_random_u64())
}

#[inline]
pub fn fill_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    let (prefix, chunks, suffix) = unsafe { dest.align_to_mut::<MaybeUninit<u64>>() };

    // We use `get_random_u64` instead of `get_random_bytes` because the latter creates
    // an allocation due to the Wit IDL [restrictions][0]. This should be fine since
    // the main use case of `getrandom` is seed generation.
    //
    // [0]: https://github.com/WebAssembly/wasi-random/issues/27
    if !prefix.is_empty() {
        let val = get_random_u64();
        let src = (&val as *const u64).cast();
        unsafe {
            copy_nonoverlapping(src, prefix.as_mut_ptr(), prefix.len());
        }
    }

    for dst in chunks {
        dst.write(get_random_u64());
    }

    if !suffix.is_empty() {
        let val = get_random_u64();
        let src = (&val as *const u64).cast();
        unsafe {
            copy_nonoverlapping(src, suffix.as_mut_ptr(), suffix.len());
        }
    }

    Ok(())
}
