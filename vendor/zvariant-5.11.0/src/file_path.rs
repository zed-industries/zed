use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::Type;

/// File name represented as a nul-terminated byte array.
///
/// While `zvariant::Type` and `serde::{Serialize, Deserialize}`, are implemented for [`Path`] and
/// [`PathBuf`], unfortunately `serde` serializes them as UTF-8 strings and that limits the number
/// of possible characters to use on a file path. This is not the desired behavior since file paths
/// are not guaranteed to contain only UTF-8 characters.
///
/// To solve this problem, this type is provided which encodes the underlying file path as a
/// null-terminated byte array.
///
/// # Examples:
///
/// ```
/// use zvariant::FilePath;
/// use std::path::{Path, PathBuf};
///
/// let path = Path::new("/hello/world\0");
/// let path_buf = PathBuf::from(path);
///
/// let p1 = FilePath::from(path);
/// let p2 = FilePath::from(path_buf);
/// let p3 = FilePath::from("/hello/world");
///
/// assert_eq!(p1, p2);
/// assert_eq!(p2, p3);
/// ```
#[derive(Type, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone, Ord, PartialOrd)]
#[zvariant(signature = "ay")]
pub struct FilePath<'f>(Cow<'f, CStr>);

impl<'f> FilePath<'f> {
    pub fn new(cow: Cow<'f, CStr>) -> Self {
        Self(cow)
    }

    /// Returns a lossy UTF-8 representation of the file path.
    ///
    /// Invalid UTF-8 sequences are replaced with `U+FFFD REPLACEMENT CHARACTER`.
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.0.to_string_lossy()
    }
}

impl From<CString> for FilePath<'_> {
    fn from(value: CString) -> Self {
        FilePath(Cow::Owned(value))
    }
}

impl<'f> From<&'f CString> for FilePath<'f> {
    fn from(value: &'f CString) -> Self {
        FilePath(Cow::Borrowed(value.as_c_str()))
    }
}

impl<'f> From<&'f OsStr> for FilePath<'f> {
    fn from(value: &'f OsStr) -> FilePath<'f> {
        FilePath(bytes_with_null(value.as_encoded_bytes()))
    }
}

impl<'f> From<&'f OsString> for FilePath<'f> {
    fn from(value: &'f OsString) -> FilePath<'f> {
        FilePath(bytes_with_null(value.as_encoded_bytes()))
    }
}

impl From<OsString> for FilePath<'_> {
    fn from(value: OsString) -> Self {
        #[allow(deprecated)]
        FilePath(vec_to_cstr(value.as_encoded_bytes().to_vec()))
    }
}

impl<'f> From<&'f PathBuf> for FilePath<'f> {
    fn from(value: &'f PathBuf) -> FilePath<'f> {
        FilePath::from(value.as_os_str())
    }
}

impl From<PathBuf> for FilePath<'_> {
    fn from(value: PathBuf) -> FilePath<'static> {
        FilePath::from(OsString::from(value))
    }
}

impl<'f> From<&'f Path> for FilePath<'f> {
    fn from(value: &'f Path) -> Self {
        Self::from(value.as_os_str())
    }
}

impl<'f> From<&'f CStr> for FilePath<'f> {
    fn from(value: &'f CStr) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<'f> From<&'f str> for FilePath<'f> {
    fn from(value: &'f str) -> Self {
        Self::from(OsStr::new(value))
    }
}

impl<'f> AsRef<FilePath<'f>> for FilePath<'f> {
    fn as_ref(&self) -> &FilePath<'f> {
        self
    }
}

impl From<FilePath<'_>> for OsString {
    fn from(value: FilePath<'_>) -> Self {
        // SAFETY: user is responsible of handling conversion from [FilePath] to [OsString]
        // since FilePath is a set of null terminated bytes and it's interpretations mainly
        // depends on the underlying platform.
        // see [std::ffi::os_str::OsString::from_encoded_bytes_unchecked]
        unsafe { OsString::from_encoded_bytes_unchecked(value.0.to_bytes().to_vec()) }
    }
}

impl<'f> From<&'f FilePath<'f>> for &'f Path {
    fn from(value: &'f FilePath<'f>) -> Self {
        // This method should fail if FilePath does not represent UTF-8 valid chars
        // since [Path] is akin to [str], hence the unwrap.
        Path::new(value.0.as_ref().to_str().unwrap())
    }
}

impl<'f> From<FilePath<'f>> for PathBuf {
    fn from(value: FilePath<'f>) -> Self {
        PathBuf::from(value.0.to_string_lossy().to_string())
    }
}

/// Converts a `Vec<u8>` into a null-terminated `CStr`.
///
/// Truncates the vector at the first null byte, if present. If no null byte exists, appends one to
/// ensure proper termination.
///
/// # Returns
///
/// A `Cow<'_, CStr>` containing a *guaranteed* null-terminated string.
#[doc(hidden)]
#[deprecated(
    since = "5.9.0",
    note = "This function was never meant to be public and will be removed in a future release."
)]
pub fn vec_to_cstr(mut bytes: Vec<u8>) -> Cow<'static, CStr> {
    if let Some(pos) = bytes.iter().position(|&b| b == 0) {
        bytes.truncate(pos + 1);
    } else {
        bytes.push(0);
    }
    // unwrap is fine here since we append the null byte.
    Cow::Owned(CString::from_vec_with_nul(bytes).unwrap())
}

/// Converts a byte slice into a null-terminated [CStr].
///
/// Returns a borrowed [CStr] if the slice already contains a null byte; otherwise, returns an
/// owned [CStr] with a null byte appended.
///
/// # Returns
///
/// A [Cow<'_, CStr>] containing a *guaranteed* null-terminated string.
fn bytes_with_null(bytes: &[u8]) -> Cow<'_, CStr> {
    if let Ok(cstr) = CStr::from_bytes_until_nul(bytes) {
        return Cow::Borrowed(cstr);
    }
    // unwrap is fine, as we handled the null termination case above.
    Cow::Owned(CString::new(bytes).unwrap())
}

#[cfg(test)]
mod file_path_test {
    use super::*;
    use crate::zvariant::Signature;
    use std::path::{Path, PathBuf};

    #[test]
    fn from_test() {
        let path = Path::new("/hello/world");
        let path_buf = PathBuf::from(path);
        let osstr = OsStr::new("/hello/world");
        let os_string = OsString::from("/hello/world");
        let cstr = CStr::from_bytes_until_nul("/hello/world\0".as_bytes()).unwrap_or_default();
        let cstring = CString::new("/hello/world").unwrap_or_default();

        let p1 = FilePath::from(path);
        let p2 = FilePath::from(path_buf);
        let p3 = FilePath::from(osstr);
        let p4 = FilePath::from(os_string);
        let p5 = FilePath::from(cstr);
        let p6 = FilePath::from(cstring);
        let p7 = FilePath::from("/hello/world");

        assert_eq!(p1, p2);
        assert_eq!(p2, p3);
        assert_eq!(p3, p4);
        assert_eq!(p4, p5);
        assert_eq!(p5, p6);
        assert_eq!(p5, p7);
    }

    #[test]
    fn filepath_signature() {
        assert_eq!(
            &Signature::static_array(&Signature::U8),
            FilePath::SIGNATURE
        );
    }

    #[test]
    fn into_test() {
        let first = PathBuf::from("/hello/world");
        let third = OsString::from("/hello/world");
        let fifth = Path::new("/hello/world");
        let p = FilePath::from(first.clone());
        let p2 = FilePath::from(third.clone());
        let p3 = FilePath::from(fifth);
        let second: PathBuf = p.into();
        let forth: OsString = p2.into();
        let sixth: &Path = (&p3).into();
        assert_eq!(first, second);
        assert_eq!(third, forth);
        assert_eq!(fifth, sixth);
    }

    #[test]
    fn vec_nul_termination() {
        #[allow(deprecated)]
        fn call_vec_to_cstr(v: Vec<u8>) -> Cow<'static, CStr> {
            vec_to_cstr(v)
        }
        let v1 = vec![];
        let v2 = vec![0x0];
        let v3 = vec![0x1, 0x2, 0x0];
        let v4 = vec![0x0, 0x0];
        let v5 = vec![0x1, 0x0, 0x2, 0x0];

        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x0]).unwrap()),
            call_vec_to_cstr(v1)
        );
        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x0]).unwrap()),
            call_vec_to_cstr(v2)
        );
        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x1, 0x2, 0x0]).unwrap()),
            call_vec_to_cstr(v3)
        );
        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x0]).unwrap()),
            call_vec_to_cstr(v4)
        );
        assert_eq!(
            Cow::Borrowed(CStr::from_bytes_with_nul(&[0x1, 0x0]).unwrap()),
            call_vec_to_cstr(v5)
        );
    }
}
