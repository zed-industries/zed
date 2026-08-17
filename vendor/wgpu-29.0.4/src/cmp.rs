//! We need to impl `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` for all handle types in wgpu.
//!
//! For types that have some already-unique property, we can use that property to implement these traits.
//!
//! For types (like WebGPU) that don't have such a property, we generate an identifier and use that.

#[cfg(supports_64bit_atomics)]
pub use core::sync::atomic::AtomicU64;
#[cfg(not(supports_64bit_atomics))]
pub use portable_atomic::AtomicU64;

use core::{num::NonZeroU64, sync::atomic::Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    inner: NonZeroU64,
}

impl Identifier {
    pub fn create() -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        // Safety: Will take 7000+ years of constant incrementing to overflow. It's fine.
        let inner = unsafe { NonZeroU64::new_unchecked(id) };
        Self { inner }
    }
}

/// Implements `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` for a type by proxying the operations to a single field.
///
/// ```ignore
/// impl_eq_ord_hash_proxy!(MyType => .field);
/// ```
macro_rules! impl_eq_ord_hash_proxy {
    ($type:ty => $($access:tt)*) => {
        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self $($access)* == other $($access)*
            }
        }

        impl Eq for $type {}

        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $type {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self $($access)*.cmp(&other $($access)*)
            }
        }

        impl core::hash::Hash for $type {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self $($access)*.hash(state)
            }
        }
    };
}

/// Implements `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` for a type by comparing the addresses of the `Arc`s.
///
/// ```ignore
/// impl_eq_ord_hash_arc_address!(MyType => .field);
/// ```
#[cfg_attr(not(any(wgpu_core, custom)), expect(unused_macros))]
macro_rules! impl_eq_ord_hash_arc_address {
    ($type:ty => $($access:tt)*) => {
        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                let address_left = alloc::sync::Arc::as_ptr(&self $($access)*);
                let address_right = alloc::sync::Arc::as_ptr(&other $($access)*);

                address_left == address_right
            }
        }

        impl Eq for $type {}

        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $type {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                let address_left = alloc::sync::Arc::as_ptr(&self $($access)*);
                let address_right = alloc::sync::Arc::as_ptr(&other $($access)*);

                address_left.cmp(&address_right)
            }
        }

        impl core::hash::Hash for $type {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                let address = alloc::sync::Arc::as_ptr(&self $($access)*);
                address.hash(state)
            }
        }
    };
}

#[cfg_attr(not(any(wgpu_core, custom)), expect(unused_imports))]
pub(crate) use {impl_eq_ord_hash_arc_address, impl_eq_ord_hash_proxy};
