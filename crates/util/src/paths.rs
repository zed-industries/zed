use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    pub static ref HOME: PathBuf = dirs::home_dir().expect("failed to determine home directory");
    pub static ref CONFIG_DIR: PathBuf = HOME.join(".config").join("zed");
    pub static ref CONVERSATIONS_DIR: PathBuf = HOME.join(".config/zed/conversations");
    pub static ref EMBEDDINGS_DIR: PathBuf = HOME.join(".config/zed/embeddings");
    pub static ref LOGS_DIR: PathBuf = HOME.join("Library/Logs/Zed");
    pub static ref SUPPORT_DIR: PathBuf = HOME.join("Library/Application Support/Zed");
    pub static ref LANGUAGES_DIR: PathBuf = HOME.join("Library/Application Support/Zed/languages");
    pub static ref COPILOT_DIR: PathBuf = HOME.join("Library/Application Support/Zed/copilot");
    pub static ref DB_DIR: PathBuf = HOME.join("Library/Application Support/Zed/db");
    pub static ref SETTINGS: PathBuf = CONFIG_DIR.join("settings.json");
    pub static ref KEYMAP: PathBuf = CONFIG_DIR.join("keymap.json");
    pub static ref LAST_USERNAME: PathBuf = CONFIG_DIR.join("last-username.txt");
    pub static ref LOG: PathBuf = LOGS_DIR.join("Zed.log");
    pub static ref OLD_LOG: PathBuf = LOGS_DIR.join("Zed.log.old");
    pub static ref LOCAL_SETTINGS_RELATIVE_PATH: &'static Path = Path::new(".zed/settings.json");
}

pub mod legacy {
    use std::path::PathBuf;

    lazy_static::lazy_static! {
        static ref CONFIG_DIR: PathBuf = super::HOME.join(".zed");
        pub static ref SETTINGS: PathBuf = CONFIG_DIR.join("settings.json");
        pub static ref KEYMAP: PathBuf = CONFIG_DIR.join("keymap.json");
    }
}

pub trait PathExt {
    fn compact(&self) -> PathBuf;
    fn icon_suffix(&self) -> Option<&str>;
}

impl PathExt for Path {
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
            match self.strip_prefix(HOME.as_path()) {
                Ok(relative_path) => {
                    let mut shortened_path = PathBuf::new();
                    shortened_path.push("~");
                    shortened_path.push(relative_path);
                    shortened_path
                }
                Err(_) => self.to_path_buf(),
            }
        } else {
            self.to_path_buf()
        }
    }

    fn icon_suffix(&self) -> Option<&str> {
        let file_name = self.file_name()?.to_str()?;

        if file_name.starts_with(".") {
            return file_name.strip_prefix(".");
        }

        self.extension()
            .map(|extension| extension.to_str())
            .flatten()
    }
}

/// A delimiter to use in `path_query:row_number:column_number` strings parsing.
pub const FILE_ROW_COLUMN_DELIMITER: char = ':';

/// A representation of a path-like string with optional row and column numbers.
/// Matching values example: `te`, `test.rs:22`, `te:22:5`, etc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathLikeWithPosition<P> {
    pub path_like: P,
    pub row: Option<u32>,
    // Absent if row is absent.
    pub column: Option<u32>,
}

impl<P> PathLikeWithPosition<P> {
    /// Parses a string that possibly has `:row:column` suffix.
    /// Ignores trailing `:`s, so `test.rs:22:` is parsed as `test.rs:22`.
    /// If any of the row/column component parsing fails, the whole string is then parsed as a path like.
    pub fn parse_str<E>(
        s: &str,
        parse_path_like_str: impl Fn(&str) -> Result<P, E>,
    ) -> Result<Self, E> {
        let fallback = |fallback_str| {
            Ok(Self {
                path_like: parse_path_like_str(fallback_str)?,
                row: None,
                column: None,
            })
        };

        match s.trim().split_once(FILE_ROW_COLUMN_DELIMITER) {
            Some((path_like_str, maybe_row_and_col_str)) => {
                let path_like_str = path_like_str.trim();
                let maybe_row_and_col_str = maybe_row_and_col_str.trim();
                if path_like_str.is_empty() {
                    fallback(s)
                } else if maybe_row_and_col_str.is_empty() {
                    fallback(path_like_str)
                } else {
                    let (row_parse_result, maybe_col_str) =
                        match maybe_row_and_col_str.split_once(FILE_ROW_COLUMN_DELIMITER) {
                            Some((maybe_row_str, maybe_col_str)) => {
                                (maybe_row_str.parse::<u32>(), maybe_col_str.trim())
                            }
                            None => (maybe_row_and_col_str.parse::<u32>(), ""),
                        };

                    match row_parse_result {
                        Ok(row) => {
                            if maybe_col_str.is_empty() {
                                Ok(Self {
                                    path_like: parse_path_like_str(path_like_str)?,
                                    row: Some(row),
                                    column: None,
                                })
                            } else {
                                match maybe_col_str.parse::<u32>() {
                                    Ok(col) => Ok(Self {
                                        path_like: parse_path_like_str(path_like_str)?,
                                        row: Some(row),
                                        column: Some(col),
                                    }),
                                    Err(_) => fallback(s),
                                }
                            }
                        }
                        Err(_) => fallback(s),
                    }
                }
            }
            None => fallback(s),
        }
    }

    pub fn map_path_like<P2, E>(
        self,
        mapping: impl FnOnce(P) -> Result<P2, E>,
    ) -> Result<PathLikeWithPosition<P2>, E> {
        Ok(PathLikeWithPosition {
            path_like: mapping(self.path_like)?,
            row: self.row,
            column: self.column,
        })
    }

    pub fn to_string(&self, path_like_to_string: impl Fn(&P) -> String) -> String {
        let path_like_string = path_like_to_string(&self.path_like);
        if let Some(row) = self.row {
            if let Some(column) = self.column {
                format!("{path_like_string}:{row}:{column}")
            } else {
                format!("{path_like_string}:{row}")
            }
        } else {
            path_like_string
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestPath = PathLikeWithPosition<String>;

    fn parse_str(s: &str) -> TestPath {
        TestPath::parse_str(s, |s| Ok::<_, std::convert::Infallible>(s.to_string()))
            .expect("infallible")
    }

    #[test]
    fn path_with_position_parsing_positive() {
        let input_and_expected = [
            (
                "test_file.rs",
                PathLikeWithPosition {
                    path_like: "test_file.rs".to_string(),
                    row: None,
                    column: None,
                },
            ),
            (
                "test_file.rs:1",
                PathLikeWithPosition {
                    path_like: "test_file.rs".to_string(),
                    row: Some(1),
                    column: None,
                },
            ),
            (
                "test_file.rs:1:2",
                PathLikeWithPosition {
                    path_like: "test_file.rs".to_string(),
                    row: Some(1),
                    column: Some(2),
                },
            ),
        ];

        for (input, expected) in input_and_expected {
            let actual = parse_str(input);
            assert_eq!(
                actual, expected,
                "For positive case input str '{input}', got a parse mismatch"
            );
        }
    }

    #[test]
    fn path_with_position_parsing_negative() {
        for input in [
            "test_file.rs:a",
            "test_file.rs:a:b",
            "test_file.rs::",
            "test_file.rs::1",
            "test_file.rs:1::",
            "test_file.rs::1:2",
            "test_file.rs:1::2",
            "test_file.rs:1:2:",
            "test_file.rs:1:2:3",
        ] {
            let actual = parse_str(input);
            assert_eq!(
                actual,
                PathLikeWithPosition {
                    path_like: input.to_string(),
                    row: None,
                    column: None,
                },
                "For negative case input str '{input}', got a parse mismatch"
            );
        }
    }

    // Trim off trailing `:`s for otherwise valid input.
    #[test]
    fn path_with_position_parsing_special() {
        let input_and_expected = [
            (
                "test_file.rs:",
                PathLikeWithPosition {
                    path_like: "test_file.rs".to_string(),
                    row: None,
                    column: None,
                },
            ),
            (
                "test_file.rs:1:",
                PathLikeWithPosition {
                    path_like: "test_file.rs".to_string(),
                    row: Some(1),
                    column: None,
                },
            ),
        ];

        for (input, expected) in input_and_expected {
            let actual = parse_str(input);
            assert_eq!(
                actual, expected,
                "For special case input str '{input}', got a parse mismatch"
            );
        }
    }

    #[test]
    fn test_path_compact() {
        let path: PathBuf = [
            HOME.to_string_lossy().to_string(),
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
    fn test_path_suffix() {
        // No dots in name
        let path = Path::new("/a/b/c/file_name.rs");
        assert_eq!(path.icon_suffix(), Some("rs"));

        // Single dot in name
        let path = Path::new("/a/b/c/file.name.rs");
        assert_eq!(path.icon_suffix(), Some("rs"));

        // Multiple dots in name
        let path = Path::new("/a/b/c/long.file.name.rs");
        assert_eq!(path.icon_suffix(), Some("rs"));

        // Hidden file, no extension
        let path = Path::new("/a/b/c/.gitignore");
        assert_eq!(path.icon_suffix(), Some("gitignore"));

        // Hidden file, with extension
        let path = Path::new("/a/b/c/.eslintrc.js");
        assert_eq!(path.icon_suffix(), Some("eslintrc.js"));
    }
}
