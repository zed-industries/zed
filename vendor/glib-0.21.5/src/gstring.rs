// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    borrow::{Borrow, Cow},
    cmp::Ordering,
    ffi::{CStr, CString, OsStr, OsString},
    fmt, hash,
    marker::PhantomData,
    mem,
    ops::Deref,
    os::raw::{c_char, c_void},
    path::{Path, PathBuf},
    ptr, slice,
};

use crate::{ffi, gobject_ffi, prelude::*, translate::*, Type, Value};

// rustdoc-stripper-ignore-next
/// Representation of a borrowed [`GString`].
///
/// This type is very similar to [`std::ffi::CStr`], but with one added constraint: the string
/// must also be valid UTF-8.
#[repr(transparent)]
pub struct GStr(str);

impl GStr {
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string wrapper from a byte slice.
    ///
    /// This function will cast the provided bytes to a `GStr` wrapper after ensuring that the byte
    /// slice is valid UTF-8 and is nul-terminated.
    #[inline]
    pub fn from_utf8_with_nul(bytes: &[u8]) -> Result<&Self, GStrError> {
        Self::check_trailing_nul(bytes)?;
        std::str::from_utf8(bytes)?;
        Ok(unsafe { mem::transmute::<&[u8], &GStr>(bytes) })
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string wrapper from a byte slice, checking for interior nul-bytes.
    ///
    /// This function will cast the provided bytes to a `GStr` wrapper after ensuring that the byte
    /// slice is valid UTF-8, is nul-terminated, and does not contain any interior nul-bytes.
    #[inline]
    pub fn from_utf8_with_nul_checked(bytes: &[u8]) -> Result<&Self, GStrError> {
        Self::check_nuls(bytes)?;
        std::str::from_utf8(bytes)?;
        Ok(unsafe { mem::transmute::<&[u8], &GStr>(bytes) })
    }
    // rustdoc-stripper-ignore-next
    /// Unsafely creates a GLib string wrapper from a byte slice.
    ///
    /// This function will cast the provided `bytes` to a `GStr` wrapper without performing any
    /// sanity checks.
    ///
    /// # Safety
    ///
    /// The provided slice **must** be valid UTF-8 and nul-terminated. It is undefined behavior to
    /// pass a slice that does not uphold those conditions.
    #[inline]
    pub const unsafe fn from_utf8_with_nul_unchecked(bytes: &[u8]) -> &Self {
        debug_assert!(!bytes.is_empty() && bytes[bytes.len() - 1] == 0);
        debug_assert!(std::str::from_utf8(bytes).is_ok());
        mem::transmute::<&[u8], &GStr>(bytes)
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string wrapper from a byte slice, truncating it at the first nul-byte.
    ///
    /// This function will cast the provided bytes to a `GStr` wrapper after ensuring that the byte
    /// slice is valid UTF-8 and contains at least one nul-byte.
    #[inline]
    pub fn from_utf8_until_nul(bytes: &[u8]) -> Result<&Self, GStrError> {
        let nul_pos = memchr::memchr(0, bytes).ok_or(GStrError::NoTrailingNul)?;
        let bytes = unsafe { bytes.get_unchecked(..nul_pos + 1) };
        std::str::from_utf8(bytes)?;
        Ok(unsafe { mem::transmute::<&[u8], &GStr>(bytes) })
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string wrapper from a string slice.
    ///
    /// The string slice must be terminated with a nul-byte.
    ///
    /// This function will cast the provided bytes to a `GStr` wrapper after ensuring
    /// that the string slice is nul-terminated.
    #[inline]
    pub fn from_str_with_nul(s: &str) -> Result<&Self, GStrError> {
        Self::check_trailing_nul(s)?;
        Ok(unsafe { mem::transmute::<&str, &GStr>(s) })
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string wrapper from a string slice, checking for interior nul-bytes.
    ///
    /// The string slice must be terminated with a nul-byte.
    ///
    /// This function will cast the provided bytes to a `GStr` wrapper after ensuring
    /// that the string slice is nul-terminated and does not contain any interior nul-bytes.
    #[inline]
    pub fn from_str_with_nul_checked(s: &str) -> Result<&Self, GStrError> {
        Self::check_nuls(s)?;
        Ok(unsafe { mem::transmute::<&str, &GStr>(s) })
    }
    // rustdoc-stripper-ignore-next
    /// Unsafely creates a GLib string wrapper from a string slice. The string slice must be
    /// terminated with a nul-byte.
    ///
    /// This function will cast the provided string slice to a `GStr` without performing any sanity
    /// checks.
    ///
    /// # Safety
    ///
    /// The provided string slice **must** be nul-terminated. It is undefined behavior to pass a
    /// slice that does not uphold those conditions.
    #[inline]
    pub const unsafe fn from_str_with_nul_unchecked(s: &str) -> &Self {
        debug_assert!(!s.is_empty() && s.as_bytes()[s.len() - 1] == 0);
        mem::transmute::<&str, &GStr>(s)
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string wrapper from a string slice, truncating it at the first nul-byte.
    ///
    /// The string slice must contain at least one nul-byte.
    ///
    /// This function will cast the provided bytes to a `GStr` wrapper after ensuring
    /// that the string slice contains at least one nul-byte.
    #[inline]
    pub fn from_str_until_nul(s: &str) -> Result<&Self, GStrError> {
        let b = s.as_bytes();
        let nul_pos = memchr::memchr(0, b).ok_or(GStrError::NoTrailingNul)?;
        let s = unsafe { std::str::from_utf8_unchecked(b.get_unchecked(..nul_pos + 1)) };
        Ok(unsafe { mem::transmute::<&str, &GStr>(s) })
    }
    // rustdoc-stripper-ignore-next
    /// Wraps a raw C string with a safe GLib string wrapper. The provided C string **must** be
    /// valid UTF-8 and nul-terminated. All constraints from [`CStr::from_ptr`] also apply here.
    ///
    /// # Safety
    ///
    /// See [`CStr::from_ptr`](std::ffi::CStr#safety).
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const c_char) -> &'a Self {
        let cstr = CStr::from_ptr(ptr);
        Self::from_utf8_with_nul_unchecked(cstr.to_bytes_with_nul())
    }
    // rustdoc-stripper-ignore-next
    /// Wraps a raw C string with a safe GLib string wrapper. The provided C string **must** be
    /// nul-terminated. All constraints from [`std::ffi::CStr::from_ptr`] also apply here.
    ///
    /// If the string is valid UTF-8 then it is directly returned, otherwise `None` is returned.
    #[inline]
    pub unsafe fn from_ptr_checked<'a>(ptr: *const c_char) -> Option<&'a Self> {
        let mut end_ptr = ptr::null();
        if ffi::g_utf8_validate(ptr as *const _, -1, &mut end_ptr) != ffi::GFALSE {
            Some(Self::from_utf8_with_nul_unchecked(slice::from_raw_parts(
                ptr as *const u8,
                end_ptr.offset_from(ptr as *const u8) as usize + 1,
            )))
        } else {
            None
        }
    }
    // rustdoc-stripper-ignore-next
    /// Wraps a raw C string with a safe GLib string wrapper. The provided C string **must** be
    /// nul-terminated. All constraints from [`std::ffi::CStr::from_ptr`] also apply here.
    ///
    /// If the string is valid UTF-8 then it is directly returned otherwise a copy is created with
    /// every invalid character replaced by the Unicode replacement character (U+FFFD).
    #[inline]
    pub unsafe fn from_ptr_lossy<'a>(ptr: *const c_char) -> Cow<'a, Self> {
        if let Some(gs) = Self::from_ptr_checked(ptr) {
            Cow::Borrowed(gs)
        } else {
            Cow::Owned(GString::from_glib_full(ffi::g_utf8_make_valid(
                ptr as *const _,
                -1,
            )))
        }
    }
    // rustdoc-stripper-ignore-next
    /// Converts this GLib string to a byte slice containing the trailing 0 byte.
    ///
    /// This function is the equivalent of [`GStr::as_bytes`] except that it will retain the
    /// trailing nul terminator instead of chopping it off.
    #[inline]
    pub const fn as_bytes_with_nul(&self) -> &[u8] {
        self.0.as_bytes()
    }
    // rustdoc-stripper-ignore-next
    /// Converts this GLib string to a byte slice.
    ///
    /// The returned slice will **not** contain the trailing nul terminator that this GLib
    /// string has.
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
    // rustdoc-stripper-ignore-next
    /// Returns the inner pointer to this GLib string.
    ///
    /// The returned pointer will be valid for as long as `self` is, and points to a contiguous
    /// region of memory terminated with a 0 byte to represent the end of the string.
    ///
    /// **WARNING**
    ///
    /// The returned pointer is read-only; writing to it (including passing it to C code that
    /// writes to it) causes undefined behavior. It is your responsibility to make
    /// sure that the underlying memory is not freed too early.
    #[inline]
    pub const fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr() as *const _
    }
    // rustdoc-stripper-ignore-next
    /// Converts this GLib string to a string slice.
    #[inline]
    pub const fn as_str(&self) -> &str {
        // Clip off the nul-byte
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.as_ptr() as *const _,
                self.0.len() - 1,
            ))
        }
    }
    // rustdoc-stripper-ignore-next
    /// Converts this GLib string to a C string slice, checking for interior nul-bytes.
    ///
    /// Returns `Err` if the string contains any interior nul-bytes.
    #[inline]
    pub fn to_cstr(&self) -> Result<&CStr, GStrInteriorNulError> {
        Self::check_interior_nuls(self.as_bytes())?;
        Ok(unsafe { self.to_cstr_unchecked() })
    }
    // rustdoc-stripper-ignore-next
    /// Converts this GLib string to a C string slice, truncating it at the first nul-byte.
    #[inline]
    pub fn to_cstr_until_nul(&self) -> &CStr {
        let b = self.as_bytes_with_nul();
        let nul_pos = memchr::memchr(0, b).unwrap();
        unsafe { CStr::from_bytes_with_nul_unchecked(b.get_unchecked(..nul_pos + 1)) }
    }
    // rustdoc-stripper-ignore-next
    /// Converts this GLib string to a C string slice, without checking for interior nul-bytes.
    ///
    /// # Safety
    ///
    /// `self` **must** not contain any interior nul-bytes besides the final terminating nul-byte.
    /// It is undefined behavior to call this on a string that contains interior nul-bytes.
    #[inline]
    pub const unsafe fn to_cstr_unchecked(&self) -> &CStr {
        CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_nul())
    }

    #[doc(alias = "g_utf8_collate")]
    #[doc(alias = "utf8_collate")]
    pub fn collate(&self, other: impl IntoGStr) -> Ordering {
        other.run_with_gstr(|other| {
            unsafe { ffi::g_utf8_collate(self.to_glib_none().0, other.to_glib_none().0) }.cmp(&0)
        })
    }

    #[inline]
    fn check_nuls(s: impl AsRef<[u8]>) -> Result<(), GStrError> {
        let s = s.as_ref();
        if let Some(nul_pos) = memchr::memchr(0, s) {
            if s.len() == nul_pos + 1 {
                Ok(())
            } else {
                Err(GStrInteriorNulError(nul_pos).into())
            }
        } else {
            Err(GStrError::NoTrailingNul)
        }
    }
    #[inline]
    fn check_trailing_nul(s: impl AsRef<[u8]>) -> Result<(), GStrError> {
        if let Some(c) = s.as_ref().last().copied() {
            if c == 0 {
                return Ok(());
            }
        }
        Err(GStrError::NoTrailingNul)
    }
    // rustdoc-stripper-ignore-next
    /// Returns `Err` if the string slice contains any nul-bytes.
    #[inline]
    pub(crate) fn check_interior_nuls(s: impl AsRef<[u8]>) -> Result<(), GStrInteriorNulError> {
        if let Some(nul_pos) = memchr::memchr(0, s.as_ref()) {
            Err(GStrInteriorNulError(nul_pos))
        } else {
            Ok(())
        }
    }
    pub const NONE: Option<&'static GStr> = None;

    // rustdoc-stripper-ignore-next
    /// Interns the string and returns the canonical representation.
    #[inline]
    #[doc(alias = "g_intern_string")]
    pub fn intern(&self) -> &'static GStr {
        unsafe {
            let s = ffi::g_intern_string(self.to_glib_none().0);
            GStr::from_ptr(s)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Interns the `'static` string and returns the canonical representation.
    #[inline]
    #[doc(alias = "g_intern_static_string")]
    pub fn intern_static(&'static self) -> &'static GStr {
        unsafe {
            let s = ffi::g_intern_static_string(self.to_glib_none().0);
            GStr::from_ptr(s)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Interns the string and returns the canonical representation.
    #[inline]
    #[doc(alias = "g_intern_string")]
    pub fn intern_from_str(s: impl AsRef<str>) -> &'static GStr {
        unsafe {
            let s = ffi::g_intern_string(s.as_ref().to_glib_none().0);
            GStr::from_ptr(s)
        }
    }
}

// rustdoc-stripper-ignore-next
/// Error type holding all possible failures when creating a [`GStr`] reference.
#[derive(Debug)]
pub enum GStrError {
    InvalidUtf8(std::str::Utf8Error),
    InteriorNul(GStrInteriorNulError),
    NoTrailingNul,
}

impl std::error::Error for GStrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidUtf8(err) => std::error::Error::source(err),
            Self::InteriorNul(err) => std::error::Error::source(err),
            Self::NoTrailingNul => None,
        }
    }
}

impl fmt::Display for GStrError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidUtf8(err) => fmt::Display::fmt(err, fmt),
            Self::InteriorNul(err) => fmt::Display::fmt(err, fmt),
            Self::NoTrailingNul => fmt.write_str("data provided is not nul terminated"),
        }
    }
}

impl std::convert::From<std::str::Utf8Error> for GStrError {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::InvalidUtf8(err)
    }
}

impl std::convert::From<GStrInteriorNulError> for GStrError {
    fn from(err: GStrInteriorNulError) -> Self {
        Self::InteriorNul(err)
    }
}

// rustdoc-stripper-ignore-next
/// Error type indicating that a buffer had unexpected nul-bytes.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct GStrInteriorNulError(usize);

impl std::error::Error for GStrInteriorNulError {}

impl fmt::Display for GStrInteriorNulError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "data provided contains an interior nul-byte at byte pos {}",
            self.0
        )
    }
}

impl GStrInteriorNulError {
    // rustdoc-stripper-ignore-next
    /// Returns the position of the nul-byte in the slice that caused the conversion to fail.
    #[inline]
    pub fn nul_position(&self) -> usize {
        self.0
    }
}

// rustdoc-stripper-ignore-next
/// Converts a static string literal into a static nul-terminated string.
///
/// The expanded expression has type [`&'static GStr`]. This macro will panic if the
/// string literal contains any interior nul-bytes.
///
/// # Examples
///
/// ```
/// # fn main() {
/// use glib::{gstr, GStr, GString};
///
/// const MY_STRING: &GStr = gstr!("Hello");
/// assert_eq!(MY_STRING.as_bytes_with_nul()[5], 0u8);
/// let owned: GString = MY_STRING.to_owned();
/// assert_eq!(MY_STRING, owned);
/// # }
/// ```
///
/// [`&'static GStr`]: crate::GStr
#[macro_export]
macro_rules! gstr {
    ($s:literal) => {
        unsafe { $crate::GStr::from_utf8_with_nul_unchecked($crate::cstr_bytes!($s)) }
    };
}

impl fmt::Debug for GStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <&str as fmt::Debug>::fmt(&self.as_str(), f)
    }
}

impl PartialEq for GStr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl Eq for GStr {}

impl PartialOrd for GStr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GStr {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl hash::Hash for GStr {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl Default for &GStr {
    #[inline]
    fn default() -> Self {
        const SLICE: &[c_char] = &[0];
        unsafe { GStr::from_ptr(SLICE.as_ptr()) }
    }
}

impl fmt::Display for GStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> TryFrom<&'a CStr> for &'a GStr {
    type Error = std::str::Utf8Error;
    #[inline]
    fn try_from(s: &'a CStr) -> Result<Self, Self::Error> {
        s.to_str()?;
        Ok(unsafe { GStr::from_utf8_with_nul_unchecked(s.to_bytes_with_nul()) })
    }
}

impl PartialEq<GStr> for String {
    #[inline]
    fn eq(&self, other: &GStr) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<str> for GStr {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for GStr {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<GStr> for &str {
    #[inline]
    fn eq(&self, other: &GStr) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for GStr {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GStr> for str {
    #[inline]
    fn eq(&self, other: &GStr) -> bool {
        self == other.as_str()
    }
}

impl PartialOrd<GStr> for String {
    #[inline]
    fn partial_cmp(&self, other: &GStr) -> Option<Ordering> {
        Some(self.cmp(&String::from(other.as_str())))
    }
}

impl PartialOrd<String> for GStr {
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl PartialOrd<GStr> for str {
    #[inline]
    fn partial_cmp(&self, other: &GStr) -> Option<Ordering> {
        Some(self.cmp(other.as_str()))
    }
}

impl PartialOrd<str> for GStr {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl AsRef<GStr> for GStr {
    #[inline]
    fn as_ref(&self) -> &GStr {
        self
    }
}

impl AsRef<str> for GStr {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<OsStr> for GStr {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_str())
    }
}

impl AsRef<Path> for GStr {
    #[inline]
    fn as_ref(&self) -> &Path {
        Path::new(self.as_str())
    }
}

impl AsRef<[u8]> for GStr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Deref for GStr {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl ToOwned for GStr {
    type Owned = GString;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        let b = self.as_bytes_with_nul();
        if self.len() < INLINE_LEN {
            let mut data = <[u8; INLINE_LEN]>::default();
            let b = self.as_bytes();
            unsafe { data.get_unchecked_mut(..b.len()) }.copy_from_slice(b);
            return GString(Inner::Inline {
                len: self.len() as u8,
                data,
            });
        }
        let inner = unsafe {
            let copy = ffi::g_strndup(b.as_ptr() as *const c_char, b.len());
            Inner::Foreign {
                ptr: ptr::NonNull::new_unchecked(copy),
                len: b.len() - 1,
            }
        };
        GString(inner)
    }
}

impl GlibPtrDefault for GStr {
    type GlibType = *mut c_char;
}

impl StaticType for GStr {
    #[inline]
    fn static_type() -> Type {
        str::static_type()
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const u8> for &GStr {
    #[inline]
    unsafe fn from_glib_none(ptr: *const u8) -> Self {
        debug_assert!(!ptr.is_null());
        let cstr = CStr::from_ptr(ptr as *const _);
        debug_assert!(cstr.to_str().is_ok(), "C string is not valid utf-8");
        GStr::from_utf8_with_nul_unchecked(cstr.to_bytes_with_nul())
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const i8> for &GStr {
    #[inline]
    unsafe fn from_glib_none(ptr: *const i8) -> Self {
        from_glib_none(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut u8> for &GStr {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut u8) -> Self {
        from_glib_none(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut i8> for &GStr {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut i8) -> Self {
        from_glib_none(ptr as *const u8)
    }
}

unsafe impl<'a> crate::value::FromValue<'a> for &'a GStr {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_string(value.to_glib_none().0);
        let cstr = CStr::from_ptr(ptr);
        debug_assert!(
            cstr.to_str().is_ok(),
            "C string in glib::Value is not valid utf-8"
        );
        GStr::from_utf8_with_nul_unchecked(cstr.to_bytes_with_nul())
    }
}

impl ToValue for GStr {
    #[inline]
    fn to_value(&self) -> Value {
        self.as_str().to_value()
    }

    #[inline]
    fn value_type(&self) -> Type {
        str::static_type()
    }
}

impl ToValue for &GStr {
    #[inline]
    fn to_value(&self) -> Value {
        (*self).to_value()
    }

    #[inline]
    fn value_type(&self) -> Type {
        str::static_type()
    }
}

impl crate::value::ToValueOptional for GStr {
    #[inline]
    fn to_value_optional(s: Option<&Self>) -> Value {
        crate::value::ToValueOptional::to_value_optional(s.map(|s| s.as_str()))
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const c_char> for GStr {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        Stash(self.as_ptr(), PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *const c_char {
        self.as_str().to_glib_full()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut c_char> for GStr {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        Stash(self.as_ptr() as *mut c_char, PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut c_char {
        self.as_str().to_glib_full()
    }
}

// rustdoc-stripper-ignore-next
/// `NULL`-terminated UTF-8 string as stored in [`StrV`](crate::StrV).
///
/// Unlike [`&GStr`](crate::GStr) this does not have its length stored.
#[repr(transparent)]
pub struct GStringPtr(ptr::NonNull<c_char>);

#[doc(hidden)]
unsafe impl TransparentPtrType for GStringPtr {}

#[doc(hidden)]
impl GlibPtrDefault for GStringPtr {
    type GlibType = *mut c_char;
}

impl GStringPtr {
    // rustdoc-stripper-ignore-next
    /// Returns the corresponding [`&GStr`](crate::GStr).
    #[inline]
    pub fn to_gstr(&self) -> &GStr {
        unsafe { GStr::from_ptr(self.0.as_ptr()) }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the corresponding [`&str`].
    #[inline]
    pub fn as_str(&self) -> &str {
        self.to_gstr().as_str()
    }

    // rustdoc-stripper-ignore-next
    /// Returns the string's C pointer.
    #[inline]
    pub const fn as_ptr(&self) -> *const c_char {
        self.0.as_ptr()
    }

    // rustdoc-stripper-ignore-next
    /// Wrapper around `libc::strcmp` returning `Ordering`.
    ///
    /// # Safety
    ///
    /// `a` and `b` must be non-null pointers to nul-terminated C strings.
    #[inline]
    unsafe fn strcmp(a: *const c_char, b: *const c_char) -> Ordering {
        from_glib(libc::strcmp(a, b))
    }
}

impl Clone for GStringPtr {
    #[inline]
    fn clone(&self) -> GStringPtr {
        unsafe { GStringPtr(ptr::NonNull::new_unchecked(ffi::g_strdup(self.0.as_ptr()))) }
    }
}

impl Deref for GStringPtr {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl IntoGlibPtr<*mut c_char> for GStringPtr {
    #[inline]
    fn into_glib_ptr(self) -> *mut c_char {
        self.0.as_ptr()
    }
}

impl Drop for GStringPtr {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::g_free(self.0.as_ptr() as *mut _);
        }
    }
}

impl fmt::Debug for GStringPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <&GStr as fmt::Debug>::fmt(&self.to_gstr(), f)
    }
}

impl fmt::Display for GStringPtr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Eq for GStringPtr {}

impl PartialEq for GStringPtr {
    #[inline]
    fn eq(&self, other: &GStringPtr) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<GStringPtr> for String {
    #[inline]
    fn eq(&self, other: &GStringPtr) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<GStringPtr> for GString {
    #[inline]
    fn eq(&self, other: &GStringPtr) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<str> for GStringPtr {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<&str> for GStringPtr {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<GStr> for GStringPtr {
    #[inline]
    fn eq(&self, other: &GStr) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<&GStr> for GStringPtr {
    #[inline]
    fn eq(&self, other: &&GStr) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<GStringPtr> for &str {
    #[inline]
    fn eq(&self, other: &GStringPtr) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<GStringPtr> for &GStr {
    #[inline]
    fn eq(&self, other: &GStringPtr) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<String> for GStringPtr {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<GString> for GStringPtr {
    #[inline]
    fn eq(&self, other: &GString) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<GStringPtr> for str {
    #[inline]
    fn eq(&self, other: &GStringPtr) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<GStringPtr> for GStr {
    #[inline]
    fn eq(&self, other: &GStringPtr) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd<GStringPtr> for GStringPtr {
    #[inline]
    fn partial_cmp(&self, other: &GStringPtr) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GStringPtr {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { GStringPtr::strcmp(self.as_ptr(), other.as_ptr()) }
    }
}

impl PartialOrd<GStringPtr> for String {
    #[inline]
    fn partial_cmp(&self, other: &GStringPtr) -> Option<std::cmp::Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl PartialOrd<GStringPtr> for GString {
    #[inline]
    fn partial_cmp(&self, other: &GStringPtr) -> Option<std::cmp::Ordering> {
        Some(unsafe { GStringPtr::strcmp(self.as_ptr(), other.as_ptr()) })
    }
}

impl PartialOrd<String> for GStringPtr {
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl PartialOrd<GString> for GStringPtr {
    #[inline]
    fn partial_cmp(&self, other: &GString) -> Option<std::cmp::Ordering> {
        Some(unsafe { GStringPtr::strcmp(self.as_ptr(), other.as_ptr()) })
    }
}

impl PartialOrd<GStringPtr> for str {
    #[inline]
    fn partial_cmp(&self, other: &GStringPtr) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other.as_str()))
    }
}

impl PartialOrd<GStringPtr> for GStr {
    #[inline]
    fn partial_cmp(&self, other: &GStringPtr) -> Option<std::cmp::Ordering> {
        Some(unsafe { GStringPtr::strcmp(self.as_ptr(), other.as_ptr()) })
    }
}

impl PartialOrd<str> for GStringPtr {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl PartialOrd<&str> for GStringPtr {
    #[inline]
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl PartialOrd<GStr> for GStringPtr {
    #[inline]
    fn partial_cmp(&self, other: &GStr) -> Option<std::cmp::Ordering> {
        Some(unsafe { GStringPtr::strcmp(self.as_ptr(), other.as_ptr()) })
    }
}

impl PartialOrd<&GStr> for GStringPtr {
    #[inline]
    fn partial_cmp(&self, other: &&GStr) -> Option<std::cmp::Ordering> {
        Some(unsafe { GStringPtr::strcmp(self.as_ptr(), other.as_ptr()) })
    }
}

impl PartialOrd<GStringPtr> for &str {
    #[inline]
    fn partial_cmp(&self, other: &GStringPtr) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other.as_str()))
    }
}

impl PartialOrd<GStringPtr> for &GStr {
    #[inline]
    fn partial_cmp(&self, other: &GStringPtr) -> Option<std::cmp::Ordering> {
        Some(unsafe { GStringPtr::strcmp(self.as_ptr(), other.as_ptr()) })
    }
}

impl AsRef<GStringPtr> for GStringPtr {
    #[inline]
    fn as_ref(&self) -> &GStringPtr {
        self
    }
}

impl std::hash::Hash for GStringPtr {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl<T: AsRef<str>> From<T> for GStringPtr {
    fn from(value: T) -> Self {
        unsafe {
            let value = value.as_ref();
            GStringPtr(ptr::NonNull::new_unchecked(ffi::g_strndup(
                value.as_ptr() as *const _,
                value.len(),
            )))
        }
    }
}

// size_of::<Inner>() minus two bytes for length and enum discriminant
const INLINE_LEN: usize =
    mem::size_of::<Option<Box<str>>>() + mem::size_of::<usize>() - mem::size_of::<u8>() * 2;

// rustdoc-stripper-ignore-next
/// A type representing an owned, C-compatible, nul-terminated UTF-8 string.
///
/// `GString` is to <code>&[GStr]</code> as [`String`] is to <code>&[str]</code>: the former in
/// each pair are owned strings; the latter are borrowed references.
///
/// This type is similar to [`std::ffi::CString`], but with some special behavior. When debug
/// assertions are enabled, <code>[From]&lt;[String]></code> will panic if there are interior
/// nul-bytes. In production builds, no checks will be made for interior nul-bytes, and strings
/// that contain interior nul-bytes will simply end at first nul-byte when converting to a C
/// string.
///
/// The constructors beginning with `from_utf8` and `from_string` can also be used to further
/// control how interior nul-bytes are handled.
pub struct GString(Inner);

enum Inner {
    Native(Box<str>),
    Foreign {
        ptr: ptr::NonNull<c_char>,
        len: usize,
    },
    Inline {
        len: u8,
        data: [u8; INLINE_LEN],
    },
}

unsafe impl Send for GString {}
unsafe impl Sync for GString {}

impl GString {
    // rustdoc-stripper-ignore-next
    /// Creates a new empty [`GString`].
    ///
    /// Does not allocate.
    #[inline]
    pub fn new() -> Self {
        Self(Inner::Inline {
            len: 0,
            data: Default::default(),
        })
    }
    // rustdoc-stripper-ignore-next
    /// Formats an [`Arguments`](std::fmt::Arguments) into a [`GString`].
    ///
    /// This function is the same as [`std::fmt::format`], except it returns a [`GString`]. The
    /// [`Arguments`](std::fmt::Arguments) instance can be created with the
    /// [`format_args!`](std::format_args) macro.
    ///
    /// Please note that using [`gformat!`](crate::gformat) might be preferable.
    pub fn format(args: fmt::Arguments) -> Self {
        if let Some(s) = args.as_str() {
            return Self::from(s);
        }

        let mut s = crate::GStringBuilder::default();
        fmt::Write::write_fmt(&mut s, args).unwrap();
        s.into_string()
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string by consuming a byte vector.
    ///
    /// Takes ownership of `bytes`. Returns `Err` if it contains invalid UTF-8.
    ///
    /// A trailing nul-byte will be appended by this function.
    #[inline]
    pub fn from_utf8(bytes: Vec<u8>) -> Result<Self, std::string::FromUtf8Error> {
        Ok(Self::from_string_unchecked(String::from_utf8(bytes)?))
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string by consuming a byte vector, checking for interior nul-bytes.
    ///
    /// Takes ownership of `bytes`, as long as it is valid UTF-8 and does not contain any interior
    /// nul-bytes. Otherwise, `Err` is returned.
    ///
    /// A trailing nul-byte will be appended by this function.
    #[inline]
    pub fn from_utf8_checked(bytes: Vec<u8>) -> Result<Self, GStringFromError<Vec<u8>>> {
        Ok(Self::from_string_checked(String::from_utf8(bytes)?)
            .map_err(|e| GStringInteriorNulError(e.0.into_bytes(), e.1))?)
    }
    // rustdoc-stripper-ignore-next
    /// Unsafely creates a GLib string by consuming a byte vector, without checking for UTF-8 or
    /// interior nul-bytes.
    ///
    /// A trailing nul-byte will be appended by this function.
    ///
    /// # Safety
    ///
    /// The byte vector **must** not contain invalid UTF-8 characters. It is undefined behavior to
    /// pass a vector that contains invalid UTF-8.
    #[inline]
    pub unsafe fn from_utf8_unchecked(mut v: Vec<u8>) -> Self {
        if v.is_empty() {
            Self::new()
        } else {
            v.reserve_exact(1);
            v.push(0);
            Self(Inner::Native(String::from_utf8_unchecked(v).into()))
        }
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string by consuming a nul-terminated byte vector, without checking for
    /// interior nul-bytes.
    ///
    /// Takes ownership of `bytes`. Returns `Err` if it contains invalid UTF-8 or does not have a
    /// trailing nul-byte.
    #[inline]
    pub fn from_utf8_with_nul(bytes: Vec<u8>) -> Result<Self, GStringFromError<Vec<u8>>> {
        let s = String::from_utf8(bytes)?;
        if s.as_bytes().last().copied() != Some(0u8) {
            return Err(GStringNoTrailingNulError(s.into_bytes()).into());
        }
        if s.len() == 1 {
            Ok(Self::new())
        } else {
            Ok(Self(Inner::Native(s.into())))
        }
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string by consuming a nul-terminated byte vector.
    ///
    /// Takes ownership of `bytes`. Returns `Err` if it contains invalid UTF-8, does not have a
    /// trailing nul-byte, or contains interior nul-bytes.
    #[inline]
    pub fn from_utf8_with_nul_checked(bytes: Vec<u8>) -> Result<Self, GStringFromError<Vec<u8>>> {
        let s = Self::from_utf8_with_nul(bytes)?;
        if let Err(e) = GStr::check_interior_nuls(&s) {
            return Err(GStringInteriorNulError(s.into_bytes(), e).into());
        }
        Ok(s)
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string by consuming a byte vector, without checking for UTF-8, a trailing
    /// nul-byte, or interior nul-bytes.
    ///
    /// # Safety
    ///
    /// The byte vector **must** not contain invalid UTF-8 characters, and **must** have a trailing
    /// nul-byte. It is undefined behavior to pass a vector that does not uphold those conditions.
    #[inline]
    pub unsafe fn from_utf8_with_nul_unchecked(v: Vec<u8>) -> Self {
        debug_assert!(!v.is_empty() && v[v.len() - 1] == 0);
        let s = if cfg!(debug_assertions) {
            let s = String::from_utf8(v).unwrap();
            GStr::check_interior_nuls(&s[..s.len() - 1]).unwrap();
            s
        } else {
            String::from_utf8_unchecked(v)
        };
        if s.len() == 1 {
            Self::new()
        } else {
            Self(Inner::Native(s.into()))
        }
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string by consuming a nul-terminated byte vector, truncating it at the first
    /// nul-byte.
    ///
    /// Takes ownership of `bytes`. Returns `Err` if it contains invalid UTF-8 or does not contain
    /// at least one nul-byte.
    #[inline]
    pub fn from_utf8_until_nul(mut bytes: Vec<u8>) -> Result<Self, GStringFromError<Vec<u8>>> {
        let nul_pos = if let Some(nul_pos) = memchr::memchr(0, &bytes) {
            nul_pos
        } else {
            return Err(GStringNoTrailingNulError(bytes).into());
        };
        if nul_pos == 0 {
            Ok(Self::new())
        } else {
            if let Err(e) = std::str::from_utf8(unsafe { bytes.get_unchecked(..nul_pos) }) {
                return Err(GStringUtf8Error(bytes, e).into());
            }
            bytes.truncate(nul_pos + 1);
            let s = unsafe { String::from_utf8_unchecked(bytes) };
            Ok(Self(Inner::Native(s.into())))
        }
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string by consuming a string, checking for interior nul-bytes.
    ///
    /// Takes ownership of `s`, as long as it does not contain any interior nul-bytes. Otherwise,
    /// `Err` is returned.
    ///
    /// A trailing nul-byte will be appended by this function.
    #[inline]
    pub fn from_string_checked(s: String) -> Result<Self, GStringInteriorNulError<String>> {
        if let Err(e) = GStr::check_interior_nuls(&s) {
            return Err(GStringInteriorNulError(s, e));
        }
        Ok(Self::from_string_unchecked(s))
    }
    // rustdoc-stripper-ignore-next
    /// Creates a GLib string by consuming a string, without checking for interior nul-bytes.
    ///
    /// A trailing nul-byte will be appended by this function.
    #[inline]
    pub fn from_string_unchecked(mut s: String) -> Self {
        if s.is_empty() {
            Self::new()
        } else {
            s.reserve_exact(1);
            s.push('\0');
            Self(Inner::Native(s.into()))
        }
    }
    // rustdoc-stripper-ignore-next
    /// Wraps a raw C string with a safe GLib string wrapper. The provided C string **must** be
    /// nul-terminated. All constraints from [`std::ffi::CStr::from_ptr`] also apply here.
    ///
    /// If the string is valid UTF-8 then it is directly returned otherwise a copy is created with
    /// every invalid character replaced by the Unicode replacement character (U+FFFD).
    #[inline]
    pub unsafe fn from_ptr_lossy<'a>(ptr: *const c_char) -> Cow<'a, GStr> {
        GStr::from_ptr_lossy(ptr)
    }

    // rustdoc-stripper-ignore-next
    /// Wraps a raw C string with a safe GLib string wrapper. The provided C string **must** be
    /// nul-terminated. All constraints from [`std::ffi::CStr::from_ptr`] also apply here.
    ///
    /// `len` is the length without the nul-terminator, i.e. if `len == 0` is passed then `*ptr`
    /// must be the nul-terminator.
    #[inline]
    pub unsafe fn from_ptr_and_len_unchecked(ptr: *const c_char, len: usize) -> Self {
        debug_assert!(!ptr.is_null());

        GString(Inner::Foreign {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            len,
        })
    }

    // rustdoc-stripper-ignore-next
    /// Return the `GString` as string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe {
            let (ptr, len) = match self.0 {
                Inner::Native(ref s) => (s.as_ptr(), s.len() - 1),
                Inner::Foreign { ptr, len } => (ptr.as_ptr() as *const u8, len),
                Inner::Inline { len, ref data } => (data.as_ptr(), len as usize),
            };
            if len == 0 {
                ""
            } else {
                let slice = slice::from_raw_parts(ptr, len);
                std::str::from_utf8_unchecked(slice)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Extracts the [`GStr`] containing the entire string.
    #[inline]
    pub fn as_gstr(&self) -> &GStr {
        let bytes = match self.0 {
            Inner::Native(ref s) => s.as_bytes(),
            Inner::Foreign { len: 0, .. } => &[0],
            Inner::Foreign { ptr, len } => unsafe {
                slice::from_raw_parts(ptr.as_ptr() as *const _, len + 1)
            },
            Inner::Inline { len, ref data } => unsafe { data.get_unchecked(..len as usize + 1) },
        };
        unsafe { GStr::from_utf8_with_nul_unchecked(bytes) }
    }

    // rustdoc-stripper-ignore-next
    /// Return the underlying pointer of the `GString`.
    #[inline]
    pub fn as_ptr(&self) -> *const c_char {
        match self.0 {
            Inner::Native(ref s) => s.as_ptr() as *const _,
            Inner::Foreign { ptr, .. } => ptr.as_ptr(),
            Inner::Inline { ref data, .. } => data.as_ptr() as *const _,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Consumes the `GString` and returns the underlying byte buffer.
    ///
    /// The returned buffer is not guaranteed to contain a trailing nul-byte.
    #[inline]
    pub fn into_bytes(mut self) -> Vec<u8> {
        match &mut self.0 {
            Inner::Native(s) => {
                let mut s = String::from(mem::replace(s, "".into()));
                let _nul = s.pop();
                debug_assert_eq!(_nul, Some('\0'));
                s.into_bytes()
            }
            Inner::Foreign { ptr, len } => {
                let bytes = unsafe { slice::from_raw_parts(ptr.as_ptr() as *const u8, *len - 1) };
                bytes.to_owned()
            }
            Inner::Inline { len, data } => {
                unsafe { data.get_unchecked(..*len as usize) }.to_owned()
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Consumes the `GString` and returns the underlying byte buffer, with trailing nul-byte.
    #[inline]
    pub fn into_bytes_with_nul(mut self) -> Vec<u8> {
        match &mut self.0 {
            Inner::Native(s) => str::into_boxed_bytes(mem::replace(s, "".into())).into(),
            Inner::Foreign { ptr, len } => {
                let bytes = unsafe { slice::from_raw_parts(ptr.as_ptr() as *const u8, *len) };
                bytes.to_owned()
            }
            Inner::Inline { len, data } => {
                unsafe { data.get_unchecked(..*len as usize + 1) }.to_owned()
            }
        }
    }
}

// rustdoc-stripper-ignore-next
/// Creates a [`GString`] using interpolation of runtime expressions.
///
/// This macro is the same as [`std::format!`] except it returns a [`GString`]. It is faster than
/// creating a [`String`] and then converting it manually to a [`GString`].
#[macro_export]
macro_rules! gformat {
    ($($arg:tt)*) => { $crate::GString::format(std::format_args!($($arg)*)) };
}

// rustdoc-stripper-ignore-next
/// Error type indicating that a buffer did not have a trailing nul-byte.
///
/// `T` is the type of the value the conversion was attempted from.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GStringNoTrailingNulError<T>(T);

impl<T> std::error::Error for GStringNoTrailingNulError<T> where T: fmt::Debug {}

impl<T> fmt::Display for GStringNoTrailingNulError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("data provided is not nul terminated")
    }
}

impl<T> GStringNoTrailingNulError<T> {
    // rustdoc-stripper-ignore-next
    /// Returns the original value that was attempted to convert to [`GString`].
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

// rustdoc-stripper-ignore-next
/// Error type indicating that a buffer had unexpected nul-bytes.
///
/// `T` is the type of the value the conversion was attempted from.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GStringInteriorNulError<T>(T, GStrInteriorNulError);

impl<T> std::error::Error for GStringInteriorNulError<T> where T: fmt::Debug {}

impl<T> fmt::Display for GStringInteriorNulError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.1, fmt)
    }
}

impl<T> GStringInteriorNulError<T> {
    // rustdoc-stripper-ignore-next
    /// Returns the original value that was attempted to convert to [`GString`].
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
    // rustdoc-stripper-ignore-next
    /// Fetch a [`GStrInteriorNulError`] to get more details about the conversion failure.
    #[inline]
    pub fn nul_error(&self) -> GStrInteriorNulError {
        self.1
    }
}

// rustdoc-stripper-ignore-next
/// Error type indicating that a buffer had invalid UTF-8.
///
/// `T` is the type of the value the conversion was attempted from.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GStringUtf8Error<T>(T, std::str::Utf8Error);

impl<T> std::error::Error for GStringUtf8Error<T> where T: fmt::Debug {}

impl<T> fmt::Display for GStringUtf8Error<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.1, fmt)
    }
}

impl<T> GStringUtf8Error<T> {
    // rustdoc-stripper-ignore-next
    /// Returns the original value that was attempted to convert to [`GString`].
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
    // rustdoc-stripper-ignore-next
    /// Fetch a [`Utf8Error`](std::str::Utf8Error) to get more details about the conversion
    /// failure.
    #[inline]
    pub fn utf8_error(&self) -> std::str::Utf8Error {
        self.1
    }
}

// rustdoc-stripper-ignore-next
/// Error type holding all possible failures when creating a [`GString`].
#[derive(Debug)]
pub enum GStringFromError<T> {
    NoTrailingNul(GStringNoTrailingNulError<T>),
    InteriorNul(GStringInteriorNulError<T>),
    InvalidUtf8(GStringUtf8Error<T>),
    Unspecified(T),
}

impl<T> std::error::Error for GStringFromError<T>
where
    T: fmt::Debug,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NoTrailingNul(err) => std::error::Error::source(err),
            Self::InteriorNul(err) => std::error::Error::source(err),
            Self::InvalidUtf8(err) => std::error::Error::source(err),
            Self::Unspecified { .. } => None,
        }
    }
}

impl<T> fmt::Display for GStringFromError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoTrailingNul(err) => fmt::Display::fmt(err, fmt),
            Self::InteriorNul(err) => fmt::Display::fmt(err, fmt),
            Self::InvalidUtf8(err) => fmt::Display::fmt(err, fmt),
            Self::Unspecified(_) => fmt.write_str("unable to convert"),
        }
    }
}

impl<T> std::convert::From<GStringNoTrailingNulError<T>> for GStringFromError<T> {
    fn from(err: GStringNoTrailingNulError<T>) -> Self {
        GStringFromError::NoTrailingNul(err)
    }
}

impl<T> std::convert::From<GStringInteriorNulError<T>> for GStringFromError<T> {
    fn from(err: GStringInteriorNulError<T>) -> Self {
        GStringFromError::InteriorNul(err)
    }
}

impl<T> std::convert::From<GStringUtf8Error<T>> for GStringFromError<T> {
    fn from(err: GStringUtf8Error<T>) -> Self {
        GStringFromError::InvalidUtf8(err)
    }
}

impl<T> GStringFromError<T> {
    pub fn into_inner(self) -> T {
        match self {
            Self::NoTrailingNul(GStringNoTrailingNulError(t)) => t,
            Self::InteriorNul(GStringInteriorNulError(t, _)) => t,
            Self::InvalidUtf8(GStringUtf8Error(t, _)) => t,
            Self::Unspecified(t) => t,
        }
    }
    #[inline]
    fn convert<R>(self, func: impl FnOnce(T) -> R) -> GStringFromError<R> {
        match self {
            Self::NoTrailingNul(GStringNoTrailingNulError(t)) => {
                GStringFromError::NoTrailingNul(GStringNoTrailingNulError(func(t)))
            }
            Self::InteriorNul(GStringInteriorNulError(t, e)) => {
                GStringFromError::InteriorNul(GStringInteriorNulError(func(t), e))
            }
            Self::InvalidUtf8(GStringUtf8Error(t, e)) => {
                GStringFromError::InvalidUtf8(GStringUtf8Error(func(t), e))
            }
            Self::Unspecified(t) => GStringFromError::Unspecified(func(t)),
        }
    }
}

impl From<std::string::FromUtf8Error> for GStringFromError<Vec<u8>> {
    #[inline]
    fn from(e: std::string::FromUtf8Error) -> Self {
        let ue = e.utf8_error();
        Self::InvalidUtf8(GStringUtf8Error(e.into_bytes(), ue))
    }
}

impl IntoGlibPtr<*mut c_char> for GString {
    // rustdoc-stripper-ignore-next
    /// Transform into a nul-terminated raw C string pointer.
    #[inline]
    fn into_glib_ptr(self) -> *mut c_char {
        match self.0 {
            Inner::Native(ref s) => unsafe { ffi::g_strndup(s.as_ptr() as *const _, s.len()) },
            Inner::Foreign { ptr, .. } => {
                let _s = mem::ManuallyDrop::new(self);
                ptr.as_ptr()
            }
            Inner::Inline { len, ref data } => unsafe {
                ffi::g_strndup(data.as_ptr() as *const _, len as usize)
            },
        }
    }
}

impl Default for GString {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GString {
    #[inline]
    fn clone(&self) -> GString {
        self.as_str().into()
    }
}

impl fmt::Debug for GString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <&str as fmt::Debug>::fmt(&self.as_str(), f)
    }
}

impl Drop for GString {
    #[inline]
    fn drop(&mut self) {
        if let Inner::Foreign { ptr, .. } = self.0 {
            unsafe {
                ffi::g_free(ptr.as_ptr() as *mut _);
            }
        }
    }
}

impl fmt::Display for GString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl hash::Hash for GString {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl Borrow<GStr> for GString {
    #[inline]
    fn borrow(&self) -> &GStr {
        self.as_gstr()
    }
}

impl Borrow<str> for GString {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Ord for GString {
    #[inline]
    fn cmp(&self, other: &GString) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for GString {
    #[inline]
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GString {
    #[inline]
    fn eq(&self, other: &GString) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GString> for String {
    #[inline]
    fn eq(&self, other: &GString) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GStr> for GString {
    #[inline]
    fn eq(&self, other: &GStr) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<&GStr> for GString {
    #[inline]
    fn eq(&self, other: &&GStr) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<str> for GString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for GString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<GString> for &GStr {
    #[inline]
    fn eq(&self, other: &GString) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GString> for &str {
    #[inline]
    fn eq(&self, other: &GString) -> bool {
        *self == other.as_str()
    }
}

impl PartialEq<String> for GString {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialEq<GString> for str {
    #[inline]
    fn eq(&self, other: &GString) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<GString> for GStr {
    #[inline]
    fn eq(&self, other: &GString) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd<GString> for String {
    #[inline]
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.cmp(&String::from(other.as_str())))
    }
}

impl PartialOrd<String> for GString {
    #[inline]
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl PartialOrd<GString> for GStr {
    #[inline]
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl PartialOrd<GStr> for GString {
    #[inline]
    fn partial_cmp(&self, other: &GStr) -> Option<Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}

impl PartialOrd<GString> for str {
    #[inline]
    fn partial_cmp(&self, other: &GString) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for GString {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.as_str().cmp(other))
    }
}

impl Eq for GString {}

impl AsRef<GStr> for GString {
    #[inline]
    fn as_ref(&self) -> &GStr {
        self.as_gstr()
    }
}

impl AsRef<str> for GString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<OsStr> for GString {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_str())
    }
}

impl AsRef<Path> for GString {
    #[inline]
    fn as_ref(&self) -> &Path {
        Path::new(self.as_str())
    }
}

impl AsRef<[u8]> for GString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl Deref for GString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl From<GString> for String {
    #[inline]
    fn from(mut s: GString) -> Self {
        match &mut s.0 {
            Inner::Native(s) => {
                // Moves the underlying string
                let mut s = String::from(mem::replace(s, "".into()));
                let _nul = s.pop();
                debug_assert_eq!(_nul, Some('\0'));
                s
            }
            Inner::Foreign { len, .. } if *len == 0 => String::new(),
            Inner::Foreign { ptr, len } => unsafe {
                // Creates a copy
                let slice = slice::from_raw_parts(ptr.as_ptr() as *const u8, *len);
                std::str::from_utf8_unchecked(slice).into()
            },
            Inner::Inline { len, data } => unsafe {
                std::str::from_utf8_unchecked(data.get_unchecked(..*len as usize)).to_owned()
            },
        }
    }
}

impl From<GString> for Box<str> {
    #[inline]
    fn from(s: GString) -> Self {
        // Potentially creates a copy
        String::from(s).into()
    }
}

impl From<GString> for Vec<u8> {
    #[inline]
    fn from(value: GString) -> Vec<u8> {
        value.into_bytes_with_nul()
    }
}

impl TryFrom<GString> for CString {
    type Error = GStringInteriorNulError<GString>;
    #[inline]
    fn try_from(value: GString) -> Result<Self, Self::Error> {
        if let Some(nul_pos) = memchr::memchr(0, value.as_bytes()) {
            return Err(GStringInteriorNulError(
                value,
                GStrInteriorNulError(nul_pos),
            ));
        }
        let v = value.into_bytes_with_nul();
        Ok(unsafe { CString::from_vec_with_nul_unchecked(v) })
    }
}

impl From<GString> for OsString {
    #[inline]
    fn from(s: GString) -> Self {
        OsString::from(String::from(s))
    }
}

impl From<GString> for PathBuf {
    #[inline]
    fn from(s: GString) -> Self {
        PathBuf::from(OsString::from(s))
    }
}

impl From<String> for GString {
    #[inline]
    fn from(mut s: String) -> Self {
        // Moves the content of the String
        if cfg!(debug_assertions) {
            GStr::check_interior_nuls(&s).unwrap();
        }
        if s.is_empty() {
            Self::new()
        } else {
            s.reserve_exact(1);
            s.push('\0');
            // No check for valid UTF-8 here
            Self(Inner::Native(s.into()))
        }
    }
}

impl From<Box<str>> for GString {
    #[inline]
    fn from(s: Box<str>) -> Self {
        // Moves the content of the String
        s.into_string().into()
    }
}

impl<'a> From<Cow<'a, str>> for GString {
    #[inline]
    fn from(s: Cow<'a, str>) -> Self {
        match s {
            Cow::Borrowed(s) => Self::from(s),
            Cow::Owned(s) => Self::from(s),
        }
    }
}

impl From<&GStr> for GString {
    #[inline]
    fn from(s: &GStr) -> GString {
        s.to_owned()
    }
}

impl From<&str> for GString {
    #[inline]
    fn from(s: &str) -> Self {
        if cfg!(debug_assertions) {
            GStr::check_interior_nuls(s).unwrap();
        }
        if s.len() < INLINE_LEN {
            let mut data = <[u8; INLINE_LEN]>::default();
            let b = s.as_bytes();
            unsafe { data.get_unchecked_mut(..b.len()) }.copy_from_slice(b);
            return Self(Inner::Inline {
                len: b.len() as u8,
                data,
            });
        }
        // Allocates with the GLib allocator
        unsafe {
            // No check for valid UTF-8 here
            let copy = ffi::g_strndup(s.as_ptr() as *const c_char, s.len());
            GString(Inner::Foreign {
                ptr: ptr::NonNull::new_unchecked(copy),
                len: s.len(),
            })
        }
    }
}

impl From<&String> for GString {
    #[inline]
    fn from(s: &String) -> Self {
        GString::from(s.as_str())
    }
}

impl From<GStringPtr> for GString {
    #[inline]
    fn from(s: GStringPtr) -> Self {
        let s = mem::ManuallyDrop::new(s);
        let len = unsafe { GStr::from_ptr(s.0.as_ptr()).len() };
        GString(Inner::Foreign { ptr: s.0, len })
    }
}

impl TryFrom<CString> for GString {
    type Error = GStringUtf8Error<CString>;
    #[inline]
    fn try_from(value: CString) -> Result<Self, Self::Error> {
        if value.as_bytes().is_empty() {
            Ok(Self::new())
        } else {
            // Moves the content of the CString
            // Also check if it's valid UTF-8
            let s = String::from_utf8(value.into_bytes_with_nul()).map_err(|e| {
                let err = e.utf8_error();
                GStringUtf8Error(
                    unsafe { CString::from_vec_with_nul_unchecked(e.into_bytes()) },
                    err,
                )
            })?;
            Ok(Self(Inner::Native(s.into())))
        }
    }
}

impl TryFrom<OsString> for GString {
    type Error = GStringFromError<OsString>;
    #[inline]
    fn try_from(value: OsString) -> Result<Self, Self::Error> {
        Self::from_string_checked(value.into_string().map_err(GStringFromError::Unspecified)?)
            .map_err(|e| GStringFromError::from(e).convert(OsString::from))
    }
}

impl TryFrom<PathBuf> for GString {
    type Error = GStringFromError<PathBuf>;
    #[inline]
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        GString::try_from(value.into_os_string()).map_err(|e| e.convert(PathBuf::from))
    }
}

impl TryFrom<&CStr> for GString {
    type Error = std::str::Utf8Error;
    #[inline]
    fn try_from(value: &CStr) -> Result<Self, Self::Error> {
        // Check if it's valid UTF-8
        value.to_str()?;
        let gstr = unsafe { GStr::from_utf8_with_nul_unchecked(value.to_bytes_with_nul()) };
        Ok(gstr.to_owned())
    }
}

impl<'a> From<Cow<'a, GStr>> for GString {
    #[inline]
    fn from(s: Cow<'a, GStr>) -> Self {
        s.into_owned()
    }
}

impl<'a> From<&'a GString> for Cow<'a, GStr> {
    #[inline]
    fn from(s: &'a GString) -> Self {
        Cow::Borrowed(s.as_gstr())
    }
}

impl From<GString> for Cow<'_, GStr> {
    #[inline]
    fn from(v: GString) -> Self {
        Cow::Owned(v)
    }
}

impl<'a> From<&'a GStr> for Cow<'a, GStr> {
    #[inline]
    fn from(v: &'a GStr) -> Self {
        Cow::Borrowed(v)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut u8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut u8) -> Self {
        debug_assert!(!ptr.is_null());

        let cstr = CStr::from_ptr(ptr as *const _);
        // Check for valid UTF-8 here
        debug_assert!(cstr.to_str().is_ok());
        Self(Inner::Foreign {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            len: cstr.to_bytes().len(),
        })
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut i8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut i8) -> Self {
        from_glib_full(ptr as *mut u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const u8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *const u8) -> Self {
        from_glib_full(ptr as *mut u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const i8> for GString {
    #[inline]
    unsafe fn from_glib_full(ptr: *const i8) -> Self {
        from_glib_full(ptr as *mut u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const u8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *const u8) -> Self {
        debug_assert!(!ptr.is_null());
        <&GStr>::from_glib_none(ptr).into()
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const i8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *const i8) -> Self {
        from_glib_none(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut u8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut u8) -> Self {
        from_glib_none(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut i8> for GString {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut i8) -> Self {
        from_glib_none(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const u8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *const u8) -> Borrowed<Self> {
        debug_assert!(!ptr.is_null());

        // Check for valid UTF-8 here
        let cstr = CStr::from_ptr(ptr as *const _);
        debug_assert!(cstr.to_str().is_ok());
        Borrowed::new(Self(Inner::Foreign {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            len: cstr.to_bytes().len(),
        }))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const i8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *const i8) -> Borrowed<Self> {
        from_glib_borrow(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut u8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut u8) -> Borrowed<Self> {
        from_glib_borrow(ptr as *const u8)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut i8> for GString {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut i8) -> Borrowed<Self> {
        from_glib_borrow(ptr as *const u8)
    }
}

#[allow(clippy::unnecessary_cast)]
#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const u8> for GString {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const u8, Self> {
        Stash(self.as_ptr() as *const u8, PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *const u8 {
        self.clone().into_glib_ptr() as *const u8
    }
}

#[allow(clippy::unnecessary_cast)]
#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const i8> for GString {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const i8, Self> {
        Stash(self.as_ptr() as *const i8, PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *const i8 {
        self.clone().into_glib_ptr() as *const i8
    }
}

#[allow(clippy::unnecessary_cast)]
#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut u8> for GString {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut u8, Self> {
        Stash(self.as_ptr() as *mut u8, PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut u8 {
        self.clone().into_glib_ptr() as *mut u8
    }
}

#[allow(clippy::unnecessary_cast)]
#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut i8> for GString {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut i8, Self> {
        Stash(self.as_ptr() as *mut i8, PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut i8 {
        self.clone().into_glib_ptr() as *mut i8
    }
}

#[doc(hidden)]
impl FromGlibContainer<*const c_char, *const i8> for GString {
    unsafe fn from_glib_none_num(ptr: *const i8, num: usize) -> Self {
        if num == 0 || ptr.is_null() {
            return Self::default();
        }
        let slice = slice::from_raw_parts(ptr as *const u8, num);
        if cfg!(debug_assertions) {
            // Also check if it's valid UTF-8
            std::str::from_utf8(slice).unwrap().into()
        } else {
            std::str::from_utf8_unchecked(slice).into()
        }
    }

    unsafe fn from_glib_container_num(ptr: *const i8, num: usize) -> Self {
        if num == 0 || ptr.is_null() {
            return Self::default();
        }

        if cfg!(debug_assertions) {
            // Check if it's valid UTF-8
            let slice = slice::from_raw_parts(ptr as *const u8, num);
            std::str::from_utf8(slice).unwrap();
        }

        GString(Inner::Foreign {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            len: num,
        })
    }

    unsafe fn from_glib_full_num(ptr: *const i8, num: usize) -> Self {
        if num == 0 || ptr.is_null() {
            return Self::default();
        }

        if cfg!(debug_assertions) {
            // Check if it's valid UTF-8
            let slice = slice::from_raw_parts(ptr as *const u8, num);
            std::str::from_utf8(slice).unwrap();
        }

        GString(Inner::Foreign {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            len: num,
        })
    }
}

#[doc(hidden)]
impl FromGlibContainer<*const c_char, *mut i8> for GString {
    unsafe fn from_glib_none_num(ptr: *mut i8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut i8, num: usize) -> Self {
        FromGlibContainer::from_glib_container_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_full_num(ptr: *mut i8, num: usize) -> Self {
        FromGlibContainer::from_glib_full_num(ptr as *const i8, num)
    }
}

#[doc(hidden)]
impl FromGlibContainer<*const c_char, *const u8> for GString {
    unsafe fn from_glib_none_num(ptr: *const u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_container_num(ptr: *const u8, num: usize) -> Self {
        FromGlibContainer::from_glib_container_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_full_num(ptr: *const u8, num: usize) -> Self {
        FromGlibContainer::from_glib_full_num(ptr as *const i8, num)
    }
}

#[doc(hidden)]
impl FromGlibContainer<*const c_char, *mut u8> for GString {
    unsafe fn from_glib_none_num(ptr: *mut u8, num: usize) -> Self {
        FromGlibContainer::from_glib_none_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_container_num(ptr: *mut u8, num: usize) -> Self {
        FromGlibContainer::from_glib_container_num(ptr as *const i8, num)
    }

    unsafe fn from_glib_full_num(ptr: *mut u8, num: usize) -> Self {
        FromGlibContainer::from_glib_full_num(ptr as *const i8, num)
    }
}

impl GlibPtrDefault for GString {
    type GlibType = *const c_char;
}

impl StaticType for GString {
    #[inline]
    fn static_type() -> Type {
        String::static_type()
    }
}

impl crate::value::ValueType for GString {
    type Type = String;
}

impl crate::value::ValueTypeOptional for GString {}

unsafe impl<'a> crate::value::FromValue<'a> for GString {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a Value) -> Self {
        Self::from(<&str>::from_value(value))
    }
}

impl crate::value::ToValue for GString {
    #[inline]
    fn to_value(&self) -> Value {
        <&str>::to_value(&self.as_str())
    }

    #[inline]
    fn value_type(&self) -> Type {
        String::static_type()
    }
}

impl crate::value::ToValueOptional for GString {
    #[inline]
    fn to_value_optional(s: Option<&Self>) -> Value {
        <str>::to_value_optional(s.as_ref().map(|s| s.as_str()))
    }
}

impl From<GString> for Value {
    #[inline]
    fn from(s: GString) -> Self {
        unsafe {
            let mut value = Value::for_value_type::<GString>();
            gobject_ffi::g_value_take_string(value.to_glib_none_mut().0, s.into_glib_ptr());
            value
        }
    }
}

impl StaticType for Vec<GString> {
    #[inline]
    fn static_type() -> Type {
        <Vec<String>>::static_type()
    }
}

impl crate::value::ValueType for Vec<GString> {
    type Type = Vec<GString>;
}

unsafe impl<'a> crate::value::FromValue<'a> for Vec<GString> {
    type Checker = crate::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a Value) -> Self {
        let ptr = gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const *const c_char;
        FromGlibPtrContainer::from_glib_none(ptr)
    }
}

impl ToValue for Vec<GString> {
    #[inline]
    fn to_value(&self) -> Value {
        unsafe {
            let mut value = Value::for_value_type::<Self>();
            let ptr: *mut *mut c_char = self.to_glib_full();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, ptr as *const c_void);
            value
        }
    }

    #[inline]
    fn value_type(&self) -> Type {
        <Vec<GString>>::static_type()
    }
}

impl From<Vec<GString>> for Value {
    #[inline]
    fn from(v: Vec<GString>) -> Self {
        unsafe {
            let v_ptr =
                ffi::g_malloc(mem::size_of::<*mut c_char>() * (v.len() + 1)) as *mut *mut c_char;
            v_ptr.add(v.len()).write(ptr::null_mut());
            for (i, s) in v.into_iter().enumerate() {
                v_ptr.add(i).write(s.into_glib_ptr());
            }

            let mut value = Value::for_value_type::<Vec<GString>>();
            gobject_ffi::g_value_take_boxed(value.to_glib_none_mut().0, v_ptr as *const c_void);
            value
        }
    }
}

impl_from_glib_container_as_vec_string!(GString, *const c_char);
impl_from_glib_container_as_vec_string!(GString, *mut c_char);

// rustdoc-stripper-ignore-next
/// A trait to accept both <code>&[str]</code> or <code>&[GStr]</code> as an argument.
pub trait IntoGStr {
    fn run_with_gstr<T, F: FnOnce(&GStr) -> T>(self, f: F) -> T;
}

impl IntoGStr for &GStr {
    #[inline]
    fn run_with_gstr<T, F: FnOnce(&GStr) -> T>(self, f: F) -> T {
        f(self)
    }
}

impl IntoGStr for GString {
    #[inline]
    fn run_with_gstr<T, F: FnOnce(&GStr) -> T>(self, f: F) -> T {
        f(self.as_gstr())
    }
}

impl IntoGStr for &GString {
    #[inline]
    fn run_with_gstr<T, F: FnOnce(&GStr) -> T>(self, f: F) -> T {
        f(self.as_gstr())
    }
}

// Limit borrowed from rust std CStr optimization:
// https://github.com/rust-lang/rust/blob/master/library/std/src/sys/common/small_c_string.rs#L10
const MAX_STACK_ALLOCATION: usize = 384;

impl IntoGStr for &str {
    #[inline]
    fn run_with_gstr<T, F: FnOnce(&GStr) -> T>(self, f: F) -> T {
        if self.len() < MAX_STACK_ALLOCATION {
            let mut s = mem::MaybeUninit::<[u8; MAX_STACK_ALLOCATION]>::uninit();
            let ptr = s.as_mut_ptr() as *mut u8;
            let gs = unsafe {
                ptr::copy_nonoverlapping(self.as_ptr(), ptr, self.len());
                ptr.add(self.len()).write(0);
                GStr::from_utf8_with_nul_unchecked(slice::from_raw_parts(ptr, self.len() + 1))
            };
            f(gs)
        } else {
            f(GString::from(self).as_gstr())
        }
    }
}

impl IntoGStr for String {
    #[inline]
    fn run_with_gstr<T, F: FnOnce(&GStr) -> T>(mut self, f: F) -> T {
        let len = self.len();
        if len < self.capacity() {
            self.reserve_exact(1);
            self.push('\0');
            let gs = unsafe { GStr::from_utf8_with_nul_unchecked(self.as_bytes()) };
            f(gs)
        } else if len < MAX_STACK_ALLOCATION {
            self.as_str().run_with_gstr(f)
        } else {
            f(GString::from(self).as_gstr())
        }
    }
}

impl IntoGStr for &String {
    #[inline]
    fn run_with_gstr<T, F: FnOnce(&GStr) -> T>(self, f: F) -> T {
        self.as_str().run_with_gstr(f)
    }
}

pub const NONE_STR: Option<&'static str> = None;

// rustdoc-stripper-ignore-next
/// A trait to accept both <code>[Option]&lt;&[str]></code> or <code>[Option]&lt;&[GStr]></code> as
/// an argument.
pub trait IntoOptionalGStr {
    fn run_with_gstr<T, F: FnOnce(Option<&GStr>) -> T>(self, f: F) -> T;
}

impl<S: IntoGStr> IntoOptionalGStr for Option<S> {
    #[inline]
    fn run_with_gstr<T, F: FnOnce(Option<&GStr>) -> T>(self, f: F) -> T {
        match self {
            Some(t) => t.run_with_gstr(|s| f(Some(s))),
            None => f(None),
        }
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_names)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn test_gstring() {
        let data = CString::new("foo").unwrap();
        let ptr = data.as_ptr();

        unsafe {
            let ptr_copy = ffi::g_strdup(ptr);
            let gstring = GString::from_glib_full(ptr_copy);
            assert_eq!(gstring.as_str(), "foo");
            let foo: Box<str> = gstring.into();
            assert_eq!(foo.as_ref(), "foo");
        }
    }

    #[test]
    fn test_owned_glib_string() {
        let data = CString::new("foo").unwrap();
        let ptr = data.as_ptr();
        unsafe {
            let ptr_copy = ffi::g_strdup(ptr);
            let gstr = GString::from_glib_full(ptr_copy);
            assert_eq!(gstr, "foo");
        }
    }

    #[test]
    fn test_gstring_from_str() {
        let gstring: GString = "foo".into();
        assert_eq!(gstring.as_str(), "foo");
        let foo: Box<str> = gstring.into();
        assert_eq!(foo.as_ref(), "foo");
    }

    #[test]
    fn test_string_from_gstring() {
        let gstring = GString::from("foo");
        assert_eq!(gstring.as_str(), "foo");
        let s = String::from(gstring);
        assert_eq!(s, "foo");
    }

    #[test]
    fn test_gstring_from_cstring() {
        let cstr = CString::new("foo").unwrap();
        let gstring = GString::try_from(cstr).unwrap();
        assert_eq!(gstring.as_str(), "foo");
        let foo: Box<str> = gstring.into();
        assert_eq!(foo.as_ref(), "foo");
    }

    #[test]
    fn test_string_from_gstring_from_cstring() {
        let cstr = CString::new("foo").unwrap();
        let gstring = GString::try_from(cstr).unwrap();
        assert_eq!(gstring.as_str(), "foo");
        let s = String::from(gstring);
        assert_eq!(s, "foo");
    }

    #[test]
    fn test_vec_u8_to_gstring() {
        let v: &[u8] = b"foo";
        let s: GString = GString::from_utf8(Vec::from(v)).unwrap();
        assert_eq!(s.as_str(), "foo");
    }

    #[test]
    fn test_value_from_vec_gstring() {
        fn roundtrip(s: GString) {
            let vec = vec![s.clone()];
            let value = crate::Value::from(vec);
            let vec: Vec<GString> = value.get().unwrap();
            assert_eq!(vec.len(), 1);
            assert_eq!(s, vec[0]);
        }

        roundtrip(GString::from("foo"));
        roundtrip(GString::from("very very very long string".to_owned()));
        roundtrip(GString::from(gstr!("very very very long string")));
    }

    #[test]
    fn test_as_ref_path() {
        fn foo<P: AsRef<Path>>(_path: P) {}
        let gstring: GString = "/my/path/".into();
        let gstr: &GStr = gstring.as_gstr();
        foo(gstr);
        foo(gstring);
    }

    #[test]
    fn test_from_glib_container() {
        unsafe {
            let test_a: GString = FromGlibContainer::from_glib_container_num(
                ffi::g_strdup("hello_world\0".as_ptr() as *const _),
                5,
            );
            assert_eq!("hello", test_a.as_str());

            let test_b: GString = FromGlibContainer::from_glib_none_num("hello_world".as_ptr(), 5);
            assert_eq!("hello", test_b.as_str());

            let test_c: GString =
                FromGlibContainer::from_glib_none_num(std::ptr::null::<std::os::raw::c_char>(), 0);
            assert_eq!("", test_c.as_str());

            let test_d: GString = FromGlibContainer::from_glib_none_num("".as_ptr(), 0);
            assert_eq!("", test_d.as_str());

            let test_e: GString =
                FromGlibContainer::from_glib_container_num(ffi::g_strdup(std::ptr::null()), 0);
            assert_eq!("", test_e.as_str());
        }
    }

    #[test]
    fn test_hashmap() {
        use std::collections::HashMap;

        let gstring = GString::from("foo");
        assert_eq!(gstring.as_str(), "foo");
        let mut h: HashMap<GString, i32> = HashMap::new();
        h.insert(gstring, 42);
        let gstring: GString = "foo".into();
        assert!(h.contains_key(&gstring));
    }

    #[test]
    fn test_gstring_from_ptr_lossy() {
        let data = CString::new("foo").unwrap();
        let ptr = data.as_ptr();

        unsafe {
            let gstring = GString::from_ptr_lossy(ptr);
            assert_eq!(gstring.as_str(), "foo");
            assert_eq!(ptr, gstring.as_ptr());
        }

        let data = b"foo\xF0\x90\x80bar\0";
        let ptr = data.as_ptr();

        unsafe {
            let gstring = GString::from_ptr_lossy(ptr as *const _);
            assert_eq!(gstring.as_str(), "foo���bar");
            assert_ne!(ptr, gstring.as_ptr() as *const _);
        }
    }

    #[test]
    fn gformat() {
        let s = gformat!("bla bla {} bla", 123);
        assert_eq!(s, "bla bla 123 bla");
    }

    #[test]
    fn layout() {
        // ensure the inline variant is not wider than the other variants
        enum NoInline {
            _Native(Box<str>),
            _Foreign(ptr::NonNull<c_char>, usize),
        }
        assert_eq!(mem::size_of::<GString>(), mem::size_of::<NoInline>());
        assert_eq!(mem::size_of::<GString>(), mem::size_of::<String>());
    }
}
