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
                (format!("{file_path}:{line_number}"), false, python_match)
            })
        } else if let Some(word_match) = regex_match_at(term, point, &mut self.word_regex) {
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
                // Treat "file://" IRIs like file paths to ensure
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

    macro_rules! test_hyperlink {
        ($($lines:expr),+; $is_iri:ident) => { {
            let test_lines = vec![$($lines),+];
            let (total_chars, longest_line_chars) = test_lines
                .iter()
                .fold((0, 0), |state, line| {
                    let line_chars = line.chars().filter(|c| "‹«»›".find(*c).is_none()).count();
                    (state.0 + line_chars, std::cmp::max(state.1, line_chars))
                });
            // Alacritty has issues with 2 columns, use 3 as the minimum for now.
            test_hyperlink!([3, longest_line_chars / 2, longest_line_chars + 1]; total_chars; &test_lines; $is_iri)
        } };
        ($($columns:literal),+; $($lines:expr),+; $is_iri:ident) => { {
            let test_lines = vec![$($lines),+];
            let total_chars = test_lines
                .iter()
                .fold(0, |state, line| {
                    state + line.chars().filter(|c| "‹«»›".find(*c).is_none()).count()
                });
            test_hyperlink!([ $($columns),+ ]; total_chars; &vec![$($lines),+]; $is_iri)
        } };
        ([ $($columns:expr),+ ]; $total_chars:expr; $lines:expr; $is_iri:ident) => { {
            for columns in vec![ $($columns),+] {
                use crate::terminal_hyperlinks::tests::test_hyperlink;
                test_hyperlink(columns, $total_chars, $lines, $is_iri,
                    format!("{}:{}:{}", std::file!(), std::line!(), std::column!()));
            }
        } };
    }

    mod iri {
        /// By default tests with terminal column counts of `[3, longest_line / 2, and longest_line + 1]`.
        /// Also accepts specific column count overrides as `test_iri!(3, 4, 5; "some stuff")`
        ///
        /// **`‹«aaaaa»›`** := **iri** match
        ///
        /// **I, J, ..., K; ** := use terminal widths of [I, J, ..., K] **columns**
        macro_rules! test_iri {
            ($iri:literal) => { { test_hyperlink!(concat!("‹«", $iri, "»›"); true) } };
            ($($columns:literal),+; $iri:literal) => { { test_hyperlink!($($columns),+; concat!("‹«", $iri, "»›"); true) } };
            ([ $($columns:expr),+ ]; $total_chars:expr; $lines:expr) => { { test_hyperlink!([ $($columns),+ ]; $total_chars; $lines; true) } };
        }

        #[test]
        fn simple() {
            // In the order they appear in URL_REGEX, except 'file://' which is treated as a path
            test_iri!("ipfs://test/cool.ipfs");
            test_iri!("ipns://test/cool.ipns");
            test_iri!("magnet://test/cool.git");
            test_iri!("mailto:someone@somewhere.here");
            test_iri!("gemini://somewhere.here");
            test_iri!("gopher://somewhere.here");
            test_iri!("http://test/cool/index.html");
            test_iri!("http://10.10.10.10:1111/cool.html");
            test_iri!("http://test/cool/index.html?amazing=1");
            test_iri!("http://test/cool/index.html#right%20here");
            test_iri!("http://test/cool/index.html?amazing=1#right%20here");
            test_iri!("https://test/cool/index.html");
            test_iri!("https://10.10.10.10:1111/cool.html");
            test_iri!("https://test/cool/index.html?amazing=1");
            test_iri!("https://test/cool/index.html#right%20here");
            test_iri!("https://test/cool/index.html?amazing=1#right%20here");
            test_iri!("news://test/cool.news");
            test_iri!("git://test/cool.git");
            test_iri!("ssh://user@somewhere.over.here:12345/test/cool.git");
            test_iri!("ftp://test/cool.ftp");
        }

        // There are likely more tests needed for IRI vs URI
        #[test]
        fn iris() {
            // These refer to the same location, see example here:
            // <https://en.wikipedia.org/wiki/Internationalized_Resource_Identifier#Compatibility>
            test_iri!("https://en.wiktionary.org/wiki/Ῥόδος"); // IRI
            test_iri!("https://en.wiktionary.org/wiki/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82"); // URI
        }

        #[test]
        #[should_panic(expected = "Expected a iri, but was a path")]
        fn file_is_a_path() {
            test_iri!("file://test/cool/index.rs");
        }
    }

    mod path {
        /// By default tests with terminal column counts of `[3, longest_line / 2, and longest_line + 1]`.
        /// Also accepts specific column count overrides as `test_path!(3, 4, 5; "some stuff")`
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
        /// **I, J, ..., K; ** := use terminal widths of [I, J, ..., K] **columns**
        macro_rules! test_path {
            ($($lines:literal),+) => { test_hyperlink!($($lines),+; false) };
            ($($columns:literal),+; $($lines:literal),+) => { test_hyperlink!($($columns),+; $($lines),+; false) };
            ([ $($columns:expr),+ ]; $total_chars:expr; $lines:expr) => { test_hyperlink!([ $($columns),+ ]; $total_chars; $lines; false) };
        }

        #[test]
        fn simple() {
            // Rust paths
            // Just the path
            test_path!("‹«/👉test/cool.rs»›");
            test_path!("‹«/test/cool👉.rs»›");

            // path and line
            test_path!("‹«/👉test/cool.rs»:«4»›");
            test_path!("‹«/test/cool.rs»👉:«4»›");
            test_path!("‹«/test/cool.rs»:«👉4»›");
            test_path!("‹«/👉test/cool.rs»(«4»)›");
            test_path!("‹«/test/cool.rs»👉(«4»)›");
            test_path!("‹«/test/cool.rs»(«👉4»)›");
            test_path!("‹«/test/cool.rs»(«4»👉)›");

            // path, line, and column
            test_path!("‹«/👉test/cool.rs»:«4»:«2»›");
            test_path!("‹«/test/cool.rs»:«4»:«👉2»›");
            test_path!("‹«/👉test/cool.rs»(«4»,«2»)›");
            test_path!("‹«/test/cool.rs»(«4»👉,«2»)›");

            // path, line, column, and ':' suffix
            test_path!("‹«/👉test/cool.rs»:«4»:«2»›:");
            test_path!("‹«/test/cool.rs»:«4»:«👉2»›:");
            test_path!("‹«/👉test/cool.rs»(«4»,«2»)›:");
            test_path!("‹«/test/cool.rs»(«4»,«2»👉)›:");

            // path, line, column, and description
            test_path!("‹«/test/cool.rs»:«4»:«2»›👉:Error!");
            test_path!("‹«/test/cool.rs»:«4»:«2»›:👉Error!");
            test_path!("‹«/test/co👉ol.rs»(«4»,«2»)›:Error!");

            // Cargo output
            test_path!("    Compiling Cool 👉(‹«/test/Cool»›)");
            test_path!("    Compiling Cool (‹«/👉test/Cool»›)");
            test_path!("    Compiling Cool (‹«/test/Cool»›👉)");

            // Python
            test_path!("‹«awe👉some.py»›");

            test_path!("    ‹F👉ile \"«/awesome.py»\", line «42»›: Wat?");
            test_path!("    ‹File \"«/awe👉some.py»\", line «42»›: Wat?");
            test_path!("    ‹File \"«/awesome.py»👉\", line «42»›: Wat?");
            test_path!("    ‹File \"«/awesome.py»\", line «4👉2»›: Wat?");
        }

        #[cfg(target_os = "windows")]
        mod windows {
            // Lots of fun to be had with long file paths and UNC paths
            // See <https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation>
            // See <https://users.rust-lang.org/t/understanding-windows-paths/58583>
            // See <https://github.com/rust-lang/cargo/issues/13919>

            #[test]
            fn unc() {
                test_path!(r#"‹«\\server\share\👉test\cool.rs»›"#);
                test_path!(r#"‹«\\server\share\test\cool👉.rs»›"#);
            }

            mod issues {
                #[test]
                #[should_panic(
                    expected = r#"Path = «\\C:\\test\\cool.rs», at grid cells (1, 0)..=(6, 0)"#
                )]
                fn verbatim() {
                    test_path!(r#"‹\\?\«C:\👉test\cool.rs»›"#);
                    test_path!(r#"‹\\?\«C:\test\cool👉.rs»›"#);
                }

                #[test]
                #[should_panic(
                    expected = r#"Path = «\\UNC\\server\\share\\test\\cool.rs», at grid cells (1, 0)..=(10, 2)"#
                )]
                fn verbatim_unc() {
                    // This isn't quite right, we want to say the path should be "\\server\share\test\cool.rs"
                    test_path!(r#"‹«\\?\UNC\server\share\👉test\cool.rs»›"#);
                    test_path!(r#"‹«\\?\UNC\server\share\test\cool👉.rs»›"#);
                }
            }
        }

        #[test]
        fn file_iri() {
            test_path!("‹file://«/👉test/cool/index.rs»›");
            test_path!("‹file://«/👉test/cool/»›");
        }

        #[test]
        fn colons_galore() {
            test_path!("‹«/test/co👉ol.rs»:«4»›");
            test_path!("‹«/test/co👉ol.rs»:«4»›:");
            test_path!("‹«/test/co👉ol.rs»:«4»:«2»›");
            test_path!("‹«/test/co👉ol.rs»:«4»:«2»›:");
            test_path!("‹«/test/co👉ol.rs»(«1»)›");
            test_path!("‹«/test/co👉ol.rs»(«1»)›:");
            test_path!("‹«/test/co👉ol.rs»(«1»,«618»)›");
            test_path!("‹«/test/co👉ol.rs»(«1»,«618»)›:");
            test_path!("‹«/test/co👉ol.rs»::«42»›");
            test_path!("‹«/test/co👉ol.rs»::«42»›:");
            test_path!("‹«/test/co👉ol.rs:4:2»(«1»,«618»)›");
            test_path!("‹«/test/co👉ol.rs»(«1»,«618»)›::");
        }

        #[test]
        fn word_wide_chars() {
            // Rust paths
            test_path!(4, 6, 12; "‹«/👉例/cool.rs»›");
            test_path!(4, 6, 12; "‹«/例👈/cool.rs»›");
            test_path!(4, 8, 16; "‹«/例/cool.rs»:«👉4»›");
            test_path!(4, 8, 16; "‹«/例/cool.rs»:«4»:«👉2»›");

            // Cargo output
            test_path!(4, 27, 30; "    Compiling Cool (‹«/👉例/Cool»›)");
            test_path!(4, 27, 30; "    Compiling Cool (‹«/例👈/Cool»›)");

            // Python
            test_path!(4, 11; "‹«👉例wesome.py»›");
            test_path!(4, 11; "‹«例👈wesome.py»›");
            test_path!(6, 17, 40; "    ‹File \"«/👉例wesome.py»\", line «42»›: Wat?");
            test_path!(6, 17, 40; "    ‹File \"«/例👈wesome.py»\", line «42»›: Wat?");
        }

        #[test]
        fn non_word_wide_chars() {
            // Mojo diagnostic message
            test_path!(4, 18, 38; "    ‹File \"«/awe👉some.🔥»\", line «42»›: Wat?");
            test_path!(4, 18, 38; "    ‹File \"«/awesome👉.🔥»\", line «42»›: Wat?");
            test_path!(4, 18, 38; "    ‹File \"«/awesome.👉🔥»\", line «42»›: Wat?");
            test_path!(4, 18, 38; "    ‹File \"«/awesome.🔥👈»\", line «42»›: Wat?");
        }

        /// These likely rise to the level of being worth fixing.
        mod issues {
            #[test]
            // We use custom columns in many tests to workaround this issue by ensuring a wrapped line
            // never ends on a wide char.
            //
            // Any wide char at the end of a wrapped line is buggy in alacritty.
            //
            // This seems worth fixing, even if no one has reported it. It is most likely
            // in the category of people experiencing failures, but somewhat randomly and not really
            // understanding what situation is causing it to work or not work, which isn't a great experience,
            // even though it might not have been reported as an actual issue with a clear repro case.
            #[cfg_attr(not(target_os = "windows"), should_panic(expected = "Path = «例»"))]
            #[cfg_attr(target_os = "windows", should_panic(expected = r#"Path = «C:\\例»"#))]
            fn issue_alacritty_bugs_with_wide_char_at_line_wrap() {
                // Rust paths
                test_path!("‹«/👉例/cool.rs»›");
                test_path!("‹«/例👈/cool.rs»›");
                test_path!("‹«/例/cool.rs»:«👉4»›");
                test_path!("‹«/例/cool.rs»:«4»:«👉2»›");

                // Cargo output
                test_path!("    Compiling Cool (‹«/👉例/Cool»›)");
                test_path!("    Compiling Cool (‹«/例👈/Cool»›)");

                // Python
                test_path!("‹«👉例wesome.py»›");
                test_path!("‹«例👈wesome.py»›");
                test_path!("    ‹File \"«/👉例wesome.py»\", line «42»›: Wat?");
                test_path!("    ‹File \"«/例👈wesome.py»\", line «42»›: Wat?");
            }

            #[test]
            #[should_panic(expected = "No hyperlink found")]
            // <https://github.com/zed-industries/zed/issues/12338>
            fn issue_12338() {
                // Issue #12338
                test_path!(".rw-r--r--     0     staff 05-27 14:03 ‹«test👉、2.txt»›");
                test_path!(".rw-r--r--     0     staff 05-27 14:03 ‹«test、👈2.txt»›");
                test_path!(".rw-r--r--     0     staff 05-27 14:03 ‹«test👉。3.txt»›");
                test_path!(".rw-r--r--     0     staff 05-27 14:03 ‹«test。👈3.txt»›");

                // Rust paths
                test_path!("‹«/👉🏃/🦀.rs»›");
                test_path!("‹«/🏃👈/🦀.rs»›");
                test_path!("‹«/🏃/👉🦀.rs»:«4»›");
                test_path!("‹«/🏃/🦀👈.rs»:«4»:«2»›");

                // Cargo output
                test_path!("    Compiling Cool (‹«/👉🏃/Cool»›)");
                test_path!("    Compiling Cool (‹«/🏃👈/Cool»›)");

                // Python
                test_path!("‹«👉🏃wesome.py»›");
                test_path!("‹«🏃👈wesome.py»›");
                test_path!("    ‹File \"«/👉🏃wesome.py»\", line «42»›: Wat?");
                test_path!("    ‹File \"«/🏃👈wesome.py»\", line «42»›: Wat?");

                // Mojo
                test_path!("‹«/awe👉some.🔥»› is some good Mojo!");
                test_path!("‹«/awesome👉.🔥»› is some good Mojo!");
                test_path!("‹«/awesome.👉🔥»› is some good Mojo!");
                test_path!("‹«/awesome.🔥👈»› is some good Mojo!");
                test_path!("    ‹File \"«/👉🏃wesome.🔥»\", line «42»›: Wat?");
                test_path!("    ‹File \"«/🏃👈wesome.🔥»\", line «42»›: Wat?");
            }

            #[test]
            #[cfg_attr(
                not(target_os = "windows"),
                should_panic(
                    expected = "Path = «test/controllers/template_items_controller_test.rb», line = 20, at grid cells (0, 0)..=(18, 1)"
                )
            )]
            #[cfg_attr(
                target_os = "windows",
                should_panic(
                    expected = r#"Path = «test\\controllers\\template_items_controller_test.rb», line = 20, at grid cells (0, 0)..=(18, 1)"#
                )
            )]
            // <https://github.com/zed-industries/zed/issues/28194>
            // TODO(davewa): #28194 was closed, but the link includes the description part (":in" here), which seems wrong...
            fn issue_28194() {
                test_path!(
                    "‹«test/controllers/template_items_controller_test.rb»:«20»›:in 'block (2 levels) in <class:TemplateItemsControllerTest>'"
                );
                test_path!(
                    "‹«test/controllers/template_items_controller_test.rb»:«19»›:in 'block in <class:TemplateItemsControllerTest>'"
                );
            }

            #[test]
            #[cfg_attr(
                not(target_os = "windows"),
                should_panic(
                    expected = "Path = «/test/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82/», at grid cells (0, 0)..=(15, 1)"
                )
            )]
            #[cfg_attr(
                target_os = "windows",
                should_panic(
                    expected = r#"Path = «C:\\test\\%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82\\», at grid cells (0, 0)..=(16, 0)"#
                )
            )]
            fn issue_file_iri_with_percent_encoded_characters() {
                // Non-space characters
                // file:///test/Ῥόδος/
                test_path!("‹file://«/👉test/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82/»›"); // URI

                // Spaces
                test_path!("‹file://«/👉te%20st/co%20ol/index.rs»›");
                test_path!("‹file://«/👉te%20st/co%20ol/»›");
            }
        }

        /// Minor issues arguably not important enough to fix/workaround...
        mod nits {
            #[test]
            #[cfg_attr(
                not(target_os = "windows"),
                should_panic(expected = "Path = «/test/cool.rs(4»")
            )]
            #[cfg_attr(
                target_os = "windows",
                should_panic(expected = r#"Path = «C:\\test\\cool.rs(4»"#)
            )]
            fn alacritty_bugs_with_two_columns() {
                test_path!(2; "‹«/👉test/cool.rs»(«4»)›");
                test_path!(2; "‹«/test/cool.rs»(«👉4»)›");
                test_path!(2; "‹«/test/cool.rs»(«4»,«👉2»)›");

                // Python
                test_path!(2; "‹«awe👉some.py»›");
            }

            #[test]
            #[cfg_attr(
                not(target_os = "windows"),
                should_panic(
                    expected = "Path = «/test/cool.rs», line = 1, at grid cells (0, 0)..=(9, 0)"
                )
            )]
            #[cfg_attr(
                target_os = "windows",
                should_panic(
                    expected = r#"Path = «C:\\test\\cool.rs», line = 1, at grid cells (0, 0)..=(9, 2)"#
                )
            )]
            fn invalid_row_column_should_be_part_of_path() {
                test_path!("‹«/👉test/cool.rs:1:618033988749»›");
                test_path!("‹«/👉test/cool.rs(1,618033988749)»›");
            }

            #[test]
            #[should_panic(expected = "Path = «»")]
            fn colon_suffix_succeeds_in_finding_an_empty_maybe_path() {
                test_path!("‹«/test/cool.rs»:«4»:«2»›👉:", "What is this?");
                test_path!("‹«/test/cool.rs»(«4»,«2»)›👉:", "What is this?");
            }

            #[test]
            #[cfg_attr(
                not(target_os = "windows"),
                should_panic(expected = "Path = «/test/cool.rs»")
            )]
            #[cfg_attr(
                target_os = "windows",
                should_panic(expected = r#"Path = «C:\\test\\cool.rs»"#)
            )]
            fn many_trailing_colons_should_be_parsed_as_part_of_the_path() {
                test_path!("‹«/test/cool.rs:::👉:»›");
                test_path!("‹«/te:st/👉co:ol.r:s:4:2::::::»›");
            }
        }
    }
    struct ExpectedHyperlink {
        hovered_grid_point: AlacPoint,
        hovered_char: char,
        is_iri: bool,
        iri_or_path: String,
        row: Option<u32>,
        column: Option<u32>,
        hyperlink_match: RangeInclusive<AlacPoint>,
    }

    fn build_term_from_test_lines<'a>(
        is_iri: bool,
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
        let mut iri_or_path_is_file_iri = false;
        let mut iri_or_path = String::new();
        let mut row = None;
        let mut column = None;
        let mut prev_input_point = AlacPoint::default();
        let mut hovered_state = HoveredState::HoveredScan;
        let mut match_state = MatchState::MatchScan;
        let mut captures_state = CapturesState::PathScan;
        const FILE_SCHEME: &str = "file://";

        let mut term = Term::new(Config::default(), &term_size, VoidListener);

        for text in test_lines.clone().into_iter() {
            let mut text = text.chars().collect_vec();

            // Convert path to Windows style if on Windows
            if cfg!(windows) && !is_iri {
                if let Some(mut path_start) =
                    text.iter().position(|&c| c == '«').map(|index| index + 1)
                {
                    let mut path_end = text.iter().position(|&c| c == '»').unwrap() - 1;

                    if text[path_start] == '/' {
                        text.insert(path_start, ':');
                        text.insert(path_start, 'C');
                        path_start += 2;
                        path_end += 2;
                    }

                    for index in path_start..=path_end {
                        if text[index] == '/' {
                            text[index] = '\\';
                        }
                    }
                }
            }

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
                                iri_or_path = term.bounds_to_string(start_point, prev_input_point);
                                CapturesState::RowScan
                            }
                            CapturesState::RowScan => CapturesState::Row(String::new()),
                            CapturesState::Row(number) => {
                                row = Some(number.parse::<u32>().unwrap());
                                CapturesState::ColumnScan
                            }
                            CapturesState::ColumnScan => CapturesState::Column(String::new()),
                            CapturesState::Column(number) => {
                                column = Some(number.parse::<u32>().unwrap());
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
                            if !is_iri {
                                let iri_scheme = term.bounds_to_string(
                                    prev_input_point.sub(&term, Boundary::Grid, FILE_SCHEME.len()),
                                    prev_input_point.sub(&term, Boundary::Grid, 1),
                                );
                                if iri_scheme == FILE_SCHEME {
                                    iri_or_path_is_file_iri = true
                                }
                            }
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

        if iri_or_path_is_file_iri {
            use url::Url;
            let file_iri = format!("{FILE_SCHEME}{iri_or_path}");
            let iri = Url::parse(&file_iri).unwrap();
            iri_or_path = iri.to_file_path().unwrap().to_string_lossy().to_string();
        }

        let hovered_char = term.grid().index(hovered_grid_point).c;
        (
            term,
            ExpectedHyperlink {
                hovered_grid_point,
                hovered_char,
                is_iri,
                iri_or_path,
                row,
                column,
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
        path_with_position: PathWithPosition,
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
            expected_hyperlink.is_iri,
            false,
            "\n    at {source_location}\nExpected a iri, but was a path:\n{}",
            format_renderable_content(term, expected_hyperlink)
        );

        assert_eq!(
            format_path_with_position_and_match(
                &PathWithPosition {
                    path: PathBuf::from(expected_hyperlink.iri_or_path.clone()),
                    row: expected_hyperlink.row,
                    column: expected_hyperlink.column
                },
                &expected_hyperlink.hyperlink_match
            ),
            format_path_with_position_and_match(&path_with_position, hyperlink_match),
            "\n    at {source_location}:\n{}",
            format_renderable_content(term, expected_hyperlink)
        );
    }

    fn check_iri_and_match(
        term: &Term<VoidListener>,
        expected_hyperlink: &ExpectedHyperlink,
        iri: String,
        hyperlink_match: &Match,
        source_location: String,
    ) {
        let format_iri_and_match = |iri: &String, hyperlink_match: &Match| {
            format!(
                "Url = «{}», at grid cells ({}, {})..=({}, {})",
                iri,
                hyperlink_match.start().line.0,
                hyperlink_match.start().column.0,
                hyperlink_match.end().line.0,
                hyperlink_match.end().column.0,
            )
        };

        assert_eq!(
            expected_hyperlink.is_iri,
            true,
            "\n    at {source_location}\nExpected a path, but was a iri:\n{}",
            format_renderable_content(term, expected_hyperlink)
        );

        assert_eq!(
            format_iri_and_match(
                &expected_hyperlink.iri_or_path,
                &expected_hyperlink.hyperlink_match
            ),
            format_iri_and_match(&iri, hyperlink_match),
            "\n    at {source_location}:\n{}",
            format_renderable_content(term, expected_hyperlink)
        );
    }

    fn test_hyperlink<'a>(
        columns: usize,
        total_chars: usize,
        test_lines: &(impl IntoIterator<Item = &'a str> + Clone),
        is_iri: bool,
        source_location: String,
    ) {
        let screen_lines = total_chars / columns + 2;
        let term_size = TermSize::new(columns, screen_lines);
        let (mut term, expected_hyperlink) =
            build_term_from_test_lines(is_iri, term_size, test_lines);
        let mut hyperlink_finder = HyperlinkFinder::new();
        match hyperlink_finder
            .find_from_grid_point(&mut term, expected_hyperlink.hovered_grid_point)
        {
            Some((hyperlink_word, false, hyperlink_match)) => {
                check_path_with_position_and_match(
                    &term,
                    &expected_hyperlink,
                    PathWithPosition::parse_str(&hyperlink_word),
                    &hyperlink_match,
                    source_location,
                );
            }
            Some((hyperlink_word, true, hyperlink_match)) => {
                check_iri_and_match(
                    &term,
                    &expected_hyperlink,
                    hyperlink_word,
                    &hyperlink_match,
                    source_location,
                );
            }
            _ => {
                assert!(
                    false,
                    "No hyperlink found\n     at {source_location}:\n{}",
                    format_renderable_content(&term, &expected_hyperlink)
                )
            }
        }
    }
}
