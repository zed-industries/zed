//! Bindings for [sync functions]
//!
//! [sync functions]: https://developer.android.com/ndk/reference/group/sync
#![cfg(feature = "sync")]

use std::{
    ffi::CStr,
    fmt::Debug,
    // TODO: Import from std::os::fd::{} since Rust 1.66
    os::unix::io::{AsRawFd, BorrowedFd, FromRawFd, OwnedFd},
    ptr::NonNull,
};

#[doc(alias = "sync_file_info")]
#[repr(transparent)]
pub struct SyncFileInfo {
    inner: NonNull<ffi::sync_file_info>,
}

impl Debug for SyncFileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncFileInfo")
            .field("name", &self.name())
            .field("status", &self.status())
            .field("flags", &self.flags())
            .field("num_fences", &self.num_fences())
            .field("fence_info", &self.fence_info())
            .finish()
    }
}

impl SyncFileInfo {
    /// Retrieve detailed information about a sync file and its fences.
    #[doc(alias = "sync_file_info")]
    pub fn new(fd: BorrowedFd<'_>) -> Option<Self> {
        let inner = NonNull::new(unsafe { ffi::sync_file_info(fd.as_raw_fd()) })?;
        Some(Self { inner })
    }

    pub fn name(&self) -> &CStr {
        let inner = unsafe { self.inner.as_ref() };
        // TODO: Switch to CStr::from_bytes_until_nul (with c_char -> u8 transmute) since MSRV 1.69
        // https://github.com/ash-rs/ash/pull/746
        unsafe { CStr::from_ptr(inner.name.as_ptr()) }
    }

    pub fn status(&self) -> i32 {
        let inner = unsafe { self.inner.as_ref() };
        inner.status
    }

    pub fn flags(&self) -> u32 {
        let inner = unsafe { self.inner.as_ref() };
        inner.flags
    }

    pub fn num_fences(&self) -> usize {
        let inner = unsafe { self.inner.as_ref() };
        inner.num_fences as usize
    }

    /// Get the array of fence infos from the sync file's info.
    #[doc(alias = "sync_get_fence_info")]
    pub fn fence_info(&self) -> &[SyncFenceInfo] {
        let inner = unsafe { self.inner.as_ref() };

        if inner.num_fences == 0 {
            &[]
        } else {
            let sync_fence_info = NonNull::new(inner.sync_fence_info as *mut _)
                .expect("sync_fence_info cannot be null if num_fences > 0");
            unsafe {
                std::slice::from_raw_parts(sync_fence_info.as_ptr(), inner.num_fences as usize)
            }
        }
    }
}

impl Drop for SyncFileInfo {
    /// Free a [`struct@ffi::sync_file_info`] structure.
    #[doc(alias = "sync_file_info_free")]
    fn drop(&mut self) {
        unsafe { ffi::sync_file_info_free(self.inner.as_ptr()) }
    }
}

#[doc(alias = "sync_fence_info")]
#[repr(transparent)]
pub struct SyncFenceInfo(ffi::sync_fence_info);

impl Debug for SyncFenceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SyncFenceInfo")
            .field("obj_name", &self.obj_name())
            .field("driver_name", &self.driver_name())
            .field("status", &self.status())
            .field("flags", &self.flags())
            .field("timestamp_ns", &self.timestamp_ns())
            .finish()
    }
}

impl SyncFenceInfo {
    pub fn obj_name(&self) -> &CStr {
        // TODO: Switch to CStr::from_bytes_until_nul (with c_char -> u8 transmute) since MSRV 1.69
        unsafe { CStr::from_ptr(self.0.obj_name.as_ptr()) }
    }

    pub fn driver_name(&self) -> &CStr {
        // TODO: Switch to CStr::from_bytes_until_nul (with c_char -> u8 transmute) since MSRV 1.69
        unsafe { CStr::from_ptr(self.0.driver_name.as_ptr()) }
    }

    pub fn status(&self) -> i32 {
        self.0.status
    }

    pub fn flags(&self) -> u32 {
        self.0.flags
    }

    pub fn timestamp_ns(&self) -> u64 {
        self.0.timestamp_ns
    }
}

/// Merge two sync files.
///
/// This produces a new sync file with the given name which has the union of the two original sync
/// file's fences; redundant fences may be removed.
///
/// If one of the input sync files is signaled or invalid, then this function may behave like
/// `dup()`: the new file descriptor refers to the valid/unsignaled sync file with its original
/// name, rather than a new sync file.
pub fn sync_merge(name: &CStr, fd1: BorrowedFd<'_>, fd2: BorrowedFd<'_>) -> OwnedFd {
    unsafe {
        OwnedFd::from_raw_fd(ffi::sync_merge(
            name.as_ptr(),
            fd1.as_raw_fd(),
            fd2.as_raw_fd(),
        ))
    }
}
