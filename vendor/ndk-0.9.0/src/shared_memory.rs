//! Bindings for [`ASharedMemory`]
//!
//! [`ASharedMemory`]: https://developer.android.com/ndk/reference/group/memory
#![cfg(feature = "api-level-26")]

use std::{
    ffi::CStr,
    io::{Error, Result},
    // TODO: Import from std::os::fd::{} since Rust 1.66
    os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd},
    ptr,
};

#[cfg(feature = "api-level-27")]
use jni_sys::{jobject, JNIEnv};

/// Enables the creation, mapping, and protection control over anonymous shared memory.
#[derive(Debug)]
#[doc(alias = "ASharedMemory")]
pub struct SharedMemory(OwnedFd);

impl SharedMemory {
    /// Create a shared memory region.
    ///
    /// Creates shared memory region and returns a file descriptor. The resulting file descriptor
    /// can be `mmap`'ed to process memory space with `PROT_READ | PROT_WRITE | PROT_EXEC`. Access
    /// to this shared memory region can be restricted with [`set_prot()`][Self::set_prot()].
    ///
    /// Use [`android.os.ParcelFileDescriptor`] to pass the file descriptor to another process.
    /// File descriptors may also be sent to other processes over a Unix domain socket with
    /// `sendmsg` and `SCM_RIGHTS`. See `sendmsg(3)` and `cmsg(3)` man pages for more information.
    ///
    /// If you intend to share this file descriptor with a child process after calling `exec(3)`,
    /// note that you will need to use `fcntl(2)` with `F_SETFD` to clear the `FD_CLOEXEC` flag for
    /// this to work on all versions of Android.
    ///
    /// [`android.os.ParcelFileDescriptor`]: https://developer.android.com/reference/android/os/ParcelFileDescriptor
    #[doc(alias = "ASharedMemory_create")]
    pub fn create(name: Option<&CStr>, size: usize) -> Result<Self> {
        let fd =
            unsafe { ffi::ASharedMemory_create(name.map_or(ptr::null(), |p| p.as_ptr()), size) };
        if fd < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(unsafe { Self::from_raw_fd(fd) })
        }
    }

    /// Returns a `dup`'d FD from the given Java [`android.os.SharedMemory`] object.
    ///
    /// The returned file descriptor has all the same properties & capabilities as the FD returned
    /// from [`create()`][Self::create()], however the protection flags will be the same as those
    /// of the [`android.os.SharedMemory`] object.
    ///
    /// [`android.os.SharedMemory`]: https://developer.android.com/reference/android/os/SharedMemory
    #[cfg(feature = "api-level-27")]
    #[doc(alias = "ASharedMemory_dupFromJava")]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn dup_from_java(env: *mut JNIEnv, shared_memory: jobject) -> Result<Self> {
        let fd = unsafe { ffi::ASharedMemory_dupFromJava(env, shared_memory) };
        if fd < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(unsafe { Self::from_raw_fd(fd) })
        }
    }

    /// Get the size of the shared memory region.
    #[doc(alias = "ASharedMemory_getSize")]
    pub fn size(&self) -> usize {
        unsafe { ffi::ASharedMemory_getSize(self.as_raw_fd()) }
    }

    /// Restrict access of shared memory region.
    ///
    /// This function restricts access of a shared memory region. Access can only be removed. The
    /// effect applies globally to all file descriptors in all processes across the system that
    /// refer to this shared memory region. Existing memory mapped regions are not affected.
    ///
    /// It is a common use case to create a shared memory region, map it read/write locally to
    /// initialize content, and then send the shared memory to another process with read only
    /// access. Code example as below:
    ///
    /// ```no_run
    /// # use ndk::shared_memory::SharedMemory;
    /// # // TODO: Import from std::os::fd::{} since Rust 1.66
    /// # use std::os::unix::io::AsRawFd;
    /// # use std::ffi::CStr;
    /// # unsafe {
    /// let mem = SharedMemory::create(Some(CStr::from_bytes_with_nul_unchecked(b"memory\0")), 127).unwrap();
    /// // By default it has PROT_READ | PROT_WRITE | PROT_EXEC.
    /// let size = mem.size();
    /// let buffer = libc::mmap(
    ///     std::ptr::null_mut(),
    ///     size,
    ///     libc::PROT_READ | libc::PROT_WRITE,
    ///     libc::MAP_SHARED,
    ///     mem.as_raw_fd(),
    ///     0,
    /// );
    /// let buffer_slice = std::slice::from_raw_parts_mut(buffer.cast(), size);
    ///
    /// // trivially initialize content
    /// buffer_slice[..7].copy_from_slice(b"hello!\0");
    ///
    /// // Existing mappings will retain their protection flags (PROT_WRITE here) after set_prod()
    /// // unless it is unmapped:
    /// libc::munmap(buffer, size);
    ///
    /// // limit access to read only
    /// mem.set_prot(libc::PROT_READ);
    ///
    /// // share fd with another process here and the other process can only map with PROT_READ.
    /// # }
    /// ```
    #[doc(alias = "ASharedMemory_setProt")]
    pub fn set_prot(&self, prot: i32) -> Result<()> {
        let status = unsafe { ffi::ASharedMemory_setProt(self.as_raw_fd(), prot) };
        if status < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl AsFd for SharedMemory {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for SharedMemory {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl IntoRawFd for SharedMemory {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl FromRawFd for SharedMemory {
    /// # Safety
    ///
    /// The resource pointed to by `fd` must be open and suitable for assuming
    /// ownership. The resource must not require any cleanup other than `close`.
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self(OwnedFd::from_raw_fd(fd))
    }
}
