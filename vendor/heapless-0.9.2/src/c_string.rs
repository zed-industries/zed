//! A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).

use crate::{vec::Vec, CapacityError, LenType};
use core::{
    borrow::Borrow,
    cmp::Ordering,
    error::Error,
    ffi::{c_char, CStr, FromBytesWithNulError},
    fmt,
    ops::Deref,
};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).
///
/// It stores up to `N - 1` non-nul characters with a trailing nul terminator.
#[derive(Clone, Hash)]
pub struct CString<const N: usize, LenT: LenType = usize> {
    inner: Vec<u8, N, LenT>,
}

#[cfg(feature = "zeroize")]
impl<const N: usize, LenT: LenType> Zeroize for CString<N, LenT> {
    fn zeroize(&mut self) {
        self.inner.zeroize();

        const {
            assert!(N > 0);
        }

        // SAFETY: We just asserted that `N > 0`.
        unsafe { self.inner.push_unchecked(b'\0') };
    }
}

impl<const N: usize, LenT: LenType> CString<N, LenT> {
    /// Creates a new C-compatible string with a terminating nul byte.
    ///
    /// ```rust
    /// use heapless::CString;
    ///
    /// // A fixed-size `CString` that can store up to 10 characters
    /// // including the nul terminator.
    /// let empty = CString::<10>::new();
    ///
    /// assert_eq!(empty.as_c_str(), c"");
    /// assert_eq!(empty.to_str(), Ok(""));
    /// ```
    pub fn new() -> Self {
        const {
            assert!(N > 0);
        }

        let mut inner = Vec::new();

        // SAFETY: We just asserted that `N > 0`.
        unsafe { inner.push_unchecked(b'\0') };

        Self { inner }
    }

    /// Unsafely creates a [`CString`] from a byte slice.
    ///
    /// This function will copy the provided `bytes` to a [`CString`] without
    /// performing any sanity checks.
    ///
    /// The function will fail if `bytes.len() > N`.
    ///
    /// # Safety
    ///
    /// The provided slice **must** be nul-terminated and not contain any interior
    /// nul bytes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use heapless::CString;
    /// let mut c_string = unsafe { CString::<7>::from_bytes_with_nul_unchecked(b"string\0").unwrap() };
    ///
    /// assert_eq!(c_string.to_str(), Ok("string"));
    /// ```
    pub unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> Result<Self, CapacityError> {
        let mut inner = Vec::new();

        inner.extend_from_slice(bytes)?;

        Ok(Self { inner })
    }

    /// Instantiates a [`CString`] copying from the giving byte slice, assuming it is
    /// nul-terminated.
    ///
    /// Fails if the given byte slice has any interior nul byte, if the slice does not
    /// end with a nul byte, or if the byte slice can't fit in `N`.
    pub fn from_bytes_with_nul(bytes: &[u8]) -> Result<Self, ExtendError> {
        let mut string = Self::new();

        string.extend_from_bytes(bytes)?;

        Ok(string)
    }

    /// Builds a [`CString`] copying from a raw C string pointer.
    ///
    /// # Safety
    ///
    /// - The memory pointed to by `ptr` must contain a valid nul terminator at the end of the
    ///   string.
    /// - `ptr` must be valid for reads of bytes up to and including the nul terminator. This means
    ///   in particular:
    ///     - The entire memory range of this `CStr` must be contained within a single allocated
    ///       object!
    ///     - `ptr` must be non-nul even for a zero-length `CStr`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use core::ffi::{c_char, CStr};
    /// use heapless::CString;
    ///
    /// const HELLO_PTR: *const c_char = {
    ///     const BYTES: &[u8] = b"Hello, world!\0";
    ///     BYTES.as_ptr().cast()
    /// };
    ///
    /// let copied = unsafe { CString::<14>::from_raw(HELLO_PTR) }.unwrap();
    ///
    /// assert_eq!(copied.to_str(), Ok("Hello, world!"));
    /// ```
    pub unsafe fn from_raw(ptr: *const c_char) -> Result<Self, ExtendError> {
        // SAFETY: The given pointer to a string is assumed to be nul-terminated.
        Self::from_bytes_with_nul(unsafe { CStr::from_ptr(ptr).to_bytes_with_nul() })
    }

    /// Converts the [`CString`] to a [`CStr`] slice.
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(&self.inner) }
    }

    /// Calculates the length of `self.inner` would have if it appended `bytes`.
    fn capacity_with_bytes(&self, bytes: &[u8]) -> Option<usize> {
        match bytes.last() {
            None => None,
            Some(0) if bytes.len() < 2 => None,
            Some(0) => {
                // `bytes` is nul-terminated and so is `self.inner`.
                // Adding up both would account for 2 nul bytes when only a single byte
                // would end up in the resulting CString.
                Some(self.inner.len() + bytes.len() - 1)
            }
            Some(_) => {
                // No terminating nul byte in `bytes` but there's one in
                // `self.inner`, so the math lines up nicely.
                //
                // In the case that `bytes` has a nul byte anywhere else, we would
                // error after `memchr` is called. So there's no problem.
                Some(self.inner.len() + bytes.len())
            }
        }
    }

    /// Extends the [`CString`] with the given bytes.
    ///
    /// This function fails if the [`CString`] would not have enough capacity to append the bytes or
    /// if the bytes contain an interior nul byte.
    ///
    /// # Example
    ///
    /// ```rust
    /// use heapless::CString;
    ///
    /// let mut c_string = CString::<10>::new();
    ///
    /// c_string.extend_from_bytes(b"hey").unwrap();
    /// c_string.extend_from_bytes(b" there\0").unwrap();
    ///
    /// assert_eq!(c_string.to_str(), Ok("hey there"));
    /// ```
    pub fn extend_from_bytes(&mut self, bytes: &[u8]) -> Result<(), ExtendError> {
        let Some(capacity) = self.capacity_with_bytes(bytes) else {
            return Ok(());
        };

        if capacity > N {
            // Cannot store these bytes due to an insufficient capacity.
            return Err(CapacityError.into());
        }

        match CStr::from_bytes_with_nul(bytes) {
            Ok(_) => {
                // SAFETY: A string is left in a valid state because appended bytes are
                // nul-terminated.
                unsafe { self.extend_from_bytes_unchecked(bytes) }?;

                Ok(())
            }
            Err(FromBytesWithNulError::InteriorNul { position }) => {
                Err(ExtendError::InteriorNul { position })
            }
            Err(FromBytesWithNulError::NotNulTerminated) => {
                // Because given bytes has no nul byte anywhere, we insert the bytes and
                // then add the nul byte terminator.
                //
                // We've ensured above that we have enough space left to insert these bytes,
                // so the operations below must succeed.
                //
                // SAFETY: We append a missing nul terminator right below.
                unsafe {
                    self.extend_from_bytes_unchecked(bytes).unwrap();
                    self.inner.push_unchecked(0);
                };

                Ok(())
            }
        }
    }

    /// Removes the nul byte terminator from the inner buffer.
    ///
    /// # Safety
    ///
    /// Callers must ensure to add the nul terminator back after this function is called.
    #[inline]
    unsafe fn pop_terminator(&mut self) {
        debug_assert_eq!(self.inner.last(), Some(&0));

        // SAFETY: We always have the nul terminator at the end.
        unsafe { self.inner.pop_unchecked() };
    }

    /// Removes the existing nul terminator and then extends `self` with the given bytes.
    ///
    /// # Safety
    ///
    /// If `additional` is not nul-terminated, the [`CString`] is left non nul-terminated, which is
    /// an invalid state. Caller must ensure that either `additional` has a terminating nul byte
    /// or ensure to append a trailing nul terminator.
    unsafe fn extend_from_bytes_unchecked(
        &mut self,
        additional: &[u8],
    ) -> Result<(), CapacityError> {
        // SAFETY: A caller is responsible for adding a nul terminator back to the inner buffer.
        unsafe { self.pop_terminator() }

        self.inner.extend_from_slice(additional)
    }

    /// Returns the underlying byte slice including the trailing nul terminator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use heapless::CString;
    ///
    /// let mut c_string = CString::<5>::new();
    /// c_string.extend_from_bytes(b"abc").unwrap();
    ///
    /// assert_eq!(c_string.as_bytes_with_nul(), b"abc\0");
    /// ```
    #[inline]
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        &self.inner
    }

    /// Returns the underlying byte slice excluding the trailing nul terminator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use heapless::CString;
    ///
    /// let mut c_string = CString::<5>::new();
    /// c_string.extend_from_bytes(b"abc").unwrap();
    ///
    /// assert_eq!(c_string.as_bytes(), b"abc");
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.inner[..self.inner.len() - 1]
    }
}

impl<const N: usize, LenT: LenType> AsRef<CStr> for CString<N, LenT> {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl<const N: usize, LenT: LenType> Borrow<CStr> for CString<N, LenT> {
    #[inline]
    fn borrow(&self) -> &CStr {
        self.as_c_str()
    }
}

impl<const N: usize, LenT: LenType> Default for CString<N, LenT> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, LenT: LenType> Deref for CString<N, LenT> {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_c_str()
    }
}

impl<const N: usize, const M: usize, LenT1: LenType, LenT2: LenType> PartialEq<CString<M, LenT2>>
    for CString<N, LenT1>
{
    #[inline]
    fn eq(&self, rhs: &CString<M, LenT2>) -> bool {
        self.as_c_str() == rhs.as_c_str()
    }
}

impl<const N: usize, LenT: LenType> Eq for CString<N, LenT> {}

impl<const N: usize, const M: usize, LenT1: LenType, LenT2: LenType> PartialOrd<CString<M, LenT2>>
    for CString<N, LenT1>
{
    #[inline]
    fn partial_cmp(&self, rhs: &CString<M, LenT2>) -> Option<Ordering> {
        self.as_c_str().partial_cmp(rhs.as_c_str())
    }
}

impl<const N: usize, LenT: LenType> Ord for CString<N, LenT> {
    #[inline]
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.as_c_str().cmp(rhs.as_c_str())
    }
}

impl<const N: usize, LenT: LenType> fmt::Debug for CString<N, LenT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_c_str().fmt(f)
    }
}

/// An error to extend [`CString`] with bytes.
#[derive(Debug)]
pub enum ExtendError {
    /// The capacity of the [`CString`] is too small.
    Capacity(CapacityError),
    /// An invalid interior nul byte found in a given byte slice.
    InteriorNul {
        /// A position of a nul byte.
        position: usize,
    },
}

impl Error for ExtendError {}

impl fmt::Display for ExtendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Capacity(error) => write!(f, "{error}"),
            Self::InteriorNul { position } => write!(f, "interior nul byte at {position}"),
        }
    }
}

impl From<CapacityError> for ExtendError {
    fn from(error: CapacityError) -> Self {
        Self::Capacity(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let empty = CString::<1>::new();

        assert_eq!(empty.as_c_str(), c"");
        assert_eq!(empty.as_bytes(), &[]);
        assert_eq!(empty.to_str(), Ok(""));
    }

    #[test]
    fn create_with_capacity_error() {
        assert!(CString::<1>::from_bytes_with_nul(b"a\0").is_err());
    }

    #[test]
    fn extend_no_byte() {
        let mut c_string = CString::<1>::new();

        c_string.extend_from_bytes(b"").unwrap();
    }

    #[test]
    fn extend_from_bytes() {
        let mut c_string = CString::<11>::new();
        assert_eq!(c_string.to_str(), Ok(""));

        c_string.extend_from_bytes(b"hello").unwrap();

        assert_eq!(c_string.to_str(), Ok("hello"));

        // Call must fail since `w\0rld` contains an interior nul byte.
        assert!(matches!(
            c_string.extend_from_bytes(b"w\0rld"),
            Err(ExtendError::InteriorNul { position: 1 })
        ));

        // However, the call above _must not_ have invalidated the state of our CString
        assert_eq!(c_string.to_str(), Ok("hello"));

        // Call must fail since we can't store "hello world\0" in 11 bytes
        assert!(matches!(
            c_string.extend_from_bytes(b" world"),
            Err(ExtendError::Capacity(CapacityError))
        ));

        // Yet again, the call above must not have invalidated the state of our CString
        // (as it would e.g. if we pushed the bytes but then failed to push the nul terminator)
        assert_eq!(c_string.to_str(), Ok("hello"));

        c_string.extend_from_bytes(b" Bill").unwrap();

        assert_eq!(c_string.to_str(), Ok("hello Bill"));
    }

    #[test]
    fn calculate_capacity_with_additional_bytes() {
        const INITIAL_BYTES: &[u8] = b"abc";

        let mut c_string = CString::<5>::new();

        c_string.extend_from_bytes(INITIAL_BYTES).unwrap();

        assert_eq!(c_string.to_bytes_with_nul().len(), 4);
        assert_eq!(c_string.capacity_with_bytes(b""), None);
        assert_eq!(c_string.capacity_with_bytes(b"\0"), None);
        assert_eq!(
            c_string.capacity_with_bytes(b"d"),
            Some(INITIAL_BYTES.len() + 2)
        );
        assert_eq!(
            c_string.capacity_with_bytes(b"d\0"),
            Some(INITIAL_BYTES.len() + 2)
        );
        assert_eq!(
            c_string.capacity_with_bytes(b"defg"),
            Some(INITIAL_BYTES.len() + 5)
        );
        assert_eq!(
            c_string.capacity_with_bytes(b"defg\0"),
            Some(INITIAL_BYTES.len() + 5)
        );
    }
    #[test]
    fn default() {
        assert_eq!(CString::<1>::default().as_c_str(), c"");
    }

    #[test]
    fn deref() {
        assert_eq!(CString::<1>::new().deref(), c"");
        assert_eq!(CString::<2>::new().deref(), c"");
        assert_eq!(CString::<3>::new().deref(), c"");

        let mut string = CString::<2>::new();
        string.extend_from_bytes(&[65]).unwrap();

        assert_eq!(string.deref(), c"A");

        let mut string = CString::<3>::new();
        string.extend_from_bytes(&[65, 66]).unwrap();

        assert_eq!(string.deref(), c"AB");

        let mut string = CString::<4>::new();
        string.extend_from_bytes(&[65, 66, 67]).unwrap();

        assert_eq!(string.deref(), c"ABC");
    }

    #[test]
    fn as_ref() {
        let mut string = CString::<4>::new();
        string.extend_from_bytes(b"foo").unwrap();
        assert_eq!(string.as_ref(), c"foo");
    }

    #[test]
    fn borrow() {
        let mut string = CString::<4>::new();
        string.extend_from_bytes(b"foo").unwrap();
        assert_eq!(Borrow::<CStr>::borrow(&string), c"foo");
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_cstring_zeroize() {
        use zeroize::Zeroize;

        let mut c_string = CString::<32>::from_bytes_with_nul(b"sensitive_password\0").unwrap();

        assert_eq!(c_string.to_str(), Ok("sensitive_password"));
        assert!(!c_string.to_bytes().is_empty());
        let original_length = c_string.to_bytes().len();
        assert_eq!(original_length, 18);

        let new_string = CString::<32>::from_bytes_with_nul(b"short\0").unwrap();
        c_string = new_string;

        assert_eq!(c_string.to_str(), Ok("short"));
        assert_eq!(c_string.to_bytes().len(), 5);

        // zeroized using Vec's implementation
        c_string.zeroize();

        assert_eq!(c_string.to_bytes().len(), 0);
        assert_eq!(c_string.to_bytes_with_nul(), &[0]);

        c_string.extend_from_bytes(b"new_data").unwrap();
        assert_eq!(c_string.to_str(), Ok("new_data"));
        assert_eq!(c_string.to_bytes().len(), 8);
    }

    mod equality {
        use super::*;

        #[test]
        fn c_string() {
            // Empty strings
            assert!(CString::<1>::new() == CString::<1>::new());
            assert!(CString::<1>::new() == CString::<2>::new());
            assert!(CString::<1>::from_bytes_with_nul(b"\0").unwrap() == CString::<3>::new());

            // Single character
            assert!(
                CString::<2>::from_bytes_with_nul(b"a\0").unwrap()
                    == CString::<2>::from_bytes_with_nul(b"a\0").unwrap()
            );
            assert!(
                CString::<2>::from_bytes_with_nul(b"a\0").unwrap()
                    == CString::<3>::from_bytes_with_nul(b"a\0").unwrap()
            );
            assert!(
                CString::<2>::from_bytes_with_nul(b"a\0").unwrap()
                    != CString::<2>::from_bytes_with_nul(b"b\0").unwrap()
            );

            // Multiple characters
            assert!(
                CString::<4>::from_bytes_with_nul(b"abc\0").unwrap()
                    == CString::<4>::from_bytes_with_nul(b"abc\0").unwrap()
            );
            assert!(
                CString::<3>::from_bytes_with_nul(b"ab\0").unwrap()
                    != CString::<4>::from_bytes_with_nul(b"abc\0").unwrap()
            );
        }
    }

    mod ordering {
        use super::*;

        #[test]
        fn c_string() {
            assert_eq!(
                CString::<1>::new().partial_cmp(&CString::<1>::new()),
                Some(Ordering::Equal)
            );
            assert_eq!(
                CString::<2>::from_bytes_with_nul(b"a\0")
                    .unwrap()
                    .partial_cmp(&CString::<2>::from_bytes_with_nul(b"b\0").unwrap()),
                Some(Ordering::Less)
            );
            assert_eq!(
                CString::<2>::from_bytes_with_nul(b"b\0")
                    .unwrap()
                    .partial_cmp(&CString::<2>::from_bytes_with_nul(b"a\0").unwrap()),
                Some(Ordering::Greater)
            );
        }

        #[test]
        fn c_str() {
            assert_eq!(c"".partial_cmp(&CString::<1>::new()), Some(Ordering::Equal));
            assert_eq!(
                c"a".partial_cmp(&CString::<2>::from_bytes_with_nul(b"b\0").unwrap()),
                Some(Ordering::Less)
            );
            assert_eq!(
                c"b".partial_cmp(&CString::<2>::from_bytes_with_nul(b"a\0").unwrap()),
                Some(Ordering::Greater)
            );
        }
    }
}
