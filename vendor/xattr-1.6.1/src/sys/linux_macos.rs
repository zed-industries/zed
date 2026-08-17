use std::ffi::{OsStr, OsString};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::BorrowedFd;
use std::path::Path;

use rustix::fs as rfs;

#[cfg(not(target_os = "macos"))]
pub const ENOATTR: i32 = rustix::io::Errno::NODATA.raw_os_error();

#[cfg(target_os = "macos")]
pub const ENOATTR: i32 = rustix::io::Errno::NOATTR.raw_os_error();

pub const ERANGE: i32 = rustix::io::Errno::RANGE.raw_os_error();

/// An iterator over a set of extended attributes names.
#[derive(Default, Clone)]
pub struct XAttrs {
    data: Box<[u8]>,
    offset: usize,
}

// Yes, I could avoid these allocations on linux/macos. However, if we ever want to be freebsd
// compatible, we need to be able to prepend the namespace to the extended attribute names.
// Furthermore, borrowing makes the API messy.
impl Iterator for XAttrs {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        let data = &self.data[self.offset..];
        if data.is_empty() {
            None
        } else {
            // always null terminated (unless empty).
            let end = data.iter().position(|&b| b == 0u8).unwrap();
            self.offset += end + 1;
            Some(OsStr::from_bytes(&data[..end]).to_owned())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.data.len() == self.offset {
            (0, Some(0))
        } else {
            (1, None)
        }
    }
}

/// A macro to abstract away some of the boilerplate when calling `allocate_loop` with rustix
/// functions. Unfortunately, we can't write this as a helper function because I need to call
/// generic rustix function with two different types.
macro_rules! allocate_loop {
    (|$buf:ident| $($e:tt)*) => {
        crate::util::allocate_loop(
            |$buf| Ok($($e)*?.0),
            || {
                let $buf: &mut [u8] = &mut [];
                Ok($($e)*?)
            },
        )
    };
}

pub fn get_fd(fd: BorrowedFd<'_>, name: &OsStr) -> io::Result<Vec<u8>> {
    allocate_loop!(|buf| rfs::fgetxattr(fd, name, buf))
}

pub fn set_fd(fd: BorrowedFd<'_>, name: &OsStr, value: &[u8]) -> io::Result<()> {
    rfs::fsetxattr(fd, name, value, rfs::XattrFlags::empty())?;
    Ok(())
}

pub fn remove_fd(fd: BorrowedFd<'_>, name: &OsStr) -> io::Result<()> {
    rfs::fremovexattr(fd, name)?;
    Ok(())
}

pub fn list_fd(fd: BorrowedFd<'_>) -> io::Result<XAttrs> {
    let vec = allocate_loop!(|buf| rfs::flistxattr(fd, buf))?;
    Ok(XAttrs {
        data: vec.into_boxed_slice(),
        offset: 0,
    })
}

pub fn get_path(path: &Path, name: &OsStr, deref: bool) -> io::Result<Vec<u8>> {
    if deref {
        allocate_loop!(|buf| rfs::getxattr(path, name, buf))
    } else {
        allocate_loop!(|buf| rfs::lgetxattr(path, name, buf))
    }
}

pub fn set_path(path: &Path, name: &OsStr, value: &[u8], deref: bool) -> io::Result<()> {
    let setxattr_func = if deref { rfs::setxattr } else { rfs::lsetxattr };
    setxattr_func(path, name, value, rfs::XattrFlags::empty())?;
    Ok(())
}

pub fn remove_path(path: &Path, name: &OsStr, deref: bool) -> io::Result<()> {
    if deref {
        rfs::removexattr(path, name)
    } else {
        rfs::lremovexattr(path, name)
    }?;
    Ok(())
}

pub fn list_path(path: &Path, deref: bool) -> io::Result<XAttrs> {
    let vec = if deref {
        allocate_loop!(|buf| rfs::listxattr(path, buf))
    } else {
        allocate_loop!(|buf| rfs::llistxattr(path, buf))
    }?;
    Ok(XAttrs {
        data: vec.into_boxed_slice(),
        offset: 0,
    })
}
