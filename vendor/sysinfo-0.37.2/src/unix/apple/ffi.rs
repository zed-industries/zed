// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "disk")]
#[link(name = "objc", kind = "dylib")]
unsafe extern "C" {
    pub fn objc_autoreleasePoolPop(pool: *mut libc::c_void);
    pub fn objc_autoreleasePoolPush() -> *mut libc::c_void;
}
