use anyhow::{Context as _, Result, anyhow};
use std::{
    borrow::{Borrow, Cow},
    fmt,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{
    PathStyle,
    abs_path::{AbsPath, AbsPathBuf},
    is_absolute,
};

/// A file system path that is guaranteed to be relative and normalized.
///
/// This type can be used to represent paths in a uniform way, regardless of
/// whether they refer to Windows or POSIX file systems, and regardless of
/// the host platform.
///
/// Internally, paths are stored in POSIX ('/'-delimited) format, but they can
/// be displayed in either POSIX or Windows format.
///
/// Relative paths are also guaranteed to be valid unicode.
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RelPath(str);

/// An owned representation of a file system path that is guaranteed to be
/// relative and normalized.
///
/// This type is to [`RelPath`] as [`std::path::PathBuf`] is to [`std::path::Path`]
#[derive(PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RelPathBuf(String);

impl RelPath {
    /// Creates an empty [`RelPath`].
    pub fn empty() -> &'static Self {
        Self::from_str("")
    }

    /// Creates an empty [`RelPath`].
    pub fn empty_arc() -> Arc<Self> {
        static EMPTY: std::sync::OnceLock<Arc<RelPath>> = std::sync::OnceLock::new();
        EMPTY.get_or_init(|| Arc::from(Self::empty())).clone()
    }

    /// Converts a path with a given style into a [`RelPath`].
    ///
    /// Returns an error if the path is absolute, or is not valid unicode.
    ///
    /// This method will normalize the path by removing `.` components,
    /// processing `..` components, and removing trailing separators. It does
    /// not allocate unless it's necessary to reformat the path.
    #[track_caller]
    pub fn new<'a>(path: &'a Path, path_style: PathStyle) -> Result<Cow<'a, Self>> {
        let mut path = path.to_str().context("non utf-8 path")?;

        let (prefixes, suffixes): (&[_], &[_]) = match path_style {
            PathStyle::Unix => (&["./"], &['/']),
            PathStyle::Windows => (&["./", ".\\"], &['/', '\\']),
        };

        while prefixes.iter().any(|prefix| path.starts_with(prefix)) {
            path = &path[prefixes[0].len()..];
        }
        while let Some(prefix) = path.strip_suffix(suffixes)
            && !prefix.is_empty()
        {
            path = prefix;
        }

        if is_absolute(&path, path_style) {
            return Err(anyhow!("absolute path not allowed: {path:?}"));
        }

        let mut string = Cow::Borrowed(path);
        if path_style == PathStyle::Windows && path.contains('\\') {
            string = Cow::Owned(string.as_ref().replace('\\', "/"))
        }

        let mut result = match string {
            Cow::Borrowed(string) => Cow::Borrowed(Self::from_str(string)),
            Cow::Owned(string) => Cow::Owned(RelPathBuf(string)),
        };

        if result
            .components()
            .any(|component| component == "" || component == "." || component == "..")
        {
            let mut normalized = RelPathBuf::new();
            for component in result.components() {
                match component {
                    "" => {}
                    "." => {}
                    ".." => {
                        if !normalized.pop() {
                            return Err(anyhow!("path is not relative: {result:?}"));
                        }
                    }
                    other => normalized.push(RelPath::from_str(other)),
                }
            }
            result = Cow::Owned(normalized)
        }

        Ok(result)
    }

    #[track_caller]
    pub fn new_test<'a>(path: &'a str) -> Cow<'a, Self> {
        Self::new(Path::new(path), PathStyle::Unix).unwrap()
    }

    /// Converts a path that is already normalized and uses '/' separators
    /// into a [`RelPath`] .
    ///
    /// Returns an error if the path is not already in the correct format.
    #[track_caller]
    pub fn from_unix_str<S: AsRef<Path> + ?Sized>(path: &S) -> anyhow::Result<&Self> {
        let path = path.as_ref();
        match Self::new(path, PathStyle::Unix)? {
            Cow::Borrowed(path) => Ok(path),
            Cow::Owned(_) => Err(anyhow!("invalid relative path {path:?}")),
        }
    }

    fn from_str(s: &str) -> &Self {
        // Safety: `RelPath` is a transparent wrapper around `str`.
        unsafe { &*(s as *const str as *const Self) }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn components(&self) -> RelPathComponents<'_> {
        RelPathComponents(&self.0)
    }

    pub fn ancestors(&self) -> RelPathAncestors<'_> {
        RelPathAncestors {
            full: &self.0,
            front: self.0.len(),
            back: 0,
            done: false,
        }
    }

    pub fn file_name(&self) -> Option<&str> {
        self.components().next_back()
    }

    pub fn file_stem(&self) -> Option<&str> {
        Some(self.as_std_path().file_stem()?.to_str().unwrap())
    }

    pub fn extension(&self) -> Option<&str> {
        Some(self.as_std_path().extension()?.to_str().unwrap())
    }

    pub fn parent(&self) -> Option<&Self> {
        let mut components = self.components();
        components.next_back()?;
        Some(components.rest())
    }

    pub fn starts_with(&self, other: &Self) -> bool {
        self.strip_prefix(other).is_ok()
    }

    /// Returns true if this path is a strict descendant of `ancestor`.
    ///
    /// Unlike `starts_with`, this returns false when `self == ancestor`
    /// and false when `ancestor` is empty (since every path trivially
    /// starts with the empty prefix).
    pub fn is_descendant_of(&self, ancestor: &Self) -> bool {
        if ancestor.is_empty() || self == ancestor {
            return false;
        }
        self.starts_with(ancestor)
    }

    pub fn ends_with(&self, other: &Self) -> bool {
        if other.is_empty() {
            return true;
        }
        if let Some(suffix) = self.0.strip_suffix(&other.0) {
            if suffix.ends_with('/') {
                return true;
            } else if suffix.is_empty() {
                return true;
            }
        }
        false
    }

    pub fn strip_prefix<'a>(&'a self, other: &Self) -> Result<&'a Self, StripPrefixError> {
        if other.is_empty() {
            return Ok(self);
        }
        if let Some(suffix) = self.0.strip_prefix(&other.0) {
            if let Some(suffix) = suffix.strip_prefix('/') {
                return Ok(Self::from_str(suffix));
            } else if suffix.is_empty() {
                return Ok(Self::empty());
            }
        }
        Err(StripPrefixError)
    }

    pub fn len(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0.matches('/').count() + 1
        }
    }

    pub fn last_n_components(&self, count: usize) -> Option<&Self> {
        let len = self.len();
        if len >= count {
            let mut components = self.components();
            for _ in 0..(len - count) {
                components.next()?;
            }
            Some(components.rest())
        } else {
            None
        }
    }

    pub fn join(&self, other: &Self) -> RelPathBuf {
        if self.0.is_empty() {
            other.to_rel_path_buf()
        } else if other.0.is_empty() {
            self.to_rel_path_buf()
        } else {
            RelPathBuf(format!("{}/{}", &self.0, &other.0))
        }
    }

    pub fn to_rel_path_buf(&self) -> RelPathBuf {
        RelPathBuf(self.0.to_string())
    }

    pub fn into_arc(&self) -> Arc<Self> {
        Arc::from(self)
    }

    /// Convert the path into a string with the given path style.
    ///
    /// Whenever a path is presented to the user, it should be converted to
    /// a string via this method.
    pub fn display(&self, style: PathStyle) -> Cow<'_, str> {
        match style {
            PathStyle::Unix => Cow::Borrowed(&self.0),
            PathStyle::Windows if self.0.contains('/') => Cow::Owned(self.0.replace('/', "\\")),
            PathStyle::Windows => Cow::Borrowed(&self.0),
        }
    }

    /// Get the internal unix-style representation of the path.
    ///
    /// This should not be shown to the user.
    pub fn as_unix_str(&self) -> &str {
        &self.0
    }

    /// Interprets the path as a [`std::path::Path`], suitable for file system calls.
    ///
    /// This is guaranteed to be a valid path regardless of the host platform, because
    /// the `/` is accepted as a path separator on windows.
    ///
    /// This should not be shown to the user.
    pub fn as_std_path(&self) -> &Path {
        Path::new(&self.0)
    }

    /// Resolves this relative path against an absolute base path.
    pub fn absolutize(&self, base: impl AsRef<AbsPath>) -> AbsPathBuf {
        base.as_ref().join(self.as_unix_str())
    }
}

#[derive(Debug)]
pub struct StripPrefixError;

impl ToOwned for RelPath {
    type Owned = RelPathBuf;

    fn to_owned(&self) -> Self::Owned {
        self.to_rel_path_buf()
    }
}

impl Borrow<RelPath> for RelPathBuf {
    fn borrow(&self) -> &RelPath {
        self.as_rel_path()
    }
}

impl PartialOrd for RelPath {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RelPath {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.components().cmp(other.components())
    }
}

impl fmt::Debug for RelPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Debug for RelPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl RelPathBuf {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn pop(&mut self) -> bool {
        if let Some(ix) = self.0.rfind('/') {
            self.0.truncate(ix);
            true
        } else if !self.is_empty() {
            self.0.clear();
            true
        } else {
            false
        }
    }

    pub fn push(&mut self, path: &RelPath) {
        if path.is_empty() {
            return;
        }
        if !self.is_empty() {
            self.0.push('/');
        }
        self.0.push_str(&path.0);
    }

    pub fn push_component(&mut self, component: &str) -> Result<()> {
        anyhow::ensure!(
            !component.is_empty()
                && !component.contains('/')
                && component != "."
                && component != "..",
            "invalid relative path component: {component:?}"
        );

        if !self.is_empty() {
            self.0.push('/');
        }
        self.0.push_str(component);
        Ok(())
    }

    pub fn as_rel_path(&self) -> &RelPath {
        RelPath::from_str(self.0.as_str())
    }

    pub fn set_extension(&mut self, extension: &str) -> bool {
        if let Some(filename) = self.file_name() {
            let mut filename = PathBuf::from(filename);
            filename.set_extension(extension);
            self.pop();
            self.push(RelPath::from_str(filename.to_str().unwrap()));
            true
        } else {
            false
        }
    }
}

impl PartialOrd for RelPathBuf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RelPathBuf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_rel_path().cmp(other.as_rel_path())
    }
}

impl From<RelPathBuf> for Arc<RelPath> {
    fn from(value: RelPathBuf) -> Self {
        Arc::from(value.as_rel_path())
    }
}

impl AsRef<RelPath> for RelPathBuf {
    fn as_ref(&self) -> &RelPath {
        self.as_rel_path()
    }
}

impl AsRef<std::path::Path> for RelPathBuf {
    fn as_ref(&self) -> &Path {
        self.as_std_path()
    }
}

impl AsRef<RelPath> for RelPath {
    fn as_ref(&self) -> &RelPath {
        self
    }
}

impl AsRef<std::path::Path> for RelPath {
    fn as_ref(&self) -> &Path {
        self.as_std_path()
    }
}

impl Deref for RelPathBuf {
    type Target = RelPath;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a> From<&'a RelPath> for Cow<'a, RelPath> {
    fn from(value: &'a RelPath) -> Self {
        Self::Borrowed(value)
    }
}

impl From<&RelPath> for Arc<RelPath> {
    fn from(rel_path: &RelPath) -> Self {
        let bytes: Arc<str> = Arc::from(&rel_path.0);
        // SAFETY: `AbsPath` is a `repr(transparent)` wrapper around `Path`.
        unsafe { Arc::from_raw(Arc::into_raw(bytes) as *const RelPath) }
    }
}

impl<'a> TryFrom<&'a str> for &'a RelPath {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self> {
        RelPath::from_unix_str(s)
    }
}

impl TryFrom<&str> for RelPathBuf {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        RelPath::new(Path::new(s), PathStyle::Unix).map(|cow| cow.into_owned())
    }
}

#[cfg(any(test, feature = "test-support"))]
#[track_caller]
pub fn rel_path(path: &str) -> &RelPath {
    RelPath::from_unix_str(path).unwrap()
}

#[cfg(any(test, feature = "test-support"))]
#[track_caller]
pub fn rel_path_buf(path: &str) -> RelPathBuf {
    rel_path(path).to_owned()
}

impl PartialEq<str> for RelPath {
    fn eq(&self, other: &str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<RelPath> for RelPathBuf {
    fn eq(&self, other: &RelPath) -> bool {
        self.as_rel_path() == other
    }
}

impl PartialEq<RelPathBuf> for RelPath {
    fn eq(&self, other: &RelPathBuf) -> bool {
        other.as_rel_path() == self
    }
}

impl fmt::Display for RelPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for RelPathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Default)]
pub struct RelPathComponents<'a>(&'a str);

pub struct RelPathAncestors<'a> {
    full: &'a str,
    front: usize,
    back: usize,
    done: bool,
}

const SEPARATOR: char = '/';

impl<'a> RelPathComponents<'a> {
    pub fn rest(&self) -> &'a RelPath {
        RelPath::from_str(self.0)
    }
}

impl<'a> Iterator for RelPathComponents<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sep_ix) = self.0.find(SEPARATOR) {
            let (head, tail) = self.0.split_at(sep_ix);
            self.0 = &tail[1..];
            Some(head)
        } else if self.0.is_empty() {
            None
        } else {
            let result = self.0;
            self.0 = "";
            Some(result)
        }
    }
}

impl<'a> Iterator for RelPathAncestors<'a> {
    type Item = &'a RelPath;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let result = &self.full[..self.front];
        if self.front == self.back {
            self.done = true;
        } else {
            self.front = result.rfind(SEPARATOR).unwrap_or_default();
        }
        Some(RelPath::from_str(result))
    }
}

impl<'a> DoubleEndedIterator for RelPathAncestors<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let result = &self.full[..self.back];
        if self.front == self.back {
            self.done = true;
        } else {
            let search_start = if self.back == 0 { 0 } else { self.back + 1 };
            self.back = match self.full[search_start..].find(SEPARATOR) {
                Some(sep_ix) => search_start + sep_ix,
                None => self.full.len(),
            };
        }
        Some(RelPath::from_str(result))
    }
}

impl<'a> DoubleEndedIterator for RelPathComponents<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(sep_ix) = self.0.rfind(SEPARATOR) {
            let (head, tail) = self.0.split_at(sep_ix);
            self.0 = head;
            Some(&tail[1..])
        } else if self.0.is_empty() {
            None
        } else {
            let result = self.0;
            self.0 = "";
            Some(result)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RelPathBuf {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path = String::deserialize(deserializer)?;
        let rel_path =
            RelPath::new(Path::new(&path), PathStyle::local()).map_err(serde::de::Error::custom)?;
        Ok(rel_path.into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rel_path_new() {
        assert!(RelPath::new(Path::new("/"), PathStyle::local()).is_err());
        assert!(RelPath::new(Path::new("//"), PathStyle::local()).is_err());
        assert!(RelPath::new(Path::new("/foo/"), PathStyle::local()).is_err());

        let path = RelPath::new("foo/".as_ref(), PathStyle::local()).unwrap();
        assert_eq!(path, rel_path("foo").into());
        assert!(matches!(path, Cow::Borrowed(_)));

        let path = RelPath::new("foo\\".as_ref(), PathStyle::Windows).unwrap();
        assert_eq!(path, rel_path("foo").into());
        assert!(matches!(path, Cow::Borrowed(_)));

        assert_eq!(
            RelPath::new("foo/bar/../baz/./quux/".as_ref(), PathStyle::local())
                .unwrap()
                .as_ref(),
            rel_path("foo/baz/quux")
        );

        let path = RelPath::new("./foo/bar".as_ref(), PathStyle::Unix).unwrap();
        assert_eq!(path.as_ref(), rel_path("foo/bar"));
        assert!(matches!(path, Cow::Borrowed(_)));

        let path = RelPath::new(".\\foo".as_ref(), PathStyle::Windows).unwrap();
        assert_eq!(path, rel_path("foo").into());
        assert!(matches!(path, Cow::Borrowed(_)));

        let path = RelPath::new("./.\\./foo/\\/".as_ref(), PathStyle::Windows).unwrap();
        assert_eq!(path, rel_path("foo").into());
        assert!(matches!(path, Cow::Borrowed(_)));

        let path = RelPath::new("foo/./bar".as_ref(), PathStyle::Unix).unwrap();
        assert_eq!(path.as_ref(), rel_path("foo/bar"));
        assert!(matches!(path, Cow::Owned(_)));

        let path = RelPath::new("./foo/bar".as_ref(), PathStyle::Windows).unwrap();
        assert_eq!(path.as_ref(), rel_path("foo/bar"));
        assert!(matches!(path, Cow::Borrowed(_)));

        let path = RelPath::new(".\\foo\\bar".as_ref(), PathStyle::Windows).unwrap();
        assert_eq!(path.as_ref(), rel_path("foo/bar"));
        assert!(matches!(path, Cow::Owned(_)));
    }

    #[test]
    fn test_rel_path_components() {
        let path = rel_path("foo/bar/baz");
        assert_eq!(
            path.components().collect::<Vec<_>>(),
            vec!["foo", "bar", "baz"]
        );
        assert_eq!(
            path.components().rev().collect::<Vec<_>>(),
            vec!["baz", "bar", "foo"]
        );

        let path = rel_path("");
        let mut components = path.components();
        assert_eq!(components.next(), None);
    }

    #[test]
    fn test_rel_path_ancestors() {
        let path = rel_path("foo/bar/baz");
        let mut ancestors = path.ancestors();
        assert_eq!(ancestors.next(), Some(rel_path("foo/bar/baz")));
        assert_eq!(ancestors.next(), Some(rel_path("foo/bar")));
        assert_eq!(ancestors.next(), Some(rel_path("foo")));
        assert_eq!(ancestors.next(), Some(rel_path("")));
        assert_eq!(ancestors.next(), None);

        let path = rel_path("foo");
        let mut ancestors = path.ancestors();
        assert_eq!(ancestors.next(), Some(rel_path("foo")));
        assert_eq!(ancestors.next(), Some(RelPath::empty()));
        assert_eq!(ancestors.next(), None);

        let path = RelPath::empty();
        let mut ancestors = path.ancestors();
        assert_eq!(ancestors.next(), Some(RelPath::empty()));
        assert_eq!(ancestors.next(), None);

        let path = rel_path("foo/bar/baz");
        let mut ancestors = path.ancestors();
        assert_eq!(ancestors.next_back(), Some(rel_path("")));
        assert_eq!(ancestors.next_back(), Some(rel_path("foo")));
        assert_eq!(ancestors.next_back(), Some(rel_path("foo/bar")));
        assert_eq!(ancestors.next_back(), Some(rel_path("foo/bar/baz")));
        assert_eq!(ancestors.next_back(), None);

        let path = rel_path("foo/bar/baz");
        let mut ancestors = path.ancestors();
        assert_eq!(ancestors.next(), Some(rel_path("foo/bar/baz")));
        assert_eq!(ancestors.next_back(), Some(rel_path("")));
        assert_eq!(ancestors.next(), Some(rel_path("foo/bar")));
        assert_eq!(ancestors.next_back(), Some(rel_path("foo")));
        assert_eq!(ancestors.next(), None);
        assert_eq!(ancestors.next_back(), None);

        let path = rel_path("foo");
        let mut ancestors = path.ancestors();
        assert_eq!(ancestors.next_back(), Some(RelPath::empty()));
        assert_eq!(ancestors.next_back(), Some(rel_path("foo")));
        assert_eq!(ancestors.next_back(), None);

        let path = RelPath::empty();
        let mut ancestors = path.ancestors();
        assert_eq!(ancestors.next_back(), Some(RelPath::empty()));
        assert_eq!(ancestors.next_back(), None);

        let path = rel_path("über/x");
        let mut ancestors = path.ancestors();
        assert_eq!(ancestors.next_back(), Some(RelPath::empty()));
        assert_eq!(ancestors.next_back(), Some(rel_path("über")));
        assert_eq!(ancestors.next_back(), Some(rel_path("über/x")));
        assert_eq!(ancestors.next_back(), None);
    }

    #[test]
    fn test_rel_path_parent() {
        assert_eq!(rel_path("foo/bar/baz").parent(), Some(rel_path("foo/bar")));
        assert_eq!(rel_path("foo").parent(), Some(RelPath::empty()));
        assert_eq!(rel_path("").parent(), None);
    }

    #[test]
    fn test_rel_path_partial_ord_is_compatible_with_std() {
        let test_cases = ["a/b/c", "relative/path/with/dot.", "relative/path/with.dot"];
        for (i, lhs) in test_cases.iter().enumerate() {
            for rhs in &test_cases[i + 1..] {
                assert_eq!(
                    Path::new(lhs).cmp(Path::new(rhs)),
                    RelPath::from_unix_str(lhs)
                        .unwrap()
                        .cmp(RelPath::from_unix_str(rhs).unwrap()),
                    "ordering mismatch for {:?} vs {:?}",
                    lhs,
                    rhs,
                );
            }
        }
    }

    #[test]
    fn test_strip_prefix() {
        let parent = rel_path("");
        let child = rel_path(".foo");

        assert!(child.starts_with(parent));
        assert_eq!(child.strip_prefix(parent).unwrap(), child);
    }

    #[test]
    fn test_ends_with() {
        assert!(rel_path("foo/bar").ends_with(rel_path("bar")));
        assert!(rel_path("foo/bar").ends_with(rel_path("foo/bar")));
        assert!(rel_path("foo/bar").ends_with(RelPath::empty()));
        assert!(RelPath::empty().ends_with(RelPath::empty()));
        assert!(!rel_path("foobar").ends_with(rel_path("bar")));
    }

    #[test]
    fn test_rel_path_constructors_absolute_path() {
        assert!(RelPath::new(Path::new("/a/b"), PathStyle::Windows).is_err());
        assert!(RelPath::new(Path::new("\\a\\b"), PathStyle::Windows).is_err());
        assert!(RelPath::new(Path::new("/a/b"), PathStyle::Unix).is_err());
        assert!(RelPath::new(Path::new("C:/a/b"), PathStyle::Windows).is_err());
        assert!(RelPath::new(Path::new("C:\\a\\b"), PathStyle::Windows).is_err());
        assert!(RelPath::new(Path::new("C:/a/b"), PathStyle::Unix).is_ok());
    }

    #[test]
    fn test_pop() {
        let mut path = rel_path_buf("a/b");
        path.pop();
        assert_eq!(path.as_rel_path().as_unix_str(), "a");
        path.pop();
        assert_eq!(path.as_rel_path().as_unix_str(), "");
        path.pop();
        assert_eq!(path.as_rel_path().as_unix_str(), "");
    }

    #[test]
    fn test_len() {
        assert_eq!(RelPath::empty().len(), 0);
        assert_eq!(rel_path("a").len(), 1);
        assert_eq!(rel_path("a/b").len(), 2);
        assert_eq!(rel_path("a/b/c").len(), 3);
    }

    #[test]
    fn test_set_extension() {
        let mut path = rel_path_buf("a/b/c.txt");
        assert!(path.set_extension("rs"));
        assert_eq!(path.as_rel_path().as_unix_str(), "a/b/c.rs");

        let mut single = rel_path_buf("file.txt");
        assert!(single.set_extension("md"));
        assert_eq!(single.as_rel_path().as_unix_str(), "file.md");

        let mut no_ext = rel_path_buf("a/b/c");
        assert!(no_ext.set_extension("rs"));
        assert_eq!(no_ext.as_rel_path().as_unix_str(), "a/b/c.rs");
    }
}
