#![allow(clippy::comparison_chain)]
//! A pure-Rust library to manage extended attributes.
//!
//! It provides support for manipulating extended attributes
//! (`xattrs`) on modern Unix filesystems. See the `attr(5)`
//! manpage for more details.
//!
//! An extension trait [`FileExt`] is provided to directly work with
//! standard `File` objects and file descriptors.
//!
//! If the path argument is a symlink, the get/set/list/remove functions
//! operate on the symlink itself. To operate on the symlink target, use
//! the _deref variant of these functions.
//!
//! ```rust
//! let mut xattrs = xattr::list("/").unwrap().peekable();
//!
//! if xattrs.peek().is_none() {
//!     println!("no xattr set on root");
//!     return;
//! }
//!
//! println!("Extended attributes:");
//! for attr in xattrs {
//!     println!(" - {:?}", attr);
//! }
//! ```

mod error;
mod sys;
mod util;

use std::ffi::OsStr;
use std::fs::File;
use std::os::unix::io::{AsRawFd, BorrowedFd};
use std::path::Path;
use std::{fmt, io};

pub use error::UnsupportedPlatformError;
pub use sys::{XAttrs, SUPPORTED_PLATFORM};

/// Get an extended attribute for the specified file.
pub fn get<N, P>(path: P, name: N) -> io::Result<Option<Vec<u8>>>
where
    P: AsRef<Path>,
    N: AsRef<OsStr>,
{
    util::extract_noattr(sys::get_path(path.as_ref(), name.as_ref(), false))
}

/// Get an extended attribute for the specified file (dereference symlinks).
pub fn get_deref<N, P>(path: P, name: N) -> io::Result<Option<Vec<u8>>>
where
    P: AsRef<Path>,
    N: AsRef<OsStr>,
{
    util::extract_noattr(sys::get_path(path.as_ref(), name.as_ref(), true))
}

/// Set an extended attribute on the specified file.
pub fn set<N, P>(path: P, name: N, value: &[u8]) -> io::Result<()>
where
    P: AsRef<Path>,
    N: AsRef<OsStr>,
{
    sys::set_path(path.as_ref(), name.as_ref(), value, false)
}

/// Set an extended attribute on the specified file (dereference symlinks).
pub fn set_deref<N, P>(path: P, name: N, value: &[u8]) -> io::Result<()>
where
    P: AsRef<Path>,
    N: AsRef<OsStr>,
{
    sys::set_path(path.as_ref(), name.as_ref(), value, true)
}

/// Remove an extended attribute from the specified file.
pub fn remove<N, P>(path: P, name: N) -> io::Result<()>
where
    P: AsRef<Path>,
    N: AsRef<OsStr>,
{
    sys::remove_path(path.as_ref(), name.as_ref(), false)
}

/// Remove an extended attribute from the specified file (dereference symlinks).
pub fn remove_deref<N, P>(path: P, name: N) -> io::Result<()>
where
    P: AsRef<Path>,
    N: AsRef<OsStr>,
{
    sys::remove_path(path.as_ref(), name.as_ref(), true)
}

/// List extended attributes attached to the specified file.
///
/// Note: this may not list *all* attributes. Speficially, it definitely won't list any trusted
/// attributes unless you are root and it may not list system attributes.
pub fn list<P>(path: P) -> io::Result<XAttrs>
where
    P: AsRef<Path>,
{
    sys::list_path(path.as_ref(), false)
}

/// List extended attributes attached to the specified file (dereference symlinks).
pub fn list_deref<P>(path: P) -> io::Result<XAttrs>
where
    P: AsRef<Path>,
{
    sys::list_path(path.as_ref(), true)
}

/// Extension trait to manipulate extended attributes on `File`-like objects.
pub trait FileExt: AsRawFd {
    /// Get an extended attribute for the specified file.
    fn get_xattr<N>(&self, name: N) -> io::Result<Option<Vec<u8>>>
    where
        N: AsRef<OsStr>,
    {
        // SAFETY: Implement I/O safety later.
        let fd = unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) };
        util::extract_noattr(sys::get_fd(fd, name.as_ref()))
    }

    /// Set an extended attribute on the specified file.
    fn set_xattr<N>(&self, name: N, value: &[u8]) -> io::Result<()>
    where
        N: AsRef<OsStr>,
    {
        let fd = unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) };
        sys::set_fd(fd, name.as_ref(), value)
    }

    /// Remove an extended attribute from the specified file.
    fn remove_xattr<N>(&self, name: N) -> io::Result<()>
    where
        N: AsRef<OsStr>,
    {
        let fd = unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) };
        sys::remove_fd(fd, name.as_ref())
    }

    /// List extended attributes attached to the specified file.
    ///
    /// Note: this may not list *all* attributes. Speficially, it definitely won't list any trusted
    /// attributes unless you are root and it may not list system attributes.
    fn list_xattr(&self) -> io::Result<XAttrs> {
        let fd = unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) };
        sys::list_fd(fd)
    }
}

impl FileExt for File {}

impl fmt::Debug for XAttrs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Waiting on https://github.com/rust-lang/rust/issues/117729 to stabilize...
        struct AsList<'a>(&'a XAttrs);
        impl<'a> fmt::Debug for AsList<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self.0.clone()).finish()
            }
        }
        f.debug_tuple("XAttrs").field(&AsList(self)).finish()
    }
}
