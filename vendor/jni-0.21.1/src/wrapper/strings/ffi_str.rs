use std::{
    borrow::{Borrow, Cow, ToOwned},
    ffi,
    os::raw::c_char,
};

use cesu8::{from_java_cesu8, to_java_cesu8};
use log::debug;

use crate::wrapper::strings::ffi_str;

/// Wrapper for `std::ffi::CString` that also takes care of encoding between
/// UTF-8 and Java's Modified UTF-8. As with `CString`, this implements `Deref`
/// to `&JNIStr`.
pub struct JNIString {
    internal: ffi::CString,
}

/// Wrapper for `std::ffi::CStr` that also takes care of encoding between
/// UTF-8 and Java's Modified UTF-8.
pub struct JNIStr {
    internal: ffi::CStr,
}

impl ::std::ops::Deref for JNIString {
    type Target = JNIStr;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.internal.as_bytes_with_nul() as *const [u8] as *const ffi_str::JNIStr) }
    }
}

impl ::std::ops::Deref for JNIStr {
    type Target = ffi::CStr;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T> From<T> for JNIString
where
    T: AsRef<str>,
{
    fn from(other: T) -> Self {
        let enc = to_java_cesu8(other.as_ref()).into_owned();
        JNIString {
            internal: unsafe { ffi::CString::from_vec_unchecked(enc) },
        }
    }
}

impl<'str_ref> From<&'str_ref JNIStr> for Cow<'str_ref, str> {
    fn from(other: &'str_ref JNIStr) -> Cow<'str_ref, str> {
        let bytes = other.to_bytes();
        match from_java_cesu8(bytes) {
            Ok(s) => s,
            Err(e) => {
                debug!("error decoding java cesu8: {:#?}", e);
                String::from_utf8_lossy(bytes)
            }
        }
    }
}

impl From<JNIString> for String {
    fn from(other: JNIString) -> String {
        Cow::from(other.borrowed()).into_owned()
    }
}

impl JNIString {
    /// Get the borrowed version of the JNIString. Equivalent to
    /// `CString::borrowed`.
    pub fn borrowed(&self) -> &JNIStr {
        self
    }
}

impl JNIStr {
    /// Construct a reference to a `JNIStr` from a pointer. Equivalent to `CStr::from_ptr`.
    ///
    /// # Safety
    ///
    /// Expects a valid pointer to a null-terminated C string and does not perform any lifetime
    /// checks for the resulting value.
    pub unsafe fn from_ptr<'jni_str>(ptr: *const c_char) -> &'jni_str JNIStr {
        &*(ffi::CStr::from_ptr(ptr) as *const ffi::CStr as *const ffi_str::JNIStr)
    }
}

// impls for CoW
impl Borrow<JNIStr> for JNIString {
    fn borrow(&self) -> &JNIStr {
        self
    }
}

impl ToOwned for JNIStr {
    type Owned = JNIString;

    fn to_owned(&self) -> JNIString {
        unsafe {
            JNIString {
                internal: ffi::CString::from_vec_unchecked(self.to_bytes().to_vec()),
            }
        }
    }
}
