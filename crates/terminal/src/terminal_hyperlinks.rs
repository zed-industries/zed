use alacritty_terminal::{
    Term,
    event::EventListener,
    grid::Dimensions,
    index::{Boundary, Column, Direction as AlacDirection, Line, Point as AlacPoint},
    term::search::{Match, RegexIter, RegexSearch},
};
use regex::Regex;
use std::{ops::Index, sync::LazyLock};

const URL_REGEX: &str = r#"(ipfs:|ipns:|magnet:|mailto:|gemini://|gopher://|https://|http://|news:|file://|git://|ssh:|ftp://)[^\u{0000}-\u{001F}\u{007F}-\u{009F}<>"\s{-}\^⟨⟩`]+"#;
// Optional suffix matches MSBuild diagnostic suffixes for path parsing in PathLikeWithPosition
// https://learn.microsoft.com/en-us/visualstudio/msbuild/msbuild-diagnostic-format-for-tasks
const WORD_REGEX: &str =
    r#"[\$\+\w.\[\]:/\\@\-~()]+(?:\((?:\d+|\d+,\d+)\))|[\$\+\w.\[\]:/\\@\-~()]+"#;

const PYTHON_FILE_LINE_REGEX: &str = r#"File "(?P<file>[^"]+)", line (?P<line>\d+)"#;

static PYTHON_FILE_LINE_MATCHER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(PYTHON_FILE_LINE_REGEX).unwrap());

fn python_extract_path_and_line(input: &str) -> Option<(&str, u32)> {
    if let Some(captures) = PYTHON_FILE_LINE_MATCHER.captures(input) {
        let path_part = captures.name("file")?.as_str();

        let line_number: u32 = captures.name("line")?.as_str().parse().ok()?;
        return Some((path_part, line_number));
    }
    None
}

pub(super) struct HyperlinkFinder {
    url_regex: RegexSearch,
    word_regex: RegexSearch,
    python_file_line_regex: RegexSearch,
}

impl HyperlinkFinder {
    pub(super) fn new() -> Self {
        Self {
            url_regex: RegexSearch::new(URL_REGEX).unwrap(),
            word_regex: RegexSearch::new(WORD_REGEX).unwrap(),
            python_file_line_regex: RegexSearch::new(PYTHON_FILE_LINE_REGEX).unwrap(),
        }
    }

    pub(super) fn find_from_grid_point<T: EventListener>(
        &mut self,
        term: &Term<T>,
        point: AlacPoint,
    ) -> Option<(String, bool, Match)> {
        let grid = term.grid();
        let link = grid.index(point).hyperlink();
        let found_word = if link.is_some() {
            let mut min_index = point;
            loop {
                let new_min_index = min_index.sub(term, Boundary::Cursor, 1);
                if new_min_index == min_index || grid.index(new_min_index).hyperlink() != link {
                    break;
                } else {
                    min_index = new_min_index
                }
            }

            let mut max_index = point;
            loop {
                let new_max_index = max_index.add(term, Boundary::Cursor, 1);
                if new_max_index == max_index || grid.index(new_max_index).hyperlink() != link {
                    break;
                } else {
                    max_index = new_max_index
                }
            }

            let url = link.unwrap().uri().to_owned();
            let url_match = min_index..=max_index;

            Some((url, true, url_match))
        } else if let Some(url_match) = regex_match_at(term, point, &mut self.url_regex) {
            let url = term.bounds_to_string(*url_match.start(), *url_match.end());
            Some((url, true, url_match))
        } else if let Some(python_match) =
            regex_match_at(term, point, &mut self.python_file_line_regex)
        {
            let matching_line = term.bounds_to_string(*python_match.start(), *python_match.end());
            python_extract_path_and_line(&matching_line).map(|(file_path, line_number)| {
                // TODO(davewa): Do we really want the hyperlink under `File ` (included by `python_match`)?
                (format!("{file_path}:{line_number}"), false, python_match)
            })
        } else if let Some(word_match) = regex_match_at(term, point, &mut self.word_regex) {
            // TODO(davewa): This code does several full char() scans of the path, when one
            // partial reverse bytes() scan would suffice. Replace with Regex.

            let file_path = term.bounds_to_string(*word_match.start(), *word_match.end());

            let (sanitized_match, sanitized_word) = 'sanitize: {
                let mut word_match = word_match;
                let mut file_path = file_path;

                if is_path_surrounded_by_common_symbols(&file_path) {
                    word_match = Match::new(
                        word_match.start().add(term, Boundary::Grid, 1),
                        word_match.end().sub(term, Boundary::Grid, 1),
                    );
                    file_path = file_path[1..file_path.len() - 1].to_owned();
                }

                while file_path.ends_with(':') {
                    file_path.pop();
                    word_match = Match::new(
                        *word_match.start(),
                        word_match.end().sub(term, Boundary::Grid, 1),
                    );
                }
                let mut colon_count = 0;
                for c in file_path.chars() {
                    if c == ':' {
                        colon_count += 1;
                    }
                }
                // strip trailing comment after colon in case of
                // file/at/path.rs:row:column:description or error message
                // so that the file path is `file/at/path.rs:row:column`
                if colon_count > 2 {
                    let last_index = file_path.rfind(':').unwrap();
                    let prev_is_digit = last_index > 0
                        && file_path
                            .chars()
                            .nth(last_index - 1)
                            .map_or(false, |c| c.is_ascii_digit());
                    let next_is_digit = last_index < file_path.len() - 1
                        && file_path
                            .chars()
                            .nth(last_index + 1)
                            .map_or(true, |c| c.is_ascii_digit());
                    if prev_is_digit && !next_is_digit {
                        let stripped_len = file_path.len() - last_index;
                        word_match = Match::new(
                            *word_match.start(),
                            word_match.end().sub(term, Boundary::Grid, stripped_len),
                        );
                        file_path = file_path[0..last_index].to_owned();
                    }
                }

                break 'sanitize (word_match, file_path);
            };

            Some((sanitized_word, false, sanitized_match))
        } else {
            None
        };

        found_word.map(|(maybe_url_or_path, is_url, word_match)| {
            if is_url {
                // Treat "file://" URLs like file paths to ensure
                // that line numbers at the end of the path are
                // handled correctly
                if let Some(path) = maybe_url_or_path.strip_prefix("file://") {
                    (path.to_string(), false, word_match)
                } else {
                    (maybe_url_or_path, true, word_match)
                }
            } else {
                (maybe_url_or_path, false, word_match)
            }
        })
    }
}

fn is_path_surrounded_by_common_symbols(path: &str) -> bool {
    // Avoid detecting `[]` or `()` strings as paths, surrounded by common symbols
    path.len() > 2
        // The rest of the brackets and various quotes cannot be matched by the [`WORD_REGEX`] hence not checked for.
        && (path.starts_with('[') && path.ends_with(']')
            || path.starts_with('(') && path.ends_with(')'))
}

/// Based on alacritty/src/display/hint.rs > regex_match_at
/// Retrieve the match, if the specified point is inside the content matching the regex.
fn regex_match_at<T>(term: &Term<T>, point: AlacPoint, regex: &mut RegexSearch) -> Option<Match> {
    visible_regex_match_iter(term, regex).find(|rm| rm.contains(&point))
}

/// Copied from alacritty/src/display/hint.rs:
/// Iterate over all visible regex matches.
// TODO(davewa): Why do we include all visible matches when we only ever accept
// matches on the line which contains point (see regex_match_at() above)? This
// seems like a (performance) bug.
fn visible_regex_match_iter<'a, T>(
    term: &'a Term<T>,
    regex: &'a mut RegexSearch,
) -> impl Iterator<Item = Match> + 'a {
    const MAX_SEARCH_LINES: usize = 100;

    let viewport_start = Line(-(term.grid().display_offset() as i32));
    let viewport_end = viewport_start + term.bottommost_line();
    let mut start = term.line_search_left(AlacPoint::new(viewport_start, Column(0)));
    let mut end = term.line_search_right(AlacPoint::new(viewport_end, Column(0)));
    start.line = start.line.max(viewport_start - MAX_SEARCH_LINES);
    end.line = end.line.min(viewport_end + MAX_SEARCH_LINES);

    RegexIter::new(start, end, AlacDirection::Right, term, regex)
        .skip_while(move |rm| rm.end().line < viewport_start)
        .take_while(move |rm| rm.start().line <= viewport_end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alacritty_terminal::{
        event::VoidListener,
        index::{Boundary, Point as AlacPoint},
        term::{Config, cell::Flags, test::TermSize},
        vte::ansi::Handler,
    };
    use itertools::Itertools;
    use std::{ops::RangeInclusive, path::PathBuf};
    use util::paths::PathWithPosition;

    fn re_test(re: &str, hay: &str, expected: Vec<&str>) {
        let results: Vec<_> = regex::Regex::new(re)
            .unwrap()
            .find_iter(hay)
            .map(|m| m.as_str())
            .collect();
        assert_eq!(results, expected);
    }
    #[test]
    fn test_url_regex() {
        re_test(
            URL_REGEX,
            "test http://example.com test mailto:bob@example.com train",
            vec!["http://example.com", "mailto:bob@example.com"],
        );
    }
    #[test]
    fn test_word_regex() {
        re_test(
            WORD_REGEX,
            "hello, world! \"What\" is this?",
            vec!["hello", "world", "What", "is", "this"],
        );
    }
    #[test]
    fn test_word_regex_with_linenum() {
        // filename(line) and filename(line,col) as used in MSBuild output
        // should be considered a single "word", even though comma is
        // usually a word separator
        re_test(WORD_REGEX, "a Main.cs(20) b", vec!["a", "Main.cs(20)", "b"]);
        re_test(
            WORD_REGEX,
            "Main.cs(20,5) Error desc",
            vec!["Main.cs(20,5)", "Error", "desc"],
        );
        // filename:line:col is a popular format for unix tools
        re_test(
            WORD_REGEX,
            "a Main.cs:20:5 b",
            vec!["a", "Main.cs:20:5", "b"],
        );
        // Some tools output "filename:line:col:message", which currently isn't
        // handled correctly, but might be in the future
        re_test(
            WORD_REGEX,
            "Main.cs:20:5:Error desc",
            vec!["Main.cs:20:5:Error", "desc"],
        );
    }

    #[test]
    fn test_python_file_line_regex() {
        re_test(
            PYTHON_FILE_LINE_REGEX,
            "hay File \"/zed/bad_py.py\", line 8 stack",
            vec!["File \"/zed/bad_py.py\", line 8"],
        );
        re_test(PYTHON_FILE_LINE_REGEX, "unrelated", vec![]);
    }

    #[test]
    fn test_python_file_line() {
        let inputs: Vec<(&str, Option<(&str, u32)>)> = vec![
            (
                "File \"/zed/bad_py.py\", line 8",
                Some(("/zed/bad_py.py", 8u32)),
            ),
            ("File \"path/to/zed/bad_py.py\"", None),
            ("unrelated", None),
            ("", None),
        ];
        let actual = inputs
            .iter()
            .map(|input| python_extract_path_and_line(input.0))
            .collect::<Vec<_>>();
        let expected = inputs.iter().map(|(_, output)| *output).collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    /// By default tests with terminal column counts of `[3, longest_line / 2, and longest_line + 1]`.
    /// Also accepts specific column count overrides as `test_hyperlink!(3, 4, 5; "some stuff")`
    ///
    /// 👉 := hovered on following char
    ///
    /// 👈 := hovered on wide char spacer of previous full width char
    ///
    /// **`‹›`** := **hyperlink** match
    ///
    /// **`«aaaaa»`** := **path** capture group
    ///
    /// **`«NN»`** := **row** or **column** capture group
    ///
    /// **NN @** := minimum terminal columns
    macro_rules! test_hyperlink {
        ($($lines:literal),+) => {
            let test_lines = vec![$($lines),+];
            let (total_chars, longest_line_chars) = test_lines
                .iter()
                .fold((0, 0), |state, line| {
                    let line_chars = line.chars().filter(|c| "‹«»›".find(*c).is_none()).count();
                    (state.0 + line_chars, std::cmp::max(state.1, line_chars))
                });
            // Alacritty has issues with 2 columns, use 3 as the minimum for now.
            test_hyperlink!([3, longest_line_chars / 2, longest_line_chars + 1]; total_chars; &test_lines)
        };
        ($($columns:literal),+; $($lines:literal),+) => {
            let test_lines = vec![$($lines),+];
            let total_chars = test_lines
                .iter()
                .fold(0, |state, line| {
                    state + line.chars().filter(|c| "‹«»›".find(*c).is_none()).count()
                });
            test_hyperlink!([ $($columns),+ ]; total_chars; &vec![$($lines),+])
        };
        ([ $($columns:expr),+ ]; $total_chars:expr; $lines:expr) => {{
            for columns in vec![ $($columns),+] {
                use crate::terminal_hyperlinks::tests::test_hyperlink;
                test_hyperlink(columns, $total_chars, $lines,
                    format!("{}:{}:{}", std::file!(), std::line!(), std::column!()));
            }
        }};
    }

    // TODO(davewa): More tests
    // - [x] Resize the terminal down to a few columns, to test matches that span multiple lines
    // - [x] MSBuild-style(line,column)
    // - [ ] Windows paths
    // - [ ] Urls

    #[test]
    fn simple() {
        // Rust paths
        // Just the path
        test_hyperlink!("‹«/👉test/cool.rs»›");
        test_hyperlink!("‹«/test/cool👉.rs»›");

        // path and line
        test_hyperlink!("‹«/👉test/cool.rs»:«4»›");
        test_hyperlink!("‹«/test/cool.rs»👉:«4»›");
        test_hyperlink!("‹«/test/cool.rs»:«👉4»›");
        test_hyperlink!("‹«/👉test/cool.rs»(«4»)›");
        test_hyperlink!("‹«/test/cool.rs»👉(«4»)›");
        test_hyperlink!("‹«/test/cool.rs»(«👉4»)›");
        test_hyperlink!("‹«/test/cool.rs»(«4»👉)›");

        // path, line, and column
        test_hyperlink!("‹«/👉test/cool.rs»:«4»:«2»›");
        test_hyperlink!("‹«/test/cool.rs»:«4»:«👉2»›");
        test_hyperlink!("‹«/👉test/cool.rs»(«4»,«2»)›");
        test_hyperlink!("‹«/test/cool.rs»(«4»👉,«2»)›");

        // path, line, column, and ':' suffix
        test_hyperlink!("‹«/👉test/cool.rs»:«4»:«2»›:");
        test_hyperlink!("‹«/test/cool.rs»:«4»:«👉2»›:");
        test_hyperlink!("‹«/👉test/cool.rs»(«4»,«2»)›:");
        test_hyperlink!("‹«/test/cool.rs»(«4»,«2»👉)›:");

        // path, line, column, and description
        test_hyperlink!("‹«/test/cool.rs»:«4»:«2»›👉:Error!");
        test_hyperlink!("‹«/test/cool.rs»:«4»:«2»›:👉Error!");
        test_hyperlink!("‹«/test/co👉ol.rs»(«4»,«2»)›:Error!");

        // Cargo output
        test_hyperlink!("    Compiling Cool 👉(‹«/test/Cool»›)");
        test_hyperlink!("    Compiling Cool (‹«/👉test/Cool»›)");
        test_hyperlink!("    Compiling Cool (‹«/test/Cool»›👉)");

        // Python
        test_hyperlink!("‹«awe👉some.py»›");

        test_hyperlink!("    ‹F👉ile \"«/awesome.py»\", line «42»›: Wat?");
        test_hyperlink!("    ‹File \"«/awe👉some.py»\", line «42»›: Wat?");
        test_hyperlink!("    ‹File \"«/awesome.py»👉\", line «42»›: Wat?");
        test_hyperlink!("    ‹File \"«/awesome.py»\", line «4👉2»›: Wat?");
    }

    #[test]
    fn colons_galore() {
        test_hyperlink!("‹«/test/co👉ol.rs»:«4»›");
        test_hyperlink!("‹«/test/co👉ol.rs»:«4»›:");
        test_hyperlink!("‹«/test/co👉ol.rs»:«4»:«2»›");
        test_hyperlink!("‹«/test/co👉ol.rs»:«4»:«2»›:");
        test_hyperlink!("‹«/test/co👉ol.rs»(«1»)›");
        test_hyperlink!("‹«/test/co👉ol.rs»(«1»)›:");
        test_hyperlink!("‹«/test/co👉ol.rs»(«1»,«618»)›");
        test_hyperlink!("‹«/test/co👉ol.rs»(«1»,«618»)›:");
        test_hyperlink!("‹«/test/co👉ol.rs»::«42»›");
        test_hyperlink!("‹«/test/co👉ol.rs»::«42»›:");
        test_hyperlink!("‹«/test/co👉ol.rs:4:2»(«1»,«618»)›");
        test_hyperlink!("‹«/test/co👉ol.rs»(«1»,«618»)›::");
    }

    #[test]
    fn word_wide_chars() {
        // Rust paths
        test_hyperlink!(4, 6, 12; "‹«/👉例/cool.rs»›");
        test_hyperlink!(4, 6, 12; "‹«/例👈/cool.rs»›");
        test_hyperlink!(4, 8, 16; "‹«/例/cool.rs»:«👉4»›");
        test_hyperlink!(4, 8, 16; "‹«/例/cool.rs»:«4»:«👉2»›");

        // Cargo output
        test_hyperlink!(4, 25, 30; "    Compiling Cool (‹«/👉例/Cool»›)");
        test_hyperlink!(4, 25, 30; "    Compiling Cool (‹«/例👈/Cool»›)");

        // Python
        test_hyperlink!(4, 11; "‹«👉例wesome.py»›");
        test_hyperlink!(4, 11; "‹«例👈wesome.py»›");
        test_hyperlink!(6, 15, 40; "    ‹File \"«/👉例wesome.py»\", line «42»›: Wat?");
        test_hyperlink!(6, 15, 40; "    ‹File \"«/例👈wesome.py»\", line «42»›: Wat?");
    }

    #[test]
    fn non_word_wide_chars() {
        // Mojo diagnostic message
        // TODO(davewa): I haven't ever run Mojo, this is assuming it uses the same format as Python.
        test_hyperlink!(4, 18, 38; "    ‹File \"«/awe👉some.🔥»\", line «42»›: Wat?");
        test_hyperlink!(4, 18, 38; "    ‹File \"«/awesome👉.🔥»\", line «42»›: Wat?");
        test_hyperlink!(4, 18, 38; "    ‹File \"«/awesome.👉🔥»\", line «42»›: Wat?");
        test_hyperlink!(4, 18, 38; "    ‹File \"«/awesome.🔥👈»\", line «42»›: Wat?");
    }

    /// These likely rise to the level of being worth fixing.
    mod issues {
        #[test]
        // We use custom columns in many tests to workaround this issue by ensuring a wrapped line
        // never ends on a wide char.
        //
        // Any wide char at the end of a wrapped line is buggy in alacritty.
        //
        // [davewa]: I feel like this is worth fixing, even if no one has reported it. It is most likely
        // in the category of people experiencing failures, but somewhat randomly and not really
        // understanding what situation is causing it to work or not work, which isn't a great experience,
        // even though it might not have been reported as an actual issue with a clear repro case.
        #[should_panic(expected = "Path = «例»")]
        fn issue_alacritty_bugs_with_wide_char_at_line_wrap() {
            // Rust paths
            test_hyperlink!("‹«/👉例/cool.rs»›");
            test_hyperlink!("‹«/例👈/cool.rs»›");
            test_hyperlink!("‹«/例/cool.rs»:«👉4»›");
            test_hyperlink!("‹«/例/cool.rs»:«4»:«👉2»›");

            // Cargo output
            test_hyperlink!("    Compiling Cool (‹«/👉例/Cool»›)");
            test_hyperlink!("    Compiling Cool (‹«/例👈/Cool»›)");

            // Python
            test_hyperlink!("‹«👉例wesome.py»›");
            test_hyperlink!("‹«例👈wesome.py»›");
            test_hyperlink!("    ‹File \"«/👉例wesome.py»\", line «42»›: Wat?");
            test_hyperlink!("    ‹File \"«/例👈wesome.py»\", line «42»›: Wat?");
        }

        #[test]
        #[should_panic(expected = "No hyperlink found")]
        fn issue_12338() {
            // Issue #12338
            test_hyperlink!(".rw-r--r--     0     staff 05-27 14:03 ‹«test👉、2.txt»›");
            test_hyperlink!(".rw-r--r--     0     staff 05-27 14:03 ‹«test、👈2.txt»›");
            test_hyperlink!(".rw-r--r--     0     staff 05-27 14:03 ‹«test👉。3.txt»›");
            test_hyperlink!(".rw-r--r--     0     staff 05-27 14:03 ‹«test。👈3.txt»›");

            // Rust paths
            test_hyperlink!("‹«/👉🏃/🦀.rs»›");
            test_hyperlink!("‹«/🏃👈/🦀.rs»›");
            test_hyperlink!("‹«/🏃/👉🦀.rs»:«4»›");
            test_hyperlink!("‹«/🏃/🦀👈.rs»:«4»:«2»›");

            // Cargo output
            test_hyperlink!("    Compiling Cool (‹«/👉🏃/Cool»›)");
            test_hyperlink!("    Compiling Cool (‹«/🏃👈/Cool»›)");

            // Python
            test_hyperlink!("‹«👉🏃wesome.py»›");
            test_hyperlink!("‹«🏃👈wesome.py»›");
            test_hyperlink!("    ‹File \"«/👉🏃wesome.py»\", line «42»›: Wat?");
            test_hyperlink!("    ‹File \"«/🏃👈wesome.py»\", line «42»›: Wat?");

            // Mojo
            test_hyperlink!("‹«/awe👉some.🔥»› is some good Mojo!");
            test_hyperlink!("‹«/awesome👉.🔥»› is some good Mojo!");
            test_hyperlink!("‹«/awesome.👉🔥»› is some good Mojo!");
            test_hyperlink!("‹«/awesome.🔥👈»› is some good Mojo!");
            test_hyperlink!("    ‹File \"«/👉🏃wesome.🔥»\", line «42»›: Wat?");
            test_hyperlink!("    ‹File \"«/🏃👈wesome.🔥»\", line «42»›: Wat?");
        }
    }

    /// Minor issues arguably not important enough to fix/workaround...
    mod nits {
        #[test]
        #[should_panic(expected = "Path = «/test/cool.rs(4»")]
        fn alacritty_bugs_with_two_columns() {
            test_hyperlink!(2; "‹«/👉test/cool.rs»(«4»)›");
            test_hyperlink!(2; "‹«/test/cool.rs»(«👉4»)›");
            test_hyperlink!(2; "‹«/test/cool.rs»(«4»,«👉2»)›");

            // Python
            test_hyperlink!(2; "‹«awe👉some.py»›");
        }

        #[test]
        #[should_panic(
            expected = "Path = «/test/cool.rs», line = 1, at grid cells (0, 0)..=(9, 0)"
        )]
        fn invalid_row_column_should_be_part_of_path() {
            test_hyperlink!("‹«/👉test/cool.rs:1:618033988749»›");
            test_hyperlink!("‹«/👉test/cool.rs(1,618033988749)»›");
        }

        #[test]
        #[should_panic(expected = "Path = «»")]
        fn colon_suffix_succeeds_in_finding_an_empty_maybe_path() {
            test_hyperlink!("‹«/test/cool.rs»:«4»:«2»›👉:", "What is this?");
            test_hyperlink!("‹«/test/cool.rs»(«4»,«2»)›👉:", "What is this?");
        }

        #[test]
        #[should_panic(expected = "Path = «/test/cool.rs»")]
        fn many_trailing_colons_should_be_parsed_as_part_of_the_path() {
            test_hyperlink!("‹«/test/cool.rs:::👉:»›");
            test_hyperlink!("‹«/te:st/👉co:ol.r:s:4:2::::::»›");
        }
    }

    struct ExpectedHyperlink {
        hovered_grid_point: AlacPoint,
        hovered_char: char,
        path_with_position: PathWithPosition,
        hyperlink_match: RangeInclusive<AlacPoint>,
    }

    fn build_term_from_test_lines<'a>(
        term_size: TermSize,
        test_lines: &(impl IntoIterator<Item = &'a str> + Clone),
    ) -> (Term<VoidListener>, ExpectedHyperlink) {
        #[derive(Eq, PartialEq)]
        enum HoveredState {
            HoveredScan,
            HoveredNextChar,
            Done,
        }

        #[derive(Eq, PartialEq)]
        enum MatchState {
            MatchScan,
            MatchNextChar,
            Match(AlacPoint),
            Done,
        }

        #[derive(Eq, PartialEq)]
        enum CapturesState {
            PathScan,
            PathNextChar,
            Path(AlacPoint),
            RowScan,
            Row(String),
            ColumnScan,
            Column(String),
            Done,
        }

        fn prev_input_point_from_term(term: &Term<VoidListener>) -> AlacPoint {
            let grid = term.grid();
            let cursor = &grid.cursor;
            let mut point = cursor.point;

            if !cursor.input_needs_wrap {
                point.column -= 1;
            }

            if grid.index(point).flags.contains(Flags::WIDE_CHAR_SPACER) {
                point.column -= 1;
            }

            point
        }

        let mut hovered_grid_point = AlacPoint::default();
        let mut hyperlink_match = AlacPoint::default()..=AlacPoint::default();
        let mut path_with_position = PathWithPosition::from_path(PathBuf::new());
        let mut prev_input_point = AlacPoint::default();
        let mut hovered_state = HoveredState::HoveredScan;
        let mut match_state = MatchState::MatchScan;
        let mut captures_state = CapturesState::PathScan;

        let mut term = Term::new(Config::default(), &term_size, VoidListener);

        for text in test_lines.clone().into_iter() {
            let text = text.chars().collect_vec();
            for index in 0..text.len() {
                match text[index] {
                    '👉' => {
                        hovered_state = HoveredState::HoveredNextChar;
                    }
                    '👈' => {
                        hovered_grid_point = prev_input_point.add(&term, Boundary::Grid, 1);
                    }
                    '«' | '»' => {
                        captures_state = match captures_state {
                            CapturesState::PathScan => CapturesState::PathNextChar,
                            CapturesState::PathNextChar => {
                                panic!("Should have been handled by char input")
                            }
                            CapturesState::Path(start_point) => {
                                path_with_position = PathWithPosition::from_path(PathBuf::from(
                                    &term.bounds_to_string(start_point, prev_input_point),
                                ));
                                CapturesState::RowScan
                            }
                            CapturesState::RowScan => CapturesState::Row(String::new()),
                            CapturesState::Row(number) => {
                                path_with_position.row = Some(number.parse::<u32>().unwrap());
                                CapturesState::ColumnScan
                            }
                            CapturesState::ColumnScan => CapturesState::Column(String::new()),
                            CapturesState::Column(number) => {
                                path_with_position.column = Some(number.parse::<u32>().unwrap());
                                CapturesState::Done
                            }
                            CapturesState::Done => {
                                panic!("Extra '«', '»'")
                            }
                        }
                    }
                    '‹' | '›' => {
                        match_state = match match_state {
                            MatchState::MatchScan => MatchState::MatchNextChar,
                            MatchState::MatchNextChar => {
                                panic!("Should have been handled by char input")
                            }
                            MatchState::Match(start_point) => {
                                hyperlink_match = start_point..=prev_input_point;
                                MatchState::Done
                            }
                            MatchState::Done => {
                                panic!("Extra '‹', '›'")
                            }
                        }
                    }
                    c => {
                        if let CapturesState::Row(number) | CapturesState::Column(number) =
                            &mut captures_state
                        {
                            number.push(c)
                        }

                        term.input(c);
                        prev_input_point = prev_input_point_from_term(&term);

                        if hovered_state == HoveredState::HoveredNextChar {
                            hovered_grid_point = prev_input_point;
                            hovered_state = HoveredState::Done;
                        }
                        if captures_state == CapturesState::PathNextChar {
                            captures_state = CapturesState::Path(prev_input_point);
                        }
                        if match_state == MatchState::MatchNextChar {
                            match_state = MatchState::Match(prev_input_point);
                        }
                    }
                }
            }
            term.move_down_and_cr(1);
        }

        let hovered_char = term.grid().index(hovered_grid_point).c;
        (
            term,
            ExpectedHyperlink {
                hovered_grid_point,
                hovered_char,
                path_with_position,
                hyperlink_match,
            },
        )
    }

    fn format_renderable_content(
        term: &Term<VoidListener>,
        expected_hyperlink: &ExpectedHyperlink,
    ) -> String {
        let mut result = format!("\nHovered on '{}'\n", expected_hyperlink.hovered_char);

        let mut first_header_row = String::new();
        let mut second_header_row = String::new();
        let mut marker_header_row = String::new();
        for index in 0..term.columns() {
            let remainder = index % 10;
            first_header_row.push_str(
                &(index > 0 && remainder == 0)
                    .then_some((index / 10).to_string())
                    .unwrap_or(" ".into()),
            );
            second_header_row += &remainder.to_string();
            marker_header_row.push(
                (index == expected_hyperlink.hovered_grid_point.column.0)
                    .then_some('↓')
                    .unwrap_or(' '),
            );
        }

        result += &format!("\n      [{}]\n", first_header_row);
        result += &format!("      [{}]\n", second_header_row);
        result += &format!("       {}", marker_header_row);

        let spacers: Flags = Flags::LEADING_WIDE_CHAR_SPACER | Flags::WIDE_CHAR_SPACER;
        for cell in term
            .renderable_content()
            .display_iter
            .filter(|cell| !cell.flags.intersects(spacers))
        {
            if cell.point.column.0 == 0 {
                let prefix = (cell.point.line == expected_hyperlink.hovered_grid_point.line)
                    .then_some('→')
                    .unwrap_or(' ');
                result += &format!("\n{prefix}[{:>3}] ", cell.point.line.to_string());
            }

            result.push(cell.c);
        }

        result
    }

    fn check_path_with_position_and_match(
        term: &Term<VoidListener>,
        expected_hyperlink: &ExpectedHyperlink,
        path_with_position: &PathWithPosition,
        hyperlink_match: &Match,
        source_location: String,
    ) {
        let format_path_with_position_and_match =
            |path_with_position: &PathWithPosition, hyperlink_match: &Match| {
                let mut result = format!("Path = «{}»", &path_with_position.path.to_string_lossy());
                if let Some(row) = path_with_position.row {
                    result += &format!(", line = {row}");
                    if let Some(column) = path_with_position.column {
                        result += &format!(", column = {column}");
                    }
                }

                result += &format!(
                    ", at grid cells ({}, {})..=({}, {})",
                    hyperlink_match.start().line.0,
                    hyperlink_match.start().column.0,
                    hyperlink_match.end().line.0,
                    hyperlink_match.end().column.0,
                );

                result
            };

        assert_eq!(
            format_path_with_position_and_match(
                &expected_hyperlink.path_with_position,
                &expected_hyperlink.hyperlink_match
            ),
            format_path_with_position_and_match(path_with_position, hyperlink_match),
            "\n    at {source_location}:\n{}",
            format_renderable_content(term, expected_hyperlink)
        );
    }

    fn test_hyperlink<'a>(
        columns: usize,
        total_chars: usize,
        test_lines: &(impl IntoIterator<Item = &'a str> + Clone),
        source_location: String,
    ) {
        let screen_lines = total_chars / columns + 2;
        let term_size = TermSize::new(columns, screen_lines);
        let (mut term, expected_hyperlink) = build_term_from_test_lines(term_size, test_lines);
        let mut hyperlink_finder = HyperlinkFinder::new();
        if let Some((hyperlink_word, false, hyperlink_match)) =
            hyperlink_finder.find_from_grid_point(&mut term, expected_hyperlink.hovered_grid_point)
        {
            let path_with_position = PathWithPosition::parse_str(&hyperlink_word);
            check_path_with_position_and_match(
                &term,
                &expected_hyperlink,
                &path_with_position,
                &hyperlink_match,
                source_location,
            );
        } else {
            assert!(
                false,
                "No hyperlink found\n     at {source_location}:\n{}",
                format_renderable_content(&term, &expected_hyperlink)
            )
        }
    }
}
