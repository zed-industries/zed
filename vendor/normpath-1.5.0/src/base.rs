use std::borrow::Borrow;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::Metadata;
use std::fs::ReadDir;
use std::hash::Hash;
use std::hash::Hasher;
use std::io;
use std::mem;
use std::ops::Deref;
use std::path::Component;
use std::path::Components;
use std::path::Path;
use std::path::PathBuf;

use super::error::MissingPrefixBufError;
use super::error::MissingPrefixError;
use super::error::ParentError;
use super::imp;
use super::PathExt;

fn cow_path_into_base_path(path: Cow<'_, Path>) -> Cow<'_, BasePath> {
    debug_assert!(imp::is_base(&path));

    match path {
        Cow::Borrowed(path) => {
            Cow::Borrowed(BasePath::from_inner(path.as_os_str()))
        }
        Cow::Owned(path) => Cow::Owned(BasePathBuf(path)),
    }
}

/// A borrowed path that has a [prefix] on Windows.
///
/// Note that comparison traits such as [`PartialEq`] will compare paths
/// literally instead of comparing components. The former is more efficient and
/// easier to use correctly.
///
/// # Safety
///
/// This type should not be used for memory safety, but implementations can
/// panic if this path is missing a prefix on Windows. A safe `new_unchecked`
/// method might be added later that can safely create invalid base paths.
///
/// [prefix]: ::std::path::Prefix
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BasePath(pub(super) OsStr);

impl BasePath {
    pub(super) fn from_inner(path: &OsStr) -> &Self {
        // SAFETY: This struct has a layout that makes this operation safe.
        unsafe { mem::transmute(path) }
    }

    /// Creates a new base path.
    ///
    /// On Windows, if `path` is missing a [prefix], it will be joined to the
    /// current directory.
    ///
    /// # Errors
    ///
    /// Returns an error if reading the current directory fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// use std::path::Path;
    ///
    /// use normpath::BasePath;
    ///
    /// if cfg!(windows) {
    ///     let path = Path::new(r"X:\foo\bar");
    ///     assert_eq!(path, *BasePath::new(path)?);
    ///
    ///     assert!(BasePath::new(Path::new(r"foo\bar")).is_ok());
    /// }
    /// #
    /// # Ok::<_, io::Error>(())
    /// ```
    ///
    /// [prefix]: ::std::path::Prefix
    #[inline]
    pub fn new<'a, P>(path: P) -> io::Result<Cow<'a, Self>>
    where
        P: Into<Cow<'a, Path>>,
    {
        let path = path.into();
        match path {
            Cow::Borrowed(path) => Self::try_new(path)
                .map(Cow::Borrowed)
                .or_else(|_| imp::to_base(path).map(Cow::Owned)),
            Cow::Owned(path) => BasePathBuf::new(path).map(Cow::Owned),
        }
    }

    /// Creates a new base path.
    ///
    /// # Errors
    ///
    /// On Windows, returns an error if `path` is missing a [prefix].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// # use normpath::error::MissingPrefixError;
    /// use normpath::BasePath;
    ///
    /// if cfg!(windows) {
    ///     let path = r"X:\foo\bar";
    ///     assert_eq!(Path::new(path), BasePath::try_new(path)?);
    ///
    ///     assert!(BasePath::try_new(r"foo\bar").is_err());
    /// }
    /// #
    /// # Ok::<_, MissingPrefixError>(())
    /// ```
    ///
    /// [prefix]: ::std::path::Prefix
    #[inline]
    pub fn try_new<P>(path: &P) -> Result<&Self, MissingPrefixError>
    where
        P: AsRef<Path> + ?Sized,
    {
        let path = path.as_ref();
        if imp::is_base(path) {
            Ok(Self::from_inner(path.as_os_str()))
        } else {
            Err(MissingPrefixError(()))
        }
    }

    /// Returns a reference to the wrapped path as a platform string.
    #[inline]
    #[must_use]
    pub fn as_os_str(&self) -> &OsStr {
        &self.0
    }

    /// Returns a reference to the wrapped path.
    #[inline]
    #[must_use]
    pub fn as_path(&self) -> &Path {
        Path::new(&self.0)
    }

    /// Equivalent to [`Path::canonicalize`].
    #[inline]
    pub fn canonicalize(&self) -> io::Result<BasePathBuf> {
        self.as_path().canonicalize().map(|base| {
            debug_assert!(imp::is_base(&base));
            BasePathBuf(base)
        })
    }

    /// Equivalent to [`Path::components`].
    #[inline]
    pub fn components(&self) -> Components<'_> {
        self.as_path().components()
    }

    /// Equivalent to [`Path::ends_with`].
    #[inline]
    #[must_use]
    pub fn ends_with<P>(&self, child: P) -> bool
    where
        P: AsRef<Path>,
    {
        self.as_path().ends_with(child)
    }

    /// Equivalent to [`Path::exists`].
    #[inline]
    #[must_use]
    pub fn exists(&self) -> bool {
        self.as_path().exists()
    }

    /// Equivalent to [`PathExt::expand`].
    #[inline]
    pub fn expand(&self) -> io::Result<Cow<'_, Self>> {
        self.as_path().expand().map(cow_path_into_base_path)
    }

    /// Equivalent to [`Path::extension`].
    #[inline]
    #[must_use]
    pub fn extension(&self) -> Option<&OsStr> {
        self.as_path().extension()
    }

    /// Equivalent to [`Path::file_name`].
    #[inline]
    #[must_use]
    pub fn file_name(&self) -> Option<&OsStr> {
        self.as_path().file_name()
    }

    /// Equivalent to [`Path::file_stem`].
    #[inline]
    #[must_use]
    pub fn file_stem(&self) -> Option<&OsStr> {
        self.as_path().file_stem()
    }

    /// Equivalent to [`Path::has_root`].
    #[inline]
    #[must_use]
    pub fn has_root(&self) -> bool {
        self.as_path().has_root()
    }

    /// Equivalent to [`Path::is_absolute`].
    #[inline]
    #[must_use]
    pub fn is_absolute(&self) -> bool {
        self.as_path().is_absolute()
    }

    /// Equivalent to [`Path::is_dir`].
    #[inline]
    #[must_use]
    pub fn is_dir(&self) -> bool {
        self.as_path().is_dir()
    }

    /// Equivalent to [`Path::is_file`].
    #[inline]
    #[must_use]
    pub fn is_file(&self) -> bool {
        self.as_path().is_file()
    }

    /// Equivalent to [`Path::is_relative`].
    #[inline]
    #[must_use]
    pub fn is_relative(&self) -> bool {
        self.as_path().is_relative()
    }

    /// Equivalent to [`Path::is_symlink`].
    #[inline]
    #[must_use]
    pub fn is_symlink(&self) -> bool {
        self.as_path().is_symlink()
    }

    /// An improved version of [`Path::join`] that handles more edge cases.
    ///
    /// For example, on Windows, leading `.` and `..` components of `path` will
    /// be normalized if possible. If `self` is a [verbatim] path, it would be
    /// invalid to normalize them later.
    ///
    /// You should still call [`normalize`] before [`parent`] to normalize some
    /// additional components.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use normpath::BasePath;
    ///
    /// if cfg!(windows) {
    ///     assert_eq!(
    ///         Path::new(r"\\?\foo\baz\test.rs"),
    ///         BasePath::try_new(r"\\?\foo\bar")
    ///             .unwrap()
    ///             .join("../baz/test.rs"),
    ///     );
    /// }
    /// ```
    ///
    /// [`normalize`]: Self::normalize
    /// [`parent`]: Self::parent
    /// [verbatim]: ::std::path::Prefix::is_verbatim
    #[inline]
    pub fn join<P>(&self, path: P) -> BasePathBuf
    where
        P: AsRef<Path>,
    {
        let mut base = self.to_owned();
        base.push(path);
        base
    }

    /// Equivalent to [`PathExt::localize_name`].
    #[cfg(feature = "localization")]
    #[cfg_attr(normpath_docs_rs, doc(cfg(feature = "localization")))]
    #[inline]
    #[must_use]
    pub fn localize_name(&self) -> Cow<'_, OsStr> {
        self.as_path().localize_name()
    }

    /// Equivalent to [`Path::metadata`].
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.as_path().metadata()
    }

    /// Equivalent to [`PathExt::normalize`].
    #[inline]
    pub fn normalize(&self) -> io::Result<BasePathBuf> {
        self.as_path().normalize()
    }

    /// Equivalent to [`PathExt::normalize_virtually`].
    #[cfg(any(doc, windows))]
    #[cfg_attr(normpath_docs_rs, doc(cfg(windows)))]
    #[inline]
    pub fn normalize_virtually(&self) -> io::Result<BasePathBuf> {
        self.as_path().normalize_virtually()
    }

    fn check_parent(&self) -> Result<(), ParentError> {
        self.components()
            .next_back()
            .filter(|x| matches!(x, Component::Normal(_) | Component::RootDir))
            .map(|_| ())
            .ok_or(ParentError(()))
    }

    /// Returns this path without its last component.
    ///
    /// Returns `Ok(None)` if the last component is [`Component::RootDir`].
    ///
    /// You should usually only call this method on [normalized] paths. They
    /// will prevent an unexpected path from being returned due to symlinks,
    /// and some `.` and `..` components will be normalized.
    ///
    /// # Errors
    ///
    /// Returns an error if the last component is not [`Component::Normal`] or
    /// [`Component::RootDir`]. To ignore this error, use [`parent_unchecked`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// # use normpath::error::ParentError;
    /// use normpath::BasePath;
    ///
    /// if cfg!(windows) {
    ///     assert_eq!(
    ///         Path::new(r"X:\foo"),
    ///         BasePath::try_new(r"X:\foo\bar").unwrap().parent()?.unwrap(),
    ///     );
    /// }
    /// #
    /// # Ok::<_, ParentError>(())
    /// ```
    ///
    /// [normalized]: Self::normalize
    /// [`parent_unchecked`]: Self::parent_unchecked
    #[inline]
    pub fn parent(&self) -> Result<Option<&Self>, ParentError> {
        self.check_parent().map(|()| self.parent_unchecked())
    }

    /// Equivalent to [`Path::parent`].
    ///
    /// It is usually better to use [`parent`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use normpath::BasePath;
    ///
    /// if cfg!(windows) {
    ///     assert_eq!(
    ///         Path::new(r"X:\foo"),
    ///         BasePath::try_new(r"X:\foo\..")
    ///             .unwrap()
    ///             .parent_unchecked()
    ///             .unwrap(),
    ///     );
    /// }
    /// ```
    ///
    /// [`parent`]: Self::parent
    #[inline]
    #[must_use]
    pub fn parent_unchecked(&self) -> Option<&Self> {
        self.as_path()
            .parent()
            .map(|x| Self::from_inner(x.as_os_str()))
    }

    /// Equivalent to [`Path::read_dir`].
    #[inline]
    pub fn read_dir(&self) -> io::Result<ReadDir> {
        self.as_path().read_dir()
    }

    /// Equivalent to [`Path::read_link`].
    #[inline]
    pub fn read_link(&self) -> io::Result<PathBuf> {
        self.as_path().read_link()
    }

    /// Equivalent to [`PathExt::shorten`].
    #[inline]
    pub fn shorten(&self) -> io::Result<Cow<'_, Self>> {
        self.as_path().shorten().map(cow_path_into_base_path)
    }

    /// Equivalent to [`Path::starts_with`].
    #[inline]
    #[must_use]
    pub fn starts_with<P>(&self, base: P) -> bool
    where
        P: AsRef<Path>,
    {
        self.as_path().starts_with(base)
    }

    /// Equivalent to [`Path::symlink_metadata`].
    #[inline]
    pub fn symlink_metadata(&self) -> io::Result<Metadata> {
        self.as_path().symlink_metadata()
    }

    /// Equivalent to [`Path::try_exists`].
    #[inline]
    pub fn try_exists(&self) -> io::Result<bool> {
        self.as_path().try_exists()
    }
}

impl AsRef<OsStr> for BasePath {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        &self.0
    }
}

impl AsRef<Path> for BasePath {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl AsRef<Self> for BasePath {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'a> From<&'a BasePath> for Cow<'a, BasePath> {
    #[inline]
    fn from(value: &'a BasePath) -> Self {
        Cow::Borrowed(value)
    }
}

impl PartialEq<Path> for BasePath {
    #[inline]
    fn eq(&self, other: &Path) -> bool {
        &self.0 == other.as_os_str()
    }
}

impl PartialEq<BasePath> for Path {
    #[inline]
    fn eq(&self, other: &BasePath) -> bool {
        other == self
    }
}

impl PartialOrd<Path> for BasePath {
    #[inline]
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        self.0.partial_cmp(other.as_os_str())
    }
}

impl PartialOrd<BasePath> for Path {
    #[inline]
    fn partial_cmp(&self, other: &BasePath) -> Option<Ordering> {
        other.partial_cmp(self)
    }
}

impl ToOwned for BasePath {
    type Owned = BasePathBuf;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        BasePathBuf(self.0.to_owned().into())
    }
}

/// An owned path that has a [prefix] on Windows.
///
/// For more information, see [`BasePath`].
///
/// [prefix]: ::std::path::Prefix
#[derive(Clone, Debug)]
pub struct BasePathBuf(pub(super) PathBuf);

impl BasePathBuf {
    /// Equivalent to [`BasePath::new`] but returns an owned path.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// use std::path::Path;
    ///
    /// use normpath::BasePathBuf;
    ///
    /// if cfg!(windows) {
    ///     let path = r"X:\foo\bar";
    ///     assert_eq!(Path::new(path), BasePathBuf::new(path)?);
    ///
    ///     assert!(BasePathBuf::new(r"foo\bar").is_ok());
    /// }
    /// #
    /// # Ok::<_, io::Error>(())
    /// ```
    #[inline]
    pub fn new<P>(path: P) -> io::Result<Self>
    where
        P: Into<PathBuf>,
    {
        Self::try_new(path).or_else(|x| imp::to_base(&x.0))
    }

    /// Equivalent to [`BasePath::try_new`] but returns an owned path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// # use normpath::error::MissingPrefixBufError;
    /// use normpath::BasePathBuf;
    ///
    /// if cfg!(windows) {
    ///     let path = r"X:\foo\bar";
    ///     assert_eq!(Path::new(path), BasePathBuf::try_new(path)?);
    ///
    ///     assert!(BasePathBuf::try_new(r"foo\bar").is_err());
    /// }
    /// #
    /// # Ok::<_, MissingPrefixBufError>(())
    /// ```
    #[inline]
    pub fn try_new<P>(path: P) -> Result<Self, MissingPrefixBufError>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        if imp::is_base(&path) {
            Ok(Self(path))
        } else {
            Err(MissingPrefixBufError(path))
        }
    }

    /// Returns the wrapped path as a platform string.
    #[inline]
    #[must_use]
    pub fn into_os_string(self) -> OsString {
        self.0.into_os_string()
    }

    /// Returns the wrapped path.
    #[inline]
    #[must_use]
    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }

    /// Equivalent to [`BasePath::parent`] but modifies `self` in place.
    ///
    /// Returns `Ok(false)` when [`BasePath::parent`] returns `Ok(None)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// # use normpath::error::ParentError;
    /// use normpath::BasePathBuf;
    ///
    /// if cfg!(windows) {
    ///     let mut path = BasePathBuf::try_new(r"X:\foo\bar").unwrap();
    ///     assert!(path.pop()?);
    ///     assert_eq!(Path::new(r"X:\foo"), path);
    /// }
    /// #
    /// # Ok::<_, ParentError>(())
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Result<bool, ParentError> {
        self.check_parent().map(|()| self.pop_unchecked())
    }

    /// Equivalent to [`PathBuf::pop`].
    ///
    /// It is usually better to use [`pop`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use normpath::BasePathBuf;
    ///
    /// if cfg!(windows) {
    ///     let mut path = BasePathBuf::try_new(r"X:\foo\..").unwrap();
    ///     assert!(path.pop_unchecked());
    ///     assert_eq!(Path::new(r"X:\foo"), path);
    /// }
    /// ```
    ///
    /// [`pop`]: Self::pop
    #[inline]
    pub fn pop_unchecked(&mut self) -> bool {
        self.0.pop()
    }

    /// Equivalent to [`BasePath::join`] but modifies `self` in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use normpath::BasePathBuf;
    ///
    /// if cfg!(windows) {
    ///     let mut path = BasePathBuf::try_new(r"\\?\foo\bar").unwrap();
    ///     path.push("../baz/test.rs");
    ///     assert_eq!(Path::new(r"\\?\foo\baz\test.rs"), path);
    /// }
    /// ```
    #[inline]
    pub fn push<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        imp::push(self, path.as_ref());
    }
}

impl AsRef<OsStr> for BasePathBuf {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl AsRef<Path> for BasePathBuf {
    #[inline]
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<BasePath> for BasePathBuf {
    #[inline]
    fn as_ref(&self) -> &BasePath {
        self
    }
}

impl Borrow<BasePath> for BasePathBuf {
    #[inline]
    fn borrow(&self) -> &BasePath {
        self
    }
}

impl Deref for BasePathBuf {
    type Target = BasePath;

    #[inline]
    fn deref(&self) -> &BasePath {
        BasePath::from_inner(self.0.as_os_str())
    }
}

impl Eq for BasePathBuf {}

impl From<BasePathBuf> for Cow<'_, BasePath> {
    #[inline]
    fn from(value: BasePathBuf) -> Self {
        Cow::Owned(value)
    }
}

impl From<BasePathBuf> for OsString {
    #[inline]
    fn from(value: BasePathBuf) -> Self {
        value.into_os_string()
    }
}

impl From<BasePathBuf> for PathBuf {
    #[inline]
    fn from(value: BasePathBuf) -> Self {
        value.0
    }
}

impl Hash for BasePathBuf {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (**self).hash(state);
    }
}

impl Ord for BasePathBuf {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl PartialEq for BasePathBuf {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl PartialOrd for BasePathBuf {
    #[expect(clippy::non_canonical_partial_ord_impl)]
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (**self).partial_cmp(&**other)
    }
}

#[cfg(feature = "print_bytes")]
#[cfg_attr(normpath_docs_rs, doc(cfg(feature = "print_bytes")))]
mod print_bytes {
    use print_bytes::ByteStr;
    use print_bytes::ToBytes;
    #[cfg(windows)]
    use print_bytes::WideStr;

    use super::BasePath;
    use super::BasePathBuf;

    impl ToBytes for BasePath {
        #[inline]
        fn to_bytes(&self) -> ByteStr<'_> {
            self.0.to_bytes()
        }

        #[cfg(windows)]
        #[inline]
        fn to_wide(&self) -> Option<WideStr> {
            self.0.to_wide()
        }
    }

    impl ToBytes for BasePathBuf {
        #[inline]
        fn to_bytes(&self) -> ByteStr<'_> {
            (**self).to_bytes()
        }

        #[cfg(windows)]
        #[inline]
        fn to_wide(&self) -> Option<WideStr> {
            (**self).to_wide()
        }
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(normpath_docs_rs, doc(cfg(feature = "serde")))]
mod serde {
    use std::ffi::OsString;

    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

    use super::BasePath;
    use super::BasePathBuf;

    impl<'de> Deserialize<'de> for BasePathBuf {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            OsString::deserialize(deserializer).map(|x| Self(x.into()))
        }
    }

    impl Serialize for BasePath {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_newtype_struct("BasePath", &self.0)
        }
    }

    impl Serialize for BasePathBuf {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer
                .serialize_newtype_struct("BasePathBuf", self.as_os_str())
        }
    }
}

#[cfg(feature = "uniquote")]
#[cfg_attr(normpath_docs_rs, doc(cfg(feature = "uniquote")))]
mod uniquote {
    use uniquote::Formatter;
    use uniquote::Quote;
    use uniquote::Result;

    use super::BasePath;
    use super::BasePathBuf;

    impl Quote for BasePath {
        #[inline]
        fn escape(&self, f: &mut Formatter<'_>) -> Result {
            self.0.escape(f)
        }
    }

    impl Quote for BasePathBuf {
        #[inline]
        fn escape(&self, f: &mut Formatter<'_>) -> Result {
            (**self).escape(f)
        }
    }
}
