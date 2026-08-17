#![cfg(feature = "std")]
#![cfg(unix)] // TODO: Use as_encoded_bytes/from_encoded_bytes_unchecked once in MSRV.
#![cfg(not(feature = "gnustep-1-7"))] // Doesn't seem to be available on GNUStep?
use core::ptr::NonNull;
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use objc2::rc::Retained;
use objc2::AnyThread;

use crate::NSURL;

const PATH_MAX: usize = 1024;

/// [`Path`] conversion.
impl NSURL {
    // FIXME(breaking): Make this private.
    pub fn from_path(
        path: &Path,
        is_directory: bool,
        // TODO: Expose this?
        base_url: Option<&NSURL>,
    ) -> Option<Retained<Self>> {
        // See comments in `CFURL::from_path`.
        let bytes = path.as_os_str().as_bytes();

        if bytes.is_empty() {
            // `initFileURLWithFileSystemRepresentation:isDirectory:relativeToURL:`,
            // checks this, but that's marked as non-null, so we'd get a panic
            // if we didn't implement the check manually ourselves.
            return None;
        }

        // TODO: Should we strip trailing \0 to fully match CoreFoundation?
        let cstr = CString::new(bytes).ok()?;
        let ptr = NonNull::new(cstr.as_ptr().cast_mut()).unwrap();

        // SAFETY: The pointer is a C string, and valid for the duration of
        // the call.
        Some(unsafe {
            Self::initFileURLWithFileSystemRepresentation_isDirectory_relativeToURL(
                Self::alloc(),
                ptr,
                is_directory,
                base_url,
            )
        })
    }

    /// Create a file url from a [`Path`].
    ///
    /// If the path is relative, it will be considered relative to the current
    /// directory.
    ///
    /// Returns `None` when given an invalid path (such as a path containing
    /// interior NUL bytes). The exact checks are not guaranteed.
    ///
    ///
    /// # Non-unicode and HFS+ support
    ///
    /// Modern Apple disk drives use APFS nowadays, which forces all paths to
    /// be valid unicode. The URL standard also uses unicode, and non-unicode
    /// parts of the URL will be percent-encoded, and the url will be given
    /// the scheme `file://`. All of this is as it should be.
    ///
    /// Unfortunately, a lot of Foundation APIs (including the `NSFileManager`
    /// and `NSData` APIs) currently assume that they can always get unicode
    /// paths _back_ by calling [`NSURL::path`] internally, which is not true.
    ///
    /// If you need to support non-unicode paths in HFS+ with these APIs, you
    /// can work around this issue by percent-encoding any non-unicode parts
    /// of the path yourself beforehand, similar to [what's done in the
    /// `trash-rs` crate](https://github.com/Byron/trash-rs/pull/127).
    /// (this function cannot do that for you, since it relies on a quirk of
    /// HFS+ that b"\xf8" and b"%F8" refer to the same file).
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use objc2_foundation::NSURL;
    ///
    /// // Absolute paths work as you'd expect.
    /// let url = NSURL::from_file_path("/tmp/file.txt").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/tmp/file.txt"));
    ///
    /// // Relative paths are relative to the current directory.
    /// let url = NSURL::from_file_path("foo.txt").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), std::env::current_dir().unwrap().join("foo.txt"));
    ///
    /// // Some invalid paths return `None`.
    /// assert!(NSURL::from_file_path("").is_none());
    /// // Another example of an invalid path containing interior NUL bytes.
    /// assert!(NSURL::from_file_path("/a/\0a").is_none());
    /// ```
    #[inline]
    #[doc(alias = "fileURLWithFileSystemRepresentation:isDirectory:relativeToURL:")]
    #[doc(alias = "initFileURLWithFileSystemRepresentation:isDirectory:relativeToURL:")]
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> Option<Retained<Self>> {
        Self::from_path(path.as_ref(), false, None)
    }

    /// Create a directory url from a [`Path`].
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
    /// use objc2_foundation::NSURL;
    ///
    /// // Directory paths get trailing slashes appended
    /// let url = NSURL::from_directory_path("/Library").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/Library/"));
    ///
    /// // Unless they already have them.
    /// let url = NSURL::from_directory_path("/Library/").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/Library/"));
    ///
    /// // Similarly for relative paths.
    /// let url = NSURL::from_directory_path("foo").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), std::env::current_dir().unwrap().join("foo/"));
    ///
    /// // Various dots may be stripped.
    /// let url = NSURL::from_directory_path("/Library/././.").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/Library/"));
    ///
    /// // Though of course not if they have semantic meaning.
    /// let url = NSURL::from_directory_path("/Library/..").unwrap();
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/Library/.."));
    /// ```
    #[inline]
    #[doc(alias = "fileURLWithFileSystemRepresentation:isDirectory:relativeToURL:")]
    #[doc(alias = "initFileURLWithFileSystemRepresentation:isDirectory:relativeToURL:")]
    pub fn from_directory_path<P: AsRef<Path>>(path: P) -> Option<Retained<Self>> {
        Self::from_path(path.as_ref(), true, None)
    }

    /// Extract the path part of the URL as a `PathBuf`.
    ///
    /// This will return a path regardless of [`isFileURL`][Self::isFileURL].
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
    /// use objc2_foundation::{NSURL, NSString};
    ///
    /// let url = unsafe { NSURL::URLWithString(&NSString::from_str("file:///tmp/foo.txt")).unwrap() };
    /// assert_eq!(url.to_file_path().unwrap(), Path::new("/tmp/foo.txt"));
    /// ```
    ///
    /// See also the examples in [`from_file_path`][Self::from_file_path].
    #[doc(alias = "getFileSystemRepresentation:maxLength:")]
    #[doc(alias = "fileSystemRepresentation")]
    pub fn to_file_path(&self) -> Option<PathBuf> {
        let mut buf = [0u8; PATH_MAX];
        let ptr = NonNull::new(buf.as_mut_ptr()).unwrap().cast();
        // SAFETY: The provided buffer is valid.
        // We prefer getFileSystemRepresentation:maxLength: over
        // `fileSystemRepresentation`, since the former is guaranteed to
        // handle internal NUL bytes (even if there probably won't be any,
        // NSURL seems to avoid that by construction).
        let result = unsafe { self.getFileSystemRepresentation_maxLength(ptr, buf.len()) };
        if !result {
            return None;
        }

        // SAFETY: Foundation is guaranteed to null-terminate the buffer if
        // the function succeeded.
        let cstr = unsafe { CStr::from_bytes_until_nul(&buf).unwrap_unchecked() };

        let path = OsStr::from_bytes(cstr.to_bytes());
        Some(PathBuf::from(path))
    }
}

// See also CFURL's tests, they're a bit more exhaustive.
#[cfg(test)]
#[cfg(unix)]
mod tests {
    use std::os::unix::ffi::OsStrExt;

    use super::*;

    #[test]
    fn invalid_path() {
        assert_eq!(NSURL::from_file_path(""), None);
        assert_eq!(NSURL::from_file_path("/\0/a"), None);
    }

    #[test]
    fn roundtrip() {
        let path = Path::new(OsStr::from_bytes(b"/abc/def"));
        let url = NSURL::from_file_path(path).unwrap();
        assert_eq!(url.to_file_path().unwrap(), path);

        let path = Path::new(OsStr::from_bytes(b"/\x08"));
        let url = NSURL::from_file_path(path).unwrap();
        assert_eq!(url.to_file_path().unwrap(), path);

        // Non-unicode
        let path = Path::new(OsStr::from_bytes(b"/\x08"));
        let url = NSURL::from_file_path(path).unwrap();
        assert_eq!(url.to_file_path().unwrap(), path);
    }

    #[test]
    #[cfg(all(feature = "NSData", feature = "NSFileManager", feature = "NSError"))]
    #[ignore = "needs HFS+ file system"]
    fn special_paths() {
        use crate::{NSData, NSFileManager};

        let manager = NSFileManager::defaultManager();

        let path = Path::new(OsStr::from_bytes(b"\xf8"));
        // Foundation is broken, needs a different encoding to work.
        let url = NSURL::from_file_path("%F8").unwrap();

        // Create, read and remove file, using different APIs.
        std::fs::write(path, "").unwrap();
        assert_eq!(NSData::dataWithContentsOfURL(&url), Some(NSData::new()));
        manager.removeItemAtURL_error(&url).unwrap();
    }

    // Useful when testing HFS+ and non-UTF-8:
    // echo > $(echo "0000000: f8" | xxd -r)
}
