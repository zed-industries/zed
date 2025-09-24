use crate::paths::{PathStyle, is_absolute};
use anyhow::{Context as _, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    ffi::OsStr,
    fmt,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Serialize)]
pub struct RelPath(str);

#[derive(Clone, Serialize, Deserialize)]
pub struct RelPathBuf(String);

impl RelPath {
    pub fn empty() -> &'static Self {
        unsafe { Self::new_unchecked("") }
    }

    #[track_caller]
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> anyhow::Result<&Self> {
        let this = unsafe { Self::new_unchecked(s) };
        if this.0.starts_with("/")
            || this.0.ends_with("/")
            || this
                .components()
                .any(|component| component == ".." || component == "." || component.is_empty())
        {
            bail!("invalid relative path: {:?}", &this.0);
        }
        Ok(this)
    }

    #[track_caller]
    pub fn from_std_path(path: &Path, path_style: PathStyle) -> Result<Arc<Self>> {
        let path = path.to_str().context("non utf-8 path")?;
        let mut string = Cow::Borrowed(path);

        if is_absolute(&string, path_style) {
            return Err(anyhow!("absolute path not allowed: {path:?}"));
        }

        if path_style == PathStyle::Windows {
            string = Cow::Owned(string.as_ref().replace('\\', "/"))
        }

        let mut this = RelPathBuf::new();
        for component in unsafe { Self::new_unchecked(string.as_ref()) }.components() {
            match component {
                "" => {}
                "." => {}
                ".." => {
                    if !this.pop() {
                        return Err(anyhow!("path is not relative: {string:?}"));
                    }
                }
                other => this.push(RelPath::new(other)?),
            }
        }

        Ok(this.into())
    }

    pub unsafe fn new_unchecked<S: AsRef<str> + ?Sized>(s: &S) -> &Self {
        unsafe { &*(s.as_ref() as *const str as *const Self) }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn components(&self) -> RelPathComponents<'_> {
        RelPathComponents(&self.0)
    }

    pub fn ancestors(&self) -> RelPathAncestors<'_> {
        RelPathAncestors(Some(&self.0))
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

    pub fn ends_with(&self, other: &Self) -> bool {
        if let Some(suffix) = self.0.strip_suffix(&other.0) {
            if suffix.ends_with('/') {
                return true;
            } else if suffix.is_empty() {
                return true;
            }
        }
        false
    }

    pub fn strip_prefix(&self, other: &Self) -> Result<&Self> {
        if other.is_empty() {
            return Ok(self);
        }
        if let Some(suffix) = self.0.strip_prefix(&other.0) {
            if let Some(suffix) = suffix.strip_prefix('/') {
                return Ok(unsafe { Self::new_unchecked(suffix) });
            } else if suffix.is_empty() {
                return Ok(Self::empty());
            }
        }
        Err(anyhow!("failed to strip prefix: {other:?} from {self:?}"))
    }

    pub fn len(&self) -> usize {
        self.0.matches('/').count() + 1
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

    pub fn push(&self, component: &str) -> Result<Arc<Self>> {
        if component.is_empty() {
            bail!("pushed component is empty");
        } else if component.contains('/') {
            bail!("pushed component contains a separator: {component:?}");
        }
        let path = format!(
            "{}{}{}",
            &self.0,
            if self.is_empty() { "" } else { "/" },
            component
        );
        Ok(Arc::from(unsafe { Self::new_unchecked(&path) }))
    }

    pub fn join(&self, other: &Self) -> Arc<Self> {
        let result = if self.0.is_empty() {
            Cow::Borrowed(&other.0)
        } else if other.0.is_empty() {
            Cow::Borrowed(&self.0)
        } else {
            Cow::Owned(format!("{}/{}", &self.0, &other.0))
        };
        Arc::from(unsafe { Self::new_unchecked(result.as_ref()) })
    }

    pub fn to_proto(&self) -> String {
        self.0.to_owned()
    }

    pub fn to_rel_path_buf(&self) -> RelPathBuf {
        RelPathBuf(self.0.to_string())
    }

    pub fn from_proto(path: &str) -> Result<Arc<Self>> {
        Ok(Arc::from(Self::new(path)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn display(&self, style: PathStyle) -> Cow<'_, str> {
        match style {
            PathStyle::Posix => Cow::Borrowed(&self.0),
            PathStyle::Windows => Cow::Owned(self.0.replace('/', "\\")),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0.as_bytes()
    }

    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_ref()
    }

    pub fn as_std_path(&self) -> &Path {
        Path::new(&self.0)
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
        if !self.is_empty() {
            self.0.push('/');
        }
        self.0.push_str(&path.0);
    }

    pub fn as_rel_path(&self) -> &RelPath {
        unsafe { RelPath::new_unchecked(self.0.as_str()) }
    }

    pub fn set_extension(&mut self, extension: &str) -> bool {
        if let Some(filename) = self.file_name() {
            let mut filename = PathBuf::from(filename);
            filename.set_extension(extension);
            self.pop();
            self.0.push_str(filename.to_str().unwrap());
            true
        } else {
            false
        }
    }
}

impl Into<Arc<RelPath>> for RelPathBuf {
    fn into(self) -> Arc<RelPath> {
        Arc::from(self.as_rel_path())
    }
}

impl AsRef<RelPath> for RelPathBuf {
    fn as_ref(&self) -> &RelPath {
        self.as_rel_path()
    }
}

impl Deref for RelPathBuf {
    type Target = RelPath;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl AsRef<Path> for RelPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl From<&RelPath> for Arc<RelPath> {
    fn from(rel_path: &RelPath) -> Self {
        let bytes: Arc<str> = Arc::from(&rel_path.0);
        unsafe { Arc::from_raw(Arc::into_raw(bytes) as *const RelPath) }
    }
}

#[cfg(any(test, feature = "test-support"))]
#[track_caller]
pub fn rel_path(path: &str) -> &RelPath {
    RelPath::new(path).unwrap()
}

impl PartialEq<str> for RelPath {
    fn eq(&self, other: &str) -> bool {
        self.0 == *other
    }
}

pub struct RelPathComponents<'a>(&'a str);

pub struct RelPathAncestors<'a>(Option<&'a str>);

const SEPARATOR: char = '/';

impl<'a> RelPathComponents<'a> {
    pub fn rest(&self) -> &'a RelPath {
        unsafe { RelPath::new_unchecked(self.0) }
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
        let result = self.0?;
        if let Some(sep_ix) = result.rfind(SEPARATOR) {
            self.0 = Some(&result[..sep_ix]);
        } else if !result.is_empty() {
            self.0 = Some("");
        } else {
            self.0 = None;
        }
        Some(unsafe { RelPath::new_unchecked(result) })
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

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::path::PathBuf;

    #[test]
    fn test_path_construction() {
        assert!(RelPath::new("/").is_err());
        assert!(RelPath::new("/foo").is_err());
        assert!(RelPath::new("foo/").is_err());
        assert!(RelPath::new("foo//bar").is_err());
        assert!(RelPath::new("foo/../bar").is_err());
        assert!(RelPath::new("./foo/bar").is_err());
        assert!(RelPath::new("..").is_err());

        assert!(RelPath::from_std_path(Path::new("/"), PathStyle::local()).is_err());
        assert!(RelPath::from_std_path(Path::new("//"), PathStyle::local()).is_err());
        assert!(RelPath::from_std_path(Path::new("/foo/"), PathStyle::local()).is_err());
        assert_eq!(
            RelPath::from_std_path(&PathBuf::from_iter(["foo", ""]), PathStyle::local()).unwrap(),
            Arc::from(rel_path("foo"))
        );
    }

    #[test]
    fn test_rel_path_from_std_path() {
        assert_eq!(
            RelPath::from_std_path(Path::new("foo/bar/../baz/./quux/"), PathStyle::local())
                .unwrap()
                .as_ref(),
            rel_path("foo/baz/quux")
        );
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
    }

    #[test]
    fn test_rel_path_parent() {
        assert_eq!(
            rel_path("foo/bar/baz").parent(),
            Some(RelPath::new("foo/bar").unwrap())
        );
        assert_eq!(rel_path("foo").parent(), Some(RelPath::empty()));
        assert_eq!(rel_path("").parent(), None);
    }

    #[test]
    fn test_rel_path_partial_ord_is_compatible_with_std() {
        let test_cases = ["a/b/c", "relative/path/with/dot.", "relative/path/with.dot"];
        for [lhs, rhs] in test_cases.iter().array_combinations::<2>() {
            assert_eq!(
                Path::new(lhs).cmp(Path::new(rhs)),
                RelPath::new(lhs).unwrap().cmp(RelPath::new(rhs).unwrap())
            );
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
    fn test_rel_path_constructors_absolute_path() {
        assert!(RelPath::from_std_path(Path::new("/a/b"), PathStyle::Windows).is_err());
        assert!(RelPath::from_std_path(Path::new("\\a\\b"), PathStyle::Windows).is_err());
        assert!(RelPath::from_std_path(Path::new("/a/b"), PathStyle::Posix).is_err());
        assert!(RelPath::from_std_path(Path::new("C:/a/b"), PathStyle::Windows).is_err());
        assert!(RelPath::from_std_path(Path::new("C:\\a\\b"), PathStyle::Windows).is_err());
        assert!(RelPath::from_std_path(Path::new("C:/a/b"), PathStyle::Posix).is_ok());
    }

    #[test]
    fn test_push() {
        assert_eq!(rel_path("a/b").push("c").unwrap().as_str(), "a/b/c");
        assert_eq!(rel_path("").push("c").unwrap().as_str(), "c");
        assert!(rel_path("a/b").push("").is_err());
        assert!(rel_path("a/b").push("c/d").is_err());
    }

    #[test]
    fn test_pop() {
        let mut path = rel_path("a/b").to_rel_path_buf();
        path.pop();
        assert_eq!(path.as_rel_path().as_str(), "a");
        path.pop();
        assert_eq!(path.as_rel_path().as_str(), "");
        path.pop();
        assert_eq!(path.as_rel_path().as_str(), "");
    }
}
