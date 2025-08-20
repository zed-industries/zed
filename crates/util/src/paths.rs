use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::path::StripPrefixError;
use std::sync::{Arc, OnceLock};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::LazyLock,
};

/// Returns the path to the user's home directory.
pub fn home_dir() -> &'static PathBuf {
    static HOME_DIR: OnceLock<PathBuf> = OnceLock::new();
    HOME_DIR.get_or_init(|| dirs::home_dir().expect("failed to determine home directory"))
}

pub trait PathExt {
    fn compact(&self) -> PathBuf;
    fn extension_or_hidden_file_name(&self) -> Option<&str>;
    fn to_sanitized_string(&self) -> String;
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
            use anyhow::Context as _;
            use tendril::fmt::{Format, WTF8};
            WTF8::validate(bytes)
                .then(|| {
                    // Safety: bytes are valid WTF-8 sequence.
                    Self::from(Path::new(unsafe {
                        OsStr::from_encoded_bytes_unchecked(bytes)
                    }))
                })
                .with_context(|| format!("Invalid WTF-8 sequence: {bytes:?}"))
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
        if cfg!(any(target_os = "linux", target_os = "freebsd")) || cfg!(target_os = "macos") {
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

    /// Returns a file's extension or, if the file is hidden, its name without the leading dot
    fn extension_or_hidden_file_name(&self) -> Option<&str> {
        let path = self.as_ref();
        let file_name = path.file_name()?.to_str()?;
        if file_name.starts_with('.') {
            return file_name.strip_prefix('.');
        }

        path.extension()
            .and_then(|e| e.to_str())
            .or_else(|| path.file_stem()?.to_str())
    }

    /// Returns a sanitized string representation of the path.
    /// Note, on Windows, this assumes that the path is a valid UTF-8 string and
    /// is not a UNC path.
    fn to_sanitized_string(&self) -> String {
        #[cfg(target_os = "windows")]
        {
            self.as_ref().to_string_lossy().replace("/", "\\")
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.as_ref().to_string_lossy().to_string()
        }
    }
}

/// Due to the issue of UNC paths on Windows, which can cause bugs in various parts of Zed, introducing this `SanitizedPath`
/// leverages Rust's type system to ensure that all paths entering Zed are always "sanitized" by removing the `\\\\?\\` prefix.
/// On non-Windows operating systems, this struct is effectively a no-op.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SanitizedPath(pub Arc<Path>);

impl SanitizedPath {
    pub fn starts_with(&self, prefix: &SanitizedPath) -> bool {
        self.0.starts_with(&prefix.0)
    }

    pub fn as_path(&self) -> &Arc<Path> {
        &self.0
    }

    pub fn to_glob_string(&self) -> String {
        #[cfg(target_os = "windows")]
        {
            self.0.to_string_lossy().replace("/", "\\")
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.0.to_string_lossy().to_string()
        }
    }

    pub fn join(&self, path: &Self) -> Self {
        self.0.join(&path.0).into()
    }

    pub fn strip_prefix(&self, base: &Self) -> Result<&Path, StripPrefixError> {
        self.0.strip_prefix(base.as_path())
    }
}

impl Display for SanitizedPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl From<SanitizedPath> for Arc<Path> {
    fn from(sanitized_path: SanitizedPath) -> Self {
        sanitized_path.0
    }
}

impl From<SanitizedPath> for PathBuf {
    fn from(sanitized_path: SanitizedPath) -> Self {
        sanitized_path.0.as_ref().into()
    }
}

impl<T: AsRef<Path>> From<T> for SanitizedPath {
    #[cfg(not(target_os = "windows"))]
    fn from(path: T) -> Self {
        let path = path.as_ref();
        SanitizedPath(path.into())
    }

    #[cfg(target_os = "windows")]
    fn from(path: T) -> Self {
        let path = path.as_ref();
        SanitizedPath(dunce::simplified(path).into())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PathStyle {
    Posix,
    Windows,
}

impl PathStyle {
    #[cfg(target_os = "windows")]
    pub const fn current() -> Self {
        PathStyle::Windows
    }

    #[cfg(not(target_os = "windows"))]
    pub const fn current() -> Self {
        PathStyle::Posix
    }

    #[inline]
    pub fn separator(&self) -> &str {
        match self {
            PathStyle::Posix => "/",
            PathStyle::Windows => "\\",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RemotePathBuf {
    inner: PathBuf,
    style: PathStyle,
    string: String, // Cached string representation
}

impl RemotePathBuf {
    pub fn new(path: PathBuf, style: PathStyle) -> Self {
        #[cfg(target_os = "windows")]
        let string = match style {
            PathStyle::Posix => path.to_string_lossy().replace('\\', "/"),
            PathStyle::Windows => path.to_string_lossy().into(),
        };
        #[cfg(not(target_os = "windows"))]
        let string = match style {
            PathStyle::Posix => path.to_string_lossy().to_string(),
            PathStyle::Windows => path.to_string_lossy().replace('/', "\\"),
        };
        Self {
            inner: path,
            style,
            string,
        }
    }

    pub fn from_str(path: &str, style: PathStyle) -> Self {
        let path_buf = PathBuf::from(path);
        Self::new(path_buf, style)
    }

    #[cfg(target_os = "windows")]
    pub fn to_proto(&self) -> String {
        match self.path_style() {
            PathStyle::Posix => self.to_string(),
            PathStyle::Windows => self.inner.to_string_lossy().replace('\\', "/"),
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn to_proto(&self) -> String {
        match self.path_style() {
            PathStyle::Posix => self.inner.to_string_lossy().to_string(),
            PathStyle::Windows => self.to_string(),
        }
    }

    pub fn as_path(&self) -> &Path {
        &self.inner
    }

    pub fn path_style(&self) -> PathStyle {
        self.style
    }

    pub fn parent(&self) -> Option<RemotePathBuf> {
        self.inner
            .parent()
            .map(|p| RemotePathBuf::new(p.to_path_buf(), self.style))
    }
}

impl Display for RemotePathBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

/// A delimiter to use in `path_query:row_number:column_number` strings parsing.
pub const FILE_ROW_COLUMN_DELIMITER: char = ':';

const ROW_COL_CAPTURE_REGEX: &str = r"(?xs)
    ([^\(]+)\:(?:
        \((\d+)[,:](\d+)\) # filename:(row,column), filename:(row:column)
        |
        \((\d+)\)()     # filename:(row)
    )
    |
    ([^\(]+)(?:
        \((\d+)[,:](\d+)\) # filename(row,column), filename(row:column)
        |
        \((\d+)\)()     # filename(row)
    )
    |
    (.+?)(?:
        \:+(\d+)\:(\d+)\:*$  # filename:row:column
        |
        \:+(\d+)\:*()$       # filename:row
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
    ///     path: PathBuf::from("test_file.rs::"),
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
            None => {
                // The `ROW_COL_CAPTURE_REGEX` deals with separated digits only,
                // but in reality there could be `foo/bar.py:22:in` inputs which we want to match too.
                // The regex mentioned is not very extendable with "digit or random string" checks, so do this here instead.
                let delimiter = ':';
                let mut path_parts = s
                    .rsplitn(3, delimiter)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .fuse();
                let mut path_string = path_parts.next().expect("rsplitn should have the rest of the string as its last parameter that we reversed").to_owned();
                let mut row = None;
                let mut column = None;
                if let Some(maybe_row) = path_parts.next() {
                    if let Ok(parsed_row) = maybe_row.parse::<u32>() {
                        row = Some(parsed_row);
                        if let Some(parsed_column) = path_parts
                            .next()
                            .and_then(|maybe_col| maybe_col.parse::<u32>().ok())
                        {
                            column = Some(parsed_column);
                        }
                    } else {
                        path_string.push(delimiter);
                        path_string.push_str(maybe_row);
                    }
                }
                for split in path_parts {
                    path_string.push(delimiter);
                    path_string.push_str(split);
                }

                Self {
                    path: PathBuf::from(path_string),
                    row,
                    column,
                }
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
    pub fn new(globs: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Self, globset::Error> {
        let globs = globs
            .into_iter()
            .map(|as_str| Glob::new(as_str.as_ref()))
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

/// Custom character comparison that prioritizes lowercase for same letters
fn compare_chars(a: char, b: char) -> Ordering {
    // First compare case-insensitive
    match a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()) {
        Ordering::Equal => {
            // If same letter, prioritize lowercase (lowercase < uppercase)
            match (a.is_ascii_lowercase(), b.is_ascii_lowercase()) {
                (true, false) => Ordering::Less,    // lowercase comes first
                (false, true) => Ordering::Greater, // uppercase comes after
                _ => Ordering::Equal,               // both same case or both non-ascii
            }
        }
        other => other,
    }
}

/// Compares two sequences of consecutive digits for natural sorting.
///
/// This function is a core component of natural sorting that handles numeric comparison
/// in a way that feels natural to humans. It extracts and compares consecutive digit
/// sequences from two iterators, handling various cases like leading zeros and very large numbers.
///
/// # Behavior
///
/// The function implements the following comparison rules:
/// 1. Different numeric values: Compares by actual numeric value (e.g., "2" < "10")
/// 2. Leading zeros: When values are equal, longer sequence wins (e.g., "002" > "2")
/// 3. Large numbers: Falls back to string comparison for numbers that would overflow u128
///
/// # Examples
///
/// ```text
/// "1" vs "2"      -> Less       (different values)
/// "2" vs "10"     -> Less       (numeric comparison)
/// "002" vs "2"    -> Greater    (leading zeros)
/// "10" vs "010"   -> Less       (leading zeros)
/// "999..." vs "1000..." -> Less (large number comparison)
/// ```
///
/// # Implementation Details
///
/// 1. Extracts consecutive digits into strings
/// 2. Compares sequence lengths for leading zero handling
/// 3. For equal lengths, compares digit by digit
/// 4. For different lengths:
///    - Attempts numeric comparison first (for numbers up to 2^128 - 1)
///    - Falls back to string comparison if numbers would overflow
///
/// The function advances both iterators past their respective numeric sequences,
/// regardless of the comparison result.
fn compare_numeric_segments<I>(
    a_iter: &mut std::iter::Peekable<I>,
    b_iter: &mut std::iter::Peekable<I>,
) -> Ordering
where
    I: Iterator<Item = char>,
{
    // Collect all consecutive digits into strings
    let mut a_num_str = String::new();
    let mut b_num_str = String::new();

    while let Some(&c) = a_iter.peek() {
        if !c.is_ascii_digit() {
            break;
        }

        a_num_str.push(c);
        a_iter.next();
    }

    while let Some(&c) = b_iter.peek() {
        if !c.is_ascii_digit() {
            break;
        }

        b_num_str.push(c);
        b_iter.next();
    }

    // First compare lengths (handle leading zeros)
    match a_num_str.len().cmp(&b_num_str.len()) {
        Ordering::Equal => {
            // Same length, compare digit by digit
            match a_num_str.cmp(&b_num_str) {
                Ordering::Equal => Ordering::Equal,
                ordering => ordering,
            }
        }

        // Different lengths but same value means leading zeros
        ordering => {
            // Try parsing as numbers first
            if let (Ok(a_val), Ok(b_val)) = (a_num_str.parse::<u128>(), b_num_str.parse::<u128>()) {
                match a_val.cmp(&b_val) {
                    Ordering::Equal => ordering, // Same value, longer one is greater (leading zeros)
                    ord => ord,
                }
            } else {
                // If parsing fails (overflow), compare as strings
                a_num_str.cmp(&b_num_str)
            }
        }
    }
}

/// Performs natural sorting comparison between two strings.
///
/// Natural sorting is an ordering that handles numeric sequences in a way that matches human expectations.
/// For example, "file2" comes before "file10" (unlike standard lexicographic sorting).
///
/// # Characteristics
///
/// * Case-sensitive with lowercase priority: When comparing same letters, lowercase comes before uppercase
/// * Numbers are compared by numeric value, not character by character
/// * Leading zeros affect ordering when numeric values are equal
/// * Can handle numbers larger than u128::MAX (falls back to string comparison)
///
/// # Algorithm
///
/// The function works by:
/// 1. Processing strings character by character
/// 2. When encountering digits, treating consecutive digits as a single number
/// 3. Comparing numbers by their numeric value rather than lexicographically
/// 4. For non-numeric characters, using case-sensitive comparison with lowercase priority
fn natural_sort(a: &str, b: &str) -> Ordering {
    let mut a_iter = a.chars().peekable();
    let mut b_iter = b.chars().peekable();

    loop {
        match (a_iter.peek(), b_iter.peek()) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(&a_char), Some(&b_char)) => {
                if a_char.is_ascii_digit() && b_char.is_ascii_digit() {
                    match compare_numeric_segments(&mut a_iter, &mut b_iter) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                } else {
                    match compare_chars(a_char, b_char) {
                        Ordering::Equal => {
                            a_iter.next();
                            b_iter.next();
                        }
                        ordering => return ordering,
                    }
                }
            }
        }
    }
}

pub fn compare_paths(
    (path_a, a_is_file): (&Path, bool),
    (path_b, b_is_file): (&Path, bool),
) -> Ordering {
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

                    let path_b = Path::new(component_b.as_os_str());
                    let path_string_b = if b_is_file {
                        path_b.file_stem()
                    } else {
                        path_b.file_name()
                    }
                    .map(|s| s.to_string_lossy());

                    let compare_components = match (path_string_a, path_string_b) {
                        (Some(a), Some(b)) => natural_sort(&a, &b),
                        (Some(_), None) => Ordering::Greater,
                        (None, Some(_)) => Ordering::Less,
                        (None, None) => Ordering::Equal,
                    };

                    compare_components.then_with(|| {
                        if a_is_file && b_is_file {
                            let ext_a = path_a.extension().unwrap_or_default();
                            let ext_b = path_b.extension().unwrap_or_default();
                            ext_a.cmp(ext_b)
                        } else {
                            Ordering::Equal
                        }
                    })
                });

                if !ordering.is_eq() {
                    return ordering;
                }
            }
            (Some(_), None) => break Ordering::Greater,
            (None, Some(_)) => break Ordering::Less,
            (None, None) => break Ordering::Equal,
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
    fn compare_paths_with_same_name_different_extensions() {
        let mut paths = vec![
            (Path::new("test_dirs/file.rs"), true),
            (Path::new("test_dirs/file.txt"), true),
            (Path::new("test_dirs/file.md"), true),
            (Path::new("test_dirs/file"), true),
            (Path::new("test_dirs/file.a"), true),
        ];
        paths.sort_by(|&a, &b| compare_paths(a, b));
        assert_eq!(
            paths,
            vec![
                (Path::new("test_dirs/file"), true),
                (Path::new("test_dirs/file.a"), true),
                (Path::new("test_dirs/file.md"), true),
                (Path::new("test_dirs/file.rs"), true),
                (Path::new("test_dirs/file.txt"), true),
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
            PathWithPosition::parse_str("test_file"),
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
                path: PathBuf::from("test_file.rs:"),
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

        assert_eq!(
            PathWithPosition::parse_str("ab\ncd"),
            PathWithPosition {
                path: PathBuf::from("ab\ncd"),
                row: None,
                column: None
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("👋\nab"),
            PathWithPosition {
                path: PathBuf::from("👋\nab"),
                row: None,
                column: None
            }
        );

        assert_eq!(
            PathWithPosition::parse_str("Types.hs:(617,9)-(670,28):"),
            PathWithPosition {
                path: PathBuf::from("Types.hs"),
                row: Some(617),
                column: Some(9),
            }
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn path_with_position_parse_posix_path_with_suffix() {
        assert_eq!(
            PathWithPosition::parse_str("foo/bar:34:in"),
            PathWithPosition {
                path: PathBuf::from("foo/bar"),
                row: Some(34),
                column: None,
            }
        );
        assert_eq!(
            PathWithPosition::parse_str("foo/bar.rs:1902:::15:"),
            PathWithPosition {
                path: PathBuf::from("foo/bar.rs:1902"),
                row: Some(15),
                column: None
            }
        );

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
        assert_eq!(
            PathWithPosition::parse_str("/testing/out/src/file_finder.odin(7:15)"),
            PathWithPosition {
                path: PathBuf::from("/testing/out/src/file_finder.odin"),
                row: Some(7),
                column: Some(15),
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
        if cfg!(any(target_os = "linux", target_os = "freebsd")) || cfg!(target_os = "macos") {
            assert_eq!(path.compact().to_str(), Some("~/some_file.txt"));
        } else {
            assert_eq!(path.compact().to_str(), path.to_str());
        }
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
        assert_eq!(path.extension_or_hidden_file_name(), Some("eslintrc.js"));
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

    #[test]
    #[cfg(target_os = "windows")]
    fn test_sanitized_path() {
        let path = Path::new("C:\\Users\\someone\\test_file.rs");
        let sanitized_path = SanitizedPath::from(path);
        assert_eq!(
            sanitized_path.to_string(),
            "C:\\Users\\someone\\test_file.rs"
        );

        let path = Path::new("\\\\?\\C:\\Users\\someone\\test_file.rs");
        let sanitized_path = SanitizedPath::from(path);
        assert_eq!(
            sanitized_path.to_string(),
            "C:\\Users\\someone\\test_file.rs"
        );
    }

    #[test]
    fn test_compare_numeric_segments() {
        // Helper function to create peekable iterators and test
        fn compare(a: &str, b: &str) -> Ordering {
            let mut a_iter = a.chars().peekable();
            let mut b_iter = b.chars().peekable();

            let result = compare_numeric_segments(&mut a_iter, &mut b_iter);

            // Verify iterators advanced correctly
            assert!(
                !a_iter.next().is_some_and(|c| c.is_ascii_digit()),
                "Iterator a should have consumed all digits"
            );
            assert!(
                !b_iter.next().is_some_and(|c| c.is_ascii_digit()),
                "Iterator b should have consumed all digits"
            );

            result
        }

        // Basic numeric comparisons
        assert_eq!(compare("0", "0"), Ordering::Equal);
        assert_eq!(compare("1", "2"), Ordering::Less);
        assert_eq!(compare("9", "10"), Ordering::Less);
        assert_eq!(compare("10", "9"), Ordering::Greater);
        assert_eq!(compare("99", "100"), Ordering::Less);

        // Leading zeros
        assert_eq!(compare("0", "00"), Ordering::Less);
        assert_eq!(compare("00", "0"), Ordering::Greater);
        assert_eq!(compare("01", "1"), Ordering::Greater);
        assert_eq!(compare("001", "1"), Ordering::Greater);
        assert_eq!(compare("001", "01"), Ordering::Greater);

        // Same value different representation
        assert_eq!(compare("000100", "100"), Ordering::Greater);
        assert_eq!(compare("100", "0100"), Ordering::Less);
        assert_eq!(compare("0100", "00100"), Ordering::Less);

        // Large numbers
        assert_eq!(compare("9999999999", "10000000000"), Ordering::Less);
        assert_eq!(
            compare(
                "340282366920938463463374607431768211455", // u128::MAX
                "340282366920938463463374607431768211456"
            ),
            Ordering::Less
        );
        assert_eq!(
            compare(
                "340282366920938463463374607431768211456", // > u128::MAX
                "340282366920938463463374607431768211455"
            ),
            Ordering::Greater
        );

        // Iterator advancement verification
        let mut a_iter = "123abc".chars().peekable();
        let mut b_iter = "456def".chars().peekable();

        compare_numeric_segments(&mut a_iter, &mut b_iter);

        assert_eq!(a_iter.collect::<String>(), "abc");
        assert_eq!(b_iter.collect::<String>(), "def");
    }

    #[test]
    fn test_natural_sort() {
        // Basic alphanumeric
        assert_eq!(natural_sort("a", "b"), Ordering::Less);
        assert_eq!(natural_sort("b", "a"), Ordering::Greater);
        assert_eq!(natural_sort("a", "a"), Ordering::Equal);

        // Case sensitivity
        assert_eq!(natural_sort("a", "A"), Ordering::Less);
        assert_eq!(natural_sort("A", "a"), Ordering::Greater);
        assert_eq!(natural_sort("aA", "aa"), Ordering::Greater);
        assert_eq!(natural_sort("aa", "aA"), Ordering::Less);

        // Numbers
        assert_eq!(natural_sort("1", "2"), Ordering::Less);
        assert_eq!(natural_sort("2", "10"), Ordering::Less);
        assert_eq!(natural_sort("02", "10"), Ordering::Less);
        assert_eq!(natural_sort("02", "2"), Ordering::Greater);

        // Mixed alphanumeric
        assert_eq!(natural_sort("a1", "a2"), Ordering::Less);
        assert_eq!(natural_sort("a2", "a10"), Ordering::Less);
        assert_eq!(natural_sort("a02", "a2"), Ordering::Greater);
        assert_eq!(natural_sort("a1b", "a1c"), Ordering::Less);

        // Multiple numeric segments
        assert_eq!(natural_sort("1a2", "1a10"), Ordering::Less);
        assert_eq!(natural_sort("1a10", "1a2"), Ordering::Greater);
        assert_eq!(natural_sort("2a1", "10a1"), Ordering::Less);

        // Special characters
        assert_eq!(natural_sort("a-1", "a-2"), Ordering::Less);
        assert_eq!(natural_sort("a_1", "a_2"), Ordering::Less);
        assert_eq!(natural_sort("a.1", "a.2"), Ordering::Less);

        // Unicode
        assert_eq!(natural_sort("文1", "文2"), Ordering::Less);
        assert_eq!(natural_sort("文2", "文10"), Ordering::Less);
        assert_eq!(natural_sort("🔤1", "🔤2"), Ordering::Less);

        // Empty and special cases
        assert_eq!(natural_sort("", ""), Ordering::Equal);
        assert_eq!(natural_sort("", "a"), Ordering::Less);
        assert_eq!(natural_sort("a", ""), Ordering::Greater);
        assert_eq!(natural_sort(" ", "  "), Ordering::Less);

        // Mixed everything
        assert_eq!(natural_sort("File-1.txt", "File-2.txt"), Ordering::Less);
        assert_eq!(natural_sort("File-02.txt", "File-2.txt"), Ordering::Greater);
        assert_eq!(natural_sort("File-2.txt", "File-10.txt"), Ordering::Less);
        assert_eq!(natural_sort("File_A1", "File_A2"), Ordering::Less);
        assert_eq!(natural_sort("File_a1", "File_A1"), Ordering::Less);
    }

    #[test]
    fn test_compare_paths() {
        // Helper function for cleaner tests
        fn compare(a: &str, is_a_file: bool, b: &str, is_b_file: bool) -> Ordering {
            compare_paths((Path::new(a), is_a_file), (Path::new(b), is_b_file))
        }

        // Basic path comparison
        assert_eq!(compare("a", true, "b", true), Ordering::Less);
        assert_eq!(compare("b", true, "a", true), Ordering::Greater);
        assert_eq!(compare("a", true, "a", true), Ordering::Equal);

        // Files vs Directories
        assert_eq!(compare("a", true, "a", false), Ordering::Greater);
        assert_eq!(compare("a", false, "a", true), Ordering::Less);
        assert_eq!(compare("b", false, "a", true), Ordering::Less);

        // Extensions
        assert_eq!(compare("a.txt", true, "a.md", true), Ordering::Greater);
        assert_eq!(compare("a.md", true, "a.txt", true), Ordering::Less);
        assert_eq!(compare("a", true, "a.txt", true), Ordering::Less);

        // Nested paths
        assert_eq!(compare("dir/a", true, "dir/b", true), Ordering::Less);
        assert_eq!(compare("dir1/a", true, "dir2/a", true), Ordering::Less);
        assert_eq!(compare("dir/sub/a", true, "dir/a", true), Ordering::Less);

        // Case sensitivity in paths
        assert_eq!(
            compare("Dir/file", true, "dir/file", true),
            Ordering::Greater
        );
        assert_eq!(
            compare("dir/File", true, "dir/file", true),
            Ordering::Greater
        );
        assert_eq!(compare("dir/file", true, "Dir/File", true), Ordering::Less);

        // Hidden files and special names
        assert_eq!(compare(".hidden", true, "visible", true), Ordering::Less);
        assert_eq!(compare("_special", true, "normal", true), Ordering::Less);
        assert_eq!(compare(".config", false, ".data", false), Ordering::Less);

        // Mixed numeric paths
        assert_eq!(
            compare("dir1/file", true, "dir2/file", true),
            Ordering::Less
        );
        assert_eq!(
            compare("dir2/file", true, "dir10/file", true),
            Ordering::Less
        );
        assert_eq!(
            compare("dir02/file", true, "dir2/file", true),
            Ordering::Greater
        );

        // Root paths
        assert_eq!(compare("/a", true, "/b", true), Ordering::Less);
        assert_eq!(compare("/", false, "/a", true), Ordering::Less);

        // Complex real-world examples
        assert_eq!(
            compare("project/src/main.rs", true, "project/src/lib.rs", true),
            Ordering::Greater
        );
        assert_eq!(
            compare(
                "project/tests/test_1.rs",
                true,
                "project/tests/test_2.rs",
                true
            ),
            Ordering::Less
        );
        assert_eq!(
            compare(
                "project/v1.0.0/README.md",
                true,
                "project/v1.10.0/README.md",
                true
            ),
            Ordering::Less
        );
    }

    #[test]
    fn test_natural_sort_case_sensitivity() {
        // Same letter different case - lowercase should come first
        assert_eq!(natural_sort("a", "A"), Ordering::Less);
        assert_eq!(natural_sort("A", "a"), Ordering::Greater);
        assert_eq!(natural_sort("a", "a"), Ordering::Equal);
        assert_eq!(natural_sort("A", "A"), Ordering::Equal);

        // Mixed case strings
        assert_eq!(natural_sort("aaa", "AAA"), Ordering::Less);
        assert_eq!(natural_sort("AAA", "aaa"), Ordering::Greater);
        assert_eq!(natural_sort("aAa", "AaA"), Ordering::Less);

        // Different letters
        assert_eq!(natural_sort("a", "b"), Ordering::Less);
        assert_eq!(natural_sort("A", "b"), Ordering::Less);
        assert_eq!(natural_sort("a", "B"), Ordering::Less);
    }

    #[test]
    fn test_natural_sort_with_numbers() {
        // Basic number ordering
        assert_eq!(natural_sort("file1", "file2"), Ordering::Less);
        assert_eq!(natural_sort("file2", "file10"), Ordering::Less);
        assert_eq!(natural_sort("file10", "file2"), Ordering::Greater);

        // Numbers in different positions
        assert_eq!(natural_sort("1file", "2file"), Ordering::Less);
        assert_eq!(natural_sort("file1text", "file2text"), Ordering::Less);
        assert_eq!(natural_sort("text1file", "text2file"), Ordering::Less);

        // Multiple numbers in string
        assert_eq!(natural_sort("file1-2", "file1-10"), Ordering::Less);
        assert_eq!(natural_sort("2-1file", "10-1file"), Ordering::Less);

        // Leading zeros
        assert_eq!(natural_sort("file002", "file2"), Ordering::Greater);
        assert_eq!(natural_sort("file002", "file10"), Ordering::Less);

        // Very large numbers
        assert_eq!(
            natural_sort("file999999999999999999999", "file999999999999999999998"),
            Ordering::Greater
        );

        // u128 edge cases

        // Numbers near u128::MAX (340,282,366,920,938,463,463,374,607,431,768,211,455)
        assert_eq!(
            natural_sort(
                "file340282366920938463463374607431768211454",
                "file340282366920938463463374607431768211455"
            ),
            Ordering::Less
        );

        // Equal length numbers that overflow u128
        assert_eq!(
            natural_sort(
                "file340282366920938463463374607431768211456",
                "file340282366920938463463374607431768211455"
            ),
            Ordering::Greater
        );

        // Different length numbers that overflow u128
        assert_eq!(
            natural_sort(
                "file3402823669209384634633746074317682114560",
                "file340282366920938463463374607431768211455"
            ),
            Ordering::Greater
        );

        // Leading zeros with numbers near u128::MAX
        assert_eq!(
            natural_sort(
                "file0340282366920938463463374607431768211455",
                "file340282366920938463463374607431768211455"
            ),
            Ordering::Greater
        );

        // Very large numbers with different lengths (both overflow u128)
        assert_eq!(
            natural_sort(
                "file999999999999999999999999999999999999999999999999",
                "file9999999999999999999999999999999999999999999999999"
            ),
            Ordering::Less
        );

        // Mixed case with numbers
        assert_eq!(natural_sort("File1", "file2"), Ordering::Greater);
        assert_eq!(natural_sort("file1", "File2"), Ordering::Less);
    }

    #[test]
    fn test_natural_sort_edge_cases() {
        // Empty strings
        assert_eq!(natural_sort("", ""), Ordering::Equal);
        assert_eq!(natural_sort("", "a"), Ordering::Less);
        assert_eq!(natural_sort("a", ""), Ordering::Greater);

        // Special characters
        assert_eq!(natural_sort("file-1", "file_1"), Ordering::Less);
        assert_eq!(natural_sort("file.1", "file_1"), Ordering::Less);
        assert_eq!(natural_sort("file 1", "file_1"), Ordering::Less);

        // Unicode characters
        // 9312 vs 9313
        assert_eq!(natural_sort("file①", "file②"), Ordering::Less);
        // 9321 vs 9313
        assert_eq!(natural_sort("file⑩", "file②"), Ordering::Greater);
        // 28450 vs 23383
        assert_eq!(natural_sort("file漢", "file字"), Ordering::Greater);

        // Mixed alphanumeric with special chars
        assert_eq!(natural_sort("file-1a", "file-1b"), Ordering::Less);
        assert_eq!(natural_sort("file-1.2", "file-1.10"), Ordering::Less);
        assert_eq!(natural_sort("file-1.10", "file-1.2"), Ordering::Greater);
    }
}
