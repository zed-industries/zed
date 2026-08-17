// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "system")]
use std::ffi::{CStr, OsStr, OsString};
#[cfg(feature = "system")]
use std::os::unix::ffi::OsStrExt;

#[cfg(feature = "system")]
#[inline]
pub unsafe fn init_mib(name: &[u8], mib: &mut [libc::c_int]) {
    let mut len = mib.len();
    libc::sysctlnametomib(name.as_ptr() as _, mib.as_mut_ptr(), &mut len);
}

#[cfg(feature = "system")]
pub(crate) fn boot_time() -> u64 {
    let mut boot_time = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut len = std::mem::size_of::<libc::timeval>();
    let mut mib: [libc::c_int; 2] = [libc::CTL_KERN, libc::KERN_BOOTTIME];
    unsafe {
        if libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as _,
            &mut boot_time as *mut libc::timeval as *mut _,
            &mut len,
            std::ptr::null_mut(),
            0,
        ) < 0
        {
            0
        } else {
            boot_time.tv_sec as _
        }
    }
}

#[cfg(any(feature = "system", feature = "network"))]
pub(crate) unsafe fn get_sys_value<T: Sized>(mib: &[libc::c_int], value: &mut T) -> bool {
    let mut len = std::mem::size_of::<T>() as libc::size_t;
    libc::sysctl(
        mib.as_ptr(),
        mib.len() as _,
        value as *mut _ as *mut _,
        &mut len,
        std::ptr::null_mut(),
        0,
    ) == 0
}

#[cfg(feature = "system")]
pub(crate) unsafe fn get_sys_value_array<T: Sized>(mib: &[libc::c_int], value: &mut [T]) -> bool {
    let mut len = std::mem::size_of_val(value) as libc::size_t;
    libc::sysctl(
        mib.as_ptr(),
        mib.len() as _,
        value.as_mut_ptr() as *mut _,
        &mut len as *mut _,
        std::ptr::null_mut(),
        0,
    ) == 0
}

#[cfg(any(feature = "disk", feature = "system", feature = "network"))]
pub(crate) fn c_buf_to_utf8_str(buf: &[libc::c_char]) -> Option<&str> {
    unsafe {
        let buf: &[u8] = std::slice::from_raw_parts(buf.as_ptr() as _, buf.len());
        std::str::from_utf8(if let Some(pos) = buf.iter().position(|x| *x == 0) {
            // Shrink buffer to terminate the null bytes
            &buf[..pos]
        } else {
            buf
        })
        .ok()
    }
}

#[cfg(any(feature = "system", feature = "network"))]
pub(crate) fn c_buf_to_utf8_string(buf: &[libc::c_char]) -> Option<String> {
    c_buf_to_utf8_str(buf).map(|s| s.to_owned())
}

#[cfg(feature = "system")]
pub(crate) fn c_buf_to_os_str(buf: &[libc::c_char]) -> &OsStr {
    unsafe {
        let buf: &[u8] = std::slice::from_raw_parts(buf.as_ptr() as _, buf.len());
        OsStr::from_bytes(if let Some(pos) = buf.iter().position(|x| *x == 0) {
            // Shrink buffer to terminate the null bytes
            &buf[..pos]
        } else {
            buf
        })
    }
}

#[cfg(feature = "system")]
pub(crate) fn c_buf_to_os_string(buf: &[libc::c_char]) -> OsString {
    c_buf_to_os_str(buf).to_owned()
}

#[cfg(feature = "system")]
pub(crate) unsafe fn get_sys_value_str(
    mib: &[libc::c_int],
    buf: &mut [libc::c_char],
) -> Option<OsString> {
    let mut len = std::mem::size_of_val(buf) as libc::size_t;
    if libc::sysctl(
        mib.as_ptr(),
        mib.len() as _,
        buf.as_mut_ptr() as *mut _,
        &mut len,
        std::ptr::null_mut(),
        0,
    ) != 0
    {
        return None;
    }
    Some(c_buf_to_os_string(
        &buf[..len / std::mem::size_of::<libc::c_char>()],
    ))
}

#[cfg(any(feature = "system", feature = "component"))]
pub(crate) unsafe fn get_sys_value_by_name<T: Sized>(name: &[u8], value: &mut T) -> bool {
    let mut len = std::mem::size_of::<T>() as libc::size_t;
    let original = len;

    libc::sysctlbyname(
        name.as_ptr() as *const libc::c_char,
        value as *mut _ as *mut _,
        &mut len,
        std::ptr::null_mut(),
        0,
    ) == 0
        && original == len
}

#[cfg(feature = "system")]
pub(crate) fn get_sys_value_str_by_name(name: &[u8]) -> Option<String> {
    let mut size = 0;

    unsafe {
        if libc::sysctlbyname(
            name.as_ptr() as *const libc::c_char,
            std::ptr::null_mut(),
            &mut size,
            std::ptr::null_mut(),
            0,
        ) == 0
            && size > 0
        {
            // now create a buffer with the size and get the real value
            let mut buf: Vec<libc::c_char> = vec![0; size as _];

            if libc::sysctlbyname(
                name.as_ptr() as *const libc::c_char,
                buf.as_mut_ptr() as *mut _,
                &mut size,
                std::ptr::null_mut(),
                0,
            ) == 0
                && size > 0
            {
                c_buf_to_utf8_string(&buf)
            } else {
                // getting the system value failed
                None
            }
        } else {
            None
        }
    }
}

#[cfg(feature = "system")]
pub(crate) unsafe fn from_cstr_array(ptr: *const *const libc::c_char) -> Vec<OsString> {
    if ptr.is_null() {
        return Vec::new();
    }
    let mut max = 0;
    loop {
        let ptr = ptr.add(max);
        if (*ptr).is_null() {
            break;
        }
        max += 1;
    }
    if max == 0 {
        return Vec::new();
    }
    let mut ret = Vec::with_capacity(max);

    for pos in 0..max {
        let p = ptr.add(pos);
        ret.push(OsStr::from_bytes(CStr::from_ptr(*p).to_bytes()).to_os_string());
    }
    ret
}

#[cfg(any(feature = "system", feature = "component"))]
pub(crate) unsafe fn get_nb_cpus() -> usize {
    let mut smp: libc::c_int = 0;
    let mut nb_cpus: libc::c_int = 1;

    if !get_sys_value_by_name(b"kern.smp.active\0", &mut smp) {
        smp = 0;
    }
    #[allow(clippy::collapsible_if)] // I keep as is for readability reasons.
    if smp != 0 {
        if !get_sys_value_by_name(b"kern.smp.cpus\0", &mut nb_cpus) || nb_cpus < 1 {
            nb_cpus = 1;
        }
    }
    nb_cpus as usize
}

// All this is needed because `kinfo_proc` doesn't implement `Send` (because it contains pointers).
#[cfg(feature = "system")]
pub(crate) struct WrapMap<'a>(
    pub std::cell::UnsafeCell<&'a mut std::collections::HashMap<crate::Pid, crate::Process>>,
);

#[cfg(feature = "system")]
unsafe impl<'a> Send for WrapMap<'a> {}
#[cfg(feature = "system")]
unsafe impl<'a> Sync for WrapMap<'a> {}

#[cfg(feature = "system")]
#[repr(transparent)]
pub(crate) struct KInfoProc(libc::kinfo_proc);

#[cfg(feature = "system")]
unsafe impl Send for KInfoProc {}
#[cfg(feature = "system")]
unsafe impl Sync for KInfoProc {}

#[cfg(feature = "system")]
impl std::ops::Deref for KInfoProc {
    type Target = libc::kinfo_proc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
