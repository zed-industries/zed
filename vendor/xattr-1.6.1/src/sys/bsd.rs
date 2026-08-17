//! FreeBSD and NetBSD xattr support.

use libc::{c_int, c_void, size_t, EPERM};
use std::ffi::{CString, OsStr, OsString};
use std::mem::MaybeUninit;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::io::{AsRawFd, BorrowedFd};
use std::path::Path;
use std::ptr;
use std::{io, slice};

use libc::{
    extattr_delete_fd, extattr_delete_file, extattr_delete_link, extattr_get_fd, extattr_get_file,
    extattr_get_link, extattr_list_fd, extattr_list_file, extattr_list_link, extattr_set_fd,
    extattr_set_file, extattr_set_link, EXTATTR_NAMESPACE_SYSTEM, EXTATTR_NAMESPACE_USER,
};

pub const ENOATTR: i32 = libc::ENOATTR;
pub const ERANGE: i32 = libc::ERANGE;

const EXTATTR_NAMESPACE_USER_STRING: &str = "user";
const EXTATTR_NAMESPACE_SYSTEM_STRING: &str = "system";
const EXTATTR_NAMESPACE_NAMES: [&str; 3] = [
    "empty",
    EXTATTR_NAMESPACE_USER_STRING,
    EXTATTR_NAMESPACE_SYSTEM_STRING,
];

fn path_to_c(path: &Path) -> io::Result<CString> {
    match CString::new(path.as_os_str().as_bytes()) {
        Ok(name) => Ok(name),
        Err(_) => Err(io::Error::new(io::ErrorKind::NotFound, "file not found")),
    }
}

#[inline]
fn cvt(res: libc::ssize_t) -> io::Result<usize> {
    if res < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res as usize)
    }
}

#[inline]
fn slice_parts(buf: &mut [MaybeUninit<u8>]) -> (*mut c_void, size_t) {
    if buf.is_empty() {
        (ptr::null_mut(), 0)
    } else {
        (buf.as_mut_ptr().cast(), buf.len() as size_t)
    }
}

fn allocate_loop<F: Fn(*mut c_void, size_t) -> libc::ssize_t>(f: F) -> io::Result<Vec<u8>> {
    crate::util::allocate_loop(
        |buf| unsafe {
            let (ptr, len) = slice_parts(buf);
            let new_len = cvt(f(ptr, len))?;
            if new_len < len {
                Ok(slice::from_raw_parts_mut(ptr.cast(), new_len))
            } else {
                // If the length of the value isn't strictly smaller than the length of the value
                // read, there may be more to read. Fake an ERANGE error so we can try again with a
                // bigger buffer.
                Err(io::Error::from_raw_os_error(crate::sys::ERANGE))
            }
        },
        // Estimate size + 1 because, on freebsd, the only way to tell if we've read the entire
        // value is read a value smaller than the buffer we passed.
        || Ok(cvt(f(ptr::null_mut(), 0))? + 1),
    )
}

/// An iterator over a set of extended attributes names.
#[derive(Default, Clone)]
pub struct XAttrs {
    user_attrs: Box<[u8]>,
    system_attrs: Box<[u8]>,
    offset: usize,
}

impl Iterator for XAttrs {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        if self.user_attrs.is_empty() && self.system_attrs.is_empty() {
            return None;
        }

        if self.offset == self.user_attrs.len() + self.system_attrs.len() {
            return None;
        }

        let data = if self.offset < self.system_attrs.len() {
            &self.system_attrs[self.offset..]
        } else {
            &self.user_attrs[self.offset - self.system_attrs.len()..]
        };

        let siz = data[0] as usize;

        self.offset += siz + 1;
        if self.offset < self.system_attrs.len() {
            Some(prefix_namespace(
                OsStr::from_bytes(&data[1..siz + 1]),
                EXTATTR_NAMESPACE_SYSTEM,
            ))
        } else {
            Some(prefix_namespace(
                OsStr::from_bytes(&data[1..siz + 1]),
                EXTATTR_NAMESPACE_USER,
            ))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.user_attrs.len() + self.system_attrs.len() == self.offset {
            (0, Some(0))
        } else {
            (1, None)
        }
    }
}

// This could use libc::extattr_string_to_namespace, but it's awkward because
// that requires nul-terminated strings, which Rust's std is loathe to provide.
fn name_to_ns(name: &OsStr) -> io::Result<(c_int, CString)> {
    let mut groups = name.as_bytes().splitn(2, |&b| b == b'.').take(2);
    let nsname = match groups.next() {
        Some(s) => s,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "couldn't find namespace",
            ))
        }
    };

    let propname = match groups.next() {
        Some(s) => s,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "couldn't find attribute",
            ))
        }
    };

    let ns_int = match EXTATTR_NAMESPACE_NAMES
        .iter()
        .position(|&s| s.as_bytes() == nsname)
    {
        Some(i) => i,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "no matching namespace",
            ))
        }
    };

    Ok((ns_int as c_int, CString::new(propname)?))
}

fn prefix_namespace(attr: &OsStr, ns: c_int) -> OsString {
    let nsname = EXTATTR_NAMESPACE_NAMES[ns as usize].as_bytes();
    let attr = attr.as_bytes();
    let mut v = Vec::with_capacity(nsname.len() + attr.len() + 1);
    v.extend_from_slice(nsname);
    v.extend_from_slice(b".");
    v.extend_from_slice(attr);
    OsString::from_vec(v)
}

pub fn get_fd(fd: BorrowedFd<'_>, name: &OsStr) -> io::Result<Vec<u8>> {
    let (ns, name) = name_to_ns(name)?;
    unsafe { allocate_loop(|ptr, len| extattr_get_fd(fd.as_raw_fd(), ns, name.as_ptr(), ptr, len)) }
}

pub fn set_fd(fd: BorrowedFd<'_>, name: &OsStr, value: &[u8]) -> io::Result<()> {
    let (ns, name) = name_to_ns(name)?;
    let ret = unsafe {
        extattr_set_fd(
            fd.as_raw_fd(),
            ns,
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            value.len() as size_t,
        )
    };
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn remove_fd(fd: BorrowedFd<'_>, name: &OsStr) -> io::Result<()> {
    let (ns, name) = name_to_ns(name)?;
    let ret = unsafe { extattr_delete_fd(fd.as_raw_fd(), ns, name.as_ptr()) };
    if ret != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn list_fd(fd: BorrowedFd<'_>) -> io::Result<XAttrs> {
    let sysvec = unsafe {
        let res = allocate_loop(|ptr, len| {
            extattr_list_fd(fd.as_raw_fd(), EXTATTR_NAMESPACE_SYSTEM, ptr, len)
        });
        // On FreeBSD, system attributes require root privileges to view. However,
        // to mimic the behavior of listxattr in linux and osx, we need to query
        // them anyway and return empty results if we get EPERM
        match res {
            Ok(v) => v,
            Err(err) => {
                if err.raw_os_error() == Some(EPERM) {
                    Vec::new()
                } else {
                    return Err(err);
                }
            }
        }
    };

    let uservec = unsafe {
        allocate_loop(|ptr, len| extattr_list_fd(fd.as_raw_fd(), EXTATTR_NAMESPACE_USER, ptr, len))?
    };

    Ok(XAttrs {
        system_attrs: sysvec.into_boxed_slice(),
        user_attrs: uservec.into_boxed_slice(),
        offset: 0,
    })
}

pub fn get_path(path: &Path, name: &OsStr, deref: bool) -> io::Result<Vec<u8>> {
    let (ns, name) = name_to_ns(name)?;
    let path = path_to_c(path)?;
    let extattr_get_func = if deref {
        extattr_get_file
    } else {
        extattr_get_link
    };
    unsafe {
        allocate_loop(|ptr, len| extattr_get_func(path.as_ptr(), ns, name.as_ptr(), ptr, len))
    }
}

pub fn set_path(path: &Path, name: &OsStr, value: &[u8], deref: bool) -> io::Result<()> {
    let (ns, name) = name_to_ns(name)?;
    let path = path_to_c(path)?;
    let extattr_set_func = if deref {
        extattr_set_file
    } else {
        extattr_set_link
    };
    let ret = unsafe {
        extattr_set_func(
            path.as_ptr(),
            ns,
            name.as_ptr(),
            value.as_ptr() as *const c_void,
            value.len() as size_t,
        )
    };
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn remove_path(path: &Path, name: &OsStr, deref: bool) -> io::Result<()> {
    let (ns, name) = name_to_ns(name)?;
    let path = path_to_c(path)?;
    let extattr_delete_func = if deref {
        extattr_delete_file
    } else {
        extattr_delete_link
    };
    let ret = unsafe { extattr_delete_func(path.as_ptr(), ns, name.as_ptr()) };
    if ret != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn list_path(path: &Path, deref: bool) -> io::Result<XAttrs> {
    let path = path_to_c(path)?;
    let extattr_list_func = if deref {
        extattr_list_file
    } else {
        extattr_list_link
    };
    let sysvec = unsafe {
        let res = allocate_loop(|ptr, len| {
            extattr_list_func(path.as_ptr(), EXTATTR_NAMESPACE_SYSTEM, ptr, len)
        });
        // On FreeBSD, system attributes require root privileges to view. However,
        // to mimic the behavior of listxattr in linux and osx, we need to query
        // them anyway and return empty results if we get EPERM
        match res {
            Ok(v) => v,
            Err(err) => {
                if err.raw_os_error() == Some(EPERM) {
                    Vec::new()
                } else {
                    return Err(err);
                }
            }
        }
    };

    let uservec = unsafe {
        allocate_loop(|ptr, len| {
            extattr_list_func(path.as_ptr(), EXTATTR_NAMESPACE_USER, ptr, len)
        })?
    };

    Ok(XAttrs {
        system_attrs: sysvec.into_boxed_slice(),
        user_attrs: uservec.into_boxed_slice(),
        offset: 0,
    })
}
