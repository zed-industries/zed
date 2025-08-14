use std::{
    borrow::Cow,
    ffi::OsStr,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Result, bail};

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelPath([u8]);

impl RelPath {
    pub fn new<S: AsRef<[u8]> + ?Sized>(s: &S) -> &Self {
        unsafe { &*(s.as_ref() as *const [u8] as *const Self) }
    }

    pub fn components(&self) -> RelPathComponents {
        RelPathComponents(&self.0)
    }

    pub fn file_name(&self) -> Option<&[u8]> {
        self.components().next_back()
    }

    pub fn parent(&self) -> Option<&Self> {
        let mut components = self.components();
        components.next_back()?;
        Some(Self::new(components.0))
    }

    pub fn starts_with(&self, other: &Self) -> bool {
        let mut components = self.components();
        other.components().all(|other_component| {
            components
                .next()
                .map_or(false, |component| component == other_component)
        })
    }

    pub fn strip_prefix(&self, other: &Self) -> Result<&Self, ()> {
        let mut components = self.components();
        other
            .components()
            .all(|other_component| {
                components
                    .next()
                    .map_or(false, |component| component == other_component)
            })
            .then(|| Self::new(components.0))
            .ok_or_else(|| ())
    }

    pub fn from_path(relative_path: &Path) -> Result<&Self> {
        use std::path::Component;
        match relative_path.components().next() {
            Some(Component::Prefix(_)) => bail!(
                "path `{}` should be relative, not a windows prefix",
                relative_path.to_string_lossy()
            ),
            Some(Component::RootDir) => {
                bail!(
                    "path `{}` should be relative",
                    relative_path.to_string_lossy()
                )
            }
            Some(Component::CurDir) => {
                bail!(
                    "path `{}` should not start with `.`",
                    relative_path.to_string_lossy()
                )
            }
            Some(Component::ParentDir) => {
                bail!(
                    "path `{}` should not start with `..`",
                    relative_path.to_string_lossy()
                )
            }
            None => bail!("relative path should not be empty"),
            _ => Ok(Self::new(relative_path.as_os_str().as_bytes())),
        }
    }

    pub fn append_to_abs_path(&self, abs_path: &Path) -> PathBuf {
        // TODO: implement this differently
        let mut result = abs_path.to_path_buf();
        for component in self.components() {
            result.push(String::from_utf8_lossy(component).as_ref());
        }
        result
    }

    pub fn to_proto(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn as_os_str(&self) -> Cow<'_, OsStr> {
        #[cfg(target_os = "windows")]
        {
            use std::ffi::OsString;
            let path = String::from_utf8_lossy(&self.0);
            match path {
                Cow::Borrowed(s) => Cow::Borrowed(OsStr::new(s)),
                Cow::Owned(s) => Cow::Owned(OsString::from(s)),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            use std::os::unix::ffi::OsStrExt;

            Cow::Borrowed(OsStr::from_bytes(&self.0))
        }
    }
}

impl From<&RelPath> for Arc<RelPath> {
    fn from(rel_path: &RelPath) -> Self {
        let bytes: Arc<[u8]> = Arc::from(&rel_path.0);
        unsafe { Arc::from_raw(Arc::into_raw(bytes) as *const RelPath) }
    }
}

impl AsRef<RelPath> for &str {
    fn as_ref(&self) -> &RelPath {
        RelPath::new(self)
    }
}

impl std::fmt::Debug for RelPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Ok(str) = std::str::from_utf8(&self.0) {
            write!(f, "RelPath({})", str)
        } else {
            write!(f, "RelPath({:?})", &self.0)
        }
    }
}

pub struct RelPathComponents<'a>(&'a [u8]);

const SEPARATOR: u8 = b'/';

impl<'a> Iterator for RelPathComponents<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sep_ix) = self.0.iter().position(|&byte| byte == SEPARATOR) {
            let (head, tail) = self.0.split_at(sep_ix);
            self.0 = &tail[1..];
            Some(head)
        } else if self.0.is_empty() {
            None
        } else {
            let result = self.0;
            self.0 = &[];
            Some(result)
        }
    }
}

impl<'a> DoubleEndedIterator for RelPathComponents<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(sep_ix) = self.0.iter().rposition(|&byte| byte == SEPARATOR) {
            let (head, tail) = self.0.split_at(sep_ix);
            self.0 = head;
            Some(&tail[1..])
        } else if self.0.is_empty() {
            None
        } else {
            let result = self.0;
            self.0 = &[];
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rel_path_components() {
        let path = RelPath::new("foo/bar/baz");
        let mut components = path.components();
        assert_eq!(components.next(), Some("foo".as_bytes()));
        assert_eq!(components.next(), Some("bar".as_bytes()));
        assert_eq!(components.next(), Some("baz".as_bytes()));
        assert_eq!(components.next(), None);
    }

    #[test]
    fn test_rel_path_parent() {
        assert_eq!(
            RelPath::new("foo/bar/baz").parent().unwrap(),
            RelPath::new("foo/bar")
        );
        assert_eq!(RelPath::new("foo").parent().unwrap(), RelPath::new(""));
        assert_eq!(RelPath::new("").parent(), None);
    }
}
