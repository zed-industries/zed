use std::sync::OnceLock;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};

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

/// A representation of a path-like string with optional row and column numbers.
/// Matching values example: `te`, `test.rs:22`, `te:22:5`, etc.
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
    /// Parses a string that possibly has `:row:column` suffix.
    /// Ignores trailing `:`s, so `test.rs:22:` is parsed as `test.rs:22`.
    /// If the suffix parsing fails, the whole string is parsed as a path.
    pub fn parse_str(s: &str) -> Self {
        let path = Path::new(s.trim());
        if path.extension().is_some() {
            Self::parse_extension_or_hidden_file(path)
        } else {
            Self::parse_filename(path)
        }
    }
    /// Parses a filename for row and column.
    /// Example strings:  test:1:2 => (test, 1, 2)
    fn parse_filename(path: &Path) -> Self {
        match path.file_name().and_then(OsStr::to_str) {
            None => Self {
                path: path.to_path_buf(),
                row: None,
                column: None,
            },
            Some(filename) => {
                let split_name: Vec<&str> = filename.split(":").collect();
                if split_name.len() < 1 {
                    return Self {
                        path: path.to_path_buf(),
                        row: None,
                        column: None,
                    };
                }

                let col_row: Vec<Option<u32>> = split_name
                    .iter()
                    .rev()
                    .take(2)
                    .copied()
                    .map(|val| val.parse::<u32>().ok())
                    .collect();
                let maybe_column = col_row.get(0).copied().unwrap_or_default();
                let maybe_row = col_row.get(1).copied().unwrap_or_default();
                let (row, column) = match (maybe_row, maybe_column) {
                    (None, None) => (None, None),
                    // example: test:a:1 - assuming column as a row
                    (None, Some(col)) => (Some(col), None),
                    // example: test:1a:b
                    (Some(_), None) => (None, None),
                    (Some(row), Some(col)) => (Some(row), Some(col)),
                };
                let name_parts: Vec<&str> = split_name
                    .iter()
                    .rev()
                    .skip(row.is_some() as usize + column.is_some() as usize)
                    .rev()
                    .copied()
                    .collect();
                let new_filename = name_parts.join(":");

                let path = {
                    let path_str = path.to_str().unwrap_or_default();
                    let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
                    let extension_len = if extension.len() > 0 {
                        // plus dot char
                        extension.len() + 1
                    } else {
                        0
                    };
                    let mut new_path =
                        path_str[0..path_str.len() - filename.len() - extension_len].to_string();
                    let new_filename = if extension.len() > 0 {
                        format!("{new_filename}.{extension}")
                    } else {
                        new_filename
                    };
                    new_path.push_str(&new_filename);

                    PathBuf::from(new_path)
                };

                Self { path, row, column }
            }
        }
    }

    fn parse_extension_or_hidden_file(path: &Path) -> Self {
        match path.extension().and_then(OsStr::to_str) {
            None => Self {
                path: path.to_path_buf(),
                row: None,
                column: None,
            },
            Some(extension) => {
                // example: rs:1:2:321 => [rs, 1, 2]
                let ext_row_col: Vec<&str> = extension
                    .trim()
                    .split(":")
                    // take only extension name, row and column
                    .take(3)
                    .collect();

                if ext_row_col.len() == 1 {
                    return Self {
                        path: path.to_path_buf(),
                        row: None,
                        column: None,
                    };
                }

                let row = ext_row_col
                    .get(1)
                    .copied()
                    .unwrap_or_default()
                    .parse::<u32>()
                    .ok();
                let column = if row.is_some() {
                    ext_row_col
                        .get(2)
                        .copied()
                        .unwrap_or_default()
                        .parse::<u32>()
                        .ok()
                } else {
                    None
                };
                let path = {
                    let path_str = path.to_str().unwrap_or_default();
                    let mut new_path = path_str[0..path_str.len() - extension.len()].to_string();
                    new_path.push_str(ext_row_col[0]);

                    PathBuf::from(new_path)
                };

                Self { path, row, column }
            }
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
            .into_iter()
            .map(|glob| Glob::new(&glob))
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
            self.glob.is_match(path)
        } else {
            self.glob.is_match(path_str.to_string() + separator)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_with_position_parsing_positive() {
        let input_and_expected = [
            (".env", ".env", None, None),
            (".env:1", ".env", Some(1), None),
            (".env:1:2", ".env", Some(1), Some(2)),
            (".env.prod:1:2", ".env.prod", Some(1), Some(2)),
            ("test_file.rs", "test_file.rs", None, None),
            ("test_file.rs:1", "test_file.rs", Some(1), None),
            ("test_file.rs:1:2", "test_file.rs", Some(1), Some(2)),
        ];

        for (input, expected_path, row, column) in input_and_expected {
            let actual = PathWithPosition::parse_str(input);
            assert_eq!(
                actual,
                PathWithPosition {
                    path: PathBuf::from(expected_path),
                    row,
                    column
                },
                "For positive case input str '{input}', got a parse mismatch"
            );
        }
    }

    #[test]
    fn path_with_position_parsing_negative() {
        for (input, expected_file_name, row, column) in [
            ("/test/test_file.rs:a", "/test/test_file.rs", None, None),
            ("test_file.:a", "test_file.", None, None),
            ("test_file.rs:a", "test_file.rs", None, None),
            ("test_file.rs:a:b", "test_file.rs", None, None),
            ("test_file.rs::", "test_file.rs", None, None),
            ("test_file.rs::1", "test_file.rs", None, None),
            ("test_file.rs:1::", "test_file.rs", Some(1), None),
            ("test_file.rs:1::2", "test_file.rs", Some(1), None),
            ("test_file.rs::1:2", "test_file.rs", None, None),
            ("test_file.rs:1::2", "test_file.rs", Some(1), None),
            ("test_file.rs:1:2:3", "test_file.rs", Some(1), Some(2)),
            (
                "/test/test_file.rs:1:2:3",
                "/test/test_file.rs",
                Some(1),
                Some(2),
            ),
            (
                "test_file:test.rs:1:2:3",
                "test_file:test.rs",
                Some(1),
                Some(2),
            ),
            ("test_file:test.rs:a:", "test_file:test.rs", None, None),
            ("test_file:a", "test_file:a", None, None),
            ("test_file:a:b", "test_file:a:b", None, None),
            ("test_file::", "test_file::", None, None),
            ("test_file::1", "test_file:", Some(1), None),
            ("test_file:1::", "test_file:1::", None, None),
            ("test_file::1:2", "test_file:", Some(1), Some(2)),
            ("test_file:1::2", "test_file:1:", Some(2), None),
            ("test_file:1:2:3", "test_file:1", Some(2), Some(3)),
            ("test_file:test:1:2:3", "test_file:test:1", Some(2), Some(3)),
            ("test_file:test:1:2:3", "test_file:test:1", Some(2), Some(3)),
            ("test_file:test:a:1::", "test_file:test:a:1::", None, None),
            ("test_file:test:a:1:a", "test_file:test:a:1:a", None, None),
        ] {
            let actual = PathWithPosition::parse_str(input);
            assert_eq!(
                actual,
                PathWithPosition {
                    path: PathBuf::from(expected_file_name),
                    row,
                    column,
                },
                "For negative case input str '{input}', got a parse mismatch"
            );
        }
    }

    // Trim off trailing `:`s for otherwise valid input.
    #[test]
    fn path_with_position_parsing_special() {
        #[cfg(not(target_os = "windows"))]
        let input_and_expected = [
            (
                "test_file.rs:",
                PathWithPosition {
                    path: PathBuf::from("test_file.rs"),
                    row: None,
                    column: None,
                },
            ),
            (
                "test_file.rs:1:",
                PathWithPosition {
                    path: PathBuf::from("test_file.rs"),
                    row: Some(1),
                    column: None,
                },
            ),
            (
                "crates/file_finder/src/file_finder.rs:1902:13:",
                PathWithPosition {
                    path: PathBuf::from("crates/file_finder/src/file_finder.rs"),
                    row: Some(1902),
                    column: Some(13),
                },
            ),
        ];

        #[cfg(target_os = "windows")]
        let input_and_expected = [
            (
                "test_file.rs:",
                PathWithPosition {
                    path: PathBuf::from("test_file.rs"),
                    row: None,
                    column: None,
                },
            ),
            (
                "test_file.rs:1:",
                PathWithPosition {
                    path: PathBuf::from("test_file.rs"),
                    row: Some(1),
                    column: None,
                },
            ),
            (
                "\\\\?\\C:\\Users\\someone\\test_file.rs:1902:13:",
                PathWithPosition {
                    path: PathBuf::from("\\\\?\\C:\\Users\\someone\\test_file.rs"),
                    row: Some(1902),
                    column: Some(13),
                },
            ),
            (
                "\\\\?\\C:\\Users\\someone\\test_file.rs:1902:13:15:",
                PathWithPosition {
                    path: PathBuf::from("\\\\?\\C:\\Users\\someone\\test_file.rs"),
                    row: Some(1902),
                    column: Some(13),
                },
            ),
            (
                "\\\\?\\C:\\Users\\someone\\test_file.rs:1902:::15:",
                PathWithPosition {
                    path: PathBuf::from("\\\\?\\C:\\Users\\someone\\test_file.rs"),
                    row: Some(1902),
                    column: None,
                },
            ),
            (
                "C:\\Users\\someone\\test_file.rs:1902:13:",
                PathWithPosition {
                    path: PathBuf::from("C:\\Users\\someone\\test_file.rs"),
                    row: Some(1902),
                    column: Some(13),
                },
            ),
            (
                "crates/utils/paths.rs",
                PathWithPosition {
                    path: PathBuf::from("crates\\utils\\paths.rs"),
                    row: None,
                    column: None,
                },
            ),
            (
                "crates/utils/paths.rs:101",
                PathWithPosition {
                    path: PathBuf::from("crates\\utils\\paths.rs"),
                    row: Some(101),
                    column: None,
                },
            ),
        ];

        for (input, expected) in input_and_expected {
            let actual = PathWithPosition::parse_str(input);
            assert_eq!(
                actual, expected,
                "For special case input str '{input}', got a parse mismatch"
            );
        }
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
