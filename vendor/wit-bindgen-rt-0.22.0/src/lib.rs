#![no_std]

extern crate alloc;

/// For more information about this see `./ci/rebuild-libcabi-realloc.sh`.
mod cabi_realloc;

/// This function is called from generated bindings and will be deleted by
/// the linker. The purpose of this function is to force a reference to the
/// symbol `cabi_realloc` to make its way through to the final linker
/// command line. That way `wasm-ld` will pick it up, see it needs to be
/// exported, and then export it.
///
/// For more information about this see `./ci/rebuild-libcabi-realloc.sh`.
pub fn maybe_link_cabi_realloc() {
    #[cfg(target_family = "wasm")]
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
/// For more information about this see `./ci/rebuild-libcabi-realloc.sh`.
pub unsafe fn cabi_realloc(
    old_ptr: *mut u8,
    old_len: usize,
    align: usize,
    new_len: usize,
) -> *mut u8 {
    use self::alloc::alloc::{self, Layout};

    let layout;
    let ptr = if old_len == 0 {
        if new_len == 0 {
            return align as *mut u8;
        }
        layout = Layout::from_size_align_unchecked(new_len, align);
        alloc::alloc(layout)
    } else {
        debug_assert_ne!(new_len, 0, "non-zero old_len requires non-zero new_len!");
        layout = Layout::from_size_align_unchecked(old_len, align);
        alloc::realloc(old_ptr, layout, new_len)
    };
    if ptr.is_null() {
        // Print a nice message in debug mode, but in release mode don't
        // pull in so many dependencies related to printing so just emit an
        // `unreachable` instruction.
        if cfg!(debug_assertions) {
            alloc::handle_alloc_error(layout);
        } else {
            #[cfg(target_arch = "wasm32")]
            core::arch::wasm32::unreachable();
            #[cfg(not(target_arch = "wasm32"))]
            unreachable!();
        }
    }
    return ptr;
}
