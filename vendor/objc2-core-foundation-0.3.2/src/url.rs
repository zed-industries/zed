#[allow(unused_imports)]
use crate::{CFIndex, CFRetained, CFURL};

/// [`Path`][std::path::Path] conversion.
#[cfg(feature = "std")]
#[cfg(unix)] // TODO: Use as_encoded_bytes/from_encoded_bytes_unchecked once in MSRV.
impl CFURL {
    #[inline]
    fn from_path(
        path: &std::path::Path,
        is_directory: bool,
        base: Option<&CFURL>,
    ) -> Option<CFRetained<CFURL>> {
        use std::os::unix::ffi::OsStrExt;

        // CFURL expects to get a string with the system encoding, and will
        // internally handle the different encodings, depending on if compiled
        // for Apple platforms or Windows (which is very rare, but could
        // technically happen).
        let bytes = path.as_os_str().as_bytes();

        // Never gonna happen, allocations can't be this large in Rust.
        debug_assert!(bytes.len() < CFIndex::MAX as usize);
        let len = bytes.len() as CFIndex;

        if let Some(base) = base {
            // The base URL must be a directory URL (have a trailing "/").
            // If the path is absolute, this URL is ignored.
            //
            // TODO: Expose this publicly?
            unsafe {
                Self::from_file_system_representation_relative_to_base(
                    None,
                    bytes.as_ptr(),
                    len,
                    is_directory,
                    Some(base),
                )
            }
        } else {
            unsafe {
                Self::from_file_system_representation(None, bytes.as_ptr(), len, is_directory)
            }
        }
    }

    /// Create a file url from a [`Path`][std::path::Path].
    ///
    /// This is useful because a lot of CoreFoundation APIs use `CFURL` to
    /// represent file-system paths as well.
    ///
    /// Non-unicode parts of the URL will be percent-encoded, and the url will
    /// have the scheme `file://`.
    ///
    /// If the path is relative, it will be considered relative to the current
    /// directory.
    ///
    /// Returns `None` when given an invalid path (such as a path containing
    /// interior NUL bytes). The exact checks are not guaranteed.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use objc2_core_foundation::CFURL;
    ///
    /// // Absolute paths work as you'd expect.
    /// let url = CFURL::from_file_path("/tmp/file.txt").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/tmp/file.txt"));
    ///
    /// // Relative paths are relative to the current directory.
    /// let url = CFURL::from_file_path("foo.txt").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), std::env::current_dir().unwrap().join("foo.txt"));
    ///
    /// // Some invalid paths return `None`.
    /// assert!(CFURL::from_file_path("").is_none());
    /// // Another example of an invalid path containing interior NUL bytes.
    /// assert!(CFURL::from_file_path("/a/\0a").is_none());
    ///
    /// // Trailing NUL bytes are stripped.
    /// // NOTE: This only seems to work on some versions of CoreFoundation.
    /// let url = CFURL::from_file_path("/a\0\0").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/a"));
    /// ```
    #[inline]
    #[doc(alias = "CFURLCreateFromFileSystemRepresentation")]
    pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Option<CFRetained<CFURL>> {
        Self::from_path(path.as_ref(), false, None)
    }

    /// Create a directory url from a [`Path`][std::path::Path].
    ///
    /// This differs from [`from_file_path`][Self::from_file_path] in that the
    /// path is treated as a directory, which means that other normalization
    /// rules are applied to it (to make it end with a `/`).
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use objc2_core_foundation::CFURL;
    ///
    /// // Directory paths get trailing slashes appended
    /// let url = CFURL::from_directory_path("/Library").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/Library/"));
    ///
    /// // Unless they already have them.
    /// let url = CFURL::from_directory_path("/Library/").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/Library/"));
    ///
    /// // Similarly for relative paths.
    /// let url = CFURL::from_directory_path("foo").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), std::env::current_dir().unwrap().join("foo/"));
    ///
    /// // Various dots may be stripped.
    /// let url = CFURL::from_directory_path("/Library/././.").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/Library/"));
    ///
    /// // Though of course not if they have semantic meaning.
    /// let url = CFURL::from_directory_path("/Library/..").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/Library/.."));
    /// ```
    #[inline]
    #[doc(alias = "CFURLCreateFromFileSystemRepresentation")]
    pub fn from_directory_path<P: AsRef<std::path::Path>>(path: P) -> Option<CFRetained<CFURL>> {
        Self::from_path(path.as_ref(), true, None)
    }

    /// Extract the path part of the URL as a [`PathBuf`][std::path::PathBuf].
    ///
    /// This will return a path regardless of whether the scheme is `file://`.
    /// It is the responsibility of the caller to ensure that the URL is valid
    /// to use as a file URL.
    ///
    ///
    /// # Compatibility note
    ///
    /// This currently does not work for non-unicode paths (which are fairly
    /// rare on macOS since HFS+ was been superseded by APFS).
    ///
    /// This also currently always returns absolute paths (it converts
    /// relative URL paths to absolute), but that may change in the future.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use objc2_core_foundation::{CFURL, CFString};
    ///
    /// let url = CFURL::from_string(None, &CFString::from_str("file:///tmp/foo.txt"), None).unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/tmp/foo.txt"));
    /// ```
    ///
    /// See also the examples in [`from_file_path`][Self::from_file_path].
    #[doc(alias = "CFURLGetFileSystemRepresentation")]
    pub fn to_file_path(&self) -> Option<std::path::PathBuf> {
        use std::os::unix::ffi::OsStrExt;

        const PATH_MAX: usize = 1024;

        // TODO: if a path is relative with no base, how do we get that
        // relative path out again (without adding current dir?).
        //
        // TODO: Should we do something to handle paths larger than PATH_MAX?
        // What can we even do? (since it's impossible for us to tell why the
        // conversion failed, so we can't know if we need to allocate, or if
        // the URL just cannot be converted).
        let mut buf = [0u8; PATH_MAX];
        let result = unsafe {
            self.file_system_representation(true, buf.as_mut_ptr(), buf.len() as CFIndex)
        };
        if !result {
            return None;
        }

        // SAFETY: CF is guaranteed to null-terminate the buffer if
        // the function succeeded.
        let cstr = unsafe { core::ffi::CStr::from_bytes_until_nul(&buf).unwrap_unchecked() };

        let path = std::ffi::OsStr::from_bytes(cstr.to_bytes());
        Some(path.into())
    }
}

/// String conversion.
impl CFURL {
    /// Create an URL from a `CFString`.
    ///
    /// Returns `None` if the URL is considered invalid by CoreFoundation. The
    /// exact details of which strings are invalid URLs are considered an
    /// implementation detail.
    ///
    /// Note in particular that not all strings that the URL spec considers
    /// invalid are considered invalid by CoreFoundation too. If you need
    /// spec-compliant parsing, consider the [`url`] crate instead.
    ///
    /// [`url`]: https://docs.rs/url/
    ///
    /// # Examples
    ///
    /// Construct and inspect a `CFURL`.
    ///
    /// ```
    /// use objc2_core_foundation::{
    ///     CFString, CFURL, CFURLCopyHostName, CFURLCopyScheme, CFURLCopyPath,
    /// };
    ///
    /// let url = CFURL::from_string(None, &CFString::from_str("http://example.com/foo"), None).unwrap();
    /// assert_eq!(url.string().to_string(), "http://example.com/foo");
    /// assert_eq!(CFURLCopyScheme(&url).unwrap().to_string(), "http");
    /// assert_eq!(CFURLCopyHostName(&url).unwrap().to_string(), "example.com");
    /// assert_eq!(CFURLCopyPath(&url).unwrap().to_string(), "/foo");
    /// ```
    ///
    /// Fail parsing certain strings.
    ///
    /// ```
    /// use objc2_core_foundation::{CFString, CFURL};
    ///
    /// // Percent-encoding needs two characters.
    /// assert_eq!(CFURL::from_string(None, &CFString::from_str("http://example.com/%A"), None), None);
    ///
    /// // Two hash-characters is disallowed.
    /// assert_eq!(CFURL::from_string(None, &CFString::from_str("http://example.com/abc#a#b"), None), None);
    /// ```
    #[inline]
    #[doc(alias = "CFURLCreateWithString")]
    pub fn from_string(
        allocator: Option<&crate::CFAllocator>,
        url_string: &crate::CFString,
        base_url: Option<&CFURL>,
    ) -> Option<CFRetained<Self>> {
        Self::__from_string(allocator, Some(url_string), base_url)
    }

    /// Create an URL from a string without checking it for validity.
    ///
    /// Returns `None` on some OS versions when the string contains interior
    /// NUL bytes.
    ///
    /// # Safety
    ///
    /// The URL must be valid.
    ///
    /// Note that it is unclear whether this is actually a safety requirement,
    /// or simply a correctness requirement. So we conservatively mark this
    /// function as `unsafe`.
    #[inline]
    #[cfg(feature = "CFString")]
    #[doc(alias = "CFURLCreateWithBytes")]
    pub unsafe fn from_str_unchecked(s: &str) -> Option<CFRetained<Self>> {
        let ptr = s.as_ptr();

        // Never gonna happen, allocations can't be this large in Rust.
        debug_assert!(s.len() < CFIndex::MAX as usize);
        let len = s.len() as CFIndex;

        let encoding = crate::CFStringBuiltInEncodings::EncodingUTF8;
        // SAFETY: The pointer and length are valid, and the encoding is a
        // superset of ASCII.
        //
        // Unlike `CFURLCreateWithString`, this does _not_ verify the URL at
        // all, and thus we propagate the validity checks to the user. See
        // also the source code for the checks:
        // https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFURL.c#L1882-L1970
        unsafe { Self::with_bytes(None, ptr, len, encoding.0, None) }
    }

    /// Get the string-representation of the URL.
    ///
    /// The string may be overly sanitized (percent-encoded), do not rely on
    /// this returning exactly the same string as was passed in
    /// [`from_string`][Self::from_string].
    #[doc(alias = "CFURLGetString")]
    pub fn string(&self) -> CFRetained<crate::CFString> {
        // URLs contain valid UTF-8, so this should only fail on allocation
        // error.
        self.__string().expect("failed getting string from CFURL")
    }
}

#[cfg(unix)]
#[cfg(test)]
#[cfg(feature = "CFString")]
#[cfg(feature = "std")]
mod tests {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;
    use std::{env::current_dir, string::ToString};

    use crate::{CFString, CFURLPathStyle};

    use super::*;

    #[test]
    fn from_string() {
        let url =
            CFURL::from_string(None, &CFString::from_str("https://example.com/xyz"), None).unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new("/xyz"));
        assert_eq!(url.string().to_string(), "https://example.com/xyz");

        // Invalid.
        let url = CFURL::from_string(None, &CFString::from_str("\0"), None);
        assert_eq!(url, None);

        // Also invalid.
        let url = CFURL::from_string(None, &CFString::from_str("http://example.com/%a"), None);
        assert_eq!(url, None);

        // Though using `from_str_unchecked` succeeds.
        let url = unsafe { CFURL::from_str_unchecked("http://example.com/%a") }.unwrap();
        assert_eq!(url.string().to_string(), "http://example.com/%25a");
        assert_eq!(url.to_file_path().unwrap(), Path::new("/%a"));

        let url = unsafe { CFURL::from_str_unchecked("/\0a\0") }.unwrap();
        assert_eq!(url.string().to_string(), "/%00a%00");
        assert_eq!(url.to_file_path(), None);
    }

    #[test]
    fn to_string_may_extra_percent_encode() {
        let url = CFURL::from_string(None, &CFString::from_str("["), None).unwrap();
        assert_eq!(url.string().to_string(), "%5B");
    }

    #[test]
    #[cfg(feature = "objc2")]
    fn invalid_with_nul_bytes() {
        // This is a bug in newer CF versions:
        // https://github.com/swiftlang/swift-corelibs-foundation/issues/5200
        let url = unsafe { CFURL::from_str_unchecked("a\0aaaaaa") };
        if objc2::available!(macos = 12.0, ios = 15.0, watchos = 8.0, tvos = 15.0, ..) {
            assert_eq!(url, None);
        } else {
            assert_eq!(url.unwrap().string().to_string(), "a%00aaaaaa");
        }
    }

    #[test]
    fn to_from_path() {
        let url = CFURL::from_file_path("/").unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new("/"));

        let url = CFURL::from_file_path("/abc/def").unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new("/abc/def"));

        let url = CFURL::from_directory_path("/abc/def").unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new("/abc/def/"));

        let url = CFURL::from_file_path("relative.txt").unwrap();
        assert_eq!(
            url.to_file_path(),
            Some(current_dir().unwrap().join("relative.txt"))
        );
        assert_eq!(
            url.absolute_url().unwrap().to_file_path(),
            Some(current_dir().unwrap().join("relative.txt"))
        );

        let str = "/with space and wéird UTF-8 chars: 😀";
        let url = CFURL::from_file_path(str).unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new(str));
    }

    #[test]
    fn invalid_path() {
        assert_eq!(CFURL::from_file_path(""), None);
        assert_eq!(CFURL::from_file_path("/\0/a"), None);
    }

    #[test]
    fn from_dir_strips_dot() {
        let url = CFURL::from_directory_path("/Library/.").unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new("/Library/"));
    }

    /// Ensure that trailing NULs are completely stripped.
    #[test]
    #[cfg_attr(
        not(target_os = "macos"),
        ignore = "seems to work differently in the simulator"
    )]
    fn path_with_trailing_nul() {
        let url = CFURL::from_file_path("/abc/def\0\0\0").unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new("/abc/def"));

        let path = url
            .file_system_path(CFURLPathStyle::CFURLPOSIXPathStyle)
            .unwrap();
        assert_eq!(path.to_string(), "/abc/def");
        #[allow(deprecated)]
        let path = url
            .file_system_path(CFURLPathStyle::CFURLHFSPathStyle)
            .unwrap();
        assert!(path.to_string().ends_with(":abc:def")); // $DISK_NAME:abc:def
        let path = url
            .file_system_path(CFURLPathStyle::CFURLWindowsPathStyle)
            .unwrap();
        assert_eq!(path.to_string(), "\\abc\\def");
    }

    #[test]
    fn path_with_base() {
        let base = CFURL::from_directory_path("/abc/").unwrap();
        let url = CFURL::from_path(Path::new("def"), false, Some(&base)).unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new("/abc/def"));
        let url = CFURL::from_path(Path::new("def/"), false, Some(&base)).unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new("/abc/def/"));
        let url = CFURL::from_path(Path::new("/def"), false, Some(&base)).unwrap();
        assert_eq!(url.to_file_path().unwrap(), Path::new("/def"));
    }

    #[test]
    fn path_invalid_utf8() {
        // Non-root path.
        let url = CFURL::from_file_path(OsStr::from_bytes(b"abc\xd4def/xyz")).unwrap();
        assert_eq!(url.to_file_path().unwrap(), current_dir().unwrap()); // Huh?
        assert!(url
            .file_system_path(CFURLPathStyle::CFURLPOSIXPathStyle)
            .is_none());
        assert_eq!(url.string().to_string(), "abc%D4def/xyz");
        assert_eq!(url.path().unwrap().to_string(), "abc%D4def/xyz");

        // Root path.
        // lone continuation byte (128) (invalid utf8)
        let url = CFURL::from_file_path(OsStr::from_bytes(b"/\xf8a/b/c")).unwrap();
        assert_eq!(
            url.to_file_path().unwrap(),
            OsStr::from_bytes(b"/\xf8a/b/c")
        );
        assert_eq!(url.string().to_string(), "file:///%F8a/b/c");
        assert_eq!(url.path().unwrap().to_string(), "/%F8a/b/c");

        // Joined paths
        let url = CFURL::from_path(
            Path::new(OsStr::from_bytes(b"sub\xd4/%D4")),
            false,
            Some(&url),
        )
        .unwrap();
        assert_eq!(url.to_file_path(), None);
        assert_eq!(url.string().to_string(), "sub%D4/%25D4");
        assert_eq!(url.path().unwrap().to_string(), "sub%D4/%25D4");
        let abs = url.absolute_url().unwrap();
        assert_eq!(abs.to_file_path(), None);
        assert_eq!(abs.string().to_string(), "file:///%F8a/b/sub%D4/%25D4");
        assert_eq!(abs.path().unwrap().to_string(), "/%F8a/b/sub%D4/%25D4");
    }

    #[test]
    fn path_percent_encoded() {
        let url = CFURL::from_file_path("/%D4").unwrap();
        assert_eq!(url.path().unwrap().to_string(), "/%25D4");
        assert_eq!(url.to_file_path().unwrap(), Path::new("/%D4"));

        let url = CFURL::from_file_path("/%invalid").unwrap();
        assert_eq!(url.path().unwrap().to_string(), "/%25invalid");
        assert_eq!(url.to_file_path().unwrap(), Path::new("/%invalid"));
    }

    #[test]
    fn path_percent_encoded_eq() {
        let normal = CFURL::from_file_path(OsStr::from_bytes(b"\xf8")).unwrap();
        let percent = CFURL::from_file_path("%F8").unwrap();
        // Not equal, even though the filesystem may consider these paths equal.
        assert_ne!(normal, percent);
    }

    #[test]
    #[allow(deprecated)]
    #[ignore = "TODO: Crashes - is this unsound?"]
    fn hfs_invalid_utf8() {
        let url = CFURL::from_file_path(OsStr::from_bytes(b"\xd4")).unwrap();
        assert!(url
            .file_system_path(CFURLPathStyle::CFURLHFSPathStyle)
            .is_none());
    }
}
