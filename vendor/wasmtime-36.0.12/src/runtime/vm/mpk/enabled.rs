//!

use super::{pkru, sys};
use crate::prelude::*;
use std::sync::OnceLock;

/// Check if the MPK feature is supported.
pub fn is_supported() -> bool {
    cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") && pkru::has_cpuid_bit_set()
}

/// Allocate up to `max` protection keys.
///
/// This asks the kernel for all available keys up to `max` in a thread-safe way
/// (we can expect 1-15; 0 is kernel-reserved). This avoids interference when
/// multiple threads try to allocate keys at the same time (e.g., during
/// testing). It also ensures that a single copy of the keys is reserved for the
/// lifetime of the process. Because of this, `max` is only a hint to
/// allocation: it only is effective on the first invocation of this function.
///
/// TODO: this is not the best-possible design. This creates global state that
/// would prevent any other code in the process from using protection keys; the
/// `KEYS` are never deallocated from the system with `pkey_dealloc`.
pub fn keys(max: usize) -> &'static [ProtectionKey] {
    let keys = KEYS.get_or_init(|| {
        let mut allocated = vec![];
        if is_supported() {
            while allocated.len() < max {
                if let Ok(key_id) = sys::pkey_alloc(0, 0) {
                    debug_assert!(key_id < 16);
                    // UNSAFETY: here we unsafely assume that the
                    // system-allocated pkey will exist forever.
                    allocated.push(ProtectionKey {
                        id: key_id,
                        stripe: allocated.len().try_into().unwrap(),
                    });
                } else {
                    break;
                }
            }
        }
        allocated
    });
    &keys[..keys.len().min(max)]
}
static KEYS: OnceLock<Vec<ProtectionKey>> = OnceLock::new();

/// Only allow access to pages marked by the keys set in `mask`.
///
/// Any accesses to pages marked by another key will result in a `SIGSEGV`
/// fault.
pub fn allow(mask: ProtectionMask) {
    let previous = if log::log_enabled!(log::Level::Trace) {
        pkru::read()
    } else {
        0
    };
    pkru::write(mask.0);
    log::trace!("PKRU change: {:#034b} => {:#034b}", previous, pkru::read());
}

/// Retrieve the current protection mask.
#[cfg(feature = "async")]
pub fn current_mask() -> ProtectionMask {
    ProtectionMask(pkru::read())
}

/// An MPK protection key.
///
/// The expected usage is:
/// - receive system-allocated keys from [`keys`]
/// - mark some regions of memory as accessible with [`ProtectionKey::protect`]
/// - [`allow`] or disallow access to the memory regions using a
///   [`ProtectionMask`]; any accesses to unmarked pages result in a fault
/// - drop the key
#[derive(Clone, Copy, Debug)]
pub struct ProtectionKey {
    id: u32,
    stripe: u32,
}

impl ProtectionKey {
    /// Mark a page as protected by this [`ProtectionKey`].
    ///
    /// This "colors" the pages of `region` via a kernel `pkey_mprotect` call to
    /// only allow reads and writes when this [`ProtectionKey`] is activated
    /// (see [`allow`]).
    ///
    /// # Errors
    ///
    /// This will fail if the region is not page aligned or for some unknown
    /// kernel reason.
    pub fn protect(&self, region: &mut [u8]) -> Result<()> {
        let addr = region.as_mut_ptr() as usize;
        let len = region.len();
        let prot = sys::PROT_NONE;
        sys::pkey_mprotect(addr, len, prot, self.id).with_context(|| {
            format!(
                "failed to mark region with pkey (addr = {addr:#x}, len = {len}, prot = {prot:#b})"
            )
        })
    }

    /// Convert the [`ProtectionKey`] to its 0-based index; this is useful for
    /// determining which allocation "stripe" a key belongs to.
    ///
    /// This function assumes that the kernel has allocated key 0 for itself.
    pub fn as_stripe(&self) -> usize {
        self.stripe as usize
    }
}

/// A bit field indicating which protection keys should be allowed and disabled.
///
/// The internal representation makes it easy to use [`ProtectionMask`] directly
/// with the PKRU register. When bits `n` and `n+1` are set, it means the
/// protection key is *not* allowed (see the PKRU write and access disabled
/// bits).
pub struct ProtectionMask(u32);
impl ProtectionMask {
    /// Allow access from all protection keys.
    #[inline]
    pub fn all() -> Self {
        Self(pkru::ALLOW_ACCESS)
    }

    /// Only allow access to memory protected with protection key 0; note that
    /// this does not mean "none" but rather allows access from the default
    /// kernel protection key.
    #[inline]
    pub fn zero() -> Self {
        Self(pkru::DISABLE_ACCESS ^ 0b11)
    }

    /// Include `pkey` as another allowed protection key in the mask.
    #[inline]
    pub fn or(self, pkey: ProtectionKey) -> Self {
        let mask = pkru::DISABLE_ACCESS ^ 0b11 << (pkey.id * 2);
        Self(self.0 & mask)
    }
}

/// Helper macro for skipping tests on systems that do not have MPK enabled
/// (e.g., older architecture, disabled by kernel, etc.)
#[cfg(test)]
macro_rules! skip_if_mpk_unavailable {
    () => {
        if !crate::runtime::vm::mpk::is_supported() {
            println!("> mpk is not supported: ignoring test");
            return;
        }
    };
}
/// Necessary for inter-module access.
#[cfg(test)]
pub(crate) use skip_if_mpk_unavailable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_is_supported() {
        println!("is pku supported = {}", is_supported());
        if std::env::var("WASMTIME_TEST_FORCE_MPK").is_ok() {
            assert!(is_supported());
        }
    }

    #[test]
    fn check_initialized_keys() {
        if is_supported() {
            assert!(!keys(15).is_empty())
        }
    }

    #[test]
    fn check_invalid_mark() {
        skip_if_mpk_unavailable!();
        let pkey = keys(15)[0];
        let unaligned_region = unsafe {
            let addr = 1 as *mut u8; // this is not page-aligned!
            let len = 1;
            std::slice::from_raw_parts_mut(addr, len)
        };
        let result = pkey.protect(unaligned_region);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "failed to mark region with pkey (addr = 0x1, len = 1, prot = 0b0)"
        );
    }

    #[test]
    fn check_masking() {
        skip_if_mpk_unavailable!();
        let original = pkru::read();

        allow(ProtectionMask::all());
        assert_eq!(0, pkru::read());

        allow(ProtectionMask::all().or(ProtectionKey { id: 5, stripe: 0 }));
        assert_eq!(0, pkru::read());

        allow(ProtectionMask::zero());
        assert_eq!(0b11111111_11111111_11111111_11111100, pkru::read());

        allow(ProtectionMask::zero().or(ProtectionKey { id: 5, stripe: 0 }));
        assert_eq!(0b11111111_11111111_11110011_11111100, pkru::read());

        // Reset the PKRU state to what we originally observed.
        pkru::write(original);
    }
}
