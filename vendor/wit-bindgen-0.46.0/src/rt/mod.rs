use core::alloc::Layout;
use core::ptr::{self, NonNull};

// Re-export `bitflags` so that we can reference it from macros.
#[cfg(feature = "bitflags")]
pub use bitflags;

#[cfg(not(feature = "bitflags"))]
pub mod bitflags {
    #[macro_export]
    macro_rules! bitflags {
        (
            $(#[$attr:meta])*
            $vis:vis struct $name:ident : $repr:ty {
                $(
                    $(#[$flag_attr:meta])*
                    const $flag:ident = $val:expr;
                )*
            }
        ) => {
            $(#[$attr])*
            $vis struct $name {
                bits: $repr,
            }

            impl $name {
                $(
                    $(#[$flag_attr])*
                    $vis const $flag: Self = Self { bits: $val };
                )*
                $vis fn empty() -> Self {
                    Self { bits: 0 }
                }

                $vis fn from_bits_retain(bits: $repr) -> Self {
                    Self { bits }
                }

                $vis fn bits(&self) -> $repr {
                    self.bits
                }
            }

            impl core::ops::BitOr<$name> for $name {
                type Output = Self;
                fn bitor(self, rhs: $name) -> $name {
                    Self { bits: self.bits | rhs.bits }
                }
            }

            impl core::ops::BitAnd<$name> for $name {
                type Output = Self;
                fn bitand(self, rhs: $name) -> $name {
                    Self { bits: self.bits & rhs.bits }
                }
            }

            impl core::ops::BitXor<$name> for $name {
                type Output = Self;
                fn bitxor(self, rhs: $name) -> $name {
                    Self { bits: self.bits ^ rhs.bits }
                }
            }
        };
    }

    pub use crate::bitflags;
}

/// For more information about this see `./ci/rebuild-libwit-bindgen-cabi.sh`.
#[cfg(not(target_env = "p2"))]
mod wit_bindgen_cabi_realloc;

/// This function is called from generated bindings and will be deleted by
/// the linker. The purpose of this function is to force a reference to the
/// symbol `cabi_realloc` to make its way through to the final linker
/// command line. That way `wasm-ld` will pick it up, see it needs to be
/// exported, and then export it.
///
/// For more information about this see `./ci/rebuild-libwit-bindgen-cabi.sh`.
pub fn maybe_link_cabi_realloc() {
    #[cfg(all(target_family = "wasm", not(target_env = "p2")))]
    {
        extern "C" {
            fn cabi_realloc(
                old_ptr: *mut u8,
                old_len: usize,
                align: usize,
                new_len: usize,
            ) -> *mut u8;
        }
        // Force the `cabi_realloc` symbol to be referenced from here. This
        // is done with a `#[used]` Rust `static` to ensure that this
        // reference makes it all the way to the linker before it's
        // considered for garbage collection. When the linker sees it it'll
        // remove this `static` here (due to it not actually being needed)
        // but the linker will have at that point seen the `cabi_realloc`
        // symbol and it should get exported.
        #[used]
        static _NAME_DOES_NOT_MATTER: unsafe extern "C" fn(
            *mut u8,
            usize,
            usize,
            usize,
        ) -> *mut u8 = cabi_realloc;
    }
}

/// NB: this function is called by a generated function in the
/// `cabi_realloc` module above. It's otherwise never explicitly called.
///
/// For more information about this see `./ci/rebuild-libwit-bindgen-cabi.sh`.
#[cfg(not(target_env = "p2"))]
pub unsafe fn cabi_realloc(
    old_ptr: *mut u8,
    old_len: usize,
    align: usize,
    new_len: usize,
) -> *mut u8 {
    use alloc::alloc::{alloc as allocate, handle_alloc_error, realloc, Layout};

    let layout;
    let ptr = if old_len == 0 {
        if new_len == 0 {
            return align as *mut u8;
        }
        layout = Layout::from_size_align_unchecked(new_len, align);
        allocate(layout)
    } else {
        debug_assert_ne!(new_len, 0, "non-zero old_len requires non-zero new_len!");
        layout = Layout::from_size_align_unchecked(old_len, align);
        realloc(old_ptr, layout, new_len)
    };
    if ptr.is_null() {
        // Print a nice message in debug mode, but in release mode don't
        // pull in so many dependencies related to printing so just emit an
        // `unreachable` instruction.
        if cfg!(debug_assertions) {
            handle_alloc_error(layout);
        } else {
            #[cfg(target_arch = "wasm32")]
            core::arch::wasm32::unreachable();
            #[cfg(not(target_arch = "wasm32"))]
            unreachable!();
        }
    }
    return ptr;
}

/// Provide a hook for generated export functions to run static constructors at
/// most once.
///
/// wit-bindgen-rust generates a call to this function at the start of all
/// component export functions. Importantly, it is not called as part of
/// `cabi_realloc`, which is a *core* export func, but should not execute ctors.
#[cfg(target_arch = "wasm32")]
pub fn run_ctors_once() {
    static mut RUN: bool = false;
    unsafe {
        if !RUN {
            // This function is synthesized by `wasm-ld` to run all static
            // constructors. wasm-ld will either provide an implementation
            // of this symbol, or synthesize a wrapper around each
            // exported function to (unconditionally) run ctors. By using
            // this function, the linked module is opting into "manually"
            // running ctors.
            extern "C" {
                fn __wasm_call_ctors();
            }
            __wasm_call_ctors();
            RUN = true;
        }
    }
}

/// Support for using the Component Model Async ABI
#[cfg(feature = "async")]
pub mod async_support;

/// Cleanup helper used to deallocate blocks of canonical ABI data from
/// lowerings.
pub struct Cleanup {
    ptr: NonNull<u8>,
    layout: Layout,
}

// Usage of the returned pointer is always unsafe and must abide by these
// conventions, but this structure itself has no inherent reason to not be
// send/sync.
unsafe impl Send for Cleanup {}
unsafe impl Sync for Cleanup {}

impl Cleanup {
    /// Allocates a chunk of memory with `layout` and returns an object to clean
    /// it up.
    ///
    /// Always returns a pointer which is null if `layout` has size zero. The
    /// optional cleanup returned will be present if `layout` has a non-zero
    /// size. When dropped `Cleanup` will deallocate the pointer returned.
    pub fn new(layout: Layout) -> (*mut u8, Option<Cleanup>) {
        use alloc::alloc;

        if layout.size() == 0 {
            return (ptr::null_mut(), None);
        }
        let ptr = unsafe { alloc::alloc(layout) };
        let ptr = match NonNull::new(ptr) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(layout),
        };
        (ptr.as_ptr(), Some(Cleanup { ptr, layout }))
    }

    /// Discards this cleanup to leak its memory or intentionally transfer
    /// ownership to some other location.
    pub fn forget(self) {
        core::mem::forget(self);
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.layout.size() {
                *self.ptr.add(i).as_ptr() = 0xff;
            }
            alloc::alloc::dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}
