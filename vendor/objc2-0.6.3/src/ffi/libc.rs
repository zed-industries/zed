use core::ffi::c_void;

// SAFETY: The signatures in here are the exact same as in `libc`.
extern_c! {
    /// The Objective-C runtime has several methods, usually with "`copy`" in
    /// their name, whose return value is allocated with C's `malloc` and
    /// deallocated with C's `free` method.
    ///
    /// As such, `free` is actually also part of the Objective-C runtime.
    ///
    /// We expose this instead of using [`libc::free`], to avoid having `libc`
    /// as a dependency.
    ///
    /// [`libc::free`]: https://docs.rs/libc/latest/libc/fn.free.html
    //
    // Note: This is linked automatically by either `std` or transitively by
    // `libobjc`.
    pub fn free(p: *mut c_void);
}
