use std::borrow::{Cow, ToOwned};
use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::rc::Rc;
use std::sync::Arc;

use crate::path::{Ancestors, Components, Display, Iter, PathBuf, StripPrefixError};
#[cfg(not(target_os = "unknown"))]
use crate::{fs, io};

/// A slice of a path.
///
/// This struct is an async version of [`std::path::Path`].
///
/// This type supports a number of operations for inspecting a path, including
/// breaking the path into its components (separated by `/` on Unix and by either
/// `/` or `\` on Windows), extracting the file name, determining whether the path
/// is absolute, and so on.
///
/// This is an *unsized* type, meaning that it must always be used behind a
/// pointer like `&` or `Box`. For an owned version of this type,
/// see [`PathBuf`].
///
/// [`PathBuf`]: struct.PathBuf.html
/// [`std::path::Path`]: https://doc.rust-lang.org/std/path/struct.Path.html
///
/// More details about the overall approach can be found in
/// the [module documentation](index.html).
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use std::ffi::OsStr;
///
/// // Note: this example does work on Windows
/// let path = Path::new("./foo/bar.txt");
///
/// let parent = path.parent();
/// assert_eq!(parent, Some(Path::new("./foo")));
///
/// let file_stem = path.file_stem();
/// assert_eq!(file_stem, Some(OsStr::new("bar")));
///
/// let extension = path.extension();
/// assert_eq!(extension, Some(OsStr::new("txt")));
/// ```
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Path {
    inner: std::path::Path,
}

impl Path {
    /// Directly wraps a string slice as a `Path` slice.
    ///
    /// This is a cost-free conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// Path::new("foo.txt");
    /// ```
    ///
    /// You can create `Path`s from `String`s, or even other `Path`s:
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let string = String::from("foo.txt");
    /// let from_string = Path::new(&string);
    /// let from_path = Path::new(&from_string);
    /// assert_eq!(from_string, from_path);
    /// ```
    pub fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &Path {
        unsafe { &*(std::path::Path::new(s) as *const std::path::Path as *const Path) }
    }

    /// Returns the underlying [`OsStr`] slice.
    ///
    /// [`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    ///
    /// use async_std::path::Path;
    ///
    /// let os_str = Path::new("foo.txt").as_os_str();
    /// assert_eq!(os_str, OsStr::new("foo.txt"));
    /// ```
    pub fn as_os_str(&self) -> &OsStr {
        self.inner.as_os_str()
    }

    /// Returns a [`&str`] slice if the `Path` is valid unicode.
    ///
    /// This conversion may entail doing a check for UTF-8 validity.
    /// Note that validation is performed because non-UTF-8 strings are
    /// perfectly valid for some OS.
    ///
    /// [`&str`]: https://doc.rust-lang.org/std/primitive.str.html
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("foo.txt");
    /// assert_eq!(path.to_str(), Some("foo.txt"));
    /// ```
    pub fn to_str(&self) -> Option<&str> {
        self.inner.to_str()
    }

    /// Converts a `Path` to a [`Cow<str>`].
    ///
    /// Any non-Unicode sequences are replaced with
    /// [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD].
    ///
    /// [`Cow<str>`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
    /// [U+FFFD]: https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html
    ///
    /// # Examples
    ///
    /// Calling `to_string_lossy` on a `Path` with valid unicode:
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("foo.txt");
    /// assert_eq!(path.to_string_lossy(), "foo.txt");
    /// ```
    ///
    /// Had `path` contained invalid unicode, the `to_string_lossy` call might
    /// have returned `"fo�.txt"`.
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.inner.to_string_lossy()
    }

    /// Converts a `Path` to an owned [`PathBuf`].
    ///
    /// [`PathBuf`]: struct.PathBuf.html
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::{Path, PathBuf};
    ///
    /// let path_buf = Path::new("foo.txt").to_path_buf();
    /// assert_eq!(path_buf, PathBuf::from("foo.txt"));
    /// ```
    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(self.inner.to_path_buf())
    }

    /// Returns `true` if the `Path` is absolute, i.e. if it is independent of
    /// the current directory.
    ///
    /// * On Unix, a path is absolute if it starts with the root, so
    ///   `is_absolute` and [`has_root`] are equivalent.
    ///
    /// * On Windows, a path is absolute if it has a prefix and starts with the
    ///   root: `c:\windows` is absolute, while `c:temp` and `\temp` are not.
    ///
    /// [`has_root`]: #method.has_root
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// assert!(!Path::new("foo.txt").is_absolute());
    /// ```
    pub fn is_absolute(&self) -> bool {
        self.inner.is_absolute()
    }

    /// Returns `true` if the `Path` is relative, i.e. not absolute.
    ///
    /// See [`is_absolute`]'s documentation for more details.
    ///
    /// [`is_absolute`]: #method.is_absolute
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// assert!(Path::new("foo.txt").is_relative());
    /// ```
    pub fn is_relative(&self) -> bool {
        self.inner.is_relative()
    }

    /// Returns `true` if the `Path` has a root.
    ///
    /// * On Unix, a path has a root if it begins with `/`.
    ///
    /// * On Windows, a path has a root if it:
    ///     * has no prefix and begins with a separator, e.g. `\windows`
    ///     * has a prefix followed by a separator, e.g. `c:\windows` but not `c:windows`
    ///     * has any non-disk prefix, e.g. `\\server\share`
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// assert!(Path::new("/etc/passwd").has_root());
    /// ```
    pub fn has_root(&self) -> bool {
        self.inner.has_root()
    }

    /// Returns the `Path` without its final component, if there is one.
    ///
    /// Returns [`None`] if the path terminates in a root or prefix.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("/foo/bar");
    /// let parent = path.parent().unwrap();
    /// assert_eq!(parent, Path::new("/foo"));
    ///
    /// let grand_parent = parent.parent().unwrap();
    /// assert_eq!(grand_parent, Path::new("/"));
    /// assert_eq!(grand_parent.parent(), None);
    /// ```
    pub fn parent(&self) -> Option<&Path> {
        self.inner.parent().map(|p| p.into())
    }

    /// Produces an iterator over `Path` and its ancestors.
    ///
    /// The iterator will yield the `Path` that is returned if the [`parent`] method is used zero
    /// or more times. That means, the iterator will yield `&self`, `&self.parent().unwrap()`,
    /// `&self.parent().unwrap().parent().unwrap()` and so on. If the [`parent`] method returns
    /// [`None`], the iterator will do likewise. The iterator will always yield at least one value,
    /// namely `&self`.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html
    /// [`parent`]: struct.Path.html#method.parent
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let mut ancestors = Path::new("/foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(Path::new("/foo/bar").into()));
    /// assert_eq!(ancestors.next(), Some(Path::new("/foo").into()));
    /// assert_eq!(ancestors.next(), Some(Path::new("/").into()));
    /// assert_eq!(ancestors.next(), None);
    /// ```
    pub fn ancestors(&self) -> Ancestors<'_> {
        Ancestors { next: Some(&self) }
    }

    /// Returns the final component of the `Path`, if there is one.
    ///
    /// If the path is a normal file, this is the file name. If it's the path of a directory, this
    /// is the directory name.
    ///
    /// Returns [`None`] if the path terminates in `..`.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    ///
    /// use async_std::path::Path;
    ///
    /// assert_eq!(Some(OsStr::new("bin")), Path::new("/usr/bin/").file_name());
    /// assert_eq!(Some(OsStr::new("foo.txt")), Path::new("tmp/foo.txt").file_name());
    /// assert_eq!(Some(OsStr::new("foo.txt")), Path::new("foo.txt/.").file_name());
    /// assert_eq!(Some(OsStr::new("foo.txt")), Path::new("foo.txt/.//").file_name());
    /// assert_eq!(None, Path::new("foo.txt/..").file_name());
    /// assert_eq!(None, Path::new("/").file_name());
    /// ```
    pub fn file_name(&self) -> Option<&OsStr> {
        self.inner.file_name()
    }

    /// Returns a path that becomes `self` when joined onto `base`.
    ///
    /// # Errors
    ///
    /// If `base` is not a prefix of `self` (i.e., [`starts_with`]
    /// returns `false`), returns [`Err`].
    ///
    /// [`starts_with`]: #method.starts_with
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::{Path, PathBuf};
    ///
    /// let path = Path::new("/test/haha/foo.txt");
    ///
    /// assert_eq!(path.strip_prefix("/"), Ok(Path::new("test/haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test"), Ok(Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/"), Ok(Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Path::new("")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Path::new("")));
    /// assert_eq!(path.strip_prefix("test").is_ok(), false);
    /// assert_eq!(path.strip_prefix("/haha").is_ok(), false);
    ///
    /// let prefix = PathBuf::from("/test/");
    /// assert_eq!(path.strip_prefix(prefix), Ok(Path::new("haha/foo.txt")));
    /// ```
    pub fn strip_prefix<P>(&self, base: P) -> Result<&Path, StripPrefixError>
    where
        P: AsRef<Path>,
    {
        Ok(self.inner.strip_prefix(base.as_ref())?.into())
    }

    /// Determines whether `base` is a prefix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("/etc/passwd");
    ///
    /// assert!(path.starts_with("/etc"));
    /// assert!(path.starts_with("/etc/"));
    /// assert!(path.starts_with("/etc/passwd"));
    /// assert!(path.starts_with("/etc/passwd/"));
    ///
    /// assert!(!path.starts_with("/e"));
    /// ```
    pub fn starts_with<P: AsRef<Path>>(&self, base: P) -> bool {
        self.inner.starts_with(base.as_ref())
    }

    /// Determines whether `child` is a suffix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("/etc/passwd");
    ///
    /// assert!(path.ends_with("passwd"));
    /// ```
    pub fn ends_with<P: AsRef<Path>>(&self, child: P) -> bool {
        self.inner.ends_with(child.as_ref())
    }

    /// Extracts the stem (non-extension) portion of [`file_name`].
    ///
    /// [`file_name`]: struct.Path.html#method.file_name
    ///
    /// The stem is:
    ///
    /// * [`None`], if there is no file name
    /// * The entire file name if there is no embedded `.`
    /// * The entire file name if the file name begins with `.` and has no other `.`s within
    /// * Otherwise, the portion of the file name before the final `.`
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("foo.rs");
    ///
    /// assert_eq!("foo", path.file_stem().unwrap());
    /// ```
    pub fn file_stem(&self) -> Option<&OsStr> {
        self.inner.file_stem()
    }

    /// Extracts the extension of [`file_name`], if possible.
    ///
    /// The extension is:
    ///
    /// * [`None`], if there is no file name
    /// * [`None`], if there is no embedded `.`
    /// * [`None`], if the file name begins with `.` and has no other `.`s within
    /// * Otherwise, the portion of the file name after the final `.`
    ///
    /// [`file_name`]: struct.Path.html#method.file_name
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("foo.rs");
    ///
    /// assert_eq!("rs", path.extension().unwrap());
    /// ```
    pub fn extension(&self) -> Option<&OsStr> {
        self.inner.extension()
    }

    /// Creates an owned [`PathBuf`] with `path` adjoined to `self`.
    ///
    /// See [`PathBuf::push`] for more details on what it means to adjoin a path.
    ///
    /// [`PathBuf`]: struct.PathBuf.html
    /// [`PathBuf::push`]: struct.PathBuf.html#method.push
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::{Path, PathBuf};
    ///
    /// assert_eq!(Path::new("/etc").join("passwd"), PathBuf::from("/etc/passwd"));
    /// ```
    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.inner.join(path.as_ref()).into()
    }

    /// Creates an owned [`PathBuf`] like `self` but with the given file name.
    ///
    /// See [`PathBuf::set_file_name`] for more details.
    ///
    /// [`PathBuf`]: struct.PathBuf.html
    /// [`PathBuf::set_file_name`]: struct.PathBuf.html#method.set_file_name
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::{Path, PathBuf};
    ///
    /// let path = Path::new("/tmp/foo.txt");
    /// assert_eq!(path.with_file_name("bar.txt"), PathBuf::from("/tmp/bar.txt"));
    ///
    /// let path = Path::new("/tmp");
    /// assert_eq!(path.with_file_name("var"), PathBuf::from("/var"));
    /// ```
    pub fn with_file_name<S: AsRef<OsStr>>(&self, file_name: S) -> PathBuf {
        self.inner.with_file_name(file_name).into()
    }

    /// Creates an owned [`PathBuf`] like `self` but with the given extension.
    ///
    /// See [`PathBuf::set_extension`] for more details.
    ///
    /// [`PathBuf`]: struct.PathBuf.html
    /// [`PathBuf::set_extension`]: struct.PathBuf.html#method.set_extension
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::{Path, PathBuf};
    ///
    /// let path = Path::new("foo.rs");
    /// assert_eq!(path.with_extension("txt"), PathBuf::from("foo.txt"));
    /// ```
    pub fn with_extension<S: AsRef<OsStr>>(&self, extension: S) -> PathBuf {
        self.inner.with_extension(extension).into()
    }

    /// Produces an iterator over the [`Component`]s of the path.
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
    /// [`Component`]: enum.Component.html
    /// [`CurDir`]: enum.Component.html#variant.CurDir
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    ///
    /// use async_std::path::{Path, Component};
    ///
    /// let mut components = Path::new("/tmp/foo.txt").components();
    ///
    /// assert_eq!(components.next(), Some(Component::RootDir));
    /// assert_eq!(components.next(), Some(Component::Normal(OsStr::new("tmp"))));
    /// assert_eq!(components.next(), Some(Component::Normal(OsStr::new("foo.txt"))));
    /// assert_eq!(components.next(), None);
    /// ```
    pub fn components(&self) -> Components<'_> {
        Components {
            inner: self.inner.components(),
        }
    }

    /// Produces an iterator over the path's components viewed as [`OsStr`]
    /// slices.
    ///
    /// For more information about the particulars of how the path is separated
    /// into components, see [`components`].
    ///
    /// [`components`]: #method.components
    /// [`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    ///
    /// use async_std::path::{self, Path};
    ///
    /// let mut it = Path::new("/tmp/foo.txt").iter();
    /// assert_eq!(it.next(), Some(OsStr::new(&path::MAIN_SEPARATOR.to_string())));
    /// assert_eq!(it.next(), Some(OsStr::new("tmp")));
    /// assert_eq!(it.next(), Some(OsStr::new("foo.txt")));
    /// assert_eq!(it.next(), None)
    /// ```
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.components(),
        }
    }

    /// Returns an object that implements [`Display`] for safely printing paths
    /// that may contain non-Unicode data.
    ///
    /// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("/tmp/foo.rs");
    ///
    /// println!("{}", path.display());
    /// ```
    pub fn display(&self) -> Display<'_> {
        self.inner.display()
    }

    /// Reads the metadata of a file or directory.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file.
    ///
    /// This is an alias to [`fs::metadata`].
    ///
    /// [`fs::metadata`]: ../fs/fn.metadata.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("/Minas/tirith");
    /// let metadata = path.metadata().await?;
    /// println!("{:?}", metadata.file_type());
    /// #
    /// # Ok(()) }) }
    /// ```
    #[cfg(not(target_os = "unknown"))]
    pub async fn metadata(&self) -> io::Result<fs::Metadata> {
        fs::metadata(self).await
    }

    /// Reads the metadata of a file or directory without following symbolic links.
    ///
    /// This is an alias to [`fs::symlink_metadata`].
    ///
    /// [`fs::symlink_metadata`]: ../fs/fn.symlink_metadata.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("/Minas/tirith");
    /// let metadata = path.symlink_metadata().await?;
    /// println!("{:?}", metadata.file_type());
    /// #
    /// # Ok(()) }) }
    /// ```
    #[cfg(not(target_os = "unknown"))]
    pub async fn symlink_metadata(&self) -> io::Result<fs::Metadata> {
        fs::symlink_metadata(self).await
    }

    /// Returns the canonical form of a path.
    ///
    /// The returned path is in absolute form with all intermediate components normalized and
    /// symbolic links resolved.
    ///
    /// This is an alias to [`fs::canonicalize`].
    ///
    /// [`fs::canonicalize`]: ../fs/fn.canonicalize.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::path::{Path, PathBuf};
    ///
    /// let path = Path::new("/foo/test/../test/bar.rs");
    /// assert_eq!(path.canonicalize().await?, PathBuf::from("/foo/test/bar.rs"));
    /// #
    /// # Ok(()) }) }
    /// ```
    #[cfg(not(target_os = "unknown"))]
    pub async fn canonicalize(&self) -> io::Result<PathBuf> {
        fs::canonicalize(self).await
    }

    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This is an alias to [`fs::read_link`].
    ///
    /// [`fs::read_link`]: ../fs/fn.read_link.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::path::Path;
    ///
    /// let path = Path::new("/laputa/sky_castle.rs");
    /// let path_link = path.read_link().await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    #[cfg(not(target_os = "unknown"))]
    pub async fn read_link(&self) -> io::Result<PathBuf> {
        fs::read_link(self).await
    }

    /// Returns a stream over the entries within a directory.
    ///
    /// The stream will yield instances of [`io::Result`]`<`[`DirEntry`]`>`. New
    /// errors may be encountered after an iterator is initially constructed.
    ///
    /// This is an alias to [`fs::read_dir`].
    ///
    /// [`io::Result`]: ../io/type.Result.html
    /// [`DirEntry`]: ../fs/struct.DirEntry.html
    /// [`fs::read_dir`]: ../fs/fn.read_dir.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::fs;
    /// use async_std::path::Path;
    /// use async_std::prelude::*;
    ///
    /// let path = Path::new("/laputa");
    /// let mut dir = fs::read_dir(&path).await?;
    ///
    /// while let Some(res) = dir.next().await {
    ///     let entry = res?;
    ///     println!("{}", entry.file_name().to_string_lossy());
    /// }
    /// #
    /// # Ok(()) }) }
    /// ```
    #[cfg(not(target_os = "unknown"))]
    pub async fn read_dir(&self) -> io::Result<fs::ReadDir> {
        fs::read_dir(self).await
    }

    /// Returns `true` if the path points at an existing entity.
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
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::path::Path;
    /// assert_eq!(Path::new("does_not_exist.txt").exists().await, false);
    /// #
    /// # Ok(()) }) }
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [fs::metadata].
    ///
    /// [fs::metadata]: ../fs/fn.metadata.html
    #[cfg(not(target_os = "unknown"))]
    pub async fn exists(&self) -> bool {
        fs::metadata(self).await.is_ok()
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
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::path::Path;
    /// assert_eq!(Path::new("./is_a_directory/").is_file().await, false);
    /// assert_eq!(Path::new("a_file.txt").is_file().await, true);
    /// #
    /// # Ok(()) }) }
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [fs::metadata] and handle its Result. Then call
    /// [fs::Metadata::is_file] if it was Ok.
    ///
    /// [fs::metadata]: ../fs/fn.metadata.html
    /// [fs::Metadata::is_file]: ../fs/struct.Metadata.html#method.is_file
    #[cfg(not(target_os = "unknown"))]
    pub async fn is_file(&self) -> bool {
        fs::metadata(self)
            .await
            .map(|m| m.is_file())
            .unwrap_or(false)
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
    /// # fn main() -> std::io::Result<()> { async_std::task::block_on(async {
    /// #
    /// use async_std::path::Path;
    ///
    /// assert_eq!(Path::new("./is_a_directory/").is_dir().await, true);
    /// assert_eq!(Path::new("a_file.txt").is_dir().await, false);
    /// #
    /// # Ok(()) }) }
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [fs::metadata] and handle its Result. Then call
    /// [fs::Metadata::is_dir] if it was Ok.
    ///
    /// [fs::metadata]: ../fs/fn.metadata.html
    /// [fs::Metadata::is_dir]: ../fs/struct.Metadata.html#method.is_dir
    #[cfg(not(target_os = "unknown"))]
    pub async fn is_dir(&self) -> bool {
        fs::metadata(self)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false)
    }

    /// Converts a [`Box<Path>`][`Box`] into a [`PathBuf`] without copying or
    /// allocating.
    ///
    /// [`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
    /// [`PathBuf`]: struct.PathBuf.html
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::path::Path;
    ///
    /// let path: Box<Path> = Path::new("foo.txt").into();
    /// let path_buf = path.into_path_buf();
    /// ```
    pub fn into_path_buf(self: Box<Path>) -> PathBuf {
        let rw = Box::into_raw(self) as *mut std::path::Path;
        let inner = unsafe { Box::from_raw(rw) };
        inner.into_path_buf().into()
    }
}

impl From<&Path> for Box<Path> {
    fn from(path: &Path) -> Box<Path> {
        let boxed: Box<std::path::Path> = path.inner.into();
        let rw = Box::into_raw(boxed) as *mut Path;
        unsafe { Box::from_raw(rw) }
    }
}

impl From<&Path> for Arc<Path> {
    /// Converts a Path into a Rc by copying the Path data into a new Rc buffer.
    #[inline]
    fn from(s: &Path) -> Arc<Path> {
        let arc: Arc<OsStr> = Arc::from(s.as_os_str());
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Path) }
    }
}

impl From<&Path> for Rc<Path> {
    #[inline]
    fn from(s: &Path) -> Rc<Path> {
        let rc: Rc<OsStr> = Rc::from(s.as_os_str());
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const Path) }
    }
}

impl ToOwned for Path {
    type Owned = PathBuf;

    fn to_owned(&self) -> PathBuf {
        self.to_path_buf()
    }
}

impl AsRef<OsStr> for Path {
    fn as_ref(&self) -> &OsStr {
        self.inner.as_ref()
    }
}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl AsRef<Path> for OsStr {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl<'a> From<&'a Path> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a Path) -> Cow<'a, Path> {
        Cow::Borrowed(s)
    }
}

impl AsRef<Path> for Cow<'_, OsStr> {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for OsString {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for str {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for String {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl<'a> IntoIterator for &'a PathBuf {
    type Item = &'a OsStr;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = &'a OsStr;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Path as PartialEq>::eq(self, other)
            }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Path as PartialEq>::eq(self, other)
            }
        }

        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                <Path as PartialOrd>::partial_cmp(self, other)
            }
        }

        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
                <Path as PartialOrd>::partial_cmp(self, other)
            }
        }
    };
}

impl_cmp!(PathBuf, Path);
impl_cmp!(PathBuf, &'a Path);
impl_cmp!(Cow<'a, Path>, Path);
impl_cmp!(Cow<'a, Path>, &'b Path);
impl_cmp!(Cow<'a, Path>, PathBuf);

macro_rules! impl_cmp_os_str {
    ($lhs:ty, $rhs: ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Path as PartialEq>::eq(self, other.as_ref())
            }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Path as PartialEq>::eq(self.as_ref(), other)
            }
        }

        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                <Path as PartialOrd>::partial_cmp(self, other.as_ref())
            }
        }

        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
                <Path as PartialOrd>::partial_cmp(self.as_ref(), other)
            }
        }
    };
}

impl_cmp_os_str!(PathBuf, OsStr);
impl_cmp_os_str!(PathBuf, &'a OsStr);
impl_cmp_os_str!(PathBuf, Cow<'a, OsStr>);
impl_cmp_os_str!(PathBuf, OsString);
impl_cmp_os_str!(Path, OsStr);
impl_cmp_os_str!(Path, &'a OsStr);
impl_cmp_os_str!(Path, Cow<'a, OsStr>);
impl_cmp_os_str!(Path, OsString);
impl_cmp_os_str!(&'a Path, OsStr);
impl_cmp_os_str!(&'a Path, Cow<'b, OsStr>);
impl_cmp_os_str!(&'a Path, OsString);

impl<'a> From<&'a std::path::Path> for &'a Path {
    fn from(path: &'a std::path::Path) -> &'a Path {
        &Path::new(path.as_os_str())
    }
}

impl<'a> Into<&'a std::path::Path> for &'a Path {
    fn into(self) -> &'a std::path::Path {
        std::path::Path::new(&self.inner)
    }
}

impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        self.into()
    }
}

impl AsRef<Path> for std::path::Path {
    fn as_ref(&self) -> &Path {
        self.into()
    }
}

impl AsRef<Path> for std::path::PathBuf {
    fn as_ref(&self) -> &Path {
        let p: &std::path::Path = self.as_ref();
        p.into()
    }
}
