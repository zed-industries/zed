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
use crate::{terminal_settings::PathHyperlinkNavigation, HoveredWord, ZedListener};
use alacritty_terminal::{index::Boundary, term::search::Match, Term};
use log::{debug, info, trace};
use regex::Regex;
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
macro_rules! default_path_chars {
    () => {
        r#"[^\s<>"|?*]+?"#
    };
}

#[cfg(target_os = "windows")]
macro_rules! default_path_chars_msvc {
    () => {
        r#"[^\s<>"|?*\(]+"#
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! default_path_chars {
    () => {
        r#"[^\s]+?"#
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! default_path_chars_msvc {
    () => {
        r#"[^\s\(]+"#
    };
}

#[cfg(target_os = "windows")]
macro_rules! advanced_path_chars {
    () => {
        r#"[^<>"|?*]+?"#
    };
}

#[cfg(target_os = "windows")]
macro_rules! advanced_path_chars_msvc {
    () => {
        r#"[^<>"|?*\(]+"#
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! advanced_path_chars {
    () => {
        r#".+?"#
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! advanced_path_chars_msvc {
    () => {
        r#"[^\(]+"#
    };
}

// If there is a word on the line that contains a colon that word up to (but not including)
// its last colon, it is treated as a maybe path.
// e.g., Ruby (see https://github.com/zed-industries/zed/issues/25086)
//
// Note that unlike the original fix for that issue, we don't check the characters before
// and after the colon for digit-ness so that in case the line and column suffix is in
// MSVC-style (<line>,<column>):message or some other style. Line and column suffixes are
// processed later in termainl_view.
const DEFAULT_PATH_ROW_COLUMN_DESC_REGEX: &str = concat!(
    r#"(?x)
    (?<path>
    (?:"#,
    default_path_chars_msvc!(),
    r#")(?:
        \((?:\d+)[,:](?:\d+)\) # path(row,column), path(row:column)
        |
        \((?:\d+)\)            # path(row)
    )
    |
    (?:"#,
    default_path_chars!(),
    r#")(?:
        \:+(?:\d+)\:(?:\d+)    # path:row:column
        |
        \:+(?:\d+)             # path:row
    ))
    :(?<desc>[^\d].+)$         # desc
    "#
);

/// Like, DEFAULT_PATH_ROW_COLUMN_DESC_REGEX, but allows spaces in the path
const ADVANCED_PATH_ROW_COLUMN_DESC_REGEX: &str = concat!(
    r#"(?x)
    (?<path>
    (?:"#,
    advanced_path_chars_msvc!(),
    r#")(?:
        \((?:\d+)[,:](?:\d+)\) # path(row,column), path(row:column)
        |
        \((?:\d+)\)            # path(row)
    )
    |
    (?:"#,
    advanced_path_chars!(),
    r#")(?:
        \:+(?:\d+)\:(?:\d+)    # path:row:column
        |
        \:+(?:\d+)             # path:row
    ))
    :(?<desc>[^\d].+)$         # desc
    "#
);

const DEFAULT_PREAPPROVED_PATH_HYPERLINK_REGEXES: [&str; 1] = [DEFAULT_PATH_ROW_COLUMN_DESC_REGEX];

const ADVANCED_PREAPPROVED_PATH_HYPERLINK_REGEXES: [&str; 1] =
    [ADVANCED_PATH_ROW_COLUMN_DESC_REGEX];

/// Used on user settings provided regexes
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
pub fn preapproved_path_hyperlink_regexes(
    path_hyperlink_navigation: PathHyperlinkNavigation,
) -> &'static Vec<Regex> {
    static DEFAULT_PREAPPROVED_MAYBE_PATH_REGEXES: LazyLock<Vec<Regex>> =
        LazyLock::new(|| load_path_hyperlink_regexes(&DEFAULT_PREAPPROVED_PATH_HYPERLINK_REGEXES));
    static ADVANCED_PREAPPROVED_MAYBE_PATH_REGEXES: LazyLock<Vec<Regex>> =
        LazyLock::new(|| load_path_hyperlink_regexes(&ADVANCED_PREAPPROVED_PATH_HYPERLINK_REGEXES));

    if path_hyperlink_navigation == PathHyperlinkNavigation::Default {
        &DEFAULT_PREAPPROVED_MAYBE_PATH_REGEXES
    } else {
        &ADVANCED_PREAPPROVED_MAYBE_PATH_REGEXES
    }
}

#[derive(Eq, PartialEq)]
pub enum PathRegexSearchMode {
    StopOnFirstMatch,
    ReturnAllMatches,
}

pub fn path_regex_match<'a>(
    maybe_path: &'a str,
    path_regex_search_mode: PathRegexSearchMode,
    path_regexes: &'a Vec<Regex>,
) -> impl Iterator<Item = Range<usize>> + 'a {
    path_regexes
        .iter()
        .filter_map(move |regex| {
            let Some(captures) = regex.captures(&maybe_path) else {
                return None;
            };
            // Note: Do NOT use captures["path"] here because it can panic. This is extra
            // paranoid because we don't load path regexes that do not contain a path
            // named capture group in the first place (see [init_path_hyperlink_regexes]).
            let Some(path_capture) = captures.name("path") else {
                debug!("'path' capture not matched in regex: {:#?}", regex.as_str());
                return None;
            };

            return Some(path_capture.range());
        })
        .take(
            if path_regex_search_mode == PathRegexSearchMode::StopOnFirstMatch {
                1
            } else {
                usize::MAX
            },
        )
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

    /// Computes the best heuristic match for link highlighting in the terminal. This
    /// will be linkified immediately even though we don't yet know if it is a real path.
    /// Once we've determined (in the background) is it is a real path, the hyperlink
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
        } else if let Some(path_range) = path_regex_match(
            &self.line[self.word_range.clone()],
            PathRegexSearchMode::StopOnFirstMatch,
            &preapproved_path_hyperlink_regexes(PathHyperlinkNavigation::Default),
        )
        .nth(0)
        {
            trace!(
                "Maybe path heuristic 'path regex' match: {:?}",
                self.text_at(&path_range)
            );
            Some(HoveredWord {
                word: self.text_at(&path_range).to_string(),
                word_match: self.match_from_text_range(term, &path_range),
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
                    ^[a-zA-Z]:     # starts with a window drive
                    |
                    [/\\]          # contains a path separator
                    |
                    \.[^\s]{1,5}$  # ends in an extension
                    "#,
            )
            .unwrap()
        });

        LOOKS_LIKE_A_PATH_REGEX.is_match(&self.line[self.word_range.clone()])
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

    fn re_test_row_col_desc(hay: &str, expected: Option<(&str, &str)>) {
        let regex = regex::Regex::new(DEFAULT_PATH_ROW_COLUMN_DESC_REGEX).unwrap();
        if let Some((_, [path, desc])) = regex.captures_iter(hay).map(|c| c.extract()).next() {
            let Some((expected_path, expected_desc)) = expected else {
                assert!(
                    false,
                    "Expected no path = \"{}\" and desc = \"{}\" for: \"{}\"",
                    path, desc, hay
                );
                return;
            };
            assert_eq!(path, expected_path);
            assert_eq!(desc, expected_desc);
        } else if let Some((expected_path, expected_desc)) = expected {
            assert!(
                false,
                "Expected path = \"{}\" and desc = \"{}\" for: \"{}\"",
                hay, expected_path, expected_desc
            );
        };
    }

    // Ruby (see https://github.com/zed-industries/zed/issues/25086)
    #[test]
    fn test_row_column_description_regex_25086() {
        re_test_row_col_desc(
            "# Main.cs:20:5:Error desc",
            Some(("Main.cs:20:5", "Error desc")),
        );
        re_test_row_col_desc(
            "# Main.cs(20,5):Error desc",
            Some(("Main.cs(20,5)", "Error desc")),
        );
        re_test_row_col_desc(
            "# Ma:n.cs:20:5:Error desc",
            Some(("Ma:n.cs:20:5", "Error desc")),
        );
        re_test_row_col_desc(
            "# Ma(n.cs(20,5):Error desc",
            Some(("n.cs(20,5)", "Error desc")),
        );
        re_test_row_col_desc("# Main.cs:20:5 Error desc", None);
        re_test_row_col_desc("# Main.cs(20,5) Error desc", None);
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
