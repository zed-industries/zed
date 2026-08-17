// Overwrite links to crate items with intra-crate links
//! [`Error::UNEXPECTED`]: Error::UNEXPECTED
//! [`fill_uninit`]: fill_uninit

#![no_std]
#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico"
)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(getrandom_backend = "efi_rng", feature(uefi_std))]
#![cfg_attr(getrandom_backend = "extern_impl", feature(extern_item_impls))]

#[macro_use]
extern crate cfg_if;

use core::mem::MaybeUninit;

mod backends;
mod error;
mod util;

#[cfg(feature = "std")]
mod error_std_impls;

/// `rand_core` adapter
#[cfg(feature = "sys_rng")]
mod sys_rng;

#[cfg(feature = "sys_rng")]
pub use rand_core;
#[cfg(feature = "sys_rng")]
pub use sys_rng::SysRng;

pub use crate::error::{Error, RawOsError};

/// Attribute macros for overwriting the core functionality of this crate.
///
/// This allows `getrandom` to provide a default implementation and a common interface
/// for all crates to use, while giving users a safe way to override that default where required.
///
/// Must be enabled via the `extern_impl` opt-in backend, as this functionality
/// is currently limited to nightly.
///
/// # Examples
///
/// ```rust
/// # use core::mem::MaybeUninit;
/// # #[cfg(getrandom_backend = "extern_impl")]
/// #[getrandom::implementation::fill_uninit]
/// fn my_fill_uninit_implementation(
///     dest: &mut [MaybeUninit<u8>]
/// ) -> Result<(), getrandom::Error> {
///     // ...
/// #   let _ = dest;
/// #   Err(Error::UNSUPPORTED)
/// }
/// ```
#[cfg(getrandom_backend = "extern_impl")]
pub mod implementation {
    pub use crate::backends::extern_impl::{fill_uninit, u32, u64};
}

/// Fill `dest` with random bytes from the system's preferred random number source.
///
/// This function returns an error on any failure, including partial reads. We
/// make no guarantees regarding the contents of `dest` on error. If `dest` is
/// empty, `getrandom` immediately returns success, making no calls to the
/// underlying operating system.
///
/// Blocking is possible, at least during early boot; see module documentation.
///
/// In general, `getrandom` will be fast enough for interactive usage, though
/// significantly slower than a user-space CSPRNG; for the latter consider
/// [`rand::thread_rng`](https://docs.rs/rand/*/rand/fn.thread_rng.html).
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), getrandom::Error> {
/// let mut buf = [0u8; 32];
/// getrandom::fill(&mut buf)?;
/// # Ok(()) }
/// ```
#[inline]
pub fn fill(dest: &mut [u8]) -> Result<(), Error> {
    // SAFETY: The `&mut MaybeUninit<_>` reference doesn't escape,
    // and `fill_uninit` guarantees it will never de-initialize
    // any part of `dest`.
    fill_uninit(unsafe { util::slice_as_uninit_mut(dest) })?;
    Ok(())
}

/// Fill potentially uninitialized buffer `dest` with random bytes from
/// the system's preferred random number source and return a mutable
/// reference to those bytes.
///
/// On successful completion this function is guaranteed to return a slice
/// which points to the same memory as `dest` and has the same length.
/// In other words, it's safe to assume that `dest` is initialized after
/// this function has returned `Ok`.
///
/// No part of `dest` will ever be de-initialized at any point, regardless
/// of what is returned.
///
/// # Examples
///
/// ```ignore
/// # // We ignore this test since `uninit_array` is unstable.
/// #![feature(maybe_uninit_uninit_array)]
/// # fn main() -> Result<(), getrandom::Error> {
/// let mut buf = core::mem::MaybeUninit::uninit_array::<1024>();
/// let buf: &mut [u8] = getrandom::fill_uninit(&mut buf)?;
/// # Ok(()) }
/// ```
#[inline]
pub fn fill_uninit(dest: &mut [MaybeUninit<u8>]) -> Result<&mut [u8], Error> {
    if !dest.is_empty() {
        backends::fill_inner(dest)?;
    }

    #[cfg(getrandom_msan)]
    unsafe extern "C" {
        fn __msan_unpoison(a: *mut core::ffi::c_void, size: usize);
    }

    // SAFETY: `dest` has been fully initialized by `imp::fill_inner`
    // since it returned `Ok`.
    Ok(unsafe { util::slice_assume_init_mut(dest) })
}

/// Get random `u32` from the system's preferred random number source.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), getrandom::Error> {
/// let rng_seed = getrandom::u32()?;
/// # Ok(()) }
/// ```
#[inline]
pub fn u32() -> Result<u32, Error> {
    backends::inner_u32()
}

/// Get random `u64` from the system's preferred random number source.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), getrandom::Error> {
/// let rng_seed = getrandom::u64()?;
/// # Ok(()) }
/// ```
#[inline]
pub fn u64() -> Result<u64, Error> {
    backends::inner_u64()
}
