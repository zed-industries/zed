//! # Raw bindings to `Block.h`

use core::cell::UnsafeCell;
use core::ffi::c_void;
use core::marker::{PhantomData, PhantomPinned};
use std::os::raw::c_int;

/// Type for block class ISAs.
///
/// This will likely become an extern type in the future.
#[repr(C)]
#[allow(missing_debug_implementations)]
pub struct Class {
    /// The size probably doesn't really matter here, as we only ever use the
    /// classes behind pointers, but let's import it with the correct size to
    /// be sure.
    ///
    /// This applies with the compiler-rt runtime and with Apple's runtime.
    #[cfg(not(any(feature = "gnustep-1-7", feature = "unstable-objfw")))]
    _priv: [*mut c_void; 32],

    /// The size of this is unknown, so let's use a ZST so the compiler
    /// doesn't assume anything about the size.
    #[cfg(any(feature = "gnustep-1-7", feature = "unstable-objfw"))]
    _priv: [u8; 0],

    /// Mark as `!Send + !Sync + !Unpin` and as mutable behind shared
    /// references (`!Freeze`).
    ///
    /// Same as `objc_sys::OpaqueData`.
    _opaque: UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>,
}

// TODO: Use `extern "C-unwind"` when in MSRV (because the runtime functions
// may call external routines).
extern "C" {
    /// Class ISA used for global blocks.
    pub static _NSConcreteGlobalBlock: Class;

    /// Class ISA used for stack blocks.
    pub static _NSConcreteStackBlock: Class;

    /// Copy/retain a block.
    ///
    /// When called on a:
    /// - Global block: Does nothing.
    /// - Stack block: `memmove`s the block to a new heap allocation, calls
    ///   the copy helper, and returns the new malloc block.
    /// - Malloc block: Increments the retain count.
    ///
    /// Returns `NULL` on allocation failure.
    #[doc(alias = "Block_copy")]
    pub fn _Block_copy(block: *const c_void) -> *mut c_void;

    /// Release a block.
    ///
    /// When called on a:
    /// - Global block: Does nothing.
    /// - Stack block: Does nothing.
    /// - Malloc block: Decrements the retain count, and if it reaches zero,
    ///   calls the dispose helper and frees the underlying storage.
    #[doc(alias = "Block_release")]
    pub fn _Block_release(block: *const c_void);

    /// Copy a block field or `__block` variable from one location to another.
    ///
    /// Called by C compilers to clone fields inside copy helper routines, and
    /// to handle memory management of `__block` marked variables.
    pub fn _Block_object_assign(dest_addr: *mut c_void, object: *const c_void, flags: c_int);

    /// Dispose an object previously copied using `_Block_object_assign`.
    ///
    /// Called by C compilers to drop fields inside dispose helper routines,
    /// and handle memory management of `__block` marked variables.
    pub fn _Block_object_dispose(object: *const c_void, flags: c_int);
}

/// `Block_private.h`
#[allow(missing_docs)]
#[cfg(any(test, feature = "unstable-private"))]
pub mod private {
    use super::*;
    #[cfg(any(doc, target_vendor = "apple", feature = "gnustep-1-7"))]
    use std::os::raw::c_char;
    #[cfg(any(doc, target_vendor = "apple", feature = "compiler-rt"))]
    use std::os::raw::c_ulong;

    extern "C" {
        pub static _NSConcreteMallocBlock: Class;
        #[cfg(any(doc, target_vendor = "apple", feature = "compiler-rt"))]
        pub static _NSConcreteAutoBlock: Class;
        #[cfg(any(doc, target_vendor = "apple", feature = "compiler-rt"))]
        pub static _NSConcreteFinalizingBlock: Class;
        #[cfg(any(doc, target_vendor = "apple", feature = "compiler-rt"))]
        pub static _NSConcreteWeakBlockVariable: Class;

        #[cfg(any(doc, target_vendor = "apple", feature = "compiler-rt"))]
        pub fn Block_size(block: *mut c_void) -> c_ulong; // usize

        // Whether the return value of the block is on the stack.
        // macOS 10.7
        #[cfg(any(doc, target_vendor = "apple"))]
        pub fn _Block_use_stret(block: *mut c_void) -> bool;

        // Returns a string describing the block's GC layout.
        // This uses the GC skip/scan encoding.
        // May return NULL.
        // macOS 10.7
        #[cfg(any(doc, target_vendor = "apple"))]
        pub fn _Block_layout(block: *mut c_void) -> *const c_char;

        // Returns a string describing the block's layout.
        // This uses the "extended layout" form described above.
        // May return NULL.
        // macOS 10.8
        #[cfg(any(doc, target_vendor = "apple"))]
        pub fn _Block_extended_layout(block: *mut c_void) -> *const c_char;

        // Callable only from the ARR weak subsystem while in exclusion zone
        // macOS 10.7
        #[cfg(any(doc, target_vendor = "apple"))]
        pub fn _Block_tryRetain(block: *const c_void) -> bool;

        // Callable only from the ARR weak subsystem while in exclusion zone
        // macOS 10.7
        #[cfg(any(doc, target_vendor = "apple"))]
        pub fn _Block_isDeallocating(block: *const c_void) -> bool;

        // indicates whether block was compiled with compiler that sets the ABI
        // related metadata bits
        // macOS 10.7
        #[cfg(any(doc, target_vendor = "apple", feature = "gnustep-1-7"))]
        pub fn _Block_has_signature(block: *mut c_void) -> bool;

        // Returns a string describing the block's parameter and return types.
        // The encoding scheme is the same as Objective-C @encode.
        // Returns NULL for blocks compiled with some compilers.
        // macOS 10.7
        #[cfg(any(doc, target_vendor = "apple", feature = "gnustep-1-7"))]
        pub fn _Block_signature(block: *mut c_void) -> *const c_char;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;
    use std::println;

    #[test]
    fn smoke() {
        assert_eq!(unsafe { _Block_copy(ptr::null_mut()) }, ptr::null_mut());
        unsafe { _Block_release(ptr::null_mut()) };
    }

    #[test]
    fn test_linkable() {
        println!("{:?}", unsafe { ptr::addr_of!(_NSConcreteGlobalBlock) });
        println!("{:?}", unsafe { ptr::addr_of!(_NSConcreteStackBlock) });
        println!("{:?}", unsafe {
            ptr::addr_of!(private::_NSConcreteMallocBlock)
        });
        println!("{:p}", _Block_copy as unsafe extern "C" fn(_) -> _);
        println!(
            "{:p}",
            _Block_object_assign as unsafe extern "C" fn(_, _, _)
        );
        println!("{:p}", _Block_object_dispose as unsafe extern "C" fn(_, _));
        println!("{:p}", _Block_release as unsafe extern "C" fn(_));
    }
}
