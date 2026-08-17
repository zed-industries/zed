//! A specialized byte slice type for performing vectored I/O operations.
//!
//! For more detail, see [`IoVec`] documentation.
//!
//! [`IoVec`]: struct.IoVec.html

#[cfg(unix)]
extern crate libc;

mod sys;

use std::{ops, mem};

#[cfg(unix)]
pub mod unix;

/// Max length of an `IoVec` slice.
///
/// Attempts to convert slices longer than this value will result in a panic.
pub const MAX_LENGTH: usize = sys::MAX_LENGTH;

/// A specialized byte slice type for performing vectored I/O operations.
///
/// On all systems, the types needed to perform vectored I/O systems have the
/// same size as Rust's [`slice`]. However, the layout is not necessarily the
/// same. `IoVec` provides a portable compatibility layer.
///
/// The `IoVec` behaves like a Rust [`slice`], providing the same functions.
/// It also provides conversion functions to and from the OS specific vectored
/// types.
///
/// [`slice`]: https://doc.rust-lang.org/std/primitive.slice.html
///
/// # Examples
///
/// ```
/// use iovec::IoVec;
///
/// let mut data = vec![];
/// data.extend_from_slice(b"hello");
///
/// let iovec: &IoVec = data.as_slice().into();
///
/// assert_eq!(&iovec[..], &b"hello"[..]);
/// ```
///
/// # Panics
///
/// Attempting to convert a zero-length slice or a slice longer than
/// [`MAX_LENGTH`] to an `IoVec` will result in a panic.
///
/// [`MAX_LENGTH`]: constant.MAX_LENGTH.html
pub struct IoVec {
    sys: sys::IoVec,
}

impl IoVec {
    pub fn from_bytes(slice: &[u8]) -> Option<&IoVec> {
        if slice.len() == 0 {
            return None
        }
        unsafe {
            let iovec: &sys::IoVec = slice.into();
            Some(mem::transmute(iovec))
        }
    }

    pub fn from_bytes_mut(slice: &mut [u8]) -> Option<&mut IoVec> {
        if slice.len() == 0 {
            return None
        }
        unsafe {
            let iovec: &mut sys::IoVec = slice.into();
            Some(mem::transmute(iovec))
        }
    }

    #[deprecated(since = "0.1.0", note = "deref instead")]
    #[doc(hidden)]
    pub fn as_bytes(&self) -> &[u8] {
        &**self
    }

    #[deprecated(since = "0.1.0", note = "deref instead")]
    #[doc(hidden)]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut **self
    }
}

impl ops::Deref for IoVec {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.sys.as_ref()
    }
}

impl ops::DerefMut for IoVec {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.sys.as_mut()
    }
}

#[doc(hidden)]
impl<'a> From<&'a [u8]> for &'a IoVec {
    fn from(bytes: &'a [u8]) -> &'a IoVec {
        IoVec::from_bytes(bytes)
            .expect("this crate accidentally accepted 0-sized slices \
                     originally but this was since discovered as a soundness \
                     hole, it's recommended to use the `from_bytes` \
                     function instead")
    }
}

#[doc(hidden)]
impl<'a> From<&'a mut [u8]> for &'a mut IoVec {
    fn from(bytes: &'a mut [u8]) -> &'a mut IoVec {
        IoVec::from_bytes_mut(bytes)
            .expect("this crate accidentally accepted 0-sized slices \
                     originally but this was since discovered as a soundness \
                     hole, it's recommended to use the `from_bytes_mut` \
                     function instead")
    }
}

#[doc(hidden)]
impl<'a> Default for &'a IoVec {
    fn default() -> Self {
        panic!("this implementation was accidentally provided but is \
                unfortunately unsound, it's recommended to stop using \
                `IoVec::default` or construct a vector with a nonzero length");
    }
}

#[doc(hidden)]
impl<'a> Default for &'a mut IoVec {
    fn default() -> Self {
        panic!("this implementation was accidentally provided but is \
                unfortunately unsound, it's recommended to stop using \
                `IoVec::default` or construct a vector with a nonzero length");
    }
}

#[cfg(test)]
mod test {
    use super::IoVec;

    #[test]
    fn convert_ref() {
        let buf: &IoVec = (&b"hello world"[..]).into();
        assert_eq!(buf[..], b"hello world"[..]);
    }

    #[test]
    fn convert_mut() {
        let mut buf: Vec<u8> = b"hello world".to_vec();
        let buf: &mut IoVec = (&mut buf[..]).into();
        assert_eq!(buf[..], b"hello world"[..]);
    }
}
