// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "system")]
pub(crate) unsafe fn get_sys_value(
    mut len: usize,
    value: *mut libc::c_void,
    mib: &mut [i32],
) -> bool {
    unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as _,
            value,
            &mut len as *mut _,
            std::ptr::null_mut(),
            0,
        ) == 0
    }
}

#[cfg(feature = "system")]
pub(crate) unsafe fn get_sys_value_by_name(
    name: &[u8],
    len: &mut usize,
    value: *mut libc::c_void,
) -> bool {
    unsafe {
        libc::sysctlbyname(
            name.as_ptr() as *const _,
            value,
            len,
            std::ptr::null_mut(),
            0,
        ) == 0
    }
}
