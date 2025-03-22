//! Logic for when the hovered word looks like a path (depending on how hard you squint).
//!
//! # Possible future improvements
//!
//! - Only match git diff if line starts with `+++ a/` and treat the whole rest of the line as the path
//! - Support chunk line navigation in git diff output, e.g. `@@ <line>,<lines> @@`
//! and `+ blah`.
//! ```
//! --- a/TODO.md
//! +++ b/TODO.md
//! @@ -15,7 +15,7 @@
//!   blah
//! + blah
//!   blah
//! ```
//! - Support navigation to line in rust diagnostic output, e.g. from the 'gutter'
//! ```
//!    --> Something bad happened here:
//! 200 |
//! 201 |
//!     |
//! ... |
//! 400 |
//! 401 |
//! ```
//! - Support escapes in paths, e.g. git octal escaping
//! See [core.quotePath](https://git-scm.com/docs/git-config#Documentation/git-config.txt-corequotePath)
//! > Double-quotes, backslash and control characters are always escaped
//! > regardless of the setting of this variable.". Currently we don't support any
//! > escaping in paths, so these currently do not work.
//!
//! # TODOs
//! ## [Cmd+click to linkify file in terminal doesn't work when there are whitespace or certain separators in the filename](https://github.com/zed-industries/zed/issues/12338)
//!
//! - [ ] Clear `last_hovered_*` when terminal content changes. See comment at `point_within_last_hovered`
//! - [ ] best_heuristic_hovered_word currently causes false positives to flicker e.g., they get linkified
//! immediately, then get clear once we confirm they are not paths. Maybe this is fine? But I think we
//! should just not hyperlink maybe path like things until they are confirmed.
//! - [ ] Add many more tests

#[cfg(doc)]
use super::WORD_REGEX;
use crate::{HoveredWord, ZedListener};
use alacritty_terminal::{index::Boundary, term::search::Match, Term};
use fancy_regex::{Captures, Regex};
use itertools::Itertools;
use log::{debug, info, trace};
use std::{
    borrow::Borrow,
    fmt::Display,
    ops::{Deref, Range},
    sync::LazyLock,
};
use unicode_segmentation::UnicodeSegmentation;

/// These are valid in paths and are not matched by [WORD_REGEX].
/// We use them to find potential paths within a line.
///
/// - **`\u{c}`** is **`\f`** (form feed - new page)
/// - **`\u{b}`** is **`\v`** (vertical tab)
///
/// See [C++ Escape sequences](https://en.cppreference.com/w/cpp/language/escape)
pub const MAIN_SEPARATORS: [char; 2] = ['\\', '/'];

/// Common symbols which often surround a path, e.g., `"` `'` `[` `]` `(` `)`
pub const COMMON_PATH_SURROUNDING_SYMBOLS: &[(char, char)] =
    &[('"', '"'), ('\'', '\''), ('[', ']'), ('(', ')')];

pub fn has_common_surrounding_symbols(maybe_path: &str) -> bool {
    for (start, end) in COMMON_PATH_SURROUNDING_SYMBOLS {
        if maybe_path.starts_with(*start) && maybe_path.ends_with(*end) {
            return true;
        }
    }
    false
}

/// Returns the longest range of matching surrounding symbols on `line` which contains `word_range`
pub fn longest_surrounding_symbols_match(
    line: &str,
    word_range: &Range<usize>,
) -> Option<Range<usize>> {
    let mut longest = None::<Range<usize>>;

    let surrounds_word = |current: &Range<usize>| {
        current.contains(&word_range.start) && current.contains(&(word_range.end - 1))
    };

    for (start, end) in COMMON_PATH_SURROUNDING_SYMBOLS {
        if let (Some(first), Some(last)) = (line.find(*start), line.rfind(*end)) {
            if first < last {
                let current = first..last + 1;
                if surrounds_word(&current) {
                    if let Some(longest_so_far) = &longest {
                        if current.len() > longest_so_far.len() {
                            longest = Some(current);
                        }
                    } else {
                        longest = Some(current);
                    };
                }
            }
        }
    }

    longest
}

#[cfg(target_os = "windows")]
macro_rules! path_char {
    () => {
        r#"[^<>"|?*:]"#
    };
}

#[cfg(target_os = "windows")]
macro_rules! path_char_msbuild {
    () => {
        r#"[^<>"|?*\(]"#
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! path_char {
    () => {
        r#"[^:]"#
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! path_char_msbuild {
    () => {
        r#"[^\(]"#
    };
}

macro_rules! row_column_desc_regex {
    ($path_char_regex:ident) => {
        concat!(
            r#"(?<path>"#,
            $path_char_regex!(),
            r#"+)((?<suffix>(:(?<line>[0-9]+))(:(?<column>[0-9]+))?)(:(?=[^0-9])(?<desc>.*)$|$))?"#
        )
    };
}

macro_rules! row_column_desc_regex_msbuild {
    ($path_char_regex:ident) => {
        concat!(
            r#"(?<path>"#,
            $path_char_regex!(),
           r#"+)((?<suffix>(\((?<line>[0-9]+))([,:](?<column>[0-9]+))?\))(:(?=[^0-9])(?<desc>.*)$|$))?"#
        )
    };
}

// If there is a word on the line that contains a colon that word up to (but not including)
// its last colon, it is treated as a maybe path.
// e.g., Ruby (see https://github.com/zed-industries/zed/issues/25086)
const PATH_ROW_COLUMN_DESC_REGEX: &str = row_column_desc_regex!(path_char);
const PATH_ROW_COLUMN_DESC_REGEX_MSBUILD: &str = row_column_desc_regex_msbuild!(path_char_msbuild);

const PREAPPROVED_PATH_HYPERLINK_REGEXES: [&str; 2] = [
    PATH_ROW_COLUMN_DESC_REGEX,
    PATH_ROW_COLUMN_DESC_REGEX_MSBUILD,
];

pub fn load_path_hyperlink_regexes<'a, T>(regexes: &'a T) -> Vec<Regex>
where
    &'a T: IntoIterator<
        Item: Deref<Target: Borrow<str>> + std::fmt::Debug,
        IntoIter: ExactSizeIterator,
    >,
{
    // Common prefix for diagnostic log messages
    macro_rules! error_prefix {
        ($regex:expr) => {
            format!(
                "Failed to load a path hyperlink regex, {:?}
, specified in 'terminal.path_hyperlink_regexes' setting, ",
                $regex
            )
        };
    }

    let regexes = regexes.into_iter();
    let mut loaded_regexes = Vec::<Regex>::with_capacity(regexes.len());
    for regex in regexes {
        let Ok(regex) = Regex::new(regex.deref().borrow())
            .inspect_err(|err| info!("{} error was: {err:?}", error_prefix!(regex)))
        else {
            continue;
        };

        if regex
            .capture_names()
            .flatten()
            .find(|&name| name == "path")
            .is_none()
        {
            info!(
                "{} missing required `path` named capture group.",
                error_prefix!(regex)
            );
            continue;
        }

        loaded_regexes.push(regex);
    }

    loaded_regexes
}

/// Returns a list of the preapproved path hyperlink regexes
pub fn preapproved_path_hyperlink_regexes() -> &'static Vec<Regex> {
    static PREAPPROVED_MAYBE_PATH_REGEXES: LazyLock<Vec<Regex>> =
        LazyLock::new(|| load_path_hyperlink_regexes(&PREAPPROVED_PATH_HYPERLINK_REGEXES));

    &PREAPPROVED_MAYBE_PATH_REGEXES
}

/// Line and column suffix information
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct RowColumn {
    pub row: u32,
    pub column: Option<u32>,
    /// Storing the length of the suffix here allows us to linkify it correctly.
    pub suffix_length: usize,
}

/// Returns the first match with a valid line number, if any.
pub fn path_with_position_regex_match<'a>(
    maybe_path: &'a str,
    path_regexes: &Vec<&'a Regex>,
) -> Option<(Range<usize>, RowColumn)> {
    fn position_from_captures<'t>(captures: &Captures<'t>) -> Option<RowColumn> {
        let Some(row) = captures
            .name("line")
            .filter(|row| row.range().len() > 0)
            .map(|row| row.as_str().parse::<u32>().ok())
            .flatten()
        else {
            return None;
        };

        let Some(suffix) = captures.name("suffix") else {
            return None;
        };

        let suffix_length = suffix.range().len();

        let column = captures
            .name("column")
            .filter(|column| column.range().len() > 0)
            .map(|column| column.as_str().parse::<u32>().ok())
            .flatten();

        Some(RowColumn {
            row,
            column,
            suffix_length,
        })
    }

    for path_regex in path_regexes {
        let Ok(Some(captures)) = path_regex.captures(&maybe_path) else {
            continue;
        };

        // Note: Do NOT use captures["path"] here because it can panic. This is extra
        // paranoid because we don't load path regexes that do not contain a path
        // named capture group in the first place (see [init_path_hyperlink_regexes]).
        let Some(path_match) = captures.name("path") else {
            debug!(
                "'path' capture not matched in regex: {:#?}",
                path_regex.as_str()
            );
            continue;
        };

        if let Some(position) = position_from_captures(&captures) {
            return Some((path_match.range(), position));
        }
    }

    None
}

/// The hovered or Cmd-clicked word in the terminal
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaybePathLike {
    line: String,
    word_range: Range<usize>,
    word_match: Match,
}

impl Display for MaybePathLike {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.word_range.start != 0 || self.word_range.end != self.line.len() {
            formatter.write_fmt(format_args!(
                "{:?} «{}»",
                self,
                &self.line[self.word_range.clone()]
            ))
        } else {
            formatter.write_fmt(format_args!("{:?}", self))
        }
    }
}

impl MaybePathLike {
    /// For file IRIs, the IRI is always the 'line'
    pub(super) fn from_file_url(file_iri: &str, file_iri_match: &Match) -> Self {
        Self {
            line: file_iri.to_string(),
            word_range: 0..file_iri.len(),
            word_match: file_iri_match.clone(),
        }
    }

    pub(super) fn from_hovered_word_match<T>(term: &mut Term<T>, word_match: &Match) -> Self {
        let word = term.bounds_to_string(*word_match.start(), *word_match.end());
        let line_start = term.line_search_left(*word_match.start());
        let mut line = if line_start == *word_match.start() {
            String::new()
        } else {
            term.bounds_to_string(line_start, word_match.start().sub(term, Boundary::Grid, 1))
        };
        let word_start = line.len();
        line.push_str(&word);
        let word_end = line.len();
        let line_end = term.line_search_right(*word_match.end());
        let remainder = if line_end == *word_match.end() {
            String::new()
        } else {
            term.bounds_to_string(word_match.end().add(term, Boundary::Grid, 1), line_end)
        };
        line.push_str(&remainder);

        MaybePathLike::from_line_and_word_range(line, word_start..word_end, word_match)
    }

    fn from_line_and_word_range(
        line: String,
        word_range: Range<usize>,
        word_match: &Match,
    ) -> Self {
        Self {
            line,
            word_range,
            word_match: word_match.clone(),
        }
    }

    pub fn to_line_and_word_range(&self) -> (String, Range<usize>) {
        (self.line.clone(), self.word_range.clone())
    }

    pub fn text_at(&self, range: &Range<usize>) -> &str {
        &self.line[range.clone()]
    }

    pub fn hyperlink_range<'a>(
        &self,
        position: &Option<RowColumn>,
        range: &Range<usize>,
    ) -> Range<usize> {
        match position.as_ref() {
            Some(RowColumn { suffix_length, .. })
                if range.start > 0 && range.end < self.line.len() =>
            {
                if has_common_surrounding_symbols(&self.line[range.start - 1..range.end + 1]) {
                    range.start - 1..range.end + 1 + suffix_length
                } else {
                    range.start..range.end + suffix_length
                }
            }
            Some(_) | None => range.clone(),
        }
    }

    /// Computes the best heuristic match for link highlighting in the terminal. This
    /// will be linkified immediately even though we don't yet know if it is a real path.
    /// Once we've determined (in the background) if it is a real path, the hyperlink
    /// will be updated to the real path if a real path was found, or cleared if not.
    pub(super) fn best_heuristic_hovered_word(
        &self,
        term: &mut Term<ZedListener>,
    ) -> Option<HoveredWord> {
        if let Some(surrounding_range) =
            longest_surrounding_symbols_match(&self.line, &self.word_range)
        {
            let stripped_range = surrounding_range.start + 1..surrounding_range.end - 1;
            trace!(
                "Maybe path heuristic 'longest surrounding symbols' match: {:?}",
                self.text_at(&stripped_range)
            );
            Some(HoveredWord {
                word: self.text_at(&stripped_range).to_string(),
                word_match: self.match_from_text_range(term, &stripped_range),
            })
        } else if self.looks_like_a_path_match() {
            trace!(
                "Maybe path heuristic 'looks like a path' match: {:?}",
                &self.line[self.word_range.clone()]
            );
            Some(HoveredWord {
                word: self.line[self.word_range.clone()].to_string(),
                word_match: self.word_match.clone(),
            })
        } else if let Some((range, position)) = path_with_position_regex_match(
            &self.line[self.word_range.clone()],
            &preapproved_path_hyperlink_regexes().iter().collect_vec(),
        ) {
            let word_range = self.word_range.start + range.start
                ..self.word_range.start + range.end + position.suffix_length;
            let word = self.text_at(&word_range).to_string();
            trace!(
                "Maybe path heuristic 'path regex' match: path = {word:?}, position = {position:?}",
            );
            Some(HoveredWord {
                word,
                word_match: self.match_from_text_range(term, &word_range),
            })
        } else {
            None
        }
    }

    fn looks_like_a_path_match(&self) -> bool {
        static LOOKS_LIKE_A_PATH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(
                r#"(?x)
                    ^\.            # does not start with a period
                    |
                    ^[a-zA-Z]:     # starts with a Windows drive
                    |
                    [/\\]          # contains a path separator
                    |
                    \.[^\s]{1,5}$  # ends in an extension
                    "#,
            )
            .unwrap()
        });

        LOOKS_LIKE_A_PATH_REGEX
            .is_match(&self.line[self.word_range.clone()])
            .unwrap_or(false)
    }

    pub(super) fn match_from_text_range(
        &self,
        term: &mut Term<ZedListener>,
        text_range: &Range<usize>,
    ) -> Match {
        let start = if text_range.start > self.word_range.start {
            let adjust_start = self.line[self.word_range.start..text_range.start]
                .graphemes(true)
                .count();
            self.word_match
                .start()
                .add(term, Boundary::Grid, adjust_start)
        } else if text_range.start < self.word_range.start {
            let adjust_start = self.line[text_range.start..self.word_range.start]
                .graphemes(true)
                .count();
            self.word_match
                .start()
                .sub(term, Boundary::Grid, adjust_start)
        } else {
            self.word_match.start().clone()
        };

        let end = if text_range.end > self.word_range.end {
            let adjust_end = self.line[self.word_range.end..text_range.end]
                .graphemes(true)
                .count();
            self.word_match.end().add(term, Boundary::Grid, adjust_end)
        } else if text_range.end < self.word_range.end {
            let adjust_end = self.line[text_range.end..self.word_range.end]
                .graphemes(true)
                .count();
            self.word_match.end().sub(term, Boundary::Grid, adjust_end)
        } else {
            self.word_match.end().clone()
        };

        Match::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use alacritty_terminal::index::{Column, Line, Point as AlacPoint};

    use super::*;

    fn re_test_row_col_desc(
        regex: &Regex,
        hay: &str,
        expected_path: Option<&str>,
        expected_suffix: Option<&str>,
        expected_line: Option<&str>,
        expected_column: Option<&str>,
        expected_desc: Option<&str>,
    ) {
        let Some(Ok(captures)) = regex.captures_iter(hay).next() else {
            if let Some(expected_path) = expected_path {
                assert!(
                    false,
                    "Expected path = {:?}, line = {:?}, column = {:?}, desc = {:?} for: {:?}",
                    expected_path, expected_line, expected_column, expected_desc, hay,
                );
            }
            return;
        };
        if let Some(path) = captures.name("path") {
            assert_eq!(Some(path.as_str()), expected_path, "{}", hay);
        } else {
            assert!(expected_path.is_none(), "{}", hay);
        }
        if let Some(suffix) = captures.name("suffix") {
            assert_eq!(Some(suffix.as_str()), expected_suffix, "{}", hay);
        } else {
            assert!(expected_suffix.is_none(), "{}", hay);
        }
        if let Some(line) = captures.name("line") {
            assert_eq!(Some(line.as_str()), expected_line, "{}", hay);
        } else {
            assert!(expected_line.is_none(), "{}", hay);
        }
        if let Some(column) = captures.name("column") {
            assert_eq!(Some(column.as_str()), expected_column, "{}", hay);
        } else {
            assert!(expected_column.is_none(), "{}", hay);
        }
        if let Some(desc) = captures.name("desc") {
            assert_eq!(Some(desc.as_str()), expected_desc, "{}", hay);
        } else {
            assert!(expected_desc.is_none(), "{}", hay);
        }
    }

    // Ruby (see https://github.com/zed-industries/zed/issues/25086)
    #[test]
    fn test_row_column_description_regex_25086() {
        let regex = Regex::new(PATH_ROW_COLUMN_DESC_REGEX).unwrap();
        let re_test = |hay, path, suffix, line, column, desc| {
            re_test_row_col_desc(&regex, hay, path, suffix, line, column, desc)
        };
        re_test(
            "Main.cs:20:5:Error",
            Some("Main.cs"),
            Some(":20:5"),
            Some("20"),
            Some("5"),
            Some("Error"),
        );
        re_test(
            "Main.cs:20:5 Error",
            Some("Main.cs"),
            None,
            None,
            None,
            None,
        );
        re_test(
            "Main.cs:20:Error",
            Some("Main.cs"),
            Some(":20"),
            Some("20"),
            None,
            Some("Error"),
        );
        re_test("Main.cs:Error", Some("Main.cs"), None, None, None, None);
        re_test(
            "Main.cs:20:5",
            Some("Main.cs"),
            Some(":20:5"),
            Some("20"),
            Some("5"),
            None,
        );
        re_test(
            "Main.cs:20",
            Some("Main.cs"),
            Some(":20"),
            Some("20"),
            None,
            None,
        );
        re_test("Main.cs", Some("Main.cs"), None, None, None, None);

        let regex = Regex::new(PATH_ROW_COLUMN_DESC_REGEX_MSBUILD).unwrap();
        let re_test = |hay, path, suffix, line, column, desc| {
            re_test_row_col_desc(&regex, hay, path, suffix, line, column, desc)
        };
        re_test(
            "Main.cs(20:5):Error",
            Some("Main.cs"),
            Some("(20:5)"),
            Some("20"),
            Some("5"),
            Some("Error"),
        );
        re_test(
            "Main.cs(20,5) Error",
            Some("Main.cs"),
            None,
            None,
            None,
            None,
        );
        re_test(
            "Main.cs(20):Error",
            Some("Main.cs"),
            Some("(20)"),
            Some("20"),
            None,
            Some("Error"),
        );
        re_test(
            "Main.cs:Error",
            Some("Main.cs:Error"),
            None,
            None,
            None,
            None,
        );
        re_test(
            "Main.cs(20:5)",
            Some("Main.cs"),
            Some("(20:5)"),
            Some("20"),
            Some("5"),
            None,
        );
        re_test(
            "Main.cs(20)",
            Some("Main.cs"),
            Some("(20)"),
            Some("20"),
            None,
            None,
        );
        re_test("Main.cs", Some("Main.cs"), None, None, None, None);
    }

    #[test]
    fn test_looks_like_a_path() {
        let match_range: Match = Match::new(
            AlacPoint::new(Line(0), Column(0)),
            AlacPoint::new(Line(0), Column(0)),
        );

        macro_rules! test_looks_like_a_path {
            ($prefix:literal, $word:literal, $suffix:literal, $looks_like:ident) => { {
                let maybe_path_like = MaybePathLike::from_line_and_word_range(
                    concat!($prefix, $word, $suffix).to_string(),
                    $prefix.len()..$prefix.len() + $word.len(),
                    &match_range,
                );

                assert_eq!($looks_like, maybe_path_like.looks_like_a_path_match(),
                    "Expected '{}' for \"{}\"", $looks_like, concat!($prefix, $word, $suffix));
            } };

            () => {};

            ($looks_like:ident @ $([ $prefix:literal, $word:literal, $suffix:literal ] $(,)?)+) => {
                $(test_looks_like_a_path!($prefix, $word, $suffix, $looks_like);)+
            };
        }

        test_looks_like_a_path!(true @
            ["Wow, so ", "C:ool!", ", dude."],
            ["Wow, so ", "C.ool!", ", dude."],
            ["Wow, so ", ".zprofyle", ", dude."],
            ["Wow, so ", "o\\ol!", ", dude."],
            ["Wow, so ", "o/ol!", ", dude."],
        );

        test_looks_like_a_path!(false @
            ["Wow, so not ", "Cool!", " Oh, well"],
            ["Wow, so not ", "Cool.", " Oh, well"],
            ["Wow, so not ", "Cool!(3,4)", " Oh, well"],
            ["Wow, so not ", "Co.ooooooool!", " Oh, well"],
        );
    }
}
