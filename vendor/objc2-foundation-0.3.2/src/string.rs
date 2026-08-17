#[cfg(feature = "NSObjCRuntime")]
use core::cmp;
use core::ffi::c_void;
use core::fmt;
use core::ops::AddAssign;
use core::panic::RefUnwindSafe;
use core::panic::UnwindSafe;
use core::str;

use objc2::msg_send;
use objc2::rc::{autoreleasepool_leaking, Allocated, AutoreleasePool, Retained};
use objc2::runtime::__nsstring::{nsstring_len, nsstring_to_str, UTF8_ENCODING};
use objc2::{AnyThread, Message};

use crate::util;
use crate::{NSMutableString, NSString};

// Even if an exception occurs inside a string method, the state of the string
// (should) still be perfectly safe to access.
impl UnwindSafe for NSString {}
impl RefUnwindSafe for NSString {}

impl NSString {
    /// The number of UTF-8 code units in `self`.
    #[doc(alias = "lengthOfBytesUsingEncoding")]
    #[doc(alias = "lengthOfBytesUsingEncoding:")]
    pub fn len(&self) -> usize {
        // SAFETY: This is an instance of `NSString`
        unsafe { nsstring_len(self) }
    }

    /// The number of UTF-16 code units in the string.
    ///
    /// See also [`NSString::len`].
    #[doc(alias = "length")]
    pub fn len_utf16(&self) -> usize {
        self.length()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: lengthOfBytesUsingEncoding: might sometimes return 0 for
        // other reasons, so this is not really correct!
        self.len() == 0
    }

    /// Convert the string into a [string slice](`prim@str`).
    ///
    /// The signature of this method can be a bit confusing, as it contains
    /// several lifetimes; the lifetime `'s` of the `NSString`, the lifetime
    /// `'p` of the current autorelease pool and the lifetime `'r` of the
    /// returned string slice.
    ///
    /// In general, this method converts the string to a newly allocated UTF-8
    /// string, autoreleases the buffer, and returns a slice pointer to this
    /// internal buffer, which will become invalid once the autorelease pool
    /// is popped. So the lifetime of the return value is bound to the current
    /// autorelease pool.
    ///
    /// However, as an optimization, this method may choose to instead return
    /// an internal reference to the `NSString` when it can, and when the
    /// string is immutable, and that is why the lifetime of the returned
    /// string slice is also bound to the string itself.
    ///
    /// You should prefer the [`to_string`] method or the
    /// [`Display` implementation][display-impl] over this method when
    /// possible.
    ///
    /// [`to_string`]: alloc::string::ToString::to_string
    /// [display-impl]: NSString#impl-Display-for-NSString
    ///
    ///
    /// # Safety
    ///
    /// The pool must be the innermost pool, see [the documentation on
    /// `autoreleasepool`][autoreleasepool].
    ///
    /// [autoreleasepool]: objc2::rc::autoreleasepool
    ///
    ///
    /// # Examples
    ///
    /// Get the string slice of the `NSString`, and compare it with another
    /// inside an autorelease pool.
    ///
    /// ```
    /// use objc2_foundation::NSString;
    /// use objc2::rc::autoreleasepool;
    ///
    /// let string = NSString::from_str("foo");
    /// autoreleasepool(|pool| {
    ///     // SAFETY: The str is not used outside the autorelease pool.
    ///     assert_eq!(unsafe { string.to_str(pool) }, "foo");
    /// });
    /// ```
    #[doc(alias = "UTF8String")]
    pub unsafe fn to_str<'r, 's: 'r, 'p: 'r>(&'s self, pool: AutoreleasePool<'p>) -> &'r str {
        // SAFETY: This is an instance of `NSString`.
        //
        // Caller upholds that the string is not moved outside the pool.
        unsafe { nsstring_to_str(self, pool) }
    }

    // TODO: Allow usecases where the NUL byte from `UTF8String` is kept?

    /// Creates an immutable `NSString` by copying the given string slice.
    ///
    /// Prefer using the [`ns_string!`] macro when possible.
    ///
    /// [`ns_string!`]: crate::ns_string
    #[doc(alias = "initWithBytes")]
    #[doc(alias = "initWithBytes:length:encoding:")]
    #[allow(clippy::should_implement_trait)] // Not really sure of a better name
    pub fn from_str(string: &str) -> Retained<Self> {
        unsafe { init_with_str(Self::alloc(), string) }
    }

    // TODO: `initWithBytesNoCopy:length:encoding:` from `&'static str`.
}

impl NSMutableString {
    /// Creates a new [`NSMutableString`] by copying the given string slice.
    #[doc(alias = "initWithBytes:length:encoding:")]
    #[allow(clippy::should_implement_trait)] // Not really sure of a better name
    pub fn from_str(string: &str) -> Retained<Self> {
        unsafe { init_with_str(Self::alloc(), string) }
    }
}

unsafe fn init_with_str<T: Message>(obj: Allocated<T>, string: &str) -> Retained<T> {
    let bytes: *const c_void = string.as_ptr().cast();
    // We use `msg_send!` instead of the generated method, since that
    // assumes the encoding is `usize`, whereas GNUStep assumes `i32`.
    unsafe {
        msg_send![
            obj,
            initWithBytes: bytes,
            length: string.len(),
            encoding: UTF8_ENCODING,
        ]
    }
}

impl PartialEq<NSString> for NSMutableString {
    #[inline]
    fn eq(&self, other: &NSString) -> bool {
        PartialEq::eq(&**self, other)
    }
}

impl PartialEq<NSMutableString> for NSString {
    #[inline]
    fn eq(&self, other: &NSMutableString) -> bool {
        PartialEq::eq(self, &**other)
    }
}

#[cfg(feature = "NSObjCRuntime")]
impl PartialOrd for NSString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "NSObjCRuntime")]
impl Ord for NSString {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).into()
    }
}

#[cfg(feature = "NSObjCRuntime")]
impl PartialOrd for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "NSObjCRuntime")]
impl PartialOrd<NSString> for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &NSString) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&**self, other)
    }
}

#[cfg(feature = "NSObjCRuntime")]
impl PartialOrd<NSMutableString> for NSString {
    #[inline]
    fn partial_cmp(&self, other: &NSMutableString) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(self, &**other)
    }
}

#[cfg(feature = "NSObjCRuntime")]
impl Ord for NSMutableString {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Ord::cmp(&**self, &**other)
    }
}

// TODO: PartialEq and PartialOrd against &str
// See `fruity`'s implementation:
// https://github.com/nvzqz/fruity/blob/320efcf715c2c5fbd2f3084f671f2be2e03a6f2b/src/foundation/ns_string/mod.rs#L69-L163

impl AddAssign<&NSString> for &NSMutableString {
    #[inline]
    fn add_assign(&mut self, other: &NSString) {
        self.appendString(other);
    }
}

impl fmt::Display for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: The object is an instance of `NSString`.
        unsafe { util::display_string(self, f) }
    }
}

impl fmt::Debug for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: Same as for `display_string`:
        // - The object is an instance of `NSString`.
        // - We control the scope in which the string is alive, so we know
        //   it is not moved outside the current autorelease pool.
        autoreleasepool_leaking(|pool| fmt::Debug::fmt(unsafe { nsstring_to_str(self, pool) }, f))
    }
}

impl fmt::Display for NSMutableString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl fmt::Write for &NSMutableString {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let nsstring = NSString::from_str(s);
        self.appendString(&nsstring);
        Ok(())
    }
}
