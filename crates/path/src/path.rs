//! Relative path types for deltadb.
//!
//! Provides [`RelPath`] and [`RelPathBuf`] — path types that are guaranteed to be
//! relative, normalized, and valid unicode. Internally stored in POSIX (`/`-delimited)
//! format regardless of host platform.
//!
//! Adapted from Zed's `util::rel_path` module.

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use crate::rel_path::RelPath;

pub mod abs_path;
pub mod rel_path;

pub trait PathExt {
    fn to_rel_path_buf(&self) -> anyhow::Result<rel_path::RelPathBuf>;
}

impl<T: AsRef<Path> + ?Sized> PathExt for T {
    fn to_rel_path_buf(&self) -> anyhow::Result<rel_path::RelPathBuf> {
        Ok(RelPath::new(self.as_ref(), PathStyle::local())?.into_owned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathStyle {
    Unix,
    Windows,
}

impl PathStyle {
    #[cfg(target_os = "windows")]
    pub const fn local() -> Self {
        PathStyle::Windows
    }

    #[cfg(not(target_os = "windows"))]
    pub const fn local() -> Self {
        PathStyle::Unix
    }

    #[inline]
    pub fn primary_separator(&self) -> &'static str {
        match self {
            PathStyle::Unix => "/",
            PathStyle::Windows => "\\",
        }
    }

    pub fn separators(&self) -> &'static [&'static str] {
        match self {
            PathStyle::Unix => &["/"],
            PathStyle::Windows => &["\\", "/"],
        }
    }

    pub fn separators_ch(&self) -> &'static [char] {
        match self {
            PathStyle::Unix => &['/'],
            PathStyle::Windows => &['\\', '/'],
        }
    }

    pub fn is_absolute(&self, path_like: &str) -> bool {
        path_like.starts_with('/')
            || *self == PathStyle::Windows
                && (path_like.starts_with('\\')
                    || path_like
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_ascii_alphabetic())
                        && path_like[1..]
                            .strip_prefix(':')
                            .is_some_and(|path| path.starts_with('/') || path.starts_with('\\')))
    }

    pub fn is_windows(&self) -> bool {
        *self == PathStyle::Windows
    }

    pub fn is_posix(&self) -> bool {
        *self == PathStyle::Unix
    }

    pub fn join(self, left: impl AsRef<Path>, right: impl AsRef<Path>) -> Option<String> {
        let right = right.as_ref().to_str()?;
        if is_absolute(right, self) {
            return None;
        }
        let left = left.as_ref().to_str()?;
        if left.is_empty() {
            Some(right.into())
        } else {
            Some(format!(
                "{left}{}{right}",
                if left.ends_with(self.primary_separator()) {
                    ""
                } else {
                    self.primary_separator()
                }
            ))
        }
    }

    pub fn join_path(
        self,
        left: impl AsRef<Path>,
        right: impl AsRef<Path>,
    ) -> anyhow::Result<PathBuf> {
        let left = left
            .as_ref()
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Path contains invalid UTF-8"))?;
        let right = right.as_ref();
        let right_string = right
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Path contains invalid UTF-8"))?;
        let joined = self
            .join(left, right_string)
            .ok_or_else(|| anyhow::anyhow!("Path must be relative: {right:?}"))?;
        Ok(PathBuf::from(self.normalize(&joined)))
    }

    pub fn normalize(self, path_like: &str) -> String {
        match self {
            PathStyle::Windows => crate::normalize_path(Path::new(path_like))
                .to_string_lossy()
                .into_owned(),
            PathStyle::Unix => {
                let is_absolute = path_like.starts_with('/');
                let remainder = if is_absolute {
                    path_like.trim_start_matches('/')
                } else {
                    path_like
                };

                let mut components = Vec::new();
                for component in remainder.split(self.separators_ch()) {
                    match component {
                        "" | "." => {}
                        ".." => {
                            if components
                                .last()
                                .is_some_and(|component| *component != "..")
                            {
                                components.pop();
                            } else if !is_absolute {
                                components.push(component);
                            }
                        }
                        component => components.push(component),
                    }
                }

                let normalized = components.join(self.primary_separator());
                if is_absolute && normalized.is_empty() {
                    "/".to_string()
                } else if is_absolute {
                    format!("/{normalized}")
                } else {
                    normalized
                }
            }
        }
    }

    pub fn split(self, path_like: &str) -> (Option<&str>, &str) {
        let Some(pos) = path_like.rfind(self.primary_separator()) else {
            return (None, path_like);
        };
        let filename_start = pos + self.primary_separator().len();
        (
            Some(&path_like[..filename_start]),
            &path_like[filename_start..],
        )
    }

    pub fn strip_prefix<'a>(
        &self,
        child: &'a Path,
        parent: &'a Path,
    ) -> Option<std::borrow::Cow<'a, RelPath>> {
        let parent = parent.to_str()?;
        if parent.is_empty() {
            return RelPath::new(child, *self).ok();
        }
        let parent = self
            .separators()
            .iter()
            .find_map(|sep| parent.strip_suffix(sep))
            .unwrap_or(parent);
        let child = child.to_str()?;

        // Match behavior of std::path::Path, which is case-insensitive for drive letters (e.g., "C:" == "c:")
        let stripped = if self.is_windows()
            && child.as_bytes().get(1) == Some(&b':')
            && parent.as_bytes().get(1) == Some(&b':')
            && child.as_bytes()[0].eq_ignore_ascii_case(&parent.as_bytes()[0])
        {
            child[2..].strip_prefix(&parent[2..])?
        } else {
            child.strip_prefix(parent)?
        };
        if let Some(relative) = self
            .separators()
            .iter()
            .find_map(|sep| stripped.strip_prefix(sep))
        {
            RelPath::new(relative.as_ref(), *self).ok()
        } else if stripped.is_empty() {
            Some(Cow::Borrowed(RelPath::empty()))
        } else {
            None
        }
    }
}

fn is_absolute(path_like: &str, path_style: PathStyle) -> bool {
    path_like.starts_with('/')
        || path_style == PathStyle::Windows
            && (path_like.starts_with('\\')
                || path_like
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic())
                    && path_like[1..]
                        .strip_prefix(':')
                        .is_some_and(|path| path.starts_with('/') || path.starts_with('\\')))
}

/// Normalizes a path by resolving `.` and `..` components without
/// requiring the path to exist on disk (unlike `canonicalize`).
pub fn normalize_path(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
