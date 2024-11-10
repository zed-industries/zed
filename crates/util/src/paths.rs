use std::cmp;
use std::sync::OnceLock;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::NumericPrefixWithSuffix;

/// Returns the path to the user's home directory.
pub fn home_dir() -> &'static PathBuf {
    static HOME_DIR: OnceLock<PathBuf> = OnceLock::new();
    HOME_DIR.get_or_init(|| dirs::home_dir().expect("failed to determine home directory"))
}

pub trait PathExt {
    fn compact(&self) -> PathBuf;
    fn icon_stem_or_suffix(&self) -> Option<&str>;
    fn extension_or_hidden_file_name(&self) -> Option<&str>;
    fn try_from_bytes<'a>(bytes: &'a [u8]) -> anyhow::Result<Self>
    where
        Self: From<&'a Path>,
    {
        #[cfg(unix)]
        {
            use std::os::unix::prelude::OsStrExt;
            Ok(Self::from(Path::new(OsStr::from_bytes(bytes))))
        }
        #[cfg(windows)]
        {
            use anyhow::anyhow;
            use tendril::fmt::{Format, WTF8};
            WTF8::validate(bytes)
                .then(|| {
                    // Safety: bytes are valid WTF-8 sequence.
                    Self::from(Path::new(unsafe {
                        OsStr::from_encoded_bytes_unchecked(bytes)
                    }))
                })
                .ok_or_else(|| anyhow!("Invalid WTF-8 sequence: {bytes:?}"))
        }
    }
}

impl<T: AsRef<Path>> PathExt for T {
    /// Compacts a given file path by replacing the user's home directory
    /// prefix with a tilde (`~`).
    ///
    /// # Returns
    ///
    /// * A `PathBuf` containing the compacted file path. If the input path
    ///   does not have the user's home directory prefix, or if we are not on
    ///   Linux or macOS, the original path is returned unchanged.
    fn compact(&self) -> PathBuf {
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            match self.as_ref().strip_prefix(home_dir().as_path()) {
                Ok(relative_path) => {
                    let mut shortened_path = PathBuf::new();
                    shortened_path.push("~");
                    shortened_path.push(relative_path);
                    shortened_path
                }
                Err(_) => self.as_ref().to_path_buf(),
            }
        } else {
            self.as_ref().to_path_buf()
        }
    }

    /// Returns either the suffix if available, or the file stem otherwise to determine which file icon to use
    fn icon_stem_or_suffix(&self) -> Option<&str> {
        let path = self.as_ref();
        let file_name = path.file_name()?.to_str()?;
        if file_name.starts_with('.') {
            return file_name.strip_prefix('.');
        }

        path.extension()
            .and_then(|e| e.to_str())
            .or_else(|| path.file_stem()?.to_str())
    }

    /// Returns a file's extension or, if the file is hidden, its name without the leading dot
    fn extension_or_hidden_file_name(&self) -> Option<&str> {
        if let Some(extension) = self.as_ref().extension() {
            return extension.to_str();
        }

        self.as_ref().file_name()?.to_str()?.split('.').last()
    }
}

/// A delimiter to use in `path_query:row_number:column_number` strings parsing.
pub const FILE_ROW_COLUMN_DELIMITER: char = ':';

const ROW_COL_CAPTURE_REGEX: &str = r"(?x)
    ([^\(]+)(?:
        \((\d+),(\d+)\) # filename(row,column)
        |
        \((\d+)\)()     # filename(row)
    )
    |
    (.+?)(?:
        \:+(\d+)\:(\d+)\:*$  # filename:row:column
        |
        \:+(\d+)\:*()$       # filename:row
        |
        \:*()()$             # filename:
    )";

/// A representation of a path-like string with optional row and column numbers.
/// Matching values example: `te`, `test.rs:22`, `te:22:5`, `test.c(22)`, `test.c(22,5)`etc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct PathWithPosition {
    pub path: PathBuf,
    pub row: Option<u32>,
    // Absent if row is absent.
    pub column: Option<u32>,
}

impl PathWithPosition {
    /// Returns a PathWithPosition from a path.
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            path,
            row: None,
            column: None,
        }
    }

    /// Parses a string that possibly has `:row:column` or `(row, column)` suffix.
    /// Parenthesis format is used by [MSBuild](https://learn.microsoft.com/en-us/visualstudio/msbuild/msbuild-diagnostic-format-for-tasks) compatible tools
    /// Ignores trailing `:`s, so `test.rs:22:` is parsed as `test.rs:22`.
    /// If the suffix parsing fails, the whole string is parsed as a path.
    ///
    /// Be mindful that `test_file:10:1:` is a valid posix filename.
    /// `PathWithPosition` class assumes that the ending position-like suffix is **not** part of the filename.
    ///
    /// # Examples
    ///
    /// ```
    /// # use util::paths::PathWithPosition;
    /// # use std::path::PathBuf;
    /// assert_eq!(PathWithPosition::parse_str("test_file"), PathWithPosition {
    ///     path: PathBuf::from("test_file"),
    ///     row: None,
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file:10"), PathWithPosition {
    ///     path: PathBuf::from("test_file"),
    ///     row: Some(10),
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs"),
    ///     row: None,
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs:1"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs"),
    ///     row: Some(1),
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs:1:2"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs"),
    ///     row: Some(1),
    ///     column: Some(2),
    /// });
    /// ```
    ///
    /// # Expected parsing results when encounter ill-formatted inputs.
    /// ```
    /// # use util::paths::PathWithPosition;
    /// # use std::path::PathBuf;
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs:a"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs:a"),
    ///     row: None,
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs:a:b"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs:a:b"),
    ///     row: None,
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs::"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs"),
    ///     row: None,
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs::1"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs"),
    ///     row: Some(1),
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs:1::"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs"),
    ///     row: Some(1),
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs::1:2"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs"),
    ///     row: Some(1),
    ///     column: Some(2),
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs:1::2"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs:1"),
    ///     row: Some(2),
    ///     column: None,
    /// });
    /// assert_eq!(PathWithPosition::parse_str("test_file.rs:1:2:3"), PathWithPosition {
    ///     path: PathBuf::from("test_file.rs:1"),
    ///     row: Some(2),
    ///     column: Some(3),
    /// });
    /// ```
    pub fn parse_str(s: &str) -> Self {
        let trimmed = s.trim();
        let path = Path::new(trimmed);
        let maybe_file_name_with_row_col = path.file_name().unwrap_or_default().to_string_lossy();
        if maybe_file_name_with_row_col.is_empty() {
            return Self {
                path: Path::new(s).to_path_buf(),
                row: None,
                column: None,
            };
        }

        // Let's avoid repeated init cost on this. It is subject to thread contention, but
        // so far this code isn't called from multiple hot paths. Getting contention here
        // in the future seems unlikely.
        static SUFFIX_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(ROW_COL_CAPTURE_REGEX).unwrap());
        match SUFFIX_RE
            .captures(&maybe_file_name_with_row_col)
            .map(|caps| caps.extract())
        {
            Some((_, [file_name, maybe_row, maybe_column])) => {
                let row = maybe_row.parse::<u32>().ok();
                let column = maybe_column.parse::<u32>().ok();

                let suffix_length = maybe_file_name_with_row_col.len() - file_name.len();
                let path_without_suffix = &trimmed[..trimmed.len() - suffix_length];

                Self {
                    path: Path::new(path_without_suffix).to_path_buf(),
                    row,
                    column,
                }
            }
            None => Self {
                path: Path::new(s).to_path_buf(),
                row: None,
                column: None,
            },
        }
    }

    pub fn map_path<E>(
        self,
        mapping: impl FnOnce(PathBuf) -> Result<PathBuf, E>,
    ) -> Result<PathWithPosition, E> {
        Ok(PathWithPosition {
            path: mapping(self.path)?,
            row: self.row,
            column: self.column,
        })
    }

    pub fn to_string(&self, path_to_string: impl Fn(&PathBuf) -> String) -> String {
        let path_string = path_to_string(&self.path);
        if let Some(row) = self.row {
            if let Some(column) = self.column {
                format!("{path_string}:{row}:{column}")
            } else {
                format!("{path_string}:{row}")
            }
        } else {
            path_string
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PathMatcher {
    sources: Vec<String>,
    glob: GlobSet,
}

// impl std::fmt::Display for PathMatcher {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.sources.fmt(f)
//     }
// }

impl PartialEq for PathMatcher {
    fn eq(&self, other: &Self) -> bool {
        self.sources.eq(&other.sources)
    }
}

impl Eq for PathMatcher {}

impl PathMatcher {
    pub fn new(globs: &[String]) -> Result<Self, globset::Error> {
        let globs = globs
            .iter()
            .map(|glob| Glob::new(glob))
            .collect::<Result<Vec<_>, _>>()?;
        let sources = globs.iter().map(|glob| glob.glob().to_owned()).collect();
        let mut glob_builder = GlobSetBuilder::new();
        for single_glob in globs {
            glob_builder.add(single_glob);
        }
        let glob = glob_builder.build()?;
        Ok(PathMatcher { glob, sources })
    }

    pub fn sources(&self) -> &[String] {
        &self.sources
    }

    pub fn is_match<P: AsRef<Path>>(&self, other: P) -> bool {
        let other_path = other.as_ref();
        self.sources.iter().any(|source| {
            let as_bytes = other_path.as_os_str().as_encoded_bytes();
            as_bytes.starts_with(source.as_bytes()) || as_bytes.ends_with(source.as_bytes())
        }) || self.glob.is_match(other_path)
            || self.check_with_end_separator(other_path)
    }

    fn check_with_end_separator(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        let separator = std::path::MAIN_SEPARATOR_STR;
        if path_str.ends_with(separator) {
            false
        } else {
            self.glob.is_match(path_str.to_string() + separator)
        }
    }
}

pub fn compare_paths(
    (path_a, a_is_file): (&Path, bool),
    (path_b, b_is_file): (&Path, bool),
) -> cmp::Ordering {
    let mut components_a = path_a.components().peekable();
    let mut components_b = path_b.components().peekable();
    loop {
        match (components_a.next(), components_b.next()) {
            (Some(component_a), Some(component_b)) => {
                let a_is_file = components_a.peek().is_none() && a_is_file;
                let b_is_file = components_b.peek().is_none() && b_is_file;
                let ordering = a_is_file.cmp(&b_is_file).then_with(|| {
                    let path_a = Path::new(component_a.as_os_str());
                    let path_string_a = if a_is_file {
                        path_a.file_stem()
                    } else {
                        path_a.file_name()
                    }
                    .map(|s| s.to_string_lossy());
                    let num_and_remainder_a = path_string_a
                        .as_deref()
                        .map(NumericPrefixWithSuffix::from_numeric_prefixed_str);

                    let path_b = Path::new(component_b.as_os_str());
                    let path_string_b = if b_is_file {
                        path_b.file_stem()
                    } else {
                        path_b.file_name()
                    }
                    .map(|s| s.to_string_lossy());
                    let num_and_remainder_b = path_string_b
                        .as_deref()
                        .map(NumericPrefixWithSuffix::from_numeric_prefixed_str);

                    num_and_remainder_a.cmp(&num_and_remainder_b)
                });
                if !ordering.is_eq() {
                    return ordering;
                }
            }
            (Some(_), None) => break cmp::Ordering::Greater,
            (None, Some(_)) => break cmp::Ordering::Less,
            (None, None) => break cmp::Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_paths_with_dots() {
        let mut paths = vec![
            (Path::new("test_dirs"), false),
            (Path::new("test_dirs/1.46"), false),
            (Path::new("test_dirs/1.46/bar_1"), true),
            (Path::new("test_dirs/1.46/bar_2"), true),
            (Path::new("test_dirs/1.45"), false),
            (Path::new("test_dirs/1.45/foo_2"), true),
            (Path::new("test_dirs/1.45/foo_1"), true),
        ];
        paths.sort_by(|&a, &b| compare_paths(a, b));
        assert_eq!(
            paths,
            vec![
                (Path::new("test_dirs"), false),
                (Path::new("test_dirs/1.45"), false),
                (Path::new("test_dirs/1.45/foo_1"), true),
                (Path::new("test_dirs/1.45/foo_2"), true),
                (Path::new("test_dirs/1.46"), false),
                (Path::new("test_dirs/1.46/bar_1"), true),
                (Path::new("test_dirs/1.46/bar_2"), true),
            ]
        );
        let mut paths = vec![
            (Path::new("root1/one.txt"), true),
            (Path::new("root1/one.two.txt"), true),
        ];
        paths.sort_by(|&a, &b| compare_paths(a, b));
        assert_eq!(
            paths,
            vec![
                (Path::new("root1/one.txt"), true),
                (Path::new("root1/one.two.txt"), true),
            ]
        );
    }

    #[test]
    fn compare_paths_case_semi_sensitive() {
        let mut paths = vec![
            (Path::new("test_DIRS"), false),
            (Path::new("test_DIRS/foo_1"), true),
            (Path::new("test_DIRS/foo_2"), true),
            (Path::new("test_DIRS/bar"), true),
            (Path::new("test_DIRS/BAR"), true),
            (Path::new("test_dirs"), false),
            (Path::new("test_dirs/foo_1"), true),
            (Path::new("test_dirs/foo_2"), true),
            (Path::new("test_dirs/bar"), true),
            (Path::new("test_dirs/BAR"), true),
        ];
        paths.sort_by(|&a, &b| compare_paths(a, b));
        assert_eq!(
            paths,
            vec![
                (Path::new("test_dirs"), false),
                (Path::new("test_dirs/bar"), true),
                (Path::new("test_dirs/BAR"), true),
                (Path::new("test_dirs/foo_1"), true),
                (Path::new("test_dirs/foo_2"), true),
                (Path::new("test_DIRS"), false),
                (Path::new("test_DIRS/bar"), true),
                (Path::new("test_DIRS/BAR"), true),
                (Path::new("test_DIRS/foo_1"), true),
                (Path::new("test_DIRS/foo_2"), true),
            ]
        );
    }

    #[test]
    fn path_with_position_parse_posix_path() {
        // Test POSIX filename edge cases
        // Read more at https://en.wikipedia.org/wiki/Filename
        assert_eq!(
            PathWithPosition::parse_str(" test_file"),
            PathWithPosition {
                path: PathBuf::from("test_file"),
                row: None,
                column: None
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("a:bc:.zip:1"),
            PathWithPosition {
                path: PathBuf::from("a:bc:.zip"),
                row: Some(1),
                column: None
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("one.second.zip:1"),
            PathWithPosition {
                path: PathBuf::from("one.second.zip"),
                row: Some(1),
                column: None
            }
        );

        // Trim off trailing `:`s for otherwise valid input.
        assert_eq!(
            PathWithPosition::parse_str("test_file:10:1:"),
            PathWithPosition {
                path: PathBuf::from("test_file"),
                row: Some(10),
                column: Some(1)
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("test_file.rs:"),
            PathWithPosition {
                path: PathBuf::from("test_file.rs"),
                row: None,
                column: None
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("test_file.rs:1:"),
            PathWithPosition {
                path: PathBuf::from("test_file.rs"),
                row: Some(1),
                column: None
            }
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn path_with_position_parse_posix_path_with_suffix() {
        assert_eq!(
            PathWithPosition::parse_str("app-editors:zed-0.143.6:20240710-201212.log:34:"),
            PathWithPosition {
                path: PathBuf::from("app-editors:zed-0.143.6:20240710-201212.log"),
                row: Some(34),
                column: None,
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("crates/file_finder/src/file_finder.rs:1902:13:"),
            PathWithPosition {
                path: PathBuf::from("crates/file_finder/src/file_finder.rs"),
                row: Some(1902),
                column: Some(13),
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("crate/utils/src/test:today.log:34"),
            PathWithPosition {
                path: PathBuf::from("crate/utils/src/test:today.log"),
                row: Some(34),
                column: None,
            }
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn path_with_position_parse_windows_path() {
        assert_eq!(
            PathWithPosition::parse_str("crates\\utils\\paths.rs"),
            PathWithPosition {
                path: PathBuf::from("crates\\utils\\paths.rs"),
                row: None,
                column: None
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("C:\\Users\\someone\\test_file.rs"),
            PathWithPosition {
                path: PathBuf::from("C:\\Users\\someone\\test_file.rs"),
                row: None,
                column: None
            }
        );
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn path_with_position_parse_windows_path_with_suffix() {
        assert_eq!(
            PathWithPosition::parse_str("crates\\utils\\paths.rs:101"),
            PathWithPosition {
                path: PathBuf::from("crates\\utils\\paths.rs"),
                row: Some(101),
                column: None
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("\\\\?\\C:\\Users\\someone\\test_file.rs:1:20"),
            PathWithPosition {
                path: PathBuf::from("\\\\?\\C:\\Users\\someone\\test_file.rs"),
                row: Some(1),
                column: Some(20)
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("C:\\Users\\someone\\test_file.rs(1902,13)"),
            PathWithPosition {
                path: PathBuf::from("C:\\Users\\someone\\test_file.rs"),
                row: Some(1902),
                column: Some(13)
            }
        );

        // Trim off trailing `:`s for otherwise valid input.
        assert_eq!(
            PathWithPosition::parse_str("\\\\?\\C:\\Users\\someone\\test_file.rs:1902:13:"),
            PathWithPosition {
                path: PathBuf::from("\\\\?\\C:\\Users\\someone\\test_file.rs"),
                row: Some(1902),
                column: Some(13)
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("\\\\?\\C:\\Users\\someone\\test_file.rs:1902:13:15:"),
            PathWithPosition {
                path: PathBuf::from("\\\\?\\C:\\Users\\someone\\test_file.rs:1902"),
                row: Some(13),
                column: Some(15)
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("\\\\?\\C:\\Users\\someone\\test_file.rs:1902:::15:"),
            PathWithPosition {
                path: PathBuf::from("\\\\?\\C:\\Users\\someone\\test_file.rs:1902"),
                row: Some(15),
                column: None
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("\\\\?\\C:\\Users\\someone\\test_file.rs(1902,13):"),
            PathWithPosition {
                path: PathBuf::from("\\\\?\\C:\\Users\\someone\\test_file.rs"),
                row: Some(1902),
                column: Some(13),
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("\\\\?\\C:\\Users\\someone\\test_file.rs(1902):"),
            PathWithPosition {
                path: PathBuf::from("\\\\?\\C:\\Users\\someone\\test_file.rs"),
                row: Some(1902),
                column: None,
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("C:\\Users\\someone\\test_file.rs:1902:13:"),
            PathWithPosition {
                path: PathBuf::from("C:\\Users\\someone\\test_file.rs"),
                row: Some(1902),
                column: Some(13),
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("C:\\Users\\someone\\test_file.rs(1902,13):"),
            PathWithPosition {
                path: PathBuf::from("C:\\Users\\someone\\test_file.rs"),
                row: Some(1902),
                column: Some(13),
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("C:\\Users\\someone\\test_file.rs(1902):"),
            PathWithPosition {
                path: PathBuf::from("C:\\Users\\someone\\test_file.rs"),
                row: Some(1902),
                column: None,
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("crates/utils/paths.rs:101"),
            PathWithPosition {
                path: PathBuf::from("crates\\utils\\paths.rs"),
                row: Some(101),
                column: None,
            }
        );
    }

    #[test]
    fn test_path_compact() {
        let path: PathBuf = [
            home_dir().to_string_lossy().to_string(),
            "some_file.txt".to_string(),
        ]
        .iter()
        .collect();
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            assert_eq!(path.compact().to_str(), Some("~/some_file.txt"));
        } else {
            assert_eq!(path.compact().to_str(), path.to_str());
        }
    }

    #[test]
    fn test_icon_stem_or_suffix() {
        // No dots in name
        let path = Path::new("/a/b/c/file_name.rs");
        assert_eq!(path.icon_stem_or_suffix(), Some("rs"));

        // Single dot in name
        let path = Path::new("/a/b/c/file.name.rs");
        assert_eq!(path.icon_stem_or_suffix(), Some("rs"));

        // No suffix
        let path = Path::new("/a/b/c/file");
        assert_eq!(path.icon_stem_or_suffix(), Some("file"));

        // Multiple dots in name
        let path = Path::new("/a/b/c/long.file.name.rs");
        assert_eq!(path.icon_stem_or_suffix(), Some("rs"));

        // Hidden file, no extension
        let path = Path::new("/a/b/c/.gitignore");
        assert_eq!(path.icon_stem_or_suffix(), Some("gitignore"));

        // Hidden file, with extension
        let path = Path::new("/a/b/c/.eslintrc.js");
        assert_eq!(path.icon_stem_or_suffix(), Some("eslintrc.js"));
    }

    #[test]
    fn test_extension_or_hidden_file_name() {
        // No dots in name
        let path = Path::new("/a/b/c/file_name.rs");
        assert_eq!(path.extension_or_hidden_file_name(), Some("rs"));

        // Single dot in name
        let path = Path::new("/a/b/c/file.name.rs");
        assert_eq!(path.extension_or_hidden_file_name(), Some("rs"));

        // Multiple dots in name
        let path = Path::new("/a/b/c/long.file.name.rs");
        assert_eq!(path.extension_or_hidden_file_name(), Some("rs"));

        // Hidden file, no extension
        let path = Path::new("/a/b/c/.gitignore");
        assert_eq!(path.extension_or_hidden_file_name(), Some("gitignore"));

        // Hidden file, with extension
        let path = Path::new("/a/b/c/.eslintrc.js");
        assert_eq!(path.extension_or_hidden_file_name(), Some("js"));
    }

    #[test]
    fn edge_of_glob() {
        let path = Path::new("/work/node_modules");
        let path_matcher = PathMatcher::new(&["**/node_modules/**".to_owned()]).unwrap();
        assert!(
            path_matcher.is_match(path),
            "Path matcher should match {path:?}"
        );
    }

    #[test]
    fn project_search() {
        let path = Path::new("/Users/someonetoignore/work/zed/zed.dev/node_modules");
        let path_matcher = PathMatcher::new(&["**/node_modules/**".to_owned()]).unwrap();
        assert!(
            path_matcher.is_match(path),
            "Path matcher should match {path:?}"
        );
    }
}
