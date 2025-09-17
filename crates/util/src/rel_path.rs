use std::{
    borrow::Cow,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context as _, Result};

use crate::paths::{PathStyle, SanitizedPath};

/// An absolute path.
#[derive(PartialEq, Debug, Eq, Hash)]
struct AbsPath {
    style: PathStyle,
    contents: String,
}

impl AbsPath {
    pub fn new<S: AsRef<str>>(path: &S, style: PathStyle) -> Result<Self> {
        // TODO: remove .. components (write cannonicalize?)
        let path = path.as_ref();
        match style {
            PathStyle::Posix => {
                anyhow::ensure!(path.starts_with('/'));
            }
            PathStyle::Windows => {
                // TODO: this needs to work the same regardless of current target
                let path = dunce::simplified(Path::new(path)).to_str().unwrap();

                let mut is_absolute = false;
                if path.starts_with('\\') {
                    is_absolute = true;
                } else {
                    let mut chars = path.chars();
                    if let Some(c) = chars.next() {
                        if c.is_ascii_alphabetic() {
                            if let Some(c) = chars.next() {
                                if c == ':' {
                                    is_absolute = true;
                                }
                            }
                        }
                    }
                }
                anyhow::ensure!(is_absolute);
            }
        }
        Ok(Self {
            style,
            contents: path.to_owned(),
        })
    }

    pub fn file_name(&self) -> Option<&str> {
        if let Some(end_ix) = self.contents.rfind(self.style.separator()) {
            let name = &self.contents[end_ix + 1..];
            if name.is_empty() { None } else { Some(name) }
        } else {
            None
        }
    }

    pub fn to_proto(&self) -> String {
        self.contents.to_owned()
    }

    pub fn path_style(&self) -> PathStyle {
        self.style
    }

    pub fn starts_with(&self, other: &Self) -> bool {
        self.strip_prefix(other).is_ok()
    }

    pub fn strip_prefix(&self, other: &Self) -> Result<&RelPath, ()> {
        if let Some(suffix) = self.contents.strip_prefix(&other.contents) {
            if suffix.starts_with(self.style.separator()) {
                return Ok(unsafe { RelPath::new_unchecked(&suffix[1..]) });
            } else if suffix.is_empty() {
                return Ok(RelPath::empty());
            }
        }
        Err(())
    }

    pub fn to_std_path(&self) -> PathBuf {
        PathBuf::from(&self.contents)
    }

    pub fn as_std_path(&self) -> Option<&Path> {
        if self.style == PathStyle::current() {
            Some(Path::new(&self.contents))
        } else {
            None
        }
    }

    pub fn as_sanitized_path(&self) -> Option<&SanitizedPath> {
        Some(SanitizedPath::new(self.as_std_path()?))
    }

    pub fn from_sanitized_path(p: &SanitizedPath) -> Result<Self> {
        Self::new(
            &p.to_str().context("non-unicode path")?,
            PathStyle::current(),
        )
    }

    pub fn from_std_path(path: &Path) -> Result<Self> {
        let path = path.to_str().context("non-unicode path")?;
        // TODO strip trailing slash?
        Self::new(&path, PathStyle::current())
    }

    // this is a temporary escape hatch for places where we're handed a std::path::Path that semantically might not be a local path
    // TODO eliminate all uses of this by using the correct types everywhere
    pub fn from_std_path_with_style(path: &Path, style: PathStyle) -> Result<Self> {
        let path = path.to_str().context("non-unicode path")?;
        Self::new(&path, style)
    }
}

impl std::fmt::Display for AbsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

#[repr(transparent)]
#[derive(PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub struct RelPath(str);

impl AsRef<Path> for RelPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl RelPath {
    pub fn empty() -> &'static Self {
        unsafe { Self::new_unchecked("") }
    }

    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> Option<&Self> {
        let this = unsafe { Self::new_unchecked(s) };
        if this.0.starts_with("/")
            || this.0.ends_with("/")
            || this
                .components()
                .any(|component| component == ".." || component == "." || component.is_empty())
        {
            log::debug!("invalid relative path: {:?}", &this.0);
            return None;
        }
        Some(this)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn from_str<S: AsRef<str> + ?Sized>(s: &S) -> &Self {
        Self::new(s.as_ref()).unwrap()
    }

    pub fn from_std_path(path: &Path, path_style: PathStyle) -> Option<Arc<Self>> {
        let mut string = Cow::Borrowed(path.to_str()?);

        if path_style == PathStyle::Windows {
            string = Cow::Owned(string.as_ref().replace('\\', "/"))
        }

        if string.ends_with('/') && string.len() > 1 {
            string = match string {
                Cow::Borrowed(string) => Cow::Borrowed(&string[..string.len() - 1]),
                Cow::Owned(mut string) => {
                    string.truncate(string.len() - 1);
                    Cow::Owned(string)
                }
            };
        }

        Self::new(&string).map(Arc::from)
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

    pub fn strip_prefix(&self, other: &Self) -> Result<&Self, ()> {
        if let Some(suffix) = self.0.strip_prefix(&other.0) {
            if suffix.starts_with('/') {
                return Ok(unsafe { Self::new_unchecked(&suffix[1..]) });
            } else if suffix.is_empty() {
                return Ok(Self::empty());
            }
        }
        Err(())
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

    pub fn append_to_abs_path(&self, abs_path: &Path) -> PathBuf {
        let mut result = abs_path.to_path_buf();
        for component in self.components() {
            result.push(component);
        }
        result
    }

    pub fn to_proto(&self) -> String {
        self.0.to_owned()
    }

    pub fn from_proto(path: &str) -> Option<Arc<Self>> {
        let this = Self::new(path);
        debug_assert!(
            this.is_some(),
            "invalid relative path from proto: {:?}",
            path
        );
        Some(Arc::from(this?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
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

impl From<&RelPath> for Arc<RelPath> {
    fn from(rel_path: &RelPath) -> Self {
        let bytes: Arc<str> = Arc::from(&rel_path.0);
        unsafe { Arc::from_raw(Arc::into_raw(bytes) as *const RelPath) }
    }
}

impl<'a> TryFrom<&'a str> for &'a RelPath {
    type Error = ();

    fn try_from(value: &str) -> std::result::Result<&RelPath, ()> {
        RelPath::new(value).ok_or(())
    }
}

pub struct RelPathComponents<'a>(&'a str);

pub struct RelPathAncestors<'a>(Option<&'a str>);

const SEPARATOR: char = '/';

impl<'a> RelPathComponents<'a> {
    fn rest(&self) -> &'a RelPath {
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
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_path_construction() {
        assert!(RelPath::new("/").is_none());
        assert!(RelPath::new("/foo").is_none());
        assert!(RelPath::new("foo/").is_none());
        assert!(RelPath::new("foo//bar").is_none());
        assert!(RelPath::new("foo/../bar").is_none());
        assert!(RelPath::new("./foo/bar").is_none());
        assert!(RelPath::new("..").is_none());

        assert!(RelPath::from_std_path(Path::new("/"), PathStyle::current()).is_none());
        assert!(RelPath::from_std_path(Path::new("//"), PathStyle::current()).is_none());
        assert!(RelPath::from_std_path(Path::new("/foo/"), PathStyle::current()).is_none());
        assert_eq!(
            RelPath::from_std_path(&PathBuf::from_iter(["foo", ""]), PathStyle::current()).unwrap(),
            Arc::from(RelPath::from_str("foo"))
        );
    }

    #[test]
    fn test_rel_path_components() {
        let path = RelPath::from_str("foo/bar/baz");
        let mut components = path.components();
        assert_eq!(components.next(), Some("foo"));
        assert_eq!(components.next(), Some("bar"));
        assert_eq!(components.next(), Some("baz"));
        assert_eq!(components.next(), None);
    }

    #[test]
    fn test_rel_path_ancestors() {
        let path = RelPath::from_str("foo/bar/baz");
        let mut components = path.ancestors();
        assert_eq!(components.next(), Some(RelPath::from_str("foo/bar/baz")));
        assert_eq!(components.next(), Some(RelPath::from_str("foo/bar")));
        assert_eq!(components.next(), Some(RelPath::from_str("foo")));
        assert_eq!(components.next(), Some(RelPath::from_str("")));
        assert_eq!(components.next(), None);

        let path = RelPath::from_str("foo");
        let mut components = path.ancestors();
        assert_eq!(components.next(), Some(RelPath::from_str("foo")));
        assert_eq!(components.next(), Some(RelPath::empty()));
        assert_eq!(components.next(), None);

        let path = RelPath::empty();
        let mut components = path.ancestors();
        assert_eq!(components.next(), Some(RelPath::empty()));
        assert_eq!(components.next(), None);
    }

    #[test]
    fn test_rel_path_parent() {
        assert_eq!(
            RelPath::from_str("foo/bar/baz").parent(),
            RelPath::new("foo/bar")
        );
        assert_eq!(RelPath::from_str("foo").parent(), Some(RelPath::empty()));
        assert_eq!(RelPath::from_str("").parent(), None);
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
}
