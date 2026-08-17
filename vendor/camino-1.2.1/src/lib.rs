// Copyright (c) The camino Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(missing_docs)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

//! UTF-8 encoded paths.
//!
//! `camino` is an extension of the [`std::path`] module that adds new [`Utf8PathBuf`] and [`Utf8Path`]
//! types. These are like the standard library's [`PathBuf`] and [`Path`] types, except they are
//! guaranteed to only contain UTF-8 encoded data. Therefore, they expose the ability to get their
//! contents as strings, they implement [`Display`], etc.
//!
//! The [`std::path`] types are not guaranteed to be valid UTF-8. This is the right decision for the standard library,
//! since it must be as general as possible. However, on all platforms, non-Unicode paths are vanishingly uncommon for a
//! number of reasons:
//! * Unicode won. There are still some legacy codebases that store paths in encodings like Shift-JIS, but most
//!   have been converted to Unicode at this point.
//! * Unicode is the common subset of supported paths across Windows and Unix platforms. (On Windows, Rust stores paths
//!   as [an extension to UTF-8](https://simonsapin.github.io/wtf-8/), and converts them to UTF-16 at Win32
//!   API boundaries.)
//! * There are already many systems, such as Cargo, that only support UTF-8 paths. If your own tool interacts with any such
//!   system, you can assume that paths are valid UTF-8 without creating any additional burdens on consumers.
//! * The ["makefile problem"](https://www.mercurial-scm.org/wiki/EncodingStrategy#The_.22makefile_problem.22)
//!   (which also applies to `Cargo.toml`, and any other metadata file that lists the names of other files) has *no general,
//!   cross-platform solution* in systems that support non-UTF-8 paths. However, restricting paths to UTF-8 eliminates
//!   this problem.
//!
//! Therefore, many programs that want to manipulate paths *do* assume they contain UTF-8 data, and convert them to [`str`]s
//! as  necessary. However, because this invariant is not encoded in the [`Path`] type, conversions such as
//! `path.to_str().unwrap()` need to be repeated again and again, creating a frustrating experience.
//!
//! Instead, `camino` allows you to check that your paths are UTF-8 *once*, and then manipulate them
//! as valid UTF-8 from there on, avoiding repeated lossy and confusing conversions.

// General note: we use #[allow(clippy::incompatible_msrv)] for code that's already guarded by a
// version-specific cfg conditional.

use std::{
    borrow::{Borrow, Cow},
    cmp::Ordering,
    convert::{Infallible, TryFrom, TryInto},
    error,
    ffi::{OsStr, OsString},
    fmt,
    fs::{self, Metadata},
    hash::{Hash, Hasher},
    io,
    iter::FusedIterator,
    ops::Deref,
    path::*,
    rc::Rc,
    str::FromStr,
    sync::Arc,
};

#[cfg(feature = "proptest1")]
mod proptest_impls;
#[cfg(feature = "serde1")]
mod serde_impls;
#[cfg(test)]
mod tests;

/// An owned, mutable UTF-8 path (akin to [`String`]).
///
/// This type provides methods like [`push`] and [`set_extension`] that mutate
/// the path in place. It also implements [`Deref`] to [`Utf8Path`], meaning that
/// all methods on [`Utf8Path`] slices are available on [`Utf8PathBuf`] values as well.
///
/// [`push`]: Utf8PathBuf::push
/// [`set_extension`]: Utf8PathBuf::set_extension
///
/// # Examples
///
/// You can use [`push`] to build up a [`Utf8PathBuf`] from
/// components:
///
/// ```
/// use camino::Utf8PathBuf;
///
/// let mut path = Utf8PathBuf::new();
///
/// path.push(r"C:\");
/// path.push("windows");
/// path.push("system32");
///
/// path.set_extension("dll");
/// ```
///
/// However, [`push`] is best used for dynamic situations. This is a better way
/// to do this when you know all of the components ahead of time:
///
/// ```
/// use camino::Utf8PathBuf;
///
/// let path: Utf8PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
/// ```
///
/// We can still do better than this! Since these are all strings, we can use
/// [`From::from`]:
///
/// ```
/// use camino::Utf8PathBuf;
///
/// let path = Utf8PathBuf::from(r"C:\windows\system32.dll");
/// ```
///
/// Which method works best depends on what kind of situation you're in.
// NB: Internal PathBuf must only contain utf8 data
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct Utf8PathBuf(PathBuf);

impl Utf8PathBuf {
    /// Allocates an empty [`Utf8PathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let path = Utf8PathBuf::new();
    /// ```
    #[must_use]
    pub fn new() -> Utf8PathBuf {
        Utf8PathBuf(PathBuf::new())
    }

    /// Creates a new [`Utf8PathBuf`] from a [`PathBuf`] containing valid UTF-8 characters.
    ///
    /// Errors with the original [`PathBuf`] if it is not valid UTF-8.
    ///
    /// For a version that returns a type that implements [`std::error::Error`],
    /// see [`TryFrom<&PathBuf>`][tryfrom].
    ///
    /// [tryfrom]: #impl-TryFrom<PathBuf>-for-Utf8PathBuf
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    /// use std::ffi::OsStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    /// use std::path::PathBuf;
    ///
    /// let unicode_path = PathBuf::from("/valid/unicode");
    /// Utf8PathBuf::from_path_buf(unicode_path).expect("valid Unicode path succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
    /// # #[cfg(unix)]
    /// let non_unicode_path = PathBuf::from(non_unicode_str);
    /// # #[cfg(unix)]
    /// Utf8PathBuf::from_path_buf(non_unicode_path).expect_err("non-Unicode path failed");
    /// ```
    pub fn from_path_buf(path: PathBuf) -> Result<Utf8PathBuf, PathBuf> {
        match path.into_os_string().into_string() {
            Ok(string) => Ok(Utf8PathBuf::from(string)),
            Err(os_string) => Err(PathBuf::from(os_string)),
        }
    }

    /// Creates a new [`Utf8PathBuf`] from an [`OsString`] containing valid UTF-8 characters.
    ///
    /// Errors with the original [`OsString`] if it is not valid UTF-8.
    ///
    /// For a version that returns a type that implements [`std::error::Error`], use the
    /// [`TryFrom<OsString>`] impl.
    ///
    /// [`TryFrom<OsString>`]: #impl-TryFrom<OsString>-for-Utf8PathBuf
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(osstring_from_str)] {
    /// use camino::Utf8PathBuf;
    /// use std::ffi::OsStr;
    /// use std::ffi::OsString;
    /// use std::convert::TryFrom;
    /// use std::str::FromStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let unicode_string = OsString::from_str("/valid/unicode").unwrap();
    /// Utf8PathBuf::from_os_string(unicode_string).expect("valid Unicode path succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_string = OsStr::from_bytes(b"\xFF\xFF\xFF").into();
    /// # #[cfg(unix)]
    /// Utf8PathBuf::from_os_string(non_unicode_string).expect_err("non-Unicode path failed");
    /// # }
    /// ```
    pub fn from_os_string(os_string: OsString) -> Result<Utf8PathBuf, OsString> {
        match os_string.into_string() {
            Ok(string) => Ok(Utf8PathBuf::from(string)),
            Err(os_string) => Err(os_string),
        }
    }

    /// Converts a [`Utf8PathBuf`] to a [`PathBuf`].
    ///
    /// This is equivalent to the [`From<Utf8PathBuf> for PathBuf`][from] implementation,
    /// but may aid in type inference.
    ///
    /// [from]: #impl-From<Utf8PathBuf>-for-PathBuf
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    /// use std::path::PathBuf;
    ///
    /// let utf8_path_buf = Utf8PathBuf::from("foo.txt");
    /// let std_path_buf = utf8_path_buf.into_std_path_buf();
    /// assert_eq!(std_path_buf.to_str(), Some("foo.txt"));
    ///
    /// // Convert back to a Utf8PathBuf.
    /// let new_utf8_path_buf = Utf8PathBuf::from_path_buf(std_path_buf).unwrap();
    /// assert_eq!(new_utf8_path_buf, "foo.txt");
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_std_path_buf(self) -> PathBuf {
        self.into()
    }

    /// Creates a new [`Utf8PathBuf`] with a given capacity used to create the internal [`PathBuf`].
    /// See [`with_capacity`] defined on [`PathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut path = Utf8PathBuf::with_capacity(10);
    /// let capacity = path.capacity();
    ///
    /// // This push is done without reallocating
    /// path.push(r"C:\");
    ///
    /// assert_eq!(capacity, path.capacity());
    /// ```
    ///
    /// [`with_capacity`]: PathBuf::with_capacity
    #[allow(clippy::incompatible_msrv)]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Utf8PathBuf {
        Utf8PathBuf(PathBuf::with_capacity(capacity))
    }

    /// Coerces to a [`Utf8Path`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let p = Utf8PathBuf::from("/test");
    /// assert_eq!(Utf8Path::new("/test"), p.as_path());
    /// ```
    #[must_use]
    pub fn as_path(&self) -> &Utf8Path {
        // SAFETY: every Utf8PathBuf constructor ensures that self is valid UTF-8
        unsafe { Utf8Path::assume_utf8(&self.0) }
    }

    /// Consumes and leaks the [`Utf8PathBuf`], returning a mutable reference to the contents,
    /// `&'a mut Utf8Path`.
    ///
    /// The caller has free choice over the returned lifetime, including 'static.
    /// Indeed, this function is ideally used for data that lives for the remainder of
    /// the program’s life, as dropping the returned reference will cause a memory leak.
    ///
    /// It does not reallocate or shrink the [`Utf8PathBuf`], so the leaked allocation may include
    /// unused capacity that is not part of the returned slice. If you want to discard excess
    /// capacity, call [`into_boxed_path`], and then [`Box::leak`] instead.
    /// However, keep in mind that trimming the capacity may result in a reallocation and copy.
    ///
    /// *Requires Rust 1.89 or newer.*
    ///
    /// [`into_boxed_path`]: Self::into_boxed_path
    #[cfg(os_string_pathbuf_leak)]
    #[allow(clippy::incompatible_msrv)]
    #[inline]
    pub fn leak<'a>(self) -> &'a mut Utf8Path {
        // SAFETY: every Utf8PathBuf constructor ensures that self is valid UTF-8
        unsafe { Utf8Path::assume_utf8_mut(self.0.leak()) }
    }

    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// On Windows:
    ///
    /// * if `path` has a root but no prefix (e.g., `\windows`), it
    ///   replaces everything except for the prefix (if any) of `self`.
    /// * if `path` has a prefix but no root, it replaces `self`.
    ///
    /// # Examples
    ///
    /// Pushing a relative path extends the existing path:
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut path = Utf8PathBuf::from("/tmp");
    /// path.push("file.bk");
    /// assert_eq!(path, Utf8PathBuf::from("/tmp/file.bk"));
    /// ```
    ///
    /// Pushing an absolute path replaces the existing path:
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut path = Utf8PathBuf::from("/tmp");
    /// path.push("/etc");
    /// assert_eq!(path, Utf8PathBuf::from("/etc"));
    /// ```
    pub fn push(&mut self, path: impl AsRef<Utf8Path>) {
        self.0.push(&path.as_ref().0)
    }

    /// Truncates `self` to [`self.parent`].
    ///
    /// Returns `false` and does nothing if [`self.parent`] is [`None`].
    /// Otherwise, returns `true`.
    ///
    /// [`self.parent`]: Utf8Path::parent
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let mut p = Utf8PathBuf::from("/spirited/away.rs");
    ///
    /// p.pop();
    /// assert_eq!(Utf8Path::new("/spirited"), p);
    /// p.pop();
    /// assert_eq!(Utf8Path::new("/"), p);
    /// ```
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    /// Updates [`self.file_name`] to `file_name`.
    ///
    /// If [`self.file_name`] was [`None`], this is equivalent to pushing
    /// `file_name`.
    ///
    /// Otherwise it is equivalent to calling [`pop`] and then pushing
    /// `file_name`. The new path will be a sibling of the original path.
    /// (That is, it will have the same parent.)
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    /// [`pop`]: Utf8PathBuf::pop
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut buf = Utf8PathBuf::from("/");
    /// assert_eq!(buf.file_name(), None);
    /// buf.set_file_name("bar");
    /// assert_eq!(buf, Utf8PathBuf::from("/bar"));
    /// assert!(buf.file_name().is_some());
    /// buf.set_file_name("baz.txt");
    /// assert_eq!(buf, Utf8PathBuf::from("/baz.txt"));
    /// ```
    pub fn set_file_name(&mut self, file_name: impl AsRef<str>) {
        self.0.set_file_name(file_name.as_ref())
    }

    /// Updates [`self.extension`] to `extension`.
    ///
    /// Returns `false` and does nothing if [`self.file_name`] is [`None`],
    /// returns `true` and updates the extension otherwise.
    ///
    /// If [`self.extension`] is [`None`], the extension is added; otherwise
    /// it is replaced.
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    /// [`self.extension`]: Utf8Path::extension
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let mut p = Utf8PathBuf::from("/feel/the");
    ///
    /// p.set_extension("force");
    /// assert_eq!(Utf8Path::new("/feel/the.force"), p.as_path());
    ///
    /// p.set_extension("dark_side");
    /// assert_eq!(Utf8Path::new("/feel/the.dark_side"), p.as_path());
    /// ```
    pub fn set_extension(&mut self, extension: impl AsRef<str>) -> bool {
        self.0.set_extension(extension.as_ref())
    }

    /// Consumes the [`Utf8PathBuf`], yielding its internal [`String`] storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let p = Utf8PathBuf::from("/the/head");
    /// let s = p.into_string();
    /// assert_eq!(s, "/the/head");
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_string(self) -> String {
        self.into_os_string().into_string().unwrap()
    }

    /// Consumes the [`Utf8PathBuf`], yielding its internal [`OsString`] storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    /// use std::ffi::OsStr;
    ///
    /// let p = Utf8PathBuf::from("/the/head");
    /// let s = p.into_os_string();
    /// assert_eq!(s, OsStr::new("/the/head"));
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_os_string(self) -> OsString {
        self.0.into_os_string()
    }

    /// Converts this [`Utf8PathBuf`] into a [boxed](Box) [`Utf8Path`].
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_boxed_path(self) -> Box<Utf8Path> {
        let ptr = Box::into_raw(self.0.into_boxed_path()) as *mut Utf8Path;
        // SAFETY:
        // * self is valid UTF-8
        // * ptr was constructed by consuming self so it represents an owned path
        // * Utf8Path is marked as #[repr(transparent)] so the conversion from *mut Path to
        //   *mut Utf8Path is valid
        unsafe { Box::from_raw(ptr) }
    }

    /// Invokes [`capacity`] on the underlying instance of [`PathBuf`].
    ///
    /// [`capacity`]: PathBuf::capacity
    #[allow(clippy::incompatible_msrv)]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Invokes [`clear`] on the underlying instance of [`PathBuf`].
    ///
    /// [`clear`]: PathBuf::clear
    #[allow(clippy::incompatible_msrv)]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Invokes [`reserve`] on the underlying instance of [`PathBuf`].
    ///
    /// [`reserve`]: PathBuf::reserve
    #[allow(clippy::incompatible_msrv)]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    /// Invokes [`try_reserve`] on the underlying instance of [`PathBuf`].
    ///
    /// *Requires Rust 1.63 or newer.*
    ///
    /// [`try_reserve`]: PathBuf::try_reserve
    #[cfg(try_reserve_2)]
    #[allow(clippy::incompatible_msrv)]
    #[inline]
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.0.try_reserve(additional)
    }

    /// Invokes [`reserve_exact`] on the underlying instance of [`PathBuf`].
    ///
    /// [`reserve_exact`]: PathBuf::reserve_exact
    #[allow(clippy::incompatible_msrv)]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional)
    }

    /// Invokes [`try_reserve_exact`] on the underlying instance of [`PathBuf`].
    ///
    /// *Requires Rust 1.63 or newer.*
    ///
    /// [`try_reserve_exact`]: PathBuf::try_reserve_exact
    #[cfg(try_reserve_2)]
    #[allow(clippy::incompatible_msrv)]
    #[inline]
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.0.try_reserve_exact(additional)
    }

    /// Invokes [`shrink_to_fit`] on the underlying instance of [`PathBuf`].
    ///
    /// [`shrink_to_fit`]: PathBuf::shrink_to_fit
    #[allow(clippy::incompatible_msrv)]
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }

    /// Invokes [`shrink_to`] on the underlying instance of [`PathBuf`].
    ///
    /// [`shrink_to`]: PathBuf::shrink_to
    #[allow(clippy::incompatible_msrv)]
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.0.shrink_to(min_capacity)
    }
}

impl Deref for Utf8PathBuf {
    type Target = Utf8Path;

    fn deref(&self) -> &Utf8Path {
        self.as_path()
    }
}

/// *Requires Rust 1.68 or newer.*
#[cfg(path_buf_deref_mut)]
#[allow(clippy::incompatible_msrv)]
impl std::ops::DerefMut for Utf8PathBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { Utf8Path::assume_utf8_mut(&mut self.0) }
    }
}

impl fmt::Debug for Utf8PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Display for Utf8PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<P: AsRef<Utf8Path>> Extend<P> for Utf8PathBuf {
    fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I) {
        for path in iter {
            self.push(path);
        }
    }
}

/// A slice of a UTF-8 path (akin to [`str`]).
///
/// This type supports a number of operations for inspecting a path, including
/// breaking the path into its components (separated by `/` on Unix and by either
/// `/` or `\` on Windows), extracting the file name, determining whether the path
/// is absolute, and so on.
///
/// This is an *unsized* type, meaning that it must always be used behind a
/// pointer like `&` or [`Box`]. For an owned version of this type,
/// see [`Utf8PathBuf`].
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
///
/// // Note: this example does work on Windows
/// let path = Utf8Path::new("./foo/bar.txt");
///
/// let parent = path.parent();
/// assert_eq!(parent, Some(Utf8Path::new("./foo")));
///
/// let file_stem = path.file_stem();
/// assert_eq!(file_stem, Some("bar"));
///
/// let extension = path.extension();
/// assert_eq!(extension, Some("txt"));
/// ```
// NB: Internal Path must only contain utf8 data
#[repr(transparent)]
pub struct Utf8Path(Path);

impl Utf8Path {
    /// Directly wraps a string slice as a [`Utf8Path`] slice.
    ///
    /// This is a cost-free conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// Utf8Path::new("foo.txt");
    /// ```
    ///
    /// You can create [`Utf8Path`]s from [`String`]s, or even other [`Utf8Path`]s:
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let string = String::from("foo.txt");
    /// let from_string = Utf8Path::new(&string);
    /// let from_path = Utf8Path::new(&from_string);
    /// assert_eq!(from_string, from_path);
    /// ```
    pub fn new(s: &(impl AsRef<str> + ?Sized)) -> &Utf8Path {
        let path = Path::new(s.as_ref());
        // SAFETY: s is a str which means it is always valid UTF-8
        unsafe { Utf8Path::assume_utf8(path) }
    }

    /// Converts a [`Path`] to a [`Utf8Path`].
    ///
    /// Returns [`None`] if the path is not valid UTF-8.
    ///
    /// For a version that returns a type that implements [`std::error::Error`],
    /// see [`TryFrom<&Path>`][tryfrom].
    ///
    /// [tryfrom]: #impl-TryFrom<%26Path>-for-%26Utf8Path
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::ffi::OsStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    /// use std::path::Path;
    ///
    /// let unicode_path = Path::new("/valid/unicode");
    /// Utf8Path::from_path(unicode_path).expect("valid Unicode path succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
    /// # #[cfg(unix)]
    /// let non_unicode_path = Path::new(non_unicode_str);
    /// # #[cfg(unix)]
    /// assert!(Utf8Path::from_path(non_unicode_path).is_none(), "non-Unicode path failed");
    /// ```
    pub fn from_path(path: &Path) -> Option<&Utf8Path> {
        path.as_os_str().to_str().map(Utf8Path::new)
    }

    /// Converts an [`OsStr`] to a [`Utf8Path`].
    ///
    /// Returns [`None`] if the path is not valid UTF-8.
    ///
    /// For a version that returns a type that implements [`std::error::Error`], use the
    /// [`TryFrom<&OsStr>`][tryfrom] impl.
    ///
    /// [tryfrom]: #impl-TryFrom<%26OsStr>-for-%26Utf8Path
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::ffi::OsStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    /// use std::path::Path;
    ///
    /// let unicode_string = OsStr::new("/valid/unicode");
    /// Utf8Path::from_os_str(unicode_string).expect("valid Unicode string succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
    /// # #[cfg(unix)]
    /// assert!(Utf8Path::from_os_str(non_unicode_str).is_none(), "non-Unicode string failed");
    /// ```
    pub fn from_os_str(path: &OsStr) -> Option<&Utf8Path> {
        path.to_str().map(Utf8Path::new)
    }

    /// Converts a [`Utf8Path`] to a [`Path`].
    ///
    /// This is equivalent to the [`AsRef<Path> for Utf8PathBuf`][asref] implementation,
    /// but may aid in type inference.
    ///
    /// [asref]: Utf8PathBuf#impl-AsRef<Path>-for-Utf8PathBuf
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::path::Path;
    ///
    /// let utf8_path = Utf8Path::new("foo.txt");
    /// let std_path: &Path = utf8_path.as_std_path();
    /// assert_eq!(std_path.to_str(), Some("foo.txt"));
    ///
    /// // Convert back to a Utf8Path.
    /// let new_utf8_path = Utf8Path::from_path(std_path).unwrap();
    /// assert_eq!(new_utf8_path, "foo.txt");
    /// ```
    #[inline]
    pub fn as_std_path(&self) -> &Path {
        self.as_ref()
    }

    /// Yields the underlying [`str`] slice.
    ///
    /// Unlike [`Path::to_str`], this always returns a slice because the contents
    /// of a [`Utf8Path`] are guaranteed to be valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let s = Utf8Path::new("foo.txt").as_str();
    /// assert_eq!(s, "foo.txt");
    /// ```
    ///
    /// [`str`]: str
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        // SAFETY: every Utf8Path constructor ensures that self is valid UTF-8
        unsafe { str_assume_utf8(self.as_os_str()) }
    }

    /// Yields the underlying [`OsStr`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let os_str = Utf8Path::new("foo.txt").as_os_str();
    /// assert_eq!(os_str, std::ffi::OsStr::new("foo.txt"));
    /// ```
    #[inline]
    #[must_use]
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }

    /// Converts a [`Utf8Path`] to an owned [`Utf8PathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path_buf = Utf8Path::new("foo.txt").to_path_buf();
    /// assert_eq!(path_buf, Utf8PathBuf::from("foo.txt"));
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub fn to_path_buf(&self) -> Utf8PathBuf {
        Utf8PathBuf(self.0.to_path_buf())
    }

    /// Returns `true` if the [`Utf8Path`] is absolute, i.e., if it is independent of
    /// the current directory.
    ///
    /// * On Unix, a path is absolute if it starts with the root, so
    ///   `is_absolute` and [`has_root`] are equivalent.
    ///
    /// * On Windows, a path is absolute if it has a prefix and starts with the
    ///   root: `C:\windows` is absolute, while `C:temp` and `\temp` are not.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert!(!Utf8Path::new("foo.txt").is_absolute());
    /// ```
    ///
    /// [`has_root`]: Utf8Path::has_root
    #[inline]
    #[must_use]
    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }

    /// Returns `true` if the [`Utf8Path`] is relative, i.e., not absolute.
    ///
    /// See [`is_absolute`]'s documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert!(Utf8Path::new("foo.txt").is_relative());
    /// ```
    ///
    /// [`is_absolute`]: Utf8Path::is_absolute
    #[inline]
    #[must_use]
    pub fn is_relative(&self) -> bool {
        self.0.is_relative()
    }

    /// Returns `true` if the [`Utf8Path`] has a root.
    ///
    /// * On Unix, a path has a root if it begins with `/`.
    ///
    /// * On Windows, a path has a root if it:
    ///     * has no prefix and begins with a separator, e.g., `\windows`
    ///     * has a prefix followed by a separator, e.g., `C:\windows` but not `C:windows`
    ///     * has any non-disk prefix, e.g., `\\server\share`
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert!(Utf8Path::new("/etc/passwd").has_root());
    /// ```
    #[inline]
    #[must_use]
    pub fn has_root(&self) -> bool {
        self.0.has_root()
    }

    /// Returns the [`Path`] without its final component, if there is one.
    ///
    /// Returns [`None`] if the path terminates in a root or prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/foo/bar");
    /// let parent = path.parent().unwrap();
    /// assert_eq!(parent, Utf8Path::new("/foo"));
    ///
    /// let grand_parent = parent.parent().unwrap();
    /// assert_eq!(grand_parent, Utf8Path::new("/"));
    /// assert_eq!(grand_parent.parent(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn parent(&self) -> Option<&Utf8Path> {
        self.0.parent().map(|path| {
            // SAFETY: self is valid UTF-8, so parent is valid UTF-8 as well
            unsafe { Utf8Path::assume_utf8(path) }
        })
    }

    /// Produces an iterator over [`Utf8Path`] and its ancestors.
    ///
    /// The iterator will yield the [`Utf8Path`] that is returned if the [`parent`] method is used zero
    /// or more times. That means, the iterator will yield `&self`, `&self.parent().unwrap()`,
    /// `&self.parent().unwrap().parent().unwrap()` and so on. If the [`parent`] method returns
    /// [`None`], the iterator will do likewise. The iterator will always yield at least one value,
    /// namely `&self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let mut ancestors = Utf8Path::new("/foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("/foo/bar")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("/foo")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("/")));
    /// assert_eq!(ancestors.next(), None);
    ///
    /// let mut ancestors = Utf8Path::new("../foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("../foo/bar")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("../foo")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("..")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("")));
    /// assert_eq!(ancestors.next(), None);
    /// ```
    ///
    /// [`parent`]: Utf8Path::parent
    #[inline]
    pub fn ancestors(&self) -> Utf8Ancestors<'_> {
        Utf8Ancestors(self.0.ancestors())
    }

    /// Returns the final component of the [`Utf8Path`], if there is one.
    ///
    /// If the path is a normal file, this is the file name. If it's the path of a directory, this
    /// is the directory name.
    ///
    /// Returns [`None`] if the path terminates in `..`.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert_eq!(Some("bin"), Utf8Path::new("/usr/bin/").file_name());
    /// assert_eq!(Some("foo.txt"), Utf8Path::new("tmp/foo.txt").file_name());
    /// assert_eq!(Some("foo.txt"), Utf8Path::new("foo.txt/.").file_name());
    /// assert_eq!(Some("foo.txt"), Utf8Path::new("foo.txt/.//").file_name());
    /// assert_eq!(None, Utf8Path::new("foo.txt/..").file_name());
    /// assert_eq!(None, Utf8Path::new("/").file_name());
    /// ```
    #[inline]
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name().map(|s| {
            // SAFETY: self is valid UTF-8, so file_name is valid UTF-8 as well
            unsafe { str_assume_utf8(s) }
        })
    }

    /// Returns a path that, when joined onto `base`, yields `self`.
    ///
    /// # Errors
    ///
    /// If `base` is not a prefix of `self` (i.e., [`starts_with`]
    /// returns `false`), returns [`Err`].
    ///
    /// [`starts_with`]: Utf8Path::starts_with
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("/test/haha/foo.txt");
    ///
    /// assert_eq!(path.strip_prefix("/"), Ok(Utf8Path::new("test/haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test"), Ok(Utf8Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/"), Ok(Utf8Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Utf8Path::new("")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Utf8Path::new("")));
    ///
    /// assert!(path.strip_prefix("test").is_err());
    /// assert!(path.strip_prefix("/haha").is_err());
    ///
    /// let prefix = Utf8PathBuf::from("/test/");
    /// assert_eq!(path.strip_prefix(prefix), Ok(Utf8Path::new("haha/foo.txt")));
    /// ```
    #[inline]
    pub fn strip_prefix(&self, base: impl AsRef<Path>) -> Result<&Utf8Path, StripPrefixError> {
        self.0.strip_prefix(base).map(|path| {
            // SAFETY: self is valid UTF-8, and strip_prefix returns a part of self (or an empty
            // string), so it is valid UTF-8 as well.
            unsafe { Utf8Path::assume_utf8(path) }
        })
    }

    /// Determines whether `base` is a prefix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/etc/passwd");
    ///
    /// assert!(path.starts_with("/etc"));
    /// assert!(path.starts_with("/etc/"));
    /// assert!(path.starts_with("/etc/passwd"));
    /// assert!(path.starts_with("/etc/passwd/")); // extra slash is okay
    /// assert!(path.starts_with("/etc/passwd///")); // multiple extra slashes are okay
    ///
    /// assert!(!path.starts_with("/e"));
    /// assert!(!path.starts_with("/etc/passwd.txt"));
    ///
    /// assert!(!Utf8Path::new("/etc/foo.rs").starts_with("/etc/foo"));
    /// ```
    #[inline]
    #[must_use]
    pub fn starts_with(&self, base: impl AsRef<Path>) -> bool {
        self.0.starts_with(base)
    }

    /// Determines whether `child` is a suffix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/etc/resolv.conf");
    ///
    /// assert!(path.ends_with("resolv.conf"));
    /// assert!(path.ends_with("etc/resolv.conf"));
    /// assert!(path.ends_with("/etc/resolv.conf"));
    ///
    /// assert!(!path.ends_with("/resolv.conf"));
    /// assert!(!path.ends_with("conf")); // use .extension() instead
    /// ```
    #[inline]
    #[must_use]
    pub fn ends_with(&self, base: impl AsRef<Path>) -> bool {
        self.0.ends_with(base)
    }

    /// Extracts the stem (non-extension) portion of [`self.file_name`].
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    ///
    /// The stem is:
    ///
    /// * [`None`], if there is no file name;
    /// * The entire file name if there is no embedded `.`;
    /// * The entire file name if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name before the final `.`
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert_eq!("foo", Utf8Path::new("foo.rs").file_stem().unwrap());
    /// assert_eq!("foo.tar", Utf8Path::new("foo.tar.gz").file_stem().unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub fn file_stem(&self) -> Option<&str> {
        self.0.file_stem().map(|s| {
            // SAFETY: self is valid UTF-8, so file_stem is valid UTF-8 as well
            unsafe { str_assume_utf8(s) }
        })
    }

    /// Extracts the extension of [`self.file_name`], if possible.
    ///
    /// The extension is:
    ///
    /// * [`None`], if there is no file name;
    /// * [`None`], if there is no embedded `.`;
    /// * [`None`], if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name after the final `.`
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert_eq!("rs", Utf8Path::new("foo.rs").extension().unwrap());
    /// assert_eq!("gz", Utf8Path::new("foo.tar.gz").extension().unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        self.0.extension().map(|s| {
            // SAFETY: self is valid UTF-8, so extension is valid UTF-8 as well
            unsafe { str_assume_utf8(s) }
        })
    }

    /// Creates an owned [`Utf8PathBuf`] with `path` adjoined to `self`.
    ///
    /// See [`Utf8PathBuf::push`] for more details on what it means to adjoin a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// assert_eq!(Utf8Path::new("/etc").join("passwd"), Utf8PathBuf::from("/etc/passwd"));
    /// ```
    #[inline]
    #[must_use]
    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.join(&path.as_ref().0))
    }

    /// Creates an owned [`PathBuf`] with `path` adjoined to `self`.
    ///
    /// See [`PathBuf::push`] for more details on what it means to adjoin a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::path::PathBuf;
    ///
    /// assert_eq!(Utf8Path::new("/etc").join_os("passwd"), PathBuf::from("/etc/passwd"));
    /// ```
    #[inline]
    #[must_use]
    pub fn join_os(&self, path: impl AsRef<Path>) -> PathBuf {
        self.0.join(path)
    }

    /// Creates an owned [`Utf8PathBuf`] like `self` but with the given file name.
    ///
    /// See [`Utf8PathBuf::set_file_name`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("/tmp/foo.txt");
    /// assert_eq!(path.with_file_name("bar.txt"), Utf8PathBuf::from("/tmp/bar.txt"));
    ///
    /// let path = Utf8Path::new("/tmp");
    /// assert_eq!(path.with_file_name("var"), Utf8PathBuf::from("/var"));
    /// ```
    #[inline]
    #[must_use]
    pub fn with_file_name(&self, file_name: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_file_name(file_name.as_ref()))
    }

    /// Creates an owned [`Utf8PathBuf`] like `self` but with the given extension.
    ///
    /// See [`Utf8PathBuf::set_extension`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("foo.rs");
    /// assert_eq!(path.with_extension("txt"), Utf8PathBuf::from("foo.txt"));
    ///
    /// let path = Utf8Path::new("foo.tar.gz");
    /// assert_eq!(path.with_extension(""), Utf8PathBuf::from("foo.tar"));
    /// assert_eq!(path.with_extension("xz"), Utf8PathBuf::from("foo.tar.xz"));
    /// assert_eq!(path.with_extension("").with_extension("txt"), Utf8PathBuf::from("foo.txt"));
    /// ```
    #[inline]
    pub fn with_extension(&self, extension: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_extension(extension.as_ref()))
    }

    /// Produces an iterator over the [`Utf8Component`]s of the path.
    ///
    /// When parsing the path, there is a small amount of normalization:
    ///
    /// * Repeated separators are ignored, so `a/b` and `a//b` both have
    ///   `a` and `b` as components.
    ///
    /// * Occurrences of `.` are normalized away, except if they are at the
    ///   beginning of the path. For example, `a/./b`, `a/b/`, `a/b/.` and
    ///   `a/b` all have `a` and `b` as components, but `./a/b` starts with
    ///   an additional [`CurDir`] component.
    ///
    /// * A trailing slash is normalized away, `/a/b` and `/a/b/` are equivalent.
    ///
    /// Note that no other normalization takes place; in particular, `a/c`
    /// and `a/b/../c` are distinct, to account for the possibility that `b`
    /// is a symbolic link (so its parent isn't `a`).
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Component, Utf8Path};
    ///
    /// let mut components = Utf8Path::new("/tmp/foo.txt").components();
    ///
    /// assert_eq!(components.next(), Some(Utf8Component::RootDir));
    /// assert_eq!(components.next(), Some(Utf8Component::Normal("tmp")));
    /// assert_eq!(components.next(), Some(Utf8Component::Normal("foo.txt")));
    /// assert_eq!(components.next(), None)
    /// ```
    ///
    /// [`CurDir`]: Utf8Component::CurDir
    #[inline]
    pub fn components(&self) -> Utf8Components<'_> {
        Utf8Components(self.0.components())
    }

    /// Produces an iterator over the path's components viewed as [`str`]
    /// slices.
    ///
    /// For more information about the particulars of how the path is separated
    /// into components, see [`components`].
    ///
    /// [`components`]: Utf8Path::components
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let mut it = Utf8Path::new("/tmp/foo.txt").iter();
    /// assert_eq!(it.next(), Some(std::path::MAIN_SEPARATOR.to_string().as_str()));
    /// assert_eq!(it.next(), Some("tmp"));
    /// assert_eq!(it.next(), Some("foo.txt"));
    /// assert_eq!(it.next(), None)
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.components(),
        }
    }

    /// Queries the file system to get information about a file, directory, etc.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file.
    ///
    /// This is an alias to [`fs::metadata`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/Minas/tirith");
    /// let metadata = path.metadata().expect("metadata call failed");
    /// println!("{:?}", metadata.file_type());
    /// ```
    #[inline]
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        self.0.metadata()
    }

    /// Queries the metadata about a file without following symlinks.
    ///
    /// This is an alias to [`fs::symlink_metadata`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/Minas/tirith");
    /// let metadata = path.symlink_metadata().expect("symlink_metadata call failed");
    /// println!("{:?}", metadata.file_type());
    /// ```
    #[inline]
    pub fn symlink_metadata(&self) -> io::Result<fs::Metadata> {
        self.0.symlink_metadata()
    }

    /// Returns the canonical, absolute form of the path with all intermediate
    /// components normalized and symbolic links resolved.
    ///
    /// This returns a [`PathBuf`] because even if a symlink is valid Unicode, its target may not
    /// be. For a version that returns a [`Utf8PathBuf`], see
    /// [`canonicalize_utf8`](Self::canonicalize_utf8).
    ///
    /// This is an alias to [`fs::canonicalize`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// use std::path::PathBuf;
    ///
    /// let path = Utf8Path::new("/foo/test/../test/bar.rs");
    /// assert_eq!(path.canonicalize().unwrap(), PathBuf::from("/foo/test/bar.rs"));
    /// ```
    #[inline]
    pub fn canonicalize(&self) -> io::Result<PathBuf> {
        self.0.canonicalize()
    }

    /// Returns the canonical, absolute form of the path with all intermediate
    /// components normalized and symbolic links resolved.
    ///
    /// This method attempts to convert the resulting [`PathBuf`] into a [`Utf8PathBuf`]. For a
    /// version that does not attempt to do this conversion, see
    /// [`canonicalize`](Self::canonicalize).
    ///
    /// # Errors
    ///
    /// The I/O operation may return an error: see the [`fs::canonicalize`]
    /// documentation for more.
    ///
    /// If the resulting path is not UTF-8, an [`io::Error`] is returned with the
    /// [`ErrorKind`](io::ErrorKind) set to [`InvalidData`](io::ErrorKind::InvalidData)
    /// and the payload set to a [`FromPathBufError`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("/foo/test/../test/bar.rs");
    /// assert_eq!(path.canonicalize_utf8().unwrap(), Utf8PathBuf::from("/foo/test/bar.rs"));
    /// ```
    pub fn canonicalize_utf8(&self) -> io::Result<Utf8PathBuf> {
        self.canonicalize()
            .and_then(|path| path.try_into().map_err(FromPathBufError::into_io_error))
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This returns a [`PathBuf`] because even if a symlink is valid Unicode, its target may not
    /// be. For a version that returns a [`Utf8PathBuf`], see
    /// [`read_link_utf8`](Self::read_link_utf8).
    ///
    /// This is an alias to [`fs::read_link`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/laputa/sky_castle.rs");
    /// let path_link = path.read_link().expect("read_link call failed");
    /// ```
    #[inline]
    pub fn read_link(&self) -> io::Result<PathBuf> {
        self.0.read_link()
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This method attempts to convert the resulting [`PathBuf`] into a [`Utf8PathBuf`]. For a
    /// version that does not attempt to do this conversion, see [`read_link`](Self::read_link).
    ///
    /// # Errors
    ///
    /// The I/O operation may return an error: see the [`fs::read_link`]
    /// documentation for more.
    ///
    /// If the resulting path is not UTF-8, an [`io::Error`] is returned with the
    /// [`ErrorKind`](io::ErrorKind) set to [`InvalidData`](io::ErrorKind::InvalidData)
    /// and the payload set to a [`FromPathBufError`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/laputa/sky_castle.rs");
    /// let path_link = path.read_link_utf8().expect("read_link call failed");
    /// ```
    pub fn read_link_utf8(&self) -> io::Result<Utf8PathBuf> {
        self.read_link()
            .and_then(|path| path.try_into().map_err(FromPathBufError::into_io_error))
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// The iterator will yield instances of [`io::Result`]`<`[`fs::DirEntry`]`>`. New
    /// errors may be encountered after an iterator is initially constructed.
    ///
    /// This is an alias to [`fs::read_dir`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/laputa");
    /// for entry in path.read_dir().expect("read_dir call failed") {
    ///     if let Ok(entry) = entry {
    ///         println!("{:?}", entry.path());
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn read_dir(&self) -> io::Result<fs::ReadDir> {
        self.0.read_dir()
    }

    /// Returns an iterator over the entries within a directory.
    ///
    /// The iterator will yield instances of [`io::Result`]`<`[`Utf8DirEntry`]`>`. New
    /// errors may be encountered after an iterator is initially constructed.
    ///
    /// # Errors
    ///
    /// The I/O operation may return an error: see the [`fs::read_dir`]
    /// documentation for more.
    ///
    /// If a directory entry is not UTF-8, an [`io::Error`] is returned with the
    /// [`ErrorKind`](io::ErrorKind) set to [`InvalidData`](io::ErrorKind::InvalidData)
    /// and the payload set to a [`FromPathBufError`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/laputa");
    /// for entry in path.read_dir_utf8().expect("read_dir call failed") {
    ///     if let Ok(entry) = entry {
    ///         println!("{}", entry.path());
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn read_dir_utf8(&self) -> io::Result<ReadDirUtf8> {
        self.0.read_dir().map(|inner| ReadDirUtf8 { inner })
    }

    /// Returns `true` if the path points at an existing entity.
    ///
    /// Warning: this method may be error-prone, consider using [`try_exists()`] instead!
    /// It also has a risk of introducing time-of-check to time-of-use (TOCTOU) bugs.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links this will return `false`.
    ///
    /// If you cannot access the directory containing the file, e.g., because of a
    /// permission error, this will return `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// assert!(!Utf8Path::new("does_not_exist.txt").exists());
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [`fs::metadata`].
    ///
    /// [`try_exists()`]: Self::try_exists
    #[must_use]
    #[inline]
    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    /// Returns `Ok(true)` if the path points at an existing entity.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links this will return `Ok(false)`.
    ///
    /// As opposed to the [`exists()`] method, this one doesn't silently ignore errors
    /// unrelated to the path not existing. (E.g. it will return [`Err`] in case of permission
    /// denied on some of the parent directories.)
    ///
    /// Note that while this avoids some pitfalls of the `exists()` method, it still can not
    /// prevent time-of-check to time-of-use (TOCTOU) bugs. You should only use it in scenarios
    /// where those bugs are not an issue.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// assert!(
    ///     !Utf8Path::new("does_not_exist.txt")
    ///         .try_exists()
    ///         .expect("Can't check existence of file does_not_exist.txt")
    /// );
    /// assert!(Utf8Path::new("/root/secret_file.txt").try_exists().is_err());
    /// ```
    ///
    /// [`exists()`]: Self::exists
    #[inline]
    pub fn try_exists(&self) -> io::Result<bool> {
        match fs::metadata(self) {
            Ok(_) => Ok(true),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
        }
    }

    /// Returns `true` if the path exists on disk and is pointing at a regular file.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links this will return `false`.
    ///
    /// If you cannot access the directory containing the file, e.g., because of a
    /// permission error, this will return `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// assert_eq!(Utf8Path::new("./is_a_directory/").is_file(), false);
    /// assert_eq!(Utf8Path::new("a_file.txt").is_file(), true);
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [`fs::metadata`] and handle its [`Result`]. Then call
    /// [`fs::Metadata::is_file`] if it was [`Ok`].
    ///
    /// When the goal is simply to read from (or write to) the source, the most
    /// reliable way to test the source can be read (or written to) is to open
    /// it. Only using `is_file` can break workflows like `diff <( prog_a )` on
    /// a Unix-like system for example. See [`fs::File::open`] or
    /// [`fs::OpenOptions::open`] for more information.
    #[must_use]
    #[inline]
    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    /// Returns `true` if the path exists on disk and is pointing at a directory.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links this will return `false`.
    ///
    /// If you cannot access the directory containing the file, e.g., because of a
    /// permission error, this will return `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// assert_eq!(Utf8Path::new("./is_a_directory/").is_dir(), true);
    /// assert_eq!(Utf8Path::new("a_file.txt").is_dir(), false);
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [`fs::metadata`] and handle its [`Result`]. Then call
    /// [`fs::Metadata::is_dir`] if it was [`Ok`].
    #[must_use]
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    /// Returns `true` if the path exists on disk and is pointing at a symbolic link.
    ///
    /// This function will not traverse symbolic links.
    /// In case of a broken symbolic link this will also return true.
    ///
    /// If you cannot access the directory containing the file, e.g., because of a
    /// permission error, this will return false.
    ///
    /// # Examples
    ///
    #[cfg_attr(unix, doc = "```no_run")]
    #[cfg_attr(not(unix), doc = "```ignore")]
    /// use camino::Utf8Path;
    /// use std::os::unix::fs::symlink;
    ///
    /// let link_path = Utf8Path::new("link");
    /// symlink("/origin_does_not_exist/", link_path).unwrap();
    /// assert_eq!(link_path.is_symlink(), true);
    /// assert_eq!(link_path.exists(), false);
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [`Utf8Path::symlink_metadata`] and handle its [`Result`]. Then call
    /// [`fs::Metadata::is_symlink`] if it was [`Ok`].
    #[must_use]
    pub fn is_symlink(&self) -> bool {
        self.symlink_metadata()
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
    }

    /// Converts a [`Box<Utf8Path>`] into a [`Utf8PathBuf`] without copying or allocating.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    pub fn into_path_buf(self: Box<Utf8Path>) -> Utf8PathBuf {
        let ptr = Box::into_raw(self) as *mut Path;
        // SAFETY:
        // * self is valid UTF-8
        // * ptr was constructed by consuming self so it represents an owned path.
        // * Utf8Path is marked as #[repr(transparent)] so the conversion from a *mut Utf8Path to a
        //   *mut Path is valid.
        let boxed_path = unsafe { Box::from_raw(ptr) };
        Utf8PathBuf(boxed_path.into_path_buf())
    }

    // invariant: Path must be guaranteed to be utf-8 data
    #[inline]
    unsafe fn assume_utf8(path: &Path) -> &Utf8Path {
        // SAFETY: Utf8Path is marked as #[repr(transparent)] so the conversion from a
        // *const Path to a *const Utf8Path is valid.
        &*(path as *const Path as *const Utf8Path)
    }

    #[cfg(path_buf_deref_mut)]
    #[inline]
    unsafe fn assume_utf8_mut(path: &mut Path) -> &mut Utf8Path {
        &mut *(path as *mut Path as *mut Utf8Path)
    }
}

impl Clone for Box<Utf8Path> {
    fn clone(&self) -> Self {
        let boxed: Box<Path> = self.0.into();
        let ptr = Box::into_raw(boxed) as *mut Utf8Path;
        // SAFETY:
        // * self is valid UTF-8
        // * ptr was created by consuming a Box<Path> so it represents an rced pointer
        // * Utf8Path is marked as #[repr(transparent)] so the conversion from *mut Path to
        //   *mut Utf8Path is valid
        unsafe { Box::from_raw(ptr) }
    }
}

impl fmt::Display for Utf8Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Debug for Utf8Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

/// An iterator over [`Utf8Path`] and its ancestors.
///
/// This `struct` is created by the [`ancestors`] method on [`Utf8Path`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
///
/// let path = Utf8Path::new("/foo/bar");
///
/// for ancestor in path.ancestors() {
///     println!("{}", ancestor);
/// }
/// ```
///
/// [`ancestors`]: Utf8Path::ancestors
#[derive(Copy, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[repr(transparent)]
pub struct Utf8Ancestors<'a>(Ancestors<'a>);

impl fmt::Debug for Utf8Ancestors<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<'a> Iterator for Utf8Ancestors<'a> {
    type Item = &'a Utf8Path;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|path| {
            // SAFETY: Utf8Ancestors was constructed from a Utf8Path, so it is guaranteed to
            // be valid UTF-8
            unsafe { Utf8Path::assume_utf8(path) }
        })
    }
}

impl FusedIterator for Utf8Ancestors<'_> {}

/// An iterator over the [`Utf8Component`]s of a [`Utf8Path`].
///
/// This `struct` is created by the [`components`] method on [`Utf8Path`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
///
/// let path = Utf8Path::new("/tmp/foo/bar.txt");
///
/// for component in path.components() {
///     println!("{:?}", component);
/// }
/// ```
///
/// [`components`]: Utf8Path::components
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Utf8Components<'a>(Components<'a>);

impl<'a> Utf8Components<'a> {
    /// Extracts a slice corresponding to the portion of the path remaining for iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let mut components = Utf8Path::new("/tmp/foo/bar.txt").components();
    /// components.next();
    /// components.next();
    ///
    /// assert_eq!(Utf8Path::new("foo/bar.txt"), components.as_path());
    /// ```
    #[must_use]
    #[inline]
    pub fn as_path(&self) -> &'a Utf8Path {
        // SAFETY: Utf8Components was constructed from a Utf8Path, so it is guaranteed to be valid
        // UTF-8
        unsafe { Utf8Path::assume_utf8(self.0.as_path()) }
    }
}

impl<'a> Iterator for Utf8Components<'a> {
    type Item = Utf8Component<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|component| {
            // SAFETY: Utf8Component was constructed from a Utf8Path, so it is guaranteed to be
            // valid UTF-8
            unsafe { Utf8Component::new(component) }
        })
    }
}

impl FusedIterator for Utf8Components<'_> {}

impl DoubleEndedIterator for Utf8Components<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|component| {
            // SAFETY: Utf8Component was constructed from a Utf8Path, so it is guaranteed to be
            // valid UTF-8
            unsafe { Utf8Component::new(component) }
        })
    }
}

impl fmt::Debug for Utf8Components<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl AsRef<Utf8Path> for Utf8Components<'_> {
    #[inline]
    fn as_ref(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl AsRef<Path> for Utf8Components<'_> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_path().as_ref()
    }
}

impl AsRef<str> for Utf8Components<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_path().as_ref()
    }
}

impl AsRef<OsStr> for Utf8Components<'_> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_path().as_os_str()
    }
}

/// An iterator over the [`Utf8Component`]s of a [`Utf8Path`], as [`str`] slices.
///
/// This `struct` is created by the [`iter`] method on [`Utf8Path`].
/// See its documentation for more.
///
/// [`iter`]: Utf8Path::iter
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a> {
    inner: Utf8Components<'a>,
}

impl fmt::Debug for Iter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugHelper<'a>(&'a Utf8Path);

        impl fmt::Debug for DebugHelper<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        f.debug_tuple("Iter")
            .field(&DebugHelper(self.as_path()))
            .finish()
    }
}

impl<'a> Iter<'a> {
    /// Extracts a slice corresponding to the portion of the path remaining for iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let mut iter = Utf8Path::new("/tmp/foo/bar.txt").iter();
    /// iter.next();
    /// iter.next();
    ///
    /// assert_eq!(Utf8Path::new("foo/bar.txt"), iter.as_path());
    /// ```
    #[must_use]
    #[inline]
    pub fn as_path(&self) -> &'a Utf8Path {
        self.inner.as_path()
    }
}

impl AsRef<Utf8Path> for Iter<'_> {
    #[inline]
    fn as_ref(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl AsRef<Path> for Iter<'_> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_path().as_ref()
    }
}

impl AsRef<str> for Iter<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_path().as_ref()
    }
}

impl AsRef<OsStr> for Iter<'_> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_path().as_os_str()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        self.inner.next().map(|component| component.as_str())
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        self.inner.next_back().map(|component| component.as_str())
    }
}

impl FusedIterator for Iter<'_> {}

/// A single component of a path.
///
/// A [`Utf8Component`] roughly corresponds to a substring between path separators
/// (`/` or `\`).
///
/// This `enum` is created by iterating over [`Utf8Components`], which in turn is
/// created by the [`components`](Utf8Path::components) method on [`Utf8Path`].
///
/// # Examples
///
/// ```rust
/// use camino::{Utf8Component, Utf8Path};
///
/// let path = Utf8Path::new("/tmp/foo/bar.txt");
/// let components = path.components().collect::<Vec<_>>();
/// assert_eq!(&components, &[
///     Utf8Component::RootDir,
///     Utf8Component::Normal("tmp"),
///     Utf8Component::Normal("foo"),
///     Utf8Component::Normal("bar.txt"),
/// ]);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Utf8Component<'a> {
    /// A Windows path prefix, e.g., `C:` or `\\server\share`.
    ///
    /// There is a large variety of prefix types, see [`Utf8Prefix`]'s documentation
    /// for more.
    ///
    /// Does not occur on Unix.
    Prefix(Utf8PrefixComponent<'a>),

    /// The root directory component, appears after any prefix and before anything else.
    ///
    /// It represents a separator that designates that a path starts from root.
    RootDir,

    /// A reference to the current directory, i.e., `.`.
    CurDir,

    /// A reference to the parent directory, i.e., `..`.
    ParentDir,

    /// A normal component, e.g., `a` and `b` in `a/b`.
    ///
    /// This variant is the most common one, it represents references to files
    /// or directories.
    Normal(&'a str),
}

impl<'a> Utf8Component<'a> {
    unsafe fn new(component: Component<'a>) -> Utf8Component<'a> {
        match component {
            Component::Prefix(prefix) => Utf8Component::Prefix(Utf8PrefixComponent(prefix)),
            Component::RootDir => Utf8Component::RootDir,
            Component::CurDir => Utf8Component::CurDir,
            Component::ParentDir => Utf8Component::ParentDir,
            Component::Normal(s) => Utf8Component::Normal(str_assume_utf8(s)),
        }
    }

    /// Extracts the underlying [`str`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("./tmp/foo/bar.txt");
    /// let components: Vec<_> = path.components().map(|comp| comp.as_str()).collect();
    /// assert_eq!(&components, &[".", "tmp", "foo", "bar.txt"]);
    /// ```
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &'a str {
        // SAFETY: Utf8Component was constructed from a Utf8Path, so it is guaranteed to be
        // valid UTF-8
        unsafe { str_assume_utf8(self.as_os_str()) }
    }

    /// Extracts the underlying [`OsStr`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("./tmp/foo/bar.txt");
    /// let components: Vec<_> = path.components().map(|comp| comp.as_os_str()).collect();
    /// assert_eq!(&components, &[".", "tmp", "foo", "bar.txt"]);
    /// ```
    #[must_use]
    pub fn as_os_str(&self) -> &'a OsStr {
        match *self {
            Utf8Component::Prefix(prefix) => prefix.as_os_str(),
            Utf8Component::RootDir => Component::RootDir.as_os_str(),
            Utf8Component::CurDir => Component::CurDir.as_os_str(),
            Utf8Component::ParentDir => Component::ParentDir.as_os_str(),
            Utf8Component::Normal(s) => OsStr::new(s),
        }
    }
}

impl fmt::Debug for Utf8Component<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_os_str(), f)
    }
}

impl fmt::Display for Utf8Component<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl AsRef<Utf8Path> for Utf8Component<'_> {
    #[inline]
    fn as_ref(&self) -> &Utf8Path {
        self.as_str().as_ref()
    }
}

impl AsRef<Path> for Utf8Component<'_> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_os_str().as_ref()
    }
}

impl AsRef<str> for Utf8Component<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<OsStr> for Utf8Component<'_> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

/// Windows path prefixes, e.g., `C:` or `\\server\share`.
///
/// Windows uses a variety of path prefix styles, including references to drive
/// volumes (like `C:`), network shared folders (like `\\server\share`), and
/// others. In addition, some path prefixes are "verbatim" (i.e., prefixed with
/// `\\?\`), in which case `/` is *not* treated as a separator and essentially
/// no normalization is performed.
///
/// # Examples
///
/// ```
/// use camino::{Utf8Component, Utf8Path, Utf8Prefix};
/// use camino::Utf8Prefix::*;
///
/// fn get_path_prefix(s: &str) -> Utf8Prefix {
///     let path = Utf8Path::new(s);
///     match path.components().next().unwrap() {
///         Utf8Component::Prefix(prefix_component) => prefix_component.kind(),
///         _ => panic!(),
///     }
/// }
///
/// # if cfg!(windows) {
/// assert_eq!(Verbatim("pictures"), get_path_prefix(r"\\?\pictures\kittens"));
/// assert_eq!(VerbatimUNC("server", "share"), get_path_prefix(r"\\?\UNC\server\share"));
/// assert_eq!(VerbatimDisk(b'C'), get_path_prefix(r"\\?\C:\"));
/// assert_eq!(DeviceNS("BrainInterface"), get_path_prefix(r"\\.\BrainInterface"));
/// assert_eq!(UNC("server", "share"), get_path_prefix(r"\\server\share"));
/// assert_eq!(Disk(b'C'), get_path_prefix(r"C:\Users\Rust\Pictures\Ferris"));
/// # }
/// ```
#[derive(Copy, Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Utf8Prefix<'a> {
    /// Verbatim prefix, e.g., `\\?\cat_pics`.
    ///
    /// Verbatim prefixes consist of `\\?\` immediately followed by the given
    /// component.
    Verbatim(&'a str),

    /// Verbatim prefix using Windows' _**U**niform **N**aming **C**onvention_,
    /// e.g., `\\?\UNC\server\share`.
    ///
    /// Verbatim UNC prefixes consist of `\\?\UNC\` immediately followed by the
    /// server's hostname and a share name.
    VerbatimUNC(&'a str, &'a str),

    /// Verbatim disk prefix, e.g., `\\?\C:`.
    ///
    /// Verbatim disk prefixes consist of `\\?\` immediately followed by the
    /// drive letter and `:`.
    VerbatimDisk(u8),

    /// Device namespace prefix, e.g., `\\.\COM42`.
    ///
    /// Device namespace prefixes consist of `\\.\` immediately followed by the
    /// device name.
    DeviceNS(&'a str),

    /// Prefix using Windows' _**U**niform **N**aming **C**onvention_, e.g.
    /// `\\server\share`.
    ///
    /// UNC prefixes consist of the server's hostname and a share name.
    UNC(&'a str, &'a str),

    /// Prefix `C:` for the given disk drive.
    Disk(u8),
}

impl Utf8Prefix<'_> {
    /// Determines if the prefix is verbatim, i.e., begins with `\\?\`.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Prefix::*;
    ///
    /// assert!(Verbatim("pictures").is_verbatim());
    /// assert!(VerbatimUNC("server", "share").is_verbatim());
    /// assert!(VerbatimDisk(b'C').is_verbatim());
    /// assert!(!DeviceNS("BrainInterface").is_verbatim());
    /// assert!(!UNC("server", "share").is_verbatim());
    /// assert!(!Disk(b'C').is_verbatim());
    /// ```
    #[must_use]
    pub fn is_verbatim(&self) -> bool {
        use Utf8Prefix::*;
        matches!(self, Verbatim(_) | VerbatimDisk(_) | VerbatimUNC(..))
    }
}

/// A structure wrapping a Windows path prefix as well as its unparsed string
/// representation.
///
/// In addition to the parsed [`Utf8Prefix`] information returned by [`kind`],
/// [`Utf8PrefixComponent`] also holds the raw and unparsed [`str`] slice,
/// returned by [`as_str`].
///
/// Instances of this `struct` can be obtained by matching against the
/// [`Prefix` variant] on [`Utf8Component`].
///
/// Does not occur on Unix.
///
/// # Examples
///
/// ```
/// # if cfg!(windows) {
/// use camino::{Utf8Component, Utf8Path, Utf8Prefix};
/// use std::ffi::OsStr;
///
/// let path = Utf8Path::new(r"C:\you\later\");
/// match path.components().next().unwrap() {
///     Utf8Component::Prefix(prefix_component) => {
///         assert_eq!(Utf8Prefix::Disk(b'C'), prefix_component.kind());
///         assert_eq!("C:", prefix_component.as_str());
///     }
///     _ => unreachable!(),
/// }
/// # }
/// ```
///
/// [`as_str`]: Utf8PrefixComponent::as_str
/// [`kind`]: Utf8PrefixComponent::kind
/// [`Prefix` variant]: Utf8Component::Prefix
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Utf8PrefixComponent<'a>(PrefixComponent<'a>);

impl<'a> Utf8PrefixComponent<'a> {
    /// Returns the parsed prefix data.
    ///
    /// See [`Utf8Prefix`]'s documentation for more information on the different
    /// kinds of prefixes.
    #[must_use]
    pub fn kind(&self) -> Utf8Prefix<'a> {
        // SAFETY for all the below unsafe blocks: the path self was originally constructed from was
        // UTF-8 so any parts of it are valid UTF-8
        match self.0.kind() {
            Prefix::Verbatim(prefix) => Utf8Prefix::Verbatim(unsafe { str_assume_utf8(prefix) }),
            Prefix::VerbatimUNC(server, share) => {
                let server = unsafe { str_assume_utf8(server) };
                let share = unsafe { str_assume_utf8(share) };
                Utf8Prefix::VerbatimUNC(server, share)
            }
            Prefix::VerbatimDisk(drive) => Utf8Prefix::VerbatimDisk(drive),
            Prefix::DeviceNS(prefix) => Utf8Prefix::DeviceNS(unsafe { str_assume_utf8(prefix) }),
            Prefix::UNC(server, share) => {
                let server = unsafe { str_assume_utf8(server) };
                let share = unsafe { str_assume_utf8(share) };
                Utf8Prefix::UNC(server, share)
            }
            Prefix::Disk(drive) => Utf8Prefix::Disk(drive),
        }
    }

    /// Returns the [`str`] slice for this prefix.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &'a str {
        // SAFETY: Utf8PrefixComponent was constructed from a Utf8Path, so it is guaranteed to be
        // valid UTF-8
        unsafe { str_assume_utf8(self.as_os_str()) }
    }

    /// Returns the raw [`OsStr`] slice for this prefix.
    #[must_use]
    #[inline]
    pub fn as_os_str(&self) -> &'a OsStr {
        self.0.as_os_str()
    }
}

impl fmt::Debug for Utf8PrefixComponent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Utf8PrefixComponent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

// ---
// read_dir_utf8
// ---

/// Iterator over the entries in a directory.
///
/// This iterator is returned from [`Utf8Path::read_dir_utf8`] and will yield instances of
/// <code>[io::Result]<[Utf8DirEntry]></code>. Through a [`Utf8DirEntry`] information like the entry's path
/// and possibly other metadata can be learned.
///
/// The order in which this iterator returns entries is platform and filesystem
/// dependent.
///
/// # Errors
///
/// This [`io::Result`] will be an [`Err`] if there's some sort of intermittent
/// IO error during iteration.
///
/// If a directory entry is not UTF-8, an [`io::Error`] is returned with the
/// [`ErrorKind`](io::ErrorKind) set to [`InvalidData`][io::ErrorKind::InvalidData]
/// and the payload set to a [`FromPathBufError`].
#[derive(Debug)]
pub struct ReadDirUtf8 {
    inner: fs::ReadDir,
}

impl Iterator for ReadDirUtf8 {
    type Item = io::Result<Utf8DirEntry>;

    fn next(&mut self) -> Option<io::Result<Utf8DirEntry>> {
        self.inner
            .next()
            .map(|entry| entry.and_then(Utf8DirEntry::new))
    }
}

/// Entries returned by the [`ReadDirUtf8`] iterator.
///
/// An instance of [`Utf8DirEntry`] represents an entry inside of a directory on the filesystem. Each
/// entry can be inspected via methods to learn about the full path or possibly other metadata.
#[derive(Debug)]
pub struct Utf8DirEntry {
    inner: fs::DirEntry,
    path: Utf8PathBuf,
}

impl Utf8DirEntry {
    fn new(inner: fs::DirEntry) -> io::Result<Self> {
        let path = inner
            .path()
            .try_into()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        Ok(Self { inner, path })
    }

    /// Returns the full path to the file that this entry represents.
    ///
    /// The full path is created by joining the original path to `read_dir`
    /// with the filename of this entry.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     for entry in Utf8Path::new(".").read_dir_utf8()? {
    ///         let dir = entry?;
    ///         println!("{}", dir.path());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This prints output like:
    ///
    /// ```text
    /// ./whatever.txt
    /// ./foo.html
    /// ./hello_world.rs
    /// ```
    ///
    /// The exact text, of course, depends on what files you have in `.`.
    #[inline]
    pub fn path(&self) -> &Utf8Path {
        &self.path
    }

    /// Returns the metadata for the file that this entry points at.
    ///
    /// This function will not traverse symlinks if this entry points at a symlink. To traverse
    /// symlinks use [`Utf8Path::metadata`] or [`fs::File::metadata`].
    ///
    /// # Platform-specific behavior
    ///
    /// On Windows this function is cheap to call (no extra system calls
    /// needed), but on Unix platforms this function is the equivalent of
    /// calling `symlink_metadata` on the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// if let Ok(entries) = Utf8Path::new(".").read_dir_utf8() {
    ///     for entry in entries {
    ///         if let Ok(entry) = entry {
    ///             // Here, `entry` is a `Utf8DirEntry`.
    ///             if let Ok(metadata) = entry.metadata() {
    ///                 // Now let's show our entry's permissions!
    ///                 println!("{}: {:?}", entry.path(), metadata.permissions());
    ///             } else {
    ///                 println!("Couldn't get metadata for {}", entry.path());
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.inner.metadata()
    }

    /// Returns the file type for the file that this entry points at.
    ///
    /// This function will not traverse symlinks if this entry points at a
    /// symlink.
    ///
    /// # Platform-specific behavior
    ///
    /// On Windows and most Unix platforms this function is free (no extra
    /// system calls needed), but some Unix platforms may require the equivalent
    /// call to `symlink_metadata` to learn about the target file type.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// if let Ok(entries) = Utf8Path::new(".").read_dir_utf8() {
    ///     for entry in entries {
    ///         if let Ok(entry) = entry {
    ///             // Here, `entry` is a `DirEntry`.
    ///             if let Ok(file_type) = entry.file_type() {
    ///                 // Now let's show our entry's file type!
    ///                 println!("{}: {:?}", entry.path(), file_type);
    ///             } else {
    ///                 println!("Couldn't get file type for {}", entry.path());
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn file_type(&self) -> io::Result<fs::FileType> {
        self.inner.file_type()
    }

    /// Returns the bare file name of this directory entry without any other
    /// leading path component.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// if let Ok(entries) = Utf8Path::new(".").read_dir_utf8() {
    ///     for entry in entries {
    ///         if let Ok(entry) = entry {
    ///             // Here, `entry` is a `DirEntry`.
    ///             println!("{}", entry.file_name());
    ///         }
    ///     }
    /// }
    /// ```
    pub fn file_name(&self) -> &str {
        self.path
            .file_name()
            .expect("path created through DirEntry must have a filename")
    }

    /// Returns the original [`fs::DirEntry`] within this [`Utf8DirEntry`].
    #[inline]
    pub fn into_inner(self) -> fs::DirEntry {
        self.inner
    }

    /// Returns the full path to the file that this entry represents.
    ///
    /// This is analogous to [`path`], but moves ownership of the path.
    ///
    /// [`path`]: struct.Utf8DirEntry.html#method.path
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_path(self) -> Utf8PathBuf {
        self.path
    }
}

impl From<String> for Utf8PathBuf {
    fn from(string: String) -> Utf8PathBuf {
        Utf8PathBuf(string.into())
    }
}

impl FromStr for Utf8PathBuf {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Utf8PathBuf(s.into()))
    }
}

// ---
// From impls: borrowed -> borrowed
// ---

impl<'a> From<&'a str> for &'a Utf8Path {
    fn from(s: &'a str) -> &'a Utf8Path {
        Utf8Path::new(s)
    }
}

// ---
// From impls: borrowed -> owned
// ---

impl<T: ?Sized + AsRef<str>> From<&T> for Utf8PathBuf {
    fn from(s: &T) -> Utf8PathBuf {
        Utf8PathBuf::from(s.as_ref().to_owned())
    }
}

impl<T: ?Sized + AsRef<str>> From<&T> for Box<Utf8Path> {
    fn from(s: &T) -> Box<Utf8Path> {
        Utf8PathBuf::from(s).into_boxed_path()
    }
}

impl From<&'_ Utf8Path> for Arc<Utf8Path> {
    fn from(path: &Utf8Path) -> Arc<Utf8Path> {
        let arc: Arc<Path> = Arc::from(AsRef::<Path>::as_ref(path));
        let ptr = Arc::into_raw(arc) as *const Utf8Path;
        // SAFETY:
        // * path is valid UTF-8
        // * ptr was created by consuming an Arc<Path> so it represents an arced pointer
        // * Utf8Path is marked as #[repr(transparent)] so the conversion from *const Path to
        //   *const Utf8Path is valid
        unsafe { Arc::from_raw(ptr) }
    }
}

impl From<&'_ Utf8Path> for Rc<Utf8Path> {
    fn from(path: &Utf8Path) -> Rc<Utf8Path> {
        let rc: Rc<Path> = Rc::from(AsRef::<Path>::as_ref(path));
        let ptr = Rc::into_raw(rc) as *const Utf8Path;
        // SAFETY:
        // * path is valid UTF-8
        // * ptr was created by consuming an Rc<Path> so it represents an rced pointer
        // * Utf8Path is marked as #[repr(transparent)] so the conversion from *const Path to
        //   *const Utf8Path is valid
        unsafe { Rc::from_raw(ptr) }
    }
}

impl<'a> From<&'a Utf8Path> for Cow<'a, Utf8Path> {
    fn from(path: &'a Utf8Path) -> Cow<'a, Utf8Path> {
        Cow::Borrowed(path)
    }
}

impl From<&'_ Utf8Path> for Box<Path> {
    fn from(path: &Utf8Path) -> Box<Path> {
        AsRef::<Path>::as_ref(path).into()
    }
}

impl From<&'_ Utf8Path> for Arc<Path> {
    fn from(path: &Utf8Path) -> Arc<Path> {
        AsRef::<Path>::as_ref(path).into()
    }
}

impl From<&'_ Utf8Path> for Rc<Path> {
    fn from(path: &Utf8Path) -> Rc<Path> {
        AsRef::<Path>::as_ref(path).into()
    }
}

impl<'a> From<&'a Utf8Path> for Cow<'a, Path> {
    fn from(path: &'a Utf8Path) -> Cow<'a, Path> {
        Cow::Borrowed(path.as_ref())
    }
}

// ---
// From impls: owned -> owned
// ---

impl From<Box<Utf8Path>> for Utf8PathBuf {
    fn from(path: Box<Utf8Path>) -> Utf8PathBuf {
        path.into_path_buf()
    }
}

impl From<Utf8PathBuf> for Box<Utf8Path> {
    fn from(path: Utf8PathBuf) -> Box<Utf8Path> {
        path.into_boxed_path()
    }
}

impl<'a> From<Cow<'a, Utf8Path>> for Utf8PathBuf {
    fn from(path: Cow<'a, Utf8Path>) -> Utf8PathBuf {
        path.into_owned()
    }
}

impl From<Utf8PathBuf> for String {
    fn from(path: Utf8PathBuf) -> String {
        path.into_string()
    }
}

impl From<Utf8PathBuf> for OsString {
    fn from(path: Utf8PathBuf) -> OsString {
        path.into_os_string()
    }
}

impl<'a> From<Utf8PathBuf> for Cow<'a, Utf8Path> {
    fn from(path: Utf8PathBuf) -> Cow<'a, Utf8Path> {
        Cow::Owned(path)
    }
}

impl From<Utf8PathBuf> for Arc<Utf8Path> {
    fn from(path: Utf8PathBuf) -> Arc<Utf8Path> {
        let arc: Arc<Path> = Arc::from(path.0);
        let ptr = Arc::into_raw(arc) as *const Utf8Path;
        // SAFETY:
        // * path is valid UTF-8
        // * ptr was created by consuming an Arc<Path> so it represents an arced pointer
        // * Utf8Path is marked as #[repr(transparent)] so the conversion from *const Path to
        //   *const Utf8Path is valid
        unsafe { Arc::from_raw(ptr) }
    }
}

impl From<Utf8PathBuf> for Rc<Utf8Path> {
    fn from(path: Utf8PathBuf) -> Rc<Utf8Path> {
        let rc: Rc<Path> = Rc::from(path.0);
        let ptr = Rc::into_raw(rc) as *const Utf8Path;
        // SAFETY:
        // * path is valid UTF-8
        // * ptr was created by consuming an Rc<Path> so it represents an rced pointer
        // * Utf8Path is marked as #[repr(transparent)] so the conversion from *const Path to
        //   *const Utf8Path is valid
        unsafe { Rc::from_raw(ptr) }
    }
}

impl From<Utf8PathBuf> for PathBuf {
    fn from(path: Utf8PathBuf) -> PathBuf {
        path.0
    }
}

impl From<Utf8PathBuf> for Box<Path> {
    fn from(path: Utf8PathBuf) -> Box<Path> {
        PathBuf::from(path).into_boxed_path()
    }
}

impl From<Utf8PathBuf> for Arc<Path> {
    fn from(path: Utf8PathBuf) -> Arc<Path> {
        PathBuf::from(path).into()
    }
}

impl From<Utf8PathBuf> for Rc<Path> {
    fn from(path: Utf8PathBuf) -> Rc<Path> {
        PathBuf::from(path).into()
    }
}

impl<'a> From<Utf8PathBuf> for Cow<'a, Path> {
    fn from(path: Utf8PathBuf) -> Cow<'a, Path> {
        PathBuf::from(path).into()
    }
}

// ---
// TryFrom impls
// ---

impl TryFrom<PathBuf> for Utf8PathBuf {
    type Error = FromPathBufError;

    fn try_from(path: PathBuf) -> Result<Utf8PathBuf, Self::Error> {
        Utf8PathBuf::from_path_buf(path).map_err(|path| FromPathBufError {
            path,
            error: FromPathError(()),
        })
    }
}

impl TryFrom<OsString> for Utf8PathBuf {
    type Error = FromOsStringError;

    fn try_from(os_string: OsString) -> Result<Utf8PathBuf, Self::Error> {
        Utf8PathBuf::from_os_string(os_string).map_err(|os_string| FromOsStringError {
            os_string,
            error: FromOsStrError(()),
        })
    }
}

/// Converts a [`Path`] to a [`Utf8Path`].
///
/// Returns [`FromPathError`] if the path is not valid UTF-8.
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
/// use std::convert::TryFrom;
/// use std::ffi::OsStr;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
/// use std::path::Path;
///
/// let unicode_path = Path::new("/valid/unicode");
/// <&Utf8Path>::try_from(unicode_path).expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// let non_unicode_path = Path::new(non_unicode_str);
/// # #[cfg(unix)]
/// assert!(<&Utf8Path>::try_from(non_unicode_path).is_err(), "non-Unicode path failed");
/// ```
impl<'a> TryFrom<&'a Path> for &'a Utf8Path {
    type Error = FromPathError;

    fn try_from(path: &'a Path) -> Result<&'a Utf8Path, Self::Error> {
        Utf8Path::from_path(path).ok_or(FromPathError(()))
    }
}

/// Converts an [`OsStr`] to a [`Utf8Path`].
///
/// Returns the original [`OsStr`] if it is not valid UTF-8.
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
/// use std::convert::TryFrom;
/// use std::ffi::OsStr;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
/// use std::path::Path;
///
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// assert!(<&Utf8Path>::try_from(non_unicode_str).is_err(), "non-Unicode string path failed");
/// ```
impl<'a> TryFrom<&'a OsStr> for &'a Utf8Path {
    type Error = FromOsStrError;

    fn try_from(os_str: &'a OsStr) -> Result<&'a Utf8Path, Self::Error> {
        Utf8Path::from_os_str(os_str).ok_or(FromOsStrError(()))
    }
}

/// A possible error value while converting a [`PathBuf`] to a [`Utf8PathBuf`].
///
/// Produced by the [`TryFrom<&PathBuf>`][tryfrom] implementation for [`Utf8PathBuf`].
///
/// [tryfrom]: Utf8PathBuf#impl-TryFrom<PathBuf>-for-Utf8PathBuf
///
/// # Examples
///
/// ```
/// use camino::{Utf8PathBuf, FromPathBufError};
/// use std::convert::{TryFrom, TryInto};
/// use std::ffi::OsStr;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
/// use std::path::PathBuf;
///
/// let unicode_path = PathBuf::from("/valid/unicode");
/// let utf8_path_buf: Utf8PathBuf = unicode_path.try_into().expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// let non_unicode_path = PathBuf::from(non_unicode_str);
/// # #[cfg(unix)]
/// let err: FromPathBufError = Utf8PathBuf::try_from(non_unicode_path.clone())
///     .expect_err("non-Unicode path failed");
/// # #[cfg(unix)]
/// assert_eq!(err.as_path(), &non_unicode_path);
/// # #[cfg(unix)]
/// assert_eq!(err.into_path_buf(), non_unicode_path);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FromPathBufError {
    path: PathBuf,
    error: FromPathError,
}

impl FromPathBufError {
    /// Returns the [`Path`] slice that was attempted to be converted to [`Utf8PathBuf`].
    #[inline]
    pub fn as_path(&self) -> &Path {
        &self.path
    }

    /// Returns the [`PathBuf`] that was attempted to be converted to [`Utf8PathBuf`].
    #[inline]
    pub fn into_path_buf(self) -> PathBuf {
        self.path
    }

    /// Fetches a [`FromPathError`] for more about the conversion failure.
    ///
    /// At the moment this struct does not contain any additional information, but is provided for
    /// completeness.
    #[inline]
    pub fn from_path_error(&self) -> FromPathError {
        self.error
    }

    /// Converts self into a [`std::io::Error`] with kind
    /// [`InvalidData`](io::ErrorKind::InvalidData).
    ///
    /// Many users of [`FromPathBufError`] will want to convert it into an [`io::Error`]. This is a
    /// convenience method to do that.
    pub fn into_io_error(self) -> io::Error {
        // NOTE: we don't currently implement `From<FromPathBufError> for io::Error` because we want
        // to ensure the user actually desires that conversion.
        io::Error::new(io::ErrorKind::InvalidData, self)
    }
}

impl fmt::Display for FromPathBufError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PathBuf contains invalid UTF-8: {}", self.path.display())
    }
}

impl error::Error for FromPathBufError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.error)
    }
}

/// A possible error value while converting a [`Path`] to a [`Utf8Path`].
///
/// Produced by the [`TryFrom<&Path>`][tryfrom] implementation for [`&Utf8Path`](Utf8Path).
///
/// [tryfrom]: Utf8Path#impl-TryFrom<%26Path>-for-%26Utf8Path
///
///
/// # Examples
///
/// ```
/// use camino::{Utf8Path, FromPathError};
/// use std::convert::{TryFrom, TryInto};
/// use std::ffi::OsStr;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
/// use std::path::Path;
///
/// let unicode_path = Path::new("/valid/unicode");
/// let utf8_path: &Utf8Path = unicode_path.try_into().expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// let non_unicode_path = Path::new(non_unicode_str);
/// # #[cfg(unix)]
/// let err: FromPathError = <&Utf8Path>::try_from(non_unicode_path)
///     .expect_err("non-Unicode path failed");
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FromPathError(());

impl FromPathError {
    /// Converts self into a [`std::io::Error`] with kind
    /// [`InvalidData`](io::ErrorKind::InvalidData).
    ///
    /// Many users of [`FromPathError`] will want to convert it into an [`io::Error`]. This is a
    /// convenience method to do that.
    pub fn into_io_error(self) -> io::Error {
        // NOTE: we don't currently implement `From<FromPathError> for io::Error` because we want
        // to ensure the user actually desires that conversion.
        io::Error::new(io::ErrorKind::InvalidData, self)
    }
}

impl fmt::Display for FromPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path contains invalid UTF-8")
    }
}

impl error::Error for FromPathError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

/// A possible error value while converting a [`OsString`] to a [`Utf8PathBuf`].
///
/// Produced by the `TryFrom<OsString>` implementation for [`Utf8PathBuf`].
///
/// # Examples
///
/// ```
/// # #[cfg(osstring_from_str)] {
/// use camino::{Utf8PathBuf, FromOsStringError};
/// use std::convert::{TryFrom, TryInto};
/// use std::ffi::OsStr;
/// use std::str::FromStr;
/// use std::ffi::OsString;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
///
/// let unicode_string = OsString::from_str("/valid/unicode").unwrap();
/// let utf8_path_buf: Utf8PathBuf = unicode_string.try_into()
///     .expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_string = OsStr::from_bytes(b"\xFF\xFF\xFF").to_owned();
/// # #[cfg(unix)]
/// let err: FromOsStringError = Utf8PathBuf::try_from(non_unicode_string.clone())
///     .expect_err("non-Unicode path failed");
/// # #[cfg(unix)]
/// assert_eq!(err.as_os_str(), &non_unicode_string);
/// # #[cfg(unix)]
/// assert_eq!(err.into_os_string(), non_unicode_string);
/// # }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FromOsStringError {
    os_string: OsString,
    error: FromOsStrError,
}

impl FromOsStringError {
    /// Returns the [`OsStr`] slice that was attempted to be converted to [`Utf8PathBuf`].
    #[inline]
    pub fn as_os_str(&self) -> &OsStr {
        &self.os_string
    }

    /// Returns the [`OsString`] that was attempted to be converted to [`Utf8PathBuf`].
    #[inline]
    pub fn into_os_string(self) -> OsString {
        self.os_string
    }

    /// Fetches a [`FromOsStrError`] for more about the conversion failure.
    ///
    /// At the moment this struct does not contain any additional information, but is provided for
    /// completeness.
    #[inline]
    pub fn from_os_str_error(&self) -> FromOsStrError {
        self.error
    }

    /// Converts self into a [`std::io::Error`] with kind
    /// [`InvalidData`](io::ErrorKind::InvalidData).
    ///
    /// Many users of [`FromOsStringError`] will want to convert it into an [`io::Error`].
    /// This is a convenience method to do that.
    pub fn into_io_error(self) -> io::Error {
        // NOTE: we don't currently implement `From<FromOsStringError> for io::Error`
        // because we want to ensure the user actually desires that conversion.
        io::Error::new(io::ErrorKind::InvalidData, self)
    }
}

impl fmt::Display for FromOsStringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OsString contains invalid UTF-8: {}",
            // self.os_string.display() // this item is stable since `1.87.0`
            PathBuf::from(&self.os_string).display() // msrv hack
        )
    }
}

impl error::Error for FromOsStringError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.error)
    }
}

/// A possible error value while converting a [`OsStr`] to a [`Utf8Path`].
///
/// Produced by the `TryFrom<&OsStr>` implementation for [`&Utf8Path`](Utf8Path).
///
///
/// # Examples
///
/// ```
/// use camino::{Utf8Path, FromOsStrError};
/// use std::convert::{TryFrom, TryInto};
/// use std::ffi::OsStr;
/// # #[cfg(unix)]
/// use std::os::unix::ffi::OsStrExt;
///
/// let unicode_str = OsStr::new("/valid/unicode");
/// let utf8_path: &Utf8Path = unicode_str.try_into().expect("valid Unicode path succeeded");
///
/// // Paths on Unix can be non-UTF-8.
/// # #[cfg(unix)]
/// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
/// # #[cfg(unix)]
/// let err: FromOsStrError = <&Utf8Path>::try_from(non_unicode_str)
///     .expect_err("non-Unicode path failed");
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FromOsStrError(());

impl FromOsStrError {
    /// Converts self into a [`std::io::Error`] with kind
    /// [`InvalidData`](io::ErrorKind::InvalidData).
    ///
    /// Many users of [`FromOsStrError`] will want to convert it into an [`io::Error`]. This is a
    /// convenience method to do that.
    pub fn into_io_error(self) -> io::Error {
        // NOTE: we don't currently implement `From<FromOsStrError> for io::Error`
        // because we want to ensure the user actually desires that conversion.
        io::Error::new(io::ErrorKind::InvalidData, self)
    }
}

impl fmt::Display for FromOsStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OsStr contains invalid UTF-8")
    }
}

impl error::Error for FromOsStrError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

// ---
// AsRef impls
// ---

impl AsRef<Utf8Path> for Utf8Path {
    #[inline]
    fn as_ref(&self) -> &Utf8Path {
        self
    }
}

impl AsRef<Utf8Path> for Utf8PathBuf {
    #[inline]
    fn as_ref(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl AsRef<Utf8Path> for str {
    #[inline]
    fn as_ref(&self) -> &Utf8Path {
        Utf8Path::new(self)
    }
}

impl AsRef<Utf8Path> for String {
    #[inline]
    fn as_ref(&self) -> &Utf8Path {
        Utf8Path::new(self)
    }
}

impl AsRef<Path> for Utf8Path {
    #[inline]
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for Utf8PathBuf {
    #[inline]
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<str> for Utf8Path {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Utf8PathBuf {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<OsStr> for Utf8Path {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl AsRef<OsStr> for Utf8PathBuf {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

// ---
// Borrow and ToOwned
// ---

impl Borrow<Utf8Path> for Utf8PathBuf {
    #[inline]
    fn borrow(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl ToOwned for Utf8Path {
    type Owned = Utf8PathBuf;

    #[inline]
    fn to_owned(&self) -> Utf8PathBuf {
        self.to_path_buf()
    }
}

impl<P: AsRef<Utf8Path>> std::iter::FromIterator<P> for Utf8PathBuf {
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Utf8PathBuf {
        let mut buf = Utf8PathBuf::new();
        buf.extend(iter);
        buf
    }
}

// ---
// [Partial]Eq, [Partial]Ord, Hash
// ---

impl PartialEq for Utf8PathBuf {
    #[inline]
    fn eq(&self, other: &Utf8PathBuf) -> bool {
        self.components() == other.components()
    }
}

impl Eq for Utf8PathBuf {}

impl Hash for Utf8PathBuf {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_path().hash(state)
    }
}

impl PartialOrd for Utf8PathBuf {
    #[inline]
    fn partial_cmp(&self, other: &Utf8PathBuf) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Utf8PathBuf {
    fn cmp(&self, other: &Utf8PathBuf) -> Ordering {
        self.components().cmp(other.components())
    }
}

impl PartialEq for Utf8Path {
    #[inline]
    fn eq(&self, other: &Utf8Path) -> bool {
        self.components().eq(other.components())
    }
}

impl Eq for Utf8Path {}

impl Hash for Utf8Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for component in self.components() {
            component.hash(state)
        }
    }
}

impl PartialOrd for Utf8Path {
    #[inline]
    fn partial_cmp(&self, other: &Utf8Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Utf8Path {
    fn cmp(&self, other: &Utf8Path) -> Ordering {
        self.components().cmp(other.components())
    }
}

impl<'a> IntoIterator for &'a Utf8PathBuf {
    type Item = &'a str;
    type IntoIter = Iter<'a>;
    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a Utf8Path {
    type Item = &'a str;
    type IntoIter = Iter<'a>;
    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Utf8Path as PartialEq>::eq(self, other)
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Utf8Path as PartialEq>::eq(self, other)
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                <Utf8Path as PartialOrd>::partial_cmp(self, other)
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
                <Utf8Path as PartialOrd>::partial_cmp(self, other)
            }
        }
    };
}

impl_cmp!(Utf8PathBuf, Utf8Path);
impl_cmp!(Utf8PathBuf, &'a Utf8Path);
impl_cmp!(Cow<'a, Utf8Path>, Utf8Path);
impl_cmp!(Cow<'a, Utf8Path>, &'b Utf8Path);
impl_cmp!(Cow<'a, Utf8Path>, Utf8PathBuf);

macro_rules! impl_cmp_std_path {
    ($lhs:ty, $rhs: ty) => {
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Path as PartialEq>::eq(self.as_ref(), other)
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Path as PartialEq>::eq(self, other.as_ref())
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<std::cmp::Ordering> {
                <Path as PartialOrd>::partial_cmp(self.as_ref(), other)
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<std::cmp::Ordering> {
                <Path as PartialOrd>::partial_cmp(self, other.as_ref())
            }
        }
    };
}

impl_cmp_std_path!(Utf8PathBuf, Path);
impl_cmp_std_path!(Utf8PathBuf, &'a Path);
impl_cmp_std_path!(Utf8PathBuf, Cow<'a, Path>);
impl_cmp_std_path!(Utf8PathBuf, PathBuf);
impl_cmp_std_path!(Utf8Path, Path);
impl_cmp_std_path!(Utf8Path, &'a Path);
impl_cmp_std_path!(Utf8Path, Cow<'a, Path>);
impl_cmp_std_path!(Utf8Path, PathBuf);
impl_cmp_std_path!(&'a Utf8Path, Path);
impl_cmp_std_path!(&'a Utf8Path, Cow<'b, Path>);
impl_cmp_std_path!(&'a Utf8Path, PathBuf);
// NOTE: impls for Cow<'a, Utf8Path> cannot be defined because of the orphan rule (E0117)

macro_rules! impl_cmp_str {
    ($lhs:ty, $rhs: ty) => {
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Utf8Path as PartialEq>::eq(self, Utf8Path::new(other))
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Utf8Path as PartialEq>::eq(Utf8Path::new(self), other)
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<std::cmp::Ordering> {
                <Utf8Path as PartialOrd>::partial_cmp(self, Utf8Path::new(other))
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<std::cmp::Ordering> {
                <Utf8Path as PartialOrd>::partial_cmp(Utf8Path::new(self), other)
            }
        }
    };
}

impl_cmp_str!(Utf8PathBuf, str);
impl_cmp_str!(Utf8PathBuf, &'a str);
impl_cmp_str!(Utf8PathBuf, Cow<'a, str>);
impl_cmp_str!(Utf8PathBuf, String);
impl_cmp_str!(Utf8Path, str);
impl_cmp_str!(Utf8Path, &'a str);
impl_cmp_str!(Utf8Path, Cow<'a, str>);
impl_cmp_str!(Utf8Path, String);
impl_cmp_str!(&'a Utf8Path, str);
impl_cmp_str!(&'a Utf8Path, Cow<'b, str>);
impl_cmp_str!(&'a Utf8Path, String);
// NOTE: impls for Cow<'a, Utf8Path> cannot be defined because of the orphan rule (E0117)

macro_rules! impl_cmp_os_str {
    ($lhs:ty, $rhs: ty) => {
        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Path as PartialEq>::eq(self.as_ref(), other.as_ref())
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Path as PartialEq>::eq(self.as_ref(), other.as_ref())
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<std::cmp::Ordering> {
                <Path as PartialOrd>::partial_cmp(self.as_ref(), other.as_ref())
            }
        }

        #[allow(clippy::extra_unused_lifetimes)]
        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<std::cmp::Ordering> {
                <Path as PartialOrd>::partial_cmp(self.as_ref(), other.as_ref())
            }
        }
    };
}

impl_cmp_os_str!(Utf8PathBuf, OsStr);
impl_cmp_os_str!(Utf8PathBuf, &'a OsStr);
impl_cmp_os_str!(Utf8PathBuf, Cow<'a, OsStr>);
impl_cmp_os_str!(Utf8PathBuf, OsString);
impl_cmp_os_str!(Utf8Path, OsStr);
impl_cmp_os_str!(Utf8Path, &'a OsStr);
impl_cmp_os_str!(Utf8Path, Cow<'a, OsStr>);
impl_cmp_os_str!(Utf8Path, OsString);
impl_cmp_os_str!(&'a Utf8Path, OsStr);
impl_cmp_os_str!(&'a Utf8Path, Cow<'b, OsStr>);
impl_cmp_os_str!(&'a Utf8Path, OsString);
// NOTE: impls for Cow<'a, Utf8Path> cannot be defined because of the orphan rule (E0117)

/// Makes the path absolute without accessing the filesystem, converting it to a [`Utf8PathBuf`].
///
/// If the path is relative, the current directory is used as the base directory. All intermediate
/// components will be resolved according to platform-specific rules, but unlike
/// [`canonicalize`][Utf8Path::canonicalize] or [`canonicalize_utf8`](Utf8Path::canonicalize_utf8),
/// this does not resolve symlinks and may succeed even if the path does not exist.
///
/// *Requires Rust 1.79 or newer.*
///
/// # Errors
///
/// Errors if:
///
/// * The path is empty.
/// * The [current directory][std::env::current_dir] cannot be determined.
/// * The path is not valid UTF-8.
///
/// # Examples
///
/// ## POSIX paths
///
/// ```
/// # #[cfg(unix)]
/// fn main() -> std::io::Result<()> {
///     use camino::Utf8Path;
///
///     // Relative to absolute
///     let absolute = camino::absolute_utf8("foo/./bar")?;
///     assert!(absolute.ends_with("foo/bar"));
///
///     // Absolute to absolute
///     let absolute = camino::absolute_utf8("/foo//test/.././bar.rs")?;
///     assert_eq!(absolute, Utf8Path::new("/foo/test/../bar.rs"));
///     Ok(())
/// }
/// # #[cfg(not(unix))]
/// # fn main() {}
/// ```
///
/// The path is resolved using [POSIX semantics][posix-semantics] except that it stops short of
/// resolving symlinks. This means it will keep `..` components and trailing slashes.
///
/// ## Windows paths
///
/// ```
/// # #[cfg(windows)]
/// fn main() -> std::io::Result<()> {
///     use camino::Utf8Path;
///
///     // Relative to absolute
///     let absolute = camino::absolute_utf8("foo/./bar")?;
///     assert!(absolute.ends_with(r"foo\bar"));
///
///     // Absolute to absolute
///     let absolute = camino::absolute_utf8(r"C:\foo//test\..\./bar.rs")?;
///
///     assert_eq!(absolute, Utf8Path::new(r"C:\foo\bar.rs"));
///     Ok(())
/// }
/// # #[cfg(not(windows))]
/// # fn main() {}
/// ```
///
/// For verbatim paths this will simply return the path as given. For other paths this is currently
/// equivalent to calling [`GetFullPathNameW`][windows-path].
///
/// Note that this [may change in the future][changes].
///
/// [changes]: io#platform-specific-behavior
/// [posix-semantics]:
///     https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_13
/// [windows-path]:
///     https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfullpathnamew
#[cfg(absolute_path)]
pub fn absolute_utf8<P: AsRef<Path>>(path: P) -> io::Result<Utf8PathBuf> {
    // Note that even if the passed in path is valid UTF-8, it is not guaranteed
    // that the absolute path will be valid UTF-8. For example, the current
    // directory may not be valid UTF-8.
    //
    // That's why we take `AsRef<Path>` instead of `AsRef<Utf8Path>` here -- we
    // have to pay the cost of checking for valid UTF-8 anyway.
    let path = path.as_ref();
    #[allow(clippy::incompatible_msrv)]
    Utf8PathBuf::try_from(std::path::absolute(path)?).map_err(|error| error.into_io_error())
}

// invariant: OsStr must be guaranteed to be utf8 data
#[inline]
unsafe fn str_assume_utf8(string: &OsStr) -> &str {
    #[cfg(os_str_bytes)]
    {
        // SAFETY: OsStr is guaranteed to be utf8 data from the invariant
        unsafe {
            std::str::from_utf8_unchecked(
                #[allow(clippy::incompatible_msrv)]
                string.as_encoded_bytes(),
            )
        }
    }
    #[cfg(not(os_str_bytes))]
    {
        // Adapted from the source code for Option::unwrap_unchecked.
        match string.to_str() {
            Some(val) => val,
            None => std::hint::unreachable_unchecked(),
        }
    }
}
