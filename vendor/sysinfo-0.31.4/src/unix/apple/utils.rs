// Take a look at the license at the top of the repository in the LICENSE file.

use core_foundation_sys::base::CFRelease;
use std::ptr::NonNull;

// A helper using to auto release the resource got from CoreFoundation.
// More information about the ownership policy for CoreFoundation pelease refer the link below:
// https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-CJBEJBHH
#[repr(transparent)]
#[allow(dead_code)]
pub(crate) struct CFReleaser<T>(NonNull<T>);

#[allow(dead_code)]
impl<T> CFReleaser<T> {
    pub(crate) fn new(ptr: *const T) -> Option<Self> {
        // This cast is OK because `NonNull` is a transparent wrapper
        // over a `*const T`. Additionally, mutability doesn't matter with
        // pointers here.
        NonNull::new(ptr as *mut T).map(Self)
    }

    pub(crate) fn inner(&self) -> *const T {
        self.0.as_ptr().cast()
    }
}

impl<T> Drop for CFReleaser<T> {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0.as_ptr().cast()) }
    }
}

// Safety: These are safe to implement because we only wrap non-mutable
// CoreFoundation types, which are generally threadsafe unless noted
// otherwise.
unsafe impl<T> Send for CFReleaser<T> {}
unsafe impl<T> Sync for CFReleaser<T> {}

#[cfg(feature = "disk")]
pub(crate) fn vec_to_rust(buf: Vec<i8>) -> Option<String> {
    String::from_utf8(
        buf.into_iter()
            .flat_map(|b| if b > 0 { Some(b as u8) } else { None })
            .collect(),
    )
    .ok()
}

#[cfg(feature = "system")]
pub(crate) unsafe fn get_sys_value(
    mut len: usize,
    value: *mut libc::c_void,
    mib: &mut [i32],
) -> bool {
    libc::sysctl(
        mib.as_mut_ptr(),
        mib.len() as _,
        value,
        &mut len as *mut _,
        std::ptr::null_mut(),
        0,
    ) == 0
}

#[cfg(feature = "system")]
pub(crate) unsafe fn get_sys_value_by_name(
    name: &[u8],
    len: &mut usize,
    value: *mut libc::c_void,
) -> bool {
    libc::sysctlbyname(
        name.as_ptr() as *const _,
        value,
        len,
        std::ptr::null_mut(),
        0,
    ) == 0
}
