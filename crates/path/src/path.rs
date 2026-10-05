//! Relative path types for deltadb.
//!
//! Provides [`RelPath`] and [`RelPathBuf`] — path types that are guaranteed to be
//! relative, normalized, and valid unicode. Internally stored in POSIX (`/`-delimited)
//! format regardless of host platform.
//!
//! Adapted from Zed's `util::rel_path` module.

use std::{
    borrow::Cow,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::path_component::{Component, Components, Prefix, PrefixExt, iter_after};
use crate::rel_path::RelPath;

pub mod abs_path;
mod path_component;
pub mod rel_path;
mod windows_prefix;

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

    pub fn join_path_preserving_components(
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
        Ok(PathBuf::from(&joined))
    }

    pub fn normalize(self, path_like: &str) -> String {
        match self {
            PathStyle::Windows => {
                let drive_and_remainder = path_like.split_once(':').filter(|(drive, _)| {
                    let mut characters = drive.chars();
                    characters
                        .next()
                        .is_some_and(|character| character.is_ascii_alphabetic())
                        && characters.next().is_none()
                });
                let unc_remainder = path_like
                    .strip_prefix("\\\\")
                    .or_else(|| path_like.strip_prefix("//"));

                let (prefix, remainder) = if let Some((drive, remainder)) = drive_and_remainder {
                    if let Some(remainder) = remainder
                        .strip_prefix('\\')
                        .or_else(|| remainder.strip_prefix('/'))
                    {
                        (format!("{drive}:\\"), remainder)
                    } else {
                        (format!("{drive}:"), remainder)
                    }
                } else if let Some(remainder) = unc_remainder {
                    let (server, remainder) = match remainder.split_once(['\\', '/']) {
                        Some(parts) => parts,
                        None => return path_like.to_string(),
                    };
                    let (share, remainder) = match remainder.split_once(['\\', '/']) {
                        Some(parts) => parts,
                        None => return format!("\\\\{server}\\{remainder}"),
                    };
                    (format!("\\\\{server}\\{share}\\"), remainder)
                } else if let Some(remainder) = path_like
                    .strip_prefix('\\')
                    .or_else(|| path_like.strip_prefix('/'))
                {
                    ("\\".to_string(), remainder)
                } else {
                    (String::new(), path_like)
                };

                let mut components: Vec<&str> = Vec::new();
                for component in remainder.split(['\\', '/']) {
                    match component {
                        "" | "." => {}
                        ".." => {
                            if components.last().is_some_and(|c| *c != "..") {
                                components.pop();
                            } else if prefix.is_empty() {
                                components.push(component);
                            }
                        }
                        component => components.push(component),
                    }
                }

                let normalized = components.join("\\");
                if prefix.is_empty() {
                    normalized
                } else {
                    format!("{prefix}{normalized}")
                }
            }
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

    pub fn file_name<'a>(&self, path: &'a Path) -> Option<&'a OsStr> {
        self.components(path).next_back().and_then(|p| match p {
            Component::Normal(p) => Some(p),
            _ => None,
        })
    }

    pub fn parent<'a>(&self, path: &'a Path) -> Option<&'a Path> {
        let mut comps = self.components(path);
        let comp = comps.next_back();
        comp.and_then(|p| match p {
            Component::Normal(_) | Component::CurDir | Component::ParentDir => {
                Some(comps.as_path())
            }
            _ => None,
        })
    }

    pub fn starts_with<P: AsRef<Path>>(&self, path: P, prefix: P) -> bool {
        iter_after(
            self.components(path.as_ref()),
            self.components(prefix.as_ref()),
        )
        .is_some()
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

// This part contains the methods adopted from the std lib
// to make the pub api of `PathStyle` compatible with `Path`
impl PathStyle {
    fn components<'a>(&self, path: &'a Path) -> Components<'a> {
        Components::new(*self, path)
    }

    fn parse_prefix<'a>(&self, path: &'a OsStr) -> Option<Prefix<'a>> {
        match self {
            PathStyle::Unix => None,
            PathStyle::Windows => windows_prefix::parse_prefix(path),
        }
    }

    fn has_prefixes(&self) -> bool {
        match self {
            PathStyle::Unix => false,
            PathStyle::Windows => true,
        }
    }

    fn is_sep_byte(&self, b: u8) -> bool {
        match self {
            Self::Unix => b == b'/',
            Self::Windows => b == b'\\' || b == b'/',
        }
    }

    fn is_verbatim_sep(&self, b: u8) -> bool {
        match self {
            Self::Unix => self.is_sep_byte(b),
            Self::Windows => b == b'\\',
        }
    }

    fn has_physical_root(&self, s: &[u8], prefix: Option<Prefix<'_>>) -> bool {
        let path = if let Some(p) = prefix {
            &s[p.len()..]
        } else {
            s
        };
        !path.is_empty() && self.is_sep_byte(path[0])
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rel_path::rel_path;

    #[test]
    fn test_join_path_uses_path_style_separator() {
        let posix_path = PathStyle::Unix
            .join_path(Path::new("/home/user/dev"), "worktrees")
            .unwrap();
        let windows_path = PathStyle::Windows
            .join_path(Path::new("C:\\Users\\user\\dev"), "worktrees")
            .unwrap();

        assert_eq!(posix_path, PathBuf::from("/home/user/dev/worktrees"));
        assert_eq!(
            windows_path.to_string_lossy(),
            "C:\\Users\\user\\dev\\worktrees"
        );
    }

    #[test]
    fn test_join_path_preserving_components() {
        let posix_path = PathStyle::Unix
            .join_path_preserving_components(Path::new("/home/user/symlink"), "../worktrees")
            .unwrap();
        let windows_path = PathStyle::Windows
            .join_path_preserving_components(Path::new(r"C:\Users\user\symlink"), r"..\worktrees")
            .unwrap();

        assert_eq!(posix_path, PathBuf::from("/home/user/symlink/../worktrees"));
        assert_eq!(
            windows_path.to_string_lossy(),
            r"C:\Users\user\symlink\..\worktrees"
        );
    }

    #[test]
    fn test_normalize_uses_path_style_separator() {
        assert_eq!(
            PathStyle::Unix.normalize("/home/user/dev/../worktrees/./zed"),
            "/home/user/worktrees/zed"
        );
        assert_eq!(
            PathStyle::Windows.normalize("C:\\Users\\user\\dev\\worktrees"),
            "C:\\Users\\user\\dev\\worktrees"
        );
    }

    #[test]
    fn test_normalize_windows_path_regardless_of_host_platform() {
        assert_eq!(
            PathStyle::Windows.normalize(r"C:\Users\user\dev\..\worktrees"),
            r"C:\Users\user\worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize(r"C:\Users\.\worktrees"),
            r"C:\Users\worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize(r"C:\Users\user\dev\sub\..\..\worktrees"),
            r"C:\Users\user\worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize("C:/Users/user/dev/../worktrees"),
            r"C:\Users\user\worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize(r"C:/Users\user/dev\..\worktrees"),
            r"C:\Users\user\worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize(r"C:\Users/user\.\worktrees"),
            r"C:\Users\user\worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize(r"\\server\share\dev\..\worktrees"),
            r"\\server\share\worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize(r"//server\share/dev\..\worktrees"),
            r"\\server\share\worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize(r"\dev\..\worktrees"),
            r"\worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize(r"dev\..\worktrees"),
            r"worktrees"
        );
        assert_eq!(
            PathStyle::Windows.normalize(r"C:\..\worktrees"),
            r"C:\worktrees"
        );
    }

    #[test]
    fn test_strip_prefix() {
        let expected = [
            (
                PathStyle::Unix,
                "/a/b/c",
                "/a/b",
                Some(rel_path("c").into_arc()),
            ),
            (
                PathStyle::Unix,
                "/a/b/c",
                "/a/b/",
                Some(rel_path("c").into_arc()),
            ),
            (
                PathStyle::Unix,
                "/a/b/c",
                "/",
                Some(rel_path("a/b/c").into_arc()),
            ),
            (PathStyle::Unix, "/a/b/c", "", None),
            (PathStyle::Unix, "/a/b//c", "/a/b/", None),
            (PathStyle::Unix, "/a/bc", "/a/b", None),
            (
                PathStyle::Unix,
                "/a/b/c",
                "/a/b/c",
                Some(rel_path("").into_arc()),
            ),
            (
                PathStyle::Windows,
                "C:\\a\\b\\c",
                "C:\\a\\b",
                Some(rel_path("c").into_arc()),
            ),
            (
                PathStyle::Windows,
                "C:\\a\\b\\c",
                "C:\\a\\b\\",
                Some(rel_path("c").into_arc()),
            ),
            (
                PathStyle::Windows,
                "C:\\a\\b\\c",
                "C:\\",
                Some(rel_path("a/b/c").into_arc()),
            ),
            (PathStyle::Windows, "C:\\a\\b\\c", "", None),
            (PathStyle::Windows, "C:\\a\\b\\\\c", "C:\\a\\b\\", None),
            (PathStyle::Windows, "C:\\a\\bc", "C:\\a\\b", None),
            (
                PathStyle::Windows,
                "C:\\a\\b/c",
                "C:\\a\\b",
                Some(rel_path("c").into_arc()),
            ),
            (
                PathStyle::Windows,
                "C:\\a\\b/c",
                "C:\\a\\b\\",
                Some(rel_path("c").into_arc()),
            ),
            (
                PathStyle::Windows,
                "C:\\a\\b/c",
                "C:\\a\\b/",
                Some(rel_path("c").into_arc()),
            ),
        ];
        let actual = expected.clone().map(|(style, child, parent, _)| {
            (
                style,
                child,
                parent,
                style
                    .strip_prefix(child.as_ref(), parent.as_ref())
                    .map(|rel_path| rel_path.into_arc()),
            )
        });
        pretty_assertions::assert_eq!(actual, expected);
    }

    #[test]
    fn test_unix_path_style_starts_with() {
        let unix_paths_and_prefixes = [
            ("", "", true),
            ("foo", "", true),
            ("", "foo", false),
            ("foo", "foo", true),
            ("foo/", "foo", true),
            ("foo//", "foo", true),
            ("foo///", "foo", true),
            ("foo/.", "foo", true),
            ("foo/./bar", "foo/bar", true),
            ("foo/.//bar", "foo/bar", true),
            ("foo//./bar", "foo/bar", true),
            ("foo/bar", "foo", true),
            ("foo/bar", "foobar", false),
            ("foo/bar/baz", "foo/bar", true),
            ("foo/bar", "foo/bar/baz", false),
            ("./foo/bar/", ".", true),
            ("/etc/passwd", "/etc", true),
            ("/etc/passwd", "/etc/", true),
            ("/etc/passwd", "/etc/passwd", true),
            ("/etc/passwd", "/etc/passwd/", true),
            ("/etc/passwd", "/etc/passwd///", true),
            ("/etc/passwd", "/e", false),
            ("/etc/passwd", "/etc/passwd.txt", false),
            ("/etc/foo.rs", "/etc/foo", false),
        ];

        for (path, prefix, expected) in unix_paths_and_prefixes {
            assert_eq!(
                PathStyle::Unix.starts_with(Path::new(path), Path::new(prefix)),
                expected,
                "{path:?}.starts_with({prefix:?})"
            );
        }
    }

    #[test]
    fn test_windows_path_style_starts_with() {
        let windows_paths_and_prefixes = [
            (
                r"C:\src\rust\cargo-test\test\Cargo.toml",
                r"c:\src\rust\cargo-test\test",
                true,
            ),
            (r"C:\foo\.\bar.txt", r"C:\foo\bar.txt", true),
            (r"C:\foo\.", r"C:\foo", true),
            (r"\\server\share", r"\\server", false),
            (r"\\server\share\foo.txt", r"\\server", false),
            (r"\\server\share\foo.txt", r"\\server\share", true),
            (r"\\server\share\foo.txt", r"\\server\share\", true),
            (r"\\server\share\foo.txt", r"\\server\share/", true),
            (r"\\server\share\foo.txt", r"\\server\other", false),
            (r"\\server\share\foo.txt", r"\\SERVER\share", false),
            (r"\\?\C:\foo\.\bar.txt", r"\\?\C:\foo\bar.txt", false),
            (r"\\?\C:\foo.txt", r"\\?\C:\", true),
            (
                r"\\?\UNC\server\share\foo.txt",
                r"\\?\UNC\server\share",
                true,
            ),
            (r"\\?\UNC\server\share\foo.txt", r"\\?\UNC\server\", false),
            (r"\\?\bar\foo.txt", r"\\?\bar", true),
            (r"\\?\C:\foo/bar.txt", r"\\?\C:\foo", false),
        ];

        for (path, prefix, expected) in windows_paths_and_prefixes {
            assert_eq!(
                PathStyle::Windows.starts_with(Path::new(path), Path::new(prefix)),
                expected,
                "{path:?}.starts_with({prefix:?})"
            );
        }
    }

    #[test]
    fn test_unix_path_style_file_name() {
        let unix_paths_and_filenames = [
            (Path::new(""), None),
            (Path::new("/usr/bin/"), Some("bin")),
            (Path::new("tmp/foo.txt"), Some("foo.txt")),
            (Path::new("."), None),
            (Path::new("./"), None),
            (Path::new("foo.txt/."), Some("foo.txt")),
            (Path::new("foo.txt/.//"), Some("foo.txt")),
            (Path::new("foo/./bar"), Some("bar")),
            (Path::new("foo.txt/.."), None),
            (Path::new("/.."), None),
            (Path::new("/"), None),
        ];

        for (path, filename) in unix_paths_and_filenames {
            assert_eq!(PathStyle::Unix.file_name(path), filename.map(OsStr::new));
        }
    }

    #[test]
    fn test_windows_path_style_file_name() {
        let windows_paths_and_filenames = [
            (Path::new(""), None),
            (Path::new("."), None),
            (Path::new(r"C:\usr\bin\"), Some("bin")),
            (Path::new(r"C:"), None),
            (Path::new(r"tmp\foo.txt"), Some("foo.txt")),
            (Path::new("tmp/foo.txt"), Some("foo.txt")),
            (Path::new(r"foo.txt\."), Some("foo.txt")),
            (Path::new(r"foo.txt\.\"), Some("foo.txt")),
            (Path::new(r"foo.txt\.."), None),
            (Path::new(r"C:\"), None),
            (Path::new(r"\\server\share"), None),
            (Path::new(r"\\server\share\foo.txt"), Some("foo.txt")),
            (Path::new("//server/share"), None),
            (Path::new("//server/share/foo.txt"), Some("foo.txt")),
            (Path::new(r"\\?\bar"), None),
            (Path::new(r"\\?\bar\foo.txt"), Some("foo.txt")),
            (Path::new(r"\\?\C:/foo/bar"), None),
            (Path::new(r"\\.\device"), None),
            (Path::new(r"\\.\device\foo.txt"), Some("foo.txt")),
        ];

        for (path, filename) in windows_paths_and_filenames {
            assert_eq!(PathStyle::Windows.file_name(path), filename.map(OsStr::new));
        }
    }

    #[test]
    fn test_unix_path_style_parent() {
        let unix_paths_and_parents = [
            ("", None),
            ("/foo/bar", Some("/foo")),
            ("/foo", Some("/")),
            ("/", None),
            ("///foo///", Some("/")),
            ("///foo///bar", Some("///foo")),
            ("foo/bar", Some("foo")),
            ("foo", Some("")),
            ("foo/.", Some("")),
            ("foo/./bar", Some("foo")),
            ("foo/../bar", Some("foo/..")),
            ("./a", Some(".")),
            ("./.", Some("")),
            ("/..", Some("/")),
            (".", Some("")),
            ("..", Some("")),
        ];

        for (path, parent) in unix_paths_and_parents {
            assert_eq!(
                PathStyle::Unix.parent(Path::new(path)),
                parent.map(Path::new)
            );
        }
    }

    #[test]
    fn test_windows_path_style_parent() {
        let windows_paths_and_parents = [
            ("", None),
            (r"C:\foo\bar", Some(r"C:\foo")),
            (r"C:\foo", Some(r"C:\")),
            (r"C:\", None),
            (r"C:foo", Some("C:")),
            (r"C:", None),
            (r"\foo", Some(r"\")),
            (r"\", None),
            (r"foo\bar", Some("foo")),
            (r"foo\\bar", Some("foo")),
            ("foo/bar", Some("foo")),
            ("foo", Some("")),
            (r"foo\.", Some("")),
            (r"foo\.\bar", Some("foo")),
            (r"foo\..\bar", Some(r"foo\..")),
            (r".\a", Some(".")),
            (r".\.", Some("")),
            (r"\\server\share", None),
            (r"\\server\share\foo.txt", Some(r"\\server\share\")),
            ("//server/share", None),
            ("//server/share/foo.txt", Some("//server/share/")),
            (r"\\?\bar", None),
            (r"\\?\bar\foo.txt", Some(r"\\?\bar\")),
            (
                r"\\?\UNC\server\share\foo.txt",
                Some(r"\\?\UNC\server\share\"),
            ),
            (r"\\?\C:\foo.txt", Some(r"\\?\C:\")),
            (r"\\?\C:/foo/bar", None),
            (r"\\.\device", None),
            (r"\\.\device\foo.txt", Some(r"\\.\device\")),
            (".", Some("")),
            ("..", Some("")),
        ];

        for (path, parent) in windows_paths_and_parents {
            assert_eq!(
                PathStyle::Windows.parent(Path::new(path)),
                parent.map(Path::new)
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn test_local_path_style_non_utf8() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let path = Path::new(OsStr::from_bytes(b"/home/user/\xff\xfe/repo/file.txt"));
        assert_eq!(
            PathStyle::Unix.file_name(path),
            Some(OsStr::new("file.txt"))
        );
        assert_eq!(
            PathStyle::Unix.parent(path),
            Some(Path::new(OsStr::from_bytes(b"/home/user/\xff\xfe/repo")))
        );

        let another_path = Path::new(OsStr::from_bytes(b"/home/user/repo/\xff\xfe.txt"));
        assert_eq!(
            PathStyle::Unix.file_name(another_path),
            Some(OsStr::from_bytes(b"\xff\xfe.txt"))
        );
        assert_eq!(
            PathStyle::Unix.parent(another_path),
            Some(Path::new("/home/user/repo"))
        );
    }

    #[cfg(windows)]
    #[test]
    fn test_local_path_style_non_utf8() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        let wide: Vec<u16> = "C:\\invalid_"
            .encode_utf16()
            .chain(Some(0xD800))
            .chain(r"\repo\file.txt".encode_utf16())
            .collect();
        let os_str = OsString::from_wide(&wide);
        let path = Path::new(&os_str);

        assert_eq!(
            PathStyle::Windows.file_name(path),
            Some(OsStr::new("file.txt"))
        );
        let parent = PathStyle::Windows.parent(path).unwrap();
        assert_eq!(
            PathStyle::Windows.file_name(parent),
            Some(OsStr::new("repo"))
        );
    }
}
