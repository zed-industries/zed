//! This is just a sample of what FFI using this crate can look like.

#![allow(missing_docs)]

#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
use crate::{BorrowedFd, OwnedFd};
#[cfg(windows)]
use crate::{BorrowedHandle, HandleOrInvalid};

#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
use libc::{c_char, c_int, c_void, size_t, ssize_t};
#[cfg(windows)]
use {
    core::ffi::c_void,
    windows_sys::core::PCWSTR,
    windows_sys::Win32::Foundation::BOOL,
    windows_sys::Win32::Security::SECURITY_ATTRIBUTES,
    windows_sys::Win32::Storage::FileSystem::{
        FILE_CREATION_DISPOSITION, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE,
    },
    windows_sys::Win32::System::IO::OVERLAPPED,
};

// Declare a few FFI functions ourselves, to show off the FFI ergonomics.
#[cfg(any(unix, target_os = "wasi", target_os = "hermit"))]
extern "C" {
    pub fn open(pathname: *const c_char, flags: c_int, ...) -> Option<OwnedFd>;
}
#[cfg(any(unix, target_os = "wasi"))]
extern "C" {
    pub fn read(fd: BorrowedFd<'_>, ptr: *mut c_void, size: size_t) -> ssize_t;
    pub fn write(fd: BorrowedFd<'_>, ptr: *const c_void, size: size_t) -> ssize_t;
}
#[cfg(any(unix, target_os = "wasi"))]
pub use libc::{O_CLOEXEC, O_CREAT, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY};

// The Windows analogs of the above. Note the use of [`HandleOrInvalid`] as
// the return type for `CreateFileW`, since that function is defined to return
// [`INVALID_HANDLE_VALUE`] on error instead of null.
#[cfg(windows)]
extern "system" {
    pub fn CreateFileW(
        lpfilename: PCWSTR,
        dwdesiredaccess: u32,
        dwsharemode: FILE_SHARE_MODE,
        lpsecurityattributes: *const SECURITY_ATTRIBUTES,
        dwcreationdisposition: FILE_CREATION_DISPOSITION,
        dwflagsandattributes: FILE_FLAGS_AND_ATTRIBUTES,
        htemplatefile: HANDLE,
    ) -> HandleOrInvalid;
    pub fn ReadFile(
        hfile: BorrowedHandle<'_>,
        lpbuffer: *mut c_void,
        nnumberofbytestoread: u32,
        lpnumberofbytesread: *mut u32,
        lpoverlapped: *mut OVERLAPPED,
    ) -> BOOL;
    pub fn WriteFile(
        hfile: BorrowedHandle<'_>,
        lpbuffer: *const c_void,
        nnumberofbytestowrite: u32,
        lpnumberofbyteswritten: *mut u32,
        lpoverlapped: *mut OVERLAPPED,
    ) -> BOOL;
}

#[cfg(windows)]
pub use {
    windows_sys::Win32::Foundation::HANDLE,
    windows_sys::Win32::Storage::FileSystem::{
        CREATE_ALWAYS, CREATE_NEW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
        OPEN_EXISTING,
    },
};
