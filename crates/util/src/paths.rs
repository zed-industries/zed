use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use globset::{Glob, GlobMatcher};
use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    pub static ref HOME: PathBuf = dirs::home_dir().expect("failed to determine home directory");
    pub static ref CONFIG_DIR: PathBuf = if cfg!(target_os = "windows") {
        dirs::config_dir()
            .expect("failed to determine RoamingAppData directory")
            .join("Zed")
    } else if cfg!(target_os = "linux") {
        if let Ok(flatpak_xdg_config) = std::env::var("FLATPAK_XDG_CONFIG_HOME") {
           flatpak_xdg_config.into()
        } else {
            dirs::config_dir().expect("failed to determine XDG_CONFIG_HOME directory")
        }.join("zed")
    } else {
        HOME.join(".config").join("zed")
    };
    pub static ref CONVERSATIONS_DIR: PathBuf = if cfg!(target_os = "macos") {
        CONFIG_DIR.join("conversations")
    } else {
        SUPPORT_DIR.join("conversations")
    };
    pub static ref PROMPTS_DIR: PathBuf = if cfg!(target_os = "macos") {
        CONFIG_DIR.join("prompts")
    } else {
        SUPPORT_DIR.join("prompts")
    };
    pub static ref EMBEDDINGS_DIR: PathBuf = if cfg!(target_os = "macos") {
        CONFIG_DIR.join("embeddings")
    } else {
        SUPPORT_DIR.join("embeddings")
    };
    pub static ref THEMES_DIR: PathBuf = CONFIG_DIR.join("themes");

    pub static ref SUPPORT_DIR: PathBuf = if cfg!(target_os = "macos") {
        HOME.join("Library/Application Support/Zed")
    } else if cfg!(target_os = "linux") {
        if let Ok(flatpak_xdg_data) = std::env::var("FLATPAK_XDG_DATA_HOME") {
            flatpak_xdg_data.into()
        } else {
            dirs::data_local_dir().expect("failed to determine XDG_DATA_HOME directory")
        }.join("zed")
    } else if cfg!(target_os = "windows") {
        dirs::data_local_dir()
            .expect("failed to determine LocalAppData directory")
            .join("Zed")
    } else {
        CONFIG_DIR.clone()
    };
    pub static ref LOGS_DIR: PathBuf = if cfg!(target_os = "macos") {
        HOME.join("Library/Logs/Zed")
    } else {
        SUPPORT_DIR.join("logs")
    };
    pub static ref EXTENSIONS_DIR: PathBuf = SUPPORT_DIR.join("extensions");
    pub static ref LANGUAGES_DIR: PathBuf = SUPPORT_DIR.join("languages");
    pub static ref COPILOT_DIR: PathBuf = SUPPORT_DIR.join("copilot");
    pub static ref SUPERMAVEN_DIR: PathBuf = SUPPORT_DIR.join("supermaven");
    pub static ref DEFAULT_PRETTIER_DIR: PathBuf = SUPPORT_DIR.join("prettier");
    pub static ref DB_DIR: PathBuf = SUPPORT_DIR.join("db");
    pub static ref CRASHES_DIR: Option<PathBuf> = cfg!(target_os = "macos")
        .then_some(HOME.join("Library/Logs/DiagnosticReports"));
    pub static ref CRASHES_RETIRED_DIR: Option<PathBuf> = CRASHES_DIR
        .as_ref()
        .map(|dir| dir.join("Retired"));

    pub static ref SETTINGS: PathBuf = CONFIG_DIR.join("settings.json");
    pub static ref KEYMAP: PathBuf = CONFIG_DIR.join("keymap.json");
    pub static ref TASKS: PathBuf = CONFIG_DIR.join("tasks.json");
    pub static ref LAST_USERNAME: PathBuf = CONFIG_DIR.join("last-username.txt");
    pub static ref LOG: PathBuf = LOGS_DIR.join("Zed.log");
    pub static ref OLD_LOG: PathBuf = LOGS_DIR.join("Zed.log.old");
    pub static ref LOCAL_SETTINGS_RELATIVE_PATH: &'static Path = Path::new(".zed/settings.json");
    pub static ref LOCAL_TASKS_RELATIVE_PATH: &'static Path = Path::new(".zed/tasks.json");
    pub static ref LOCAL_VSCODE_TASKS_RELATIVE_PATH: &'static Path = Path::new(".vscode/tasks.json");
    pub static ref TEMP_DIR: PathBuf = if cfg!(target_os = "windows") {
        dirs::cache_dir()
            .expect("failed to determine LocalAppData directory")
            .join("Zed")
    } else if cfg!(target_os = "linux") {
        if let Ok(flatpak_xdg_cache) = std::env::var("FLATPAK_XDG_CACHE_HOME") {
            flatpak_xdg_cache.into()
        } else {
            dirs::cache_dir().expect("failed to determine XDG_CACHE_HOME directory")
        }.join("zed")
    } else {
        HOME.join(".cache").join("zed")
    };
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
            match self.as_ref().strip_prefix(HOME.as_path()) {
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

        let trimmed = s.trim();

        #[cfg(target_os = "windows")]
        {
            let is_absolute = trimmed.starts_with(r"\\?\");
            if is_absolute {
                return Self::parse_absolute_path(trimmed, parse_path_like_str);
            }
        }

        match trimmed.split_once(FILE_ROW_COLUMN_DELIMITER) {
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
                                let (maybe_col_str, _) =
                                    maybe_col_str.split_once(':').unwrap_or((maybe_col_str, ""));
                                match maybe_col_str.parse::<u32>() {
                                    Ok(col) => Ok(Self {
                                        path_like: parse_path_like_str(path_like_str)?,
                                        row: Some(row),
                                        column: Some(col),
                                    }),
                                    Err(_) => Ok(Self {
                                        path_like: parse_path_like_str(path_like_str)?,
                                        row: Some(row),
                                        column: None,
                                    }),
                                }
                            }
                        }
                        Err(_) => Ok(Self {
                            path_like: parse_path_like_str(path_like_str)?,
                            row: None,
                            column: None,
                        }),
                    }
                }
            }
            None => fallback(s),
        }
    }

    /// This helper function is used for parsing absolute paths on Windows. It exists because absolute paths on Windows are quite different from other platforms. See [this page](https://learn.microsoft.com/en-us/dotnet/standard/io/file-path-formats#dos-device-paths) for more information.
    #[cfg(target_os = "windows")]
    fn parse_absolute_path<E>(
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

        let mut iterator = s.split(FILE_ROW_COLUMN_DELIMITER);

        let drive_prefix = iterator.next().unwrap_or_default();
        let file_path = iterator.next().unwrap_or_default();

        // TODO: How to handle drives without a letter? UNC paths?
        let complete_path = drive_prefix.replace("\\\\?\\", "") + ":" + &file_path;

        if let Some(row_str) = iterator.next() {
            if let Some(column_str) = iterator.next() {
                match row_str.parse::<u32>() {
                    Ok(row) => match column_str.parse::<u32>() {
                        Ok(col) => {
                            return Ok(Self {
                                path_like: parse_path_like_str(&complete_path)?,
                                row: Some(row),
                                column: Some(col),
                            });
                        }

                        Err(_) => {
                            return Ok(Self {
                                path_like: parse_path_like_str(&complete_path)?,
                                row: Some(row),
                                column: None,
                            });
                        }
                    },

                    Err(_) => {
                        return fallback(&complete_path);
                    }
                }
            }
        }
        return fallback(&complete_path);
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

#[derive(Clone, Debug)]
pub struct PathMatcher {
    maybe_path: PathBuf,
    glob: GlobMatcher,
}

impl std::fmt::Display for PathMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.maybe_path.to_string_lossy().fmt(f)
    }
}

impl PartialEq for PathMatcher {
    fn eq(&self, other: &Self) -> bool {
        self.maybe_path.eq(&other.maybe_path)
    }
}

impl Eq for PathMatcher {}

impl PathMatcher {
    pub fn new(maybe_glob: &str) -> Result<Self, globset::Error> {
        Ok(PathMatcher {
            glob: Glob::new(maybe_glob)?.compile_matcher(),
            maybe_path: PathBuf::from(maybe_glob),
        })
    }

    pub fn is_match<P: AsRef<Path>>(&self, other: P) -> bool {
        let other_path = other.as_ref();
        other_path.starts_with(&self.maybe_path)
            || other_path.ends_with(&self.maybe_path)
            || self.glob.is_match(other_path)
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
        for (input, row, column) in [
            ("test_file.rs:a", None, None),
            ("test_file.rs:a:b", None, None),
            ("test_file.rs::", None, None),
            ("test_file.rs::1", None, None),
            ("test_file.rs:1::", Some(1), None),
            ("test_file.rs::1:2", None, None),
            ("test_file.rs:1::2", Some(1), None),
            ("test_file.rs:1:2:3", Some(1), Some(2)),
        ] {
            let actual = parse_str(input);
            assert_eq!(
                actual,
                PathLikeWithPosition {
                    path_like: "test_file.rs".to_string(),
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
            (
                "crates/file_finder/src/file_finder.rs:1902:13:",
                PathLikeWithPosition {
                    path_like: "crates/file_finder/src/file_finder.rs".to_string(),
                    row: Some(1902),
                    column: Some(13),
                },
            ),
        ];

        #[cfg(target_os = "windows")]
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
            (
                "\\\\?\\C:\\Users\\someone\\test_file.rs:1902:13:",
                PathLikeWithPosition {
                    path_like: "C:\\Users\\someone\\test_file.rs".to_string(),
                    row: Some(1902),
                    column: Some(13),
                },
            ),
            (
                "\\\\?\\C:\\Users\\someone\\test_file.rs:1902:13:15:",
                PathLikeWithPosition {
                    path_like: "C:\\Users\\someone\\test_file.rs".to_string(),
                    row: Some(1902),
                    column: Some(13),
                },
            ),
            (
                "\\\\?\\C:\\Users\\someone\\test_file.rs:1902:::15:",
                PathLikeWithPosition {
                    path_like: "C:\\Users\\someone\\test_file.rs".to_string(),
                    row: Some(1902),
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
        let path_matcher = PathMatcher::new("**/node_modules/**").unwrap();
        assert!(
            path_matcher.is_match(path),
            "Path matcher {path_matcher} should match {path:?}"
        );
    }

    #[test]
    fn project_search() {
        let path = Path::new("/Users/someonetoignore/work/zed/zed.dev/node_modules");
        let path_matcher = PathMatcher::new("**/node_modules/**").unwrap();
        assert!(
            path_matcher.is_match(path),
            "Path matcher {path_matcher} should match {path:?}"
        );
    }
}
