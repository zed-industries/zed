cfg_if! {
    if #[cfg(any(target_os = "netbsd", target_os = "openbsd", target_os = "android", target_os = "cygwin"))] {
        use libc::__errno as errno_location;
    } else if #[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "hurd", target_os = "redox", target_os = "dragonfly"))] {
        use libc::__errno_location as errno_location;
    } else if #[cfg(target_os = "illumos")] {
        use libc::___errno as errno_location;
    } else if #[cfg(any(target_os = "macos", target_os = "freebsd"))] {
        use libc::__error as errno_location;
    } else if #[cfg(target_os = "haiku")] {
        use libc::_errnop as errno_location;
    } else if #[cfg(target_os = "nto")] {
        use libc::__get_errno_ptr as errno_location;
    } else if #[cfg(any(all(target_os = "horizon", target_arch = "arm"), target_os = "vita"))] {
        unsafe extern "C" {
            // Not provided by libc: https://github.com/rust-lang/libc/issues/1995
            fn __errno() -> *mut libc::c_int;
        }
        use __errno as errno_location;
    } else if #[cfg(target_os = "aix")] {
        use libc::_Errno as errno_location;
    } else {
        compile_error!("errno_location is not provided for the target");
    }
}

pub(crate) fn get_errno() -> libc::c_int {
    unsafe { core::ptr::read(errno_location()) }
}
