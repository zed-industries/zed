// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! # Examples
//!
//! ```
//! use glib::prelude::*; // or `use gtk::prelude::*;`
//! use glib::ByteArray;
//!
//! let ba = ByteArray::from(b"abc");
//! assert_eq!(ba, "abc".as_bytes());
//! ```

use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    slice,
};

use crate::{ffi, translate::*};

wrapper! {
    #[doc(alias = "GByteArray")]
    pub struct ByteArray(Shared<ffi::GByteArray>);

    match fn {
        ref => |ptr| ffi::g_byte_array_ref(ptr),
        unref => |ptr| ffi::g_byte_array_unref(ptr),
        type_ => || ffi::g_byte_array_get_type(),
    }
}

impl Deref for ByteArray {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        unsafe {
            let self_ptr: *const ffi::GByteArray = self.to_glib_none().0;
            let ptr = (*self_ptr).data;
            let len = (*self_ptr).len as usize;
            debug_assert!(!ptr.is_null() || len == 0);
            if ptr.is_null() {
                &[]
            } else {
                slice::from_raw_parts(ptr as *const u8, len)
            }
        }
    }
}

impl AsRef<[u8]> for ByteArray {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl<'a, T: ?Sized + Borrow<[u8]> + 'a> From<&'a T> for ByteArray {
    fn from(value: &'a T) -> ByteArray {
        let value = value.borrow();
        unsafe {
            let ba = ffi::g_byte_array_new();
            ffi::g_byte_array_append(ba, value.as_ptr(), value.len() as u32);
            from_glib_full(ba)
        }
    }
}

impl fmt::Debug for ByteArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.as_ref()).finish()
    }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        #[allow(clippy::redundant_slicing)]
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                self[..].eq(&other[..])
            }
        }

        #[allow(clippy::redundant_slicing)]
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                self[..].eq(&other[..])
            }
        }

        #[allow(clippy::redundant_slicing)]
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                self[..].partial_cmp(&other[..])
            }
        }

        #[allow(clippy::redundant_slicing)]
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
                self[..].partial_cmp(&other[..])
            }
        }
    };
}

impl_cmp!(ByteArray, [u8]);
impl_cmp!(ByteArray, &'a [u8]);
impl_cmp!(&'a ByteArray, [u8]);
impl_cmp!(ByteArray, Vec<u8>);
impl_cmp!(&'a ByteArray, Vec<u8>);

impl PartialEq for ByteArray {
    fn eq(&self, other: &Self) -> bool {
        self[..] == other[..]
    }
}

impl Eq for ByteArray {}

impl Hash for ByteArray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash_slice(&self[..], state)
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn various() {
        let ba = ByteArray::from(b"foobar");
        assert_eq!(ba, b"foobar" as &[u8]);
    }

    #[test]
    fn hash() {
        let b1 = ByteArray::from(b"this is a test");
        let b2 = ByteArray::from(b"this is a test");
        let b3 = ByteArray::from(b"test");
        let mut set = HashSet::new();
        set.insert(b1);
        assert!(set.contains(&b2));
        assert!(!set.contains(&b3));
    }
}
