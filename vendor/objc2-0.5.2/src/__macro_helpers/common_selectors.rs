//! Common selectors.
//!
//! These are put here to deduplicate the cached selector, and when using
//! `unstable-static-sel`, the statics.
//!
//! Note that our assembly tests of `unstable-static-sel-inlined` output a GOT
//! entry for such accesses, but that is just a limitation of our tests - the
//! actual assembly is as one would expect.
use crate::runtime::Sel;
use crate::{__sel_data, __sel_inner};

#[inline]
pub fn alloc_sel() -> Sel {
    __sel_inner!(__sel_data!(alloc), "alloc")
}

#[inline]
pub fn init_sel() -> Sel {
    __sel_inner!(__sel_data!(init), "init")
}

#[inline]
pub fn new_sel() -> Sel {
    __sel_inner!(__sel_data!(new), "new")
}

#[inline]
pub fn dealloc_sel() -> Sel {
    __sel_inner!(__sel_data!(dealloc), "dealloc")
}

/// An undocumented selector called by the Objective-C runtime when
/// initalizing instance variables.
#[inline]
#[allow(dead_code)] // May be useful in the future
fn cxx_construct_sel() -> Sel {
    __sel_inner!(".cxx_construct\0", ".cxx_construct")
}

/// Objective-C runtimes call `.cxx_destruct` as part of the final `dealloc`
/// call inside `NSObject` (and has done so since macOS 10.4).
///
/// While [GCC does document it somewhat][gcc-docs], this is still severely
/// undocumented in clang - but since the selector is emitted into the final
/// binary, it is fine to rely on it being used.
///
/// Unfortunately though, this only works if the class has been declared
/// statically, since in that case a flag is set to inform the runtime that it
/// needs to run destructors. So unfortunately we can't use this on
/// dynamically declared classes.
///
///
/// # ABI
///
/// The ABI of `.cxx_destruct` in Apple's runtime is actually that it does NOT
/// take a selector, unlike every other Objective-C method, see:
/// <https://github.com/apple-oss-distributions/objc4/blob/objc4-906/runtime/objc-class.mm#L457>
///
/// So the signature is `extern "C" fn(*mut AnyObject)`.
///
/// This is likely because it's not a real Objective-C method that can be
/// called from userspace / objc_msgSend, and it's more efficient to not pass
/// the selector.
///
/// Note that even if Apple decides to suddenly add the selector one day,
/// ignoring it will still be sound, since the function uses the C calling
/// convention, where such an ignored parameter would be allowed on all
/// relevant architectures.
///
/// TODO: Unsure whether "C-unwind" is allowed?
///
/// [gcc-docs]: https://gcc.gnu.org/onlinedocs/gcc/Objective-C-and-Objective-C_002b_002b-Dialect-Options.html#index-fobjc-call-cxx-cdtors
#[inline]
#[allow(dead_code)] // May be useful in the future
fn cxx_destruct_sel() -> Sel {
    __sel_inner!(".cxx_destruct\0", ".cxx_destruct")
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicBool, Ordering};

    use crate::rc::Retained;
    use crate::runtime::ClassBuilder;
    use crate::runtime::NSObject;
    use crate::{msg_send_id, ClassType};

    use super::*;

    /// Test the unfortunate fact that we can't use .cxx_destruct on dynamic classes.
    #[test]
    fn test_destruct_dynamic() {
        static HAS_RUN: AtomicBool = AtomicBool::new(false);

        let mut builder = ClassBuilder::new("TestCxxDestruct", NSObject::class()).unwrap();

        unsafe extern "C" fn destruct(_: *mut NSObject, _: Sel) {
            HAS_RUN.store(true, Ordering::Relaxed);
        }

        // Note: The ABI is not upheld here, but its fine for this test
        unsafe { builder.add_method(cxx_destruct_sel(), destruct as unsafe extern "C" fn(_, _)) };

        let cls = builder.register();

        let obj: Retained<NSObject> = unsafe { msg_send_id![cls, new] };
        drop(obj);
        let has_run_destruct = HAS_RUN.load(Ordering::Relaxed);

        // This does work on GNUStep, but unfortunately not in Apple's objc4
        if cfg!(feature = "gnustep-1-7") {
            assert!(has_run_destruct);
        } else {
            assert!(!has_run_destruct);
        }
    }
}
