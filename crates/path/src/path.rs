//! Relative path types for deltadb.
//!
//! Provides [`RelPath`] and [`RelPathBuf`] — path types that are guaranteed to be
//! relative, normalized, and valid unicode. Internally stored in POSIX (`/`-delimited)
//! format regardless of host platform.
//!
//! Adapted from Zed's `util::rel_path` module.

#[cfg(windows)]
use anyhow::Context;
#[cfg(all(unix, not(target_family = "wasm")))]
use std::os::unix::prelude::OsStrExt;
use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};
#[cfg(windows)]
use tendril::fmt::{Format, WTF8};

use crate::path_component::{
    Component, Components, Prefix, PrefixExt, iter_after, rsplit_file_at_dot,
};
use crate::rel_path::{RelPath, StripPrefixError};

pub mod abs_path;
mod path_component;
pub mod rel_path;
mod windows_prefix;

pub fn try_from_bytes<'a, T>(bytes: &'a [u8]) -> anyhow::Result<T>
where
    T: From<&'a Path>,
{
    #[cfg(target_family = "wasm")]
    {
        std::str::from_utf8(bytes)
            .map(Path::new)
            .map(Into::into)
            .map_err(Into::into)
    }
    #[cfg(all(unix, not(target_family = "wasm")))]
    {
        Ok(T::from(Path::new(OsStr::from_bytes(bytes))))
    }
    #[cfg(windows)]
    {
        WTF8::validate(bytes)
            .then(|| {
                // Safety: bytes are valid WTF-8 sequence.
                T::from(Path::new(unsafe {
                    OsStr::from_encoded_bytes_unchecked(bytes)
                }))
            })
            .with_context(|| format!("Invalid WTF-8 sequence: {bytes:?}"))
    }
}

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

    pub fn is_absolute(&self, path: &Path) -> bool {
        match self {
            PathStyle::Unix => self.has_root(path),
            PathStyle::Windows => self.has_root(path) && self.components(path).prefix().is_some(),
        }
    }

    pub fn has_root(&self, path: &Path) -> bool {
        self.components(path).has_root()
    }

    pub fn is_windows(&self) -> bool {
        *self == PathStyle::Windows
    }

    pub fn is_posix(&self) -> bool {
        *self == PathStyle::Unix
    }

    pub fn join(self, left: impl AsRef<Path>, right: impl AsRef<Path>) -> Option<String> {
        let right = right.as_ref();
        if self.has_root(right) {
            return None;
        }
        let right = right.to_str()?;
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
        let right = self.normalize_separators(right);
        let right_string = right
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Path contains invalid UTF-8"))?;
        let joined = self
            .join(left, right_string)
            .ok_or_else(|| anyhow::anyhow!("Path must be relative: {right:?}"))?;
        Ok(self.normalize_lexically(&joined))
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

    pub fn normalize_separators(&self, path: impl AsRef<Path>) -> PathBuf {
        self.collect_components(self.components(path.as_ref()))
    }

    pub fn normalize_lexically(self, path: impl AsRef<Path>) -> PathBuf {
        let path = path.as_ref();
        let mut normalized: Vec<Component<'_>> = Vec::new();

        for component in self.components(path) {
            match component {
                Component::CurDir => {}
                Component::ParentDir => match normalized.last() {
                    Some(Component::Normal(_)) => {
                        normalized.pop();
                    }
                    // There is no parent above a root, so discard the attempt.
                    Some(Component::RootDir) => {}
                    _ => normalized.push(component),
                },
                Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                    normalized.push(component);
                }
            }
        }

        if normalized.is_empty() && !path.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            self.collect_components(normalized)
        }
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

    pub fn starts_with(&self, path: impl AsRef<Path>, prefix: impl AsRef<Path>) -> bool {
        iter_after(
            self.components(path.as_ref()),
            self.components(prefix.as_ref()),
        )
        .is_some()
    }

    pub fn strip_prefix<'a>(
        &self,
        path: &'a (impl AsRef<Path> + ?Sized),
        prefix: impl AsRef<Path>,
    ) -> Result<&'a Path, StripPrefixError> {
        iter_after(
            self.components(path.as_ref()),
            self.components(prefix.as_ref()),
        )
        .map(|c| c.as_path())
        .ok_or(StripPrefixError)
    }

    pub fn file_stem<'a>(&self, path: &'a Path) -> Option<&'a OsStr> {
        self.file_name(path)
            .map(rsplit_file_at_dot)
            .and_then(|(before, after)| before.or(after))
    }

    pub fn extension<'a>(&self, path: &'a Path) -> Option<&'a OsStr> {
        self.file_name(path)
            .map(rsplit_file_at_dot)
            .and_then(|(before, after)| before.and(after))
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

    /// Collects `components` into a `PathBuf`, replacing std's
    /// `impl<P: AsRef<Path>> FromIterator<P> for PathBuf` (which is implemented
    /// in terms of `PathBuf::push`) with a [`PathStyle`]-aware version.
    ///
    /// Unlike std's implementation, this appends each component's raw `OsStr`
    /// rather than pushing it as a standalone path. That distinction matters for
    /// Windows verbatim paths: `\\?\C:\a/b` has a single `Normal("a/b")`
    /// component, std lib is buggy here (rust-lang/rust#164467)
    fn collect_components<'a>(
        self,
        components: impl IntoIterator<Item = Component<'a>>,
    ) -> PathBuf {
        let mut result = OsString::new();
        let mut need_sep = false;

        for component in components {
            match component {
                Component::Prefix(_) | Component::RootDir => {
                    result.push(component.as_os_str(self));
                    need_sep = false;
                }
                _ => {
                    if need_sep {
                        result.push(self.primary_separator());
                    }
                    result.push(component.as_os_str(self));
                    need_sep = true;
                }
            }
        }

        PathBuf::from(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_normalize_separators() {
        let paths_and_normalized_unix = [
            ("", ""),
            ("/", "/"),
            ("a//b/", "a/b"),
            ("./a/./b", "./a/b"),
            ("a/../b", "a/../b"),
            (r"a\b", r"a\b"),
        ];
        for (path, normalized) in paths_and_normalized_unix {
            assert_eq!(
                PathStyle::Unix.normalize_separators(path).as_os_str(),
                OsStr::new(normalized),
                "normalize_separators({path:?})"
            );
        }

        let paths_and_normalized_windows = [
            ("C:/Users/user", r"C:\Users\user"),
            (r"C:/Users//.\user/", r"C:\Users\user"),
            ("./Users", r".\Users"),
            ("Users/../worktrees", r"Users\..\worktrees"),
            // no normalization inside Path prefixes
            (r"//server/share/dev", r"//server/share\dev"),
            (r"\\?\C:\Users/user", r"\\?\C:\Users/user"),
        ];
        for (path, normalized) in paths_and_normalized_windows {
            assert_eq!(
                PathStyle::Windows.normalize_separators(path).as_os_str(),
                OsStr::new(normalized),
                "normalize_separators({path:?})"
            );
        }
    }

    #[test]
    fn test_normalize_lexically() {
        let paths_and_normalized_unix = [
            ("", ""),
            ("/", "/"),
            ("/a/b/c", "/a/b/c"),
            ("/a/b/c/.././d", "/a/b/d"),
            ("/..", "/"),
            ("/../a/b", "/a/b"),
            ("./a", "a"),
            ("a//b/", "a/b"),
            ("a/..", "."),
            ("../..", "../.."),
            ("a/../../b", "../b"),
            ("a/..b/...", "a/..b/..."),
            (r"a\..\b", r"a\..\b"),
        ];
        for (path, normalized) in paths_and_normalized_unix {
            assert_eq!(
                PathStyle::Unix.normalize_lexically(path).as_os_str(),
                OsStr::new(normalized),
                "normalize_lexically({path:?})"
            );
        }

        let paths_and_normalized_windows = [
            (
                r"C:\Users\user\dev\..\worktrees",
                r"C:\Users\user\worktrees",
            ),
            (r"C:\Users\.\worktrees", r"C:\Users\worktrees"),
            (
                r"C:\Users\user\dev\sub\..\..\worktrees",
                r"C:\Users\user\worktrees",
            ),
            ("C:/Users/user/dev/../worktrees", r"C:\Users\user\worktrees"),
            (
                r"C:/Users\user/dev\..\worktrees",
                r"C:\Users\user\worktrees",
            ),
            (r"C:\Users/user\.\worktrees", r"C:\Users\user\worktrees"),
            (
                r"\\server\share\dev\..\worktrees",
                r"\\server\share\worktrees",
            ),
            (r"\\server\share\..\worktrees", r"\\server\share\worktrees"),
            // no normalization inside Path prefixes
            (
                r"//server/share/dev\sub\worktrees",
                r"//server/share\dev\sub\worktrees",
            ),
            (r"//./pipe/dev\..\worktrees", r"//./pipe\worktrees"),
            (r"c:/Users/user", r"c:\Users\user"),
            // `//?/` is not a verbatim prefix, so it must not become `\\?\`.
            (r"//?/C:/Users/user", r"//?/C:\Users\user"),
            (r"\dev\..\worktrees", r"\worktrees"),
            (r"dev\..\worktrees", r"worktrees"),
            (r"C:\..\worktrees", r"C:\worktrees"),
            (r"C:..\worktrees", r"C:..\worktrees"),
            (r"..\..\worktrees", r"..\..\worktrees"),
            (r"\\server\share", r"\\server\share\"),
            (r"\\?\C:\Users/user", r"\\?\C:\Users/user"),
        ];
        for (path, normalized) in paths_and_normalized_windows {
            assert_eq!(
                PathStyle::Windows.normalize_lexically(path).as_os_str(),
                OsStr::new(normalized),
                "normalize_lexically({path:?})"
            );
        }
    }

    #[test]
    fn test_unix_path_style_starts_with_and_strip_prefix() {
        let unix_paths_and_prefixes = [
            ("", "", true, Some("")),
            ("foo", "", true, Some("foo")),
            ("", "foo", false, None),
            ("foo", "foo", true, Some("")),
            ("foo/", "foo", true, Some("")),
            ("foo//", "foo", true, Some("")),
            ("foo///", "foo", true, Some("")),
            ("foo/.", "foo", true, Some("")),
            ("foo/./bar", "foo/bar", true, Some("")),
            ("foo/.//bar", "foo/bar", true, Some("")),
            ("foo//./bar", "foo/bar", true, Some("")),
            ("foo/bar", "foo", true, Some("bar")),
            ("foo/bar", "foobar", false, None),
            ("foo/bar/baz", "foo/bar", true, Some("baz")),
            ("foo/bar", "foo/bar/baz", false, None),
            ("./foo/bar/", ".", true, Some("foo/bar")),
            ("/etc/passwd", "/etc", true, Some("passwd")),
            ("/etc/passwd", "/etc/", true, Some("passwd")),
            ("/etc/passwd", "/etc/passwd", true, Some("")),
            ("/etc/passwd", "/etc/passwd/", true, Some("")),
            ("/etc/passwd", "/etc/passwd///", true, Some("")),
            ("/etc/passwd", "/e", false, None),
            ("/etc/passwd", "/etc/passwd.txt", false, None),
            ("/etc/foo.rs", "/etc/foo", false, None),
            ("/foo/bar/baz", "/", true, Some("foo/bar/baz")),
            ("/foo/bar/baz", "", true, Some("/foo/bar/baz")),
            ("/foo/bar//baz", "/foo/bar/", true, Some("baz")),
            ("/foo/barbaz", "/foo/bar", false, None),
        ];

        for (path, prefix, expected_starts_with, expected_stripped) in unix_paths_and_prefixes {
            assert_eq!(
                PathStyle::Unix.starts_with(Path::new(path), Path::new(prefix)),
                expected_starts_with,
                "{path:?}.starts_with({prefix:?})"
            );
            assert_eq!(
                PathStyle::Unix
                    .strip_prefix(Path::new(path), Path::new(prefix))
                    .ok(),
                expected_stripped.map(Path::new),
                "{path:?}.strip_prefix({prefix:?})"
            );
        }
    }

    #[test]
    fn test_windows_path_style_starts_with_and_strip_prefix() {
        let windows_paths_and_prefixes = [
            (
                r"C:\src\rust\cargo-test\test\Cargo.toml",
                r"c:\src\rust\cargo-test\test",
                true,
                Some("Cargo.toml"),
            ),
            (r"C:\foo\.\bar.txt", r"C:\foo\bar.txt", true, Some("")),
            (r"C:\foo\.", r"C:\foo", true, Some("")),
            (r"\\server\share", r"\\server", false, None),
            (r"\\server\share\foo.txt", r"\\server", false, None),
            (
                r"\\server\share\foo.txt",
                r"\\server\share",
                true,
                Some("foo.txt"),
            ),
            (
                r"\\server\share\foo.txt",
                r"\\server\share\",
                true,
                Some("foo.txt"),
            ),
            (
                r"\\server\share\foo.txt",
                r"\\server\share/",
                true,
                Some("foo.txt"),
            ),
            (r"\\server\share\foo.txt", r"\\server\other", false, None),
            (r"\\server\share\foo.txt", r"\\SERVER\share", false, None),
            (r"\\?\C:\foo\.\bar.txt", r"\\?\C:\foo\bar.txt", false, None),
            (r"\\?\C:\foo.txt", r"\\?\C:\", true, Some("foo.txt")),
            (
                r"\\?\UNC\server\share\foo.txt",
                r"\\?\UNC\server\share",
                true,
                Some(r"\foo.txt"),
            ),
            (
                r"\\?\UNC\server\share\foo.txt",
                r"\\?\UNC\server\",
                false,
                None,
            ),
            (r"\\?\bar\foo.txt", r"\\?\bar", true, Some(r"\foo.txt")),
            (r"\\?\C:\foo/bar.txt", r"\\?\C:\foo", false, None),
            (r"C:\foo\bar.txt", r"C:\foo", true, Some("bar.txt")),
            (r"C:\foo\bar.txt", r"C:\foo\", true, Some("bar.txt")),
            (r"C:\foo\bar.txt", r"C:\", true, Some(r"foo\bar.txt")),
            (r"C:\foo\bar.txt", "", true, Some(r"C:\foo\bar.txt")),
            (r"C:\foo\\bar.txt", r"C:\foo\", true, Some("bar.txt")),
            (r"C:\foobar.txt", r"C:\foo", false, None),
            (r"C:\foo/bar.txt", r"C:\foo", true, Some("bar.txt")),
            (r"C:\foo/bar.txt", r"C:\foo\", true, Some("bar.txt")),
            (r"C:\foo/bar.txt", r"C:\foo/", true, Some("bar.txt")),
        ];

        for (path, prefix, expected_starts_with, expected_stripped) in windows_paths_and_prefixes {
            assert_eq!(
                PathStyle::Windows.starts_with(Path::new(path), Path::new(prefix)),
                expected_starts_with,
                "{path:?}.starts_with({prefix:?})"
            );
            assert_eq!(
                PathStyle::Windows
                    .strip_prefix(Path::new(path), Path::new(prefix))
                    .ok(),
                expected_stripped.map(Path::new),
                "{path:?}.strip_prefix({prefix:?})"
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
    fn test_path_style_file_stem_and_extension() {
        let unix_paths_and_stems_and_extensions = [
            ("foo", Some("foo"), None),
            ("foo.", Some("foo"), Some("")),
            (".foo", Some(".foo"), None),
            ("foo.txt", Some("foo"), Some("txt")),
            ("/tmp/archive.tar.gz", Some("archive.tar"), Some("gz")),
            ("foo.bar.", Some("foo.bar"), Some("")),
            (".x.y.z", Some(".x.y"), Some("z")),
            ("a/.rustfmt.toml", Some(".rustfmt"), Some("toml")),
            ("a.b/c.md", Some("c"), Some("md")),
            ("", None, None),
            ("/", None, None),
            (".", None, None),
            ("..", None, None),
            ("foo/.", Some("foo"), None),
            ("foo/..", None, None),
            ("/foo/bar/", Some("bar"), None),
            ("/etc/passwd", Some("passwd"), None),
        ];

        for (path, expected_stem, expected_extension) in unix_paths_and_stems_and_extensions {
            let path = Path::new(path);
            assert_eq!(
                PathStyle::Unix.file_stem(path),
                expected_stem.map(OsStr::new),
                "file_stem({path:?})"
            );
            assert_eq!(
                PathStyle::Unix.extension(path),
                expected_extension.map(OsStr::new),
                "extension({path:?})"
            );
        }

        let windows_paths_and_stems_and_extensions = [
            (
                r"C:\Users\user\Documents\report.docx",
                Some("report"),
                Some("docx"),
            ),
            (
                r"C:\Users\user\Downloads\archive.tar.gz",
                Some("archive.tar"),
                Some("gz"),
            ),
            (r"C:\Users\user\.gitignore", Some(".gitignore"), None),
            (r"C:\Users\user\config.", Some("config"), Some("")),
            (r"C:\Users\user\README", Some("README"), None),
            (r"D:\projects\my.app\src\main.rs", Some("main"), Some("rs")),
            ("C:/Users/user/report.txt", Some("report"), Some("txt")),
            (
                r"\\server\share\reports\2024.summary.xlsx",
                Some("2024.summary"),
                Some("xlsx"),
            ),
            (r"\\?\C:\Users\user\report.txt", Some("report"), Some("txt")),
            (r"C:\Users\user\.", Some("user"), None),
            (r"C:\Users\user\..", None, None),
        ];

        for (path, expected_stem, expected_extension) in windows_paths_and_stems_and_extensions {
            let path = Path::new(path);
            assert_eq!(
                PathStyle::Windows.file_stem(path),
                expected_stem.map(OsStr::new),
                "file_stem({path:?})"
            );
            assert_eq!(
                PathStyle::Windows.extension(path),
                expected_extension.map(OsStr::new),
                "extension({path:?})"
            );
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
