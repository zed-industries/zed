//! A library to help grow the stack when it runs out of space.
//!
//! This is an implementation of manually instrumented segmented stacks where points in a program's
//! control flow are annotated with "maybe grow the stack here". Each point of annotation indicates
//! how far away from the end of the stack it's allowed to be, plus the amount of stack to allocate
//! if it does reach the end.
//!
//! Once a program has reached the end of its stack, a temporary stack on the heap is allocated and
//! is switched to for the duration of a closure.
//!
//! For a set of lower-level primitives, consider the `psm` crate.
//!
//! # Examples
//!
//! ```
//! // Grow the stack if we are within the "red zone" of 32K, and if we allocate
//! // a new stack allocate 1MB of stack space.
//! //
//! // If we're already in bounds, just run the provided closure on current stack.
//! stacker::maybe_grow(32 * 1024, 1024 * 1024, || {
//!     // guaranteed to have at least 32K of stack
//! });
//! ```

#![allow(improper_ctypes)]

#[macro_use]
extern crate cfg_if;
extern crate libc;
#[cfg(windows)]
extern crate windows_sys;
#[macro_use]
extern crate psm;

mod backends;

use std::cell::Cell;

/// Grows the call stack if necessary.
///
/// This function is intended to be called at manually instrumented points in a program where
/// recursion is known to happen quite a bit. This function will check to see if we're within
/// `red_zone` bytes of the end of the stack, and if so it will allocate a new stack of at least
/// `stack_size` bytes.
///
/// The closure `f` is guaranteed to run on a stack with at least `red_zone` bytes, and it will be
/// run on the current stack if there's space available.
#[inline(always)]
pub fn maybe_grow<R, F: FnOnce() -> R>(red_zone: usize, stack_size: usize, callback: F) -> R {
    // if we can't guess the remaining stack (unsupported on some platforms) we immediately grow
    // the stack and then cache the new stack size (which we do know now because we allocated it.
    let enough_space = match remaining_stack() {
        Some(remaining) => remaining >= red_zone,
        None => false,
    };
    if enough_space {
        callback()
    } else {
        grow(stack_size, callback)
    }
}

/// Always creates a new stack for the passed closure to run on.
/// The closure will still be on the same thread as the caller of `grow`.
/// This will allocate a new stack with at least `stack_size` bytes.
pub fn grow<R, F: FnOnce() -> R>(stack_size: usize, callback: F) -> R {
    // To avoid monomorphizing `_grow()` and everything it calls,
    // we convert the generic callback to a dynamic one.
    let mut opt_callback = Some(callback);
    let mut ret = None;
    let ret_ref = &mut ret;

    // This wrapper around `callback` achieves two things:
    // * It converts the `impl FnOnce` to a `dyn FnMut`.
    //   `dyn` because we want it to not be generic, and
    //   `FnMut` because we can't pass a `dyn FnOnce` around without boxing it.
    // * It eliminates the generic return value, by writing it to the stack of this function.
    //   Otherwise the closure would have to return an unsized value, which isn't possible.
    let dyn_callback: &mut dyn FnMut() = &mut || {
        let taken_callback = opt_callback.take().unwrap();
        *ret_ref = Some(taken_callback());
    };

    _grow(stack_size, dyn_callback);
    ret.unwrap()
}

/// Queries the amount of remaining stack as interpreted by this library.
///
/// This function will return the amount of stack space left which will be used
/// to determine whether a stack switch should be made or not.
pub fn remaining_stack() -> Option<usize> {
    let current_ptr = current_stack_ptr();
    get_stack_limit().map(|limit| current_ptr.saturating_sub(limit))
}

psm_stack_information!(
    yes {
        fn current_stack_ptr() -> usize {
            psm::stack_pointer() as usize
        }
    }
    no {
        #[inline(always)]
        fn current_stack_ptr() -> usize {
            unsafe {
                let mut x = std::mem::MaybeUninit::<u8>::uninit();
                // Unlikely to be ever exercised. As a fallback we execute a volatile read to a
                // local (to hopefully defeat the optimisations that would make this local a static
                // global) and take its address. This way we get a very approximate address of the
                // current frame.
                x.as_mut_ptr().write_volatile(42);
                x.as_ptr() as usize
            }
        }
    }
);

thread_local! {
    static STACK_LIMIT: Cell<Option<usize>> = Cell::new(unsafe {
        backends::guess_os_stack_limit()
    })
}

#[inline(always)]
fn get_stack_limit() -> Option<usize> {
    STACK_LIMIT.with(|s| s.get())
}

#[inline(always)]
#[allow(unused)]
fn set_stack_limit(l: Option<usize>) {
    STACK_LIMIT.with(|s| s.set(l))
}

psm_stack_manipulation! {
    yes {
        #[cfg(not(any(target_arch = "wasm32",target_os = "hermit", target_os = "motor")))]
        #[path = "mmap_stack_restore_guard.rs"]
        mod stack_restore_guard;

        #[cfg(any(target_arch = "wasm32",target_os = "hermit", target_os = "motor"))]
        #[path = "alloc_stack_restore_guard.rs"]
        mod stack_restore_guard;

        use stack_restore_guard::StackRestoreGuard;

        fn _grow(requested_stack_size: usize, callback: &mut dyn FnMut()) {
            // Other than that this code has no meaningful gotchas.
            unsafe {
                // We use a guard pattern to ensure we deallocate the allocated stack when we leave
                // this function and also try to uphold various safety invariants required by `psm`
                // (such as not unwinding from the callback we pass to it).
                // `StackRestoreGuard` allocates a memory area with suitable size and alignment.
                // It also sets up stack guards if supported on target.
                let guard = StackRestoreGuard::new(requested_stack_size);
                let (stack_base, allocated_stack_size) = guard.stack_area();
                debug_assert!(allocated_stack_size >= requested_stack_size);
                set_stack_limit(Some(stack_base as usize));
                let panic = psm::on_stack(stack_base, allocated_stack_size, move || {
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(callback)).err()
                });
                drop(guard);
                if let Some(p) = panic {
                    std::panic::resume_unwind(p);
                }
            }
        }
    }

    no {
        #[cfg(not(all(windows, not(miri))))]
        fn _grow(stack_size: usize, callback: &mut dyn FnMut()) {
            let _ = stack_size;
            callback();
        }
        #[cfg(all(windows, not(miri)))]
        use backends::windows::_grow;
    }
}
