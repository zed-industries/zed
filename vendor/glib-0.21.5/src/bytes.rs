// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    mem,
    ops::{Bound, Deref, RangeBounds},
    slice,
};

use crate::{ffi, translate::*};

wrapper! {
    // rustdoc-stripper-ignore-next
    /// A shared immutable byte slice (the equivalent of `Rc<[u8]>`).
    ///
    /// `From` implementations that take references (e.g. `&[u8]`) copy the
    /// data. The `from_static` constructor avoids copying static data.
    ///
    /// ```
    /// use glib::Bytes;
    ///
    /// let v = vec![1, 2, 3];
    /// let b = Bytes::from(&v);
    /// assert_eq!(v, b);
    ///
    /// let s = b"xyz";
    /// let b = Bytes::from_static(s);
    /// assert_eq!(&s[..], b);
    /// ```
    #[doc(alias = "GBytes")]
    pub struct Bytes(Shared<ffi::GBytes>);

    match fn {
        ref => |ptr| ffi::g_bytes_ref(ptr),
        unref => |ptr| ffi::g_bytes_unref(ptr),
        type_ => || ffi::g_bytes_get_type(),
    }
}

impl Bytes {
    // rustdoc-stripper-ignore-next
    /// Copies `data` into a new shared slice.
    #[doc(alias = "g_bytes_new")]
    #[inline]
    fn new<T: AsRef<[u8]>>(data: T) -> Bytes {
        let data = data.as_ref();
        unsafe { from_glib_full(ffi::g_bytes_new(data.as_ptr() as *const _, data.len())) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a view into static `data` without copying.
    #[doc(alias = "g_bytes_new_static")]
    #[inline]
    pub fn from_static(data: &'static [u8]) -> Bytes {
        unsafe {
            from_glib_full(ffi::g_bytes_new_static(
                data.as_ptr() as *const _,
                data.len(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Takes ownership of `data` and creates a new `Bytes` without copying.
    #[doc(alias = "g_bytes_new")]
    pub fn from_owned<T: AsRef<[u8]> + Send + 'static>(data: T) -> Bytes {
        let data: Box<T> = Box::new(data);
        let (size, data_ptr) = {
            let data = (*data).as_ref();
            (data.len(), data.as_ptr())
        };

        unsafe extern "C" fn drop_box<T: AsRef<[u8]> + Send + 'static>(b: ffi::gpointer) {
            let _: Box<T> = Box::from_raw(b as *mut _);
        }

        unsafe {
            from_glib_full(ffi::g_bytes_new_with_free_func(
                data_ptr as *const _,
                size,
                Some(drop_box::<T>),
                Box::into_raw(data) as *mut _,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the underlying data of the `Bytes`.
    ///
    /// If there is no other reference to `self` then this does not copy the data, otherwise
    /// it is copied into newly allocated heap memory.
    #[doc(alias = "g_bytes_unref_to_data")]
    pub fn into_data(self) -> crate::collections::Slice<u8> {
        unsafe {
            let mut size = mem::MaybeUninit::uninit();
            let ret = ffi::g_bytes_unref_to_data(self.into_glib_ptr(), size.as_mut_ptr());
            crate::collections::Slice::from_glib_full_num(ret as *mut u8, size.assume_init())
        }
    }

    fn calculate_offset_size(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
        let len = self.len();

        let start_offset = match range.start_bound() {
            Bound::Included(v) => *v,
            Bound::Excluded(v) => v.checked_add(1).expect("Invalid start offset"),
            Bound::Unbounded => 0,
        };
        assert!(start_offset <= len, "Start offset after valid range");

        let end_offset = match range.end_bound() {
            Bound::Included(v) => v.checked_add(1).expect("Invalid end offset"),
            Bound::Excluded(v) => *v,
            Bound::Unbounded => len,
        };
        assert!(end_offset <= len, "End offset after valid range");

        let size = end_offset.saturating_sub(start_offset);

        (start_offset, size)
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new `Bytes` that references the given `range` of `bytes`.
    #[doc(alias = "g_bytes_new_from_bytes")]
    pub fn from_bytes(bytes: &Self, range: impl RangeBounds<usize>) -> Self {
        let (offset, size) = bytes.calculate_offset_size(range);
        unsafe {
            from_glib_full(ffi::g_bytes_new_from_bytes(
                bytes.to_glib_none().0,
                offset,
                size,
            ))
        }
    }
}

unsafe impl Send for Bytes {}
unsafe impl Sync for Bytes {}

impl<'a, T: ?Sized + Borrow<[u8]> + 'a> From<&'a T> for Bytes {
    #[inline]
    fn from(value: &'a T) -> Bytes {
        Bytes::new(value.borrow())
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Bytes")
            .field("ptr", &ToGlibPtr::<*const _>::to_glib_none(self).0)
            .field("data", &&self[..])
            .finish()
    }
}

impl AsRef<[u8]> for Bytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl Deref for Bytes {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = ffi::g_bytes_get_data(self.to_glib_none().0, &mut len);
            if ptr.is_null() || len == 0 {
                &[]
            } else {
                slice::from_raw_parts(ptr as *const u8, len)
            }
        }
    }
}

impl PartialEq for Bytes {
    #[doc(alias = "g_bytes_equal")]
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::g_bytes_equal(
                ToGlibPtr::<*const _>::to_glib_none(self).0 as *const _,
                ToGlibPtr::<*const _>::to_glib_none(other).0 as *const _,
            ))
        }
    }
}

impl Eq for Bytes {}

impl PartialOrd for Bytes {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bytes {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe {
            let ret = ffi::g_bytes_compare(
                ToGlibPtr::<*const _>::to_glib_none(self).0 as *const _,
                ToGlibPtr::<*const _>::to_glib_none(other).0 as *const _,
            );
            ret.cmp(&0)
        }
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

impl_cmp!(Bytes, [u8]);
impl_cmp!(Bytes, &'a [u8]);
impl_cmp!(&'a Bytes, [u8]);
impl_cmp!(Bytes, Vec<u8>);
impl_cmp!(&'a Bytes, Vec<u8>);

impl Hash for Bytes {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        Hash::hash_slice(self, state)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn eq() {
        let abc: &[u8] = b"abc";
        let def: &[u8] = b"def";
        let a1 = Bytes::from(abc);
        let a2 = Bytes::from(abc);
        let d = Bytes::from(def);
        assert_eq!(a1, a2);
        assert_eq!(def, d);
        assert_ne!(a1, d);
        assert_ne!(a1, def);
    }

    #[test]
    fn ord() {
        let abc: &[u8] = b"abc";
        let def: &[u8] = b"def";
        let a = Bytes::from(abc);
        let d = Bytes::from(def);
        assert!(a < d);
        assert!(a < def);
        assert!(abc < d);
        assert!(d > a);
        assert!(d > abc);
        assert!(def > a);
    }

    #[test]
    fn hash() {
        let b1 = Bytes::from(b"this is a test");
        let b2 = Bytes::from(b"this is a test");
        let b3 = Bytes::from(b"test");
        let mut set = HashSet::new();
        set.insert(b1);
        assert!(set.contains(&b2));
        assert!(!set.contains(&b3));
    }

    #[test]
    fn from_static() {
        let b1 = Bytes::from_static(b"this is a test");
        let b2 = Bytes::from(b"this is a test");
        assert_eq!(b1, b2);
    }

    #[test]
    fn from_owned() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        assert_eq!(b, [1u8, 2u8, 3u8].as_ref());
    }

    #[test]
    fn from_bytes() {
        let b1 = Bytes::from_owned(vec![1, 2, 3]);
        let b2 = Bytes::from_bytes(&b1, 1..=1);
        assert_eq!(b2, [2u8].as_ref());
        let b2 = Bytes::from_bytes(&b1, 1..);
        assert_eq!(b2, [2u8, 3u8].as_ref());
        let b2 = Bytes::from_bytes(&b1, ..2);
        assert_eq!(b2, [1u8, 2u8].as_ref());
        let b2 = Bytes::from_bytes(&b1, ..);
        assert_eq!(b2, [1u8, 2u8, 3u8].as_ref());
        let b2 = Bytes::from_bytes(&b1, 3..);
        assert_eq!(b2, [].as_ref());
    }

    #[test]
    pub fn into_data() {
        let b = Bytes::from(b"this is a test");
        let d = b.into_data();
        assert_eq!(d.as_slice(), b"this is a test");
    }
}
