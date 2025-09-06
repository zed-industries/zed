use alacritty_terminal::{
    Term,
    event::EventListener,
    grid::Dimensions,
    index::{Boundary, Column, Direction as AlacDirection, Line, Point as AlacPoint},
    term::search::{Match, RegexIter, RegexSearch},
};
use regex::Regex;
use std::{ops::Index, sync::LazyLock};

const URL_REGEX: &str = r#"(ipfs:|ipns:|magnet:|mailto:|gemini://|gopher://|https://|http://|news:|file://|git://|ssh:|ftp://)[^\u{0000}-\u{001F}\u{007F}-\u{009F}<>"\s{-}\^⟨⟩`']+"#;
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

pub(super) struct RegexSearches {
    url_regex: RegexSearch,
    word_regex: RegexSearch,
    python_file_line_regex: RegexSearch,
}

impl RegexSearches {
    pub(super) fn new() -> Self {
        Self {
            url_regex: RegexSearch::new(URL_REGEX).unwrap(),
            word_regex: RegexSearch::new(WORD_REGEX).unwrap(),
            python_file_line_regex: RegexSearch::new(PYTHON_FILE_LINE_REGEX).unwrap(),
        }
    }
}

pub(super) fn find_from_grid_point<T: EventListener>(
    term: &Term<T>,
    point: AlacPoint,
    regex_searches: &mut RegexSearches,
) -> Option<(String, bool, Match)> {
    let grid = term.grid();
    let link = grid.index(point).hyperlink();
    let found_word = if let Some(ref url) = link {
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

        let url = url.uri().to_owned();
        let url_match = min_index..=max_index;

        Some((url, true, url_match))
    } else if let Some(url_match) = regex_match_at(term, point, &mut regex_searches.url_regex) {
        let url = term.bounds_to_string(*url_match.start(), *url_match.end());
        let (sanitized_url, sanitized_match) = sanitize_url_punctuation(url, url_match, term);
        Some((sanitized_url, true, sanitized_match))
    } else if let Some(python_match) =
        regex_match_at(term, point, &mut regex_searches.python_file_line_regex)
    {
        let matching_line = term.bounds_to_string(*python_match.start(), *python_match.end());
        python_extract_path_and_line(&matching_line).map(|(file_path, line_number)| {
            (format!("{file_path}:{line_number}"), false, python_match)
        })
    } else if let Some(word_match) = regex_match_at(term, point, &mut regex_searches.word_regex) {
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
                        .is_some_and(|c| c.is_ascii_digit());
                let next_is_digit = last_index < file_path.len() - 1
                    && file_path
                        .chars()
                        .nth(last_index + 1)
                        .is_none_or(|c| c.is_ascii_digit());
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

fn sanitize_url_punctuation<T: EventListener>(
    url: String,
    url_match: Match,
    term: &Term<T>,
) -> (String, Match) {
    let mut sanitized_url = url;
    let mut chars_trimmed = 0;
    
    // First, handle parentheses balancing
    let open_parens = sanitized_url.chars().filter(|&c| c == '(').count();
    let close_parens = sanitized_url.chars().filter(|&c| c == ')').count();
    
    // If there are more closing parentheses than opening ones, and the URL ends with ')',
    // trim trailing ')' characters until balanced or no more trailing ')' exist
    if close_parens > open_parens && sanitized_url.ends_with(')') {
        while sanitized_url.ends_with(')') {
            let remaining_open = sanitized_url.chars().filter(|&c| c == '(').count();
            let remaining_close = sanitized_url.chars().filter(|&c| c == ')').count();
            
            // If removing this ')' would make parentheses balanced or reduce the imbalance
            if remaining_close > remaining_open {
                sanitized_url.pop();
                chars_trimmed += 1;
            } else {
                break;
            }
        }
    }
    
    // Second, handle trailing periods (likely sentence punctuation)
    if sanitized_url.ends_with('.') {
        // Count trailing periods
        let mut trailing_periods = 0;
        for char in sanitized_url.chars().rev() {
            if char == '.' {
                trailing_periods += 1;
            } else {
                break;
            }
        }
        
        // Always remove multiple trailing periods (unlikely to be valid in URLs)
        if trailing_periods > 1 {
            sanitized_url.truncate(sanitized_url.len() - trailing_periods);
            chars_trimmed += trailing_periods;
        }
        // For single trailing period, use heuristics to determine if it's sentence punctuation
        else if trailing_periods == 1 {
            // Simple heuristic: if the URL is likely at the end of a sentence, remove the period
            // This covers most common cases where URLs appear in text followed by sentence punctuation
            let should_remove_period = 
                // Check if URL looks like it ends with a complete component (domain, path, file)
                // and the period is likely sentence punctuation
                sanitized_url.len() > 1 && {
                    let chars: Vec<char> = sanitized_url.chars().collect();
                    let second_last_char = chars[chars.len() - 2];
                    
                    // Remove period if the character before it is alphanumeric (common case)
                    // This catches: example.com., file.html., path/something.
                    second_last_char.is_alphanumeric() ||
                    // Also remove if it's a slash (path ending: /path.)
                    second_last_char == '/'
                };
                
            if should_remove_period {
                sanitized_url.pop();
                chars_trimmed += 1;
            }
        }
    }
    
    if chars_trimmed > 0 {
        // Adjust the match range to reflect the trimmed characters
        let new_end = url_match.end().sub(term, Boundary::Grid, chars_trimmed);
        let sanitized_match = Match::new(*url_match.start(), new_end);
        (sanitized_url, sanitized_match)
    } else {
        (sanitized_url, url_match)
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
    use std::{cell::RefCell, ops::RangeInclusive, path::PathBuf};
    use url::Url;
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
            "test http://example.com test 'https://website1.com' test mailto:bob@example.com train",
            vec![
                "http://example.com",
                "https://website1.com",
                "mailto:bob@example.com",
            ],
        );
    }

    #[test]
    fn test_url_parentheses_sanitization() {
        // Test our sanitize_url_parentheses function directly
        let test_cases = vec![
            // Cases that should be sanitized (unbalanced parentheses)
            ("https://www.google.com/)", "https://www.google.com/"),
            ("https://example.com/path)", "https://example.com/path"),
            ("https://test.com/))", "https://test.com/"),
            
            // Cases that should NOT be sanitized (balanced parentheses)
            ("https://en.wikipedia.org/wiki/Example_(disambiguation)", "https://en.wikipedia.org/wiki/Example_(disambiguation)"),
            ("https://test.com/(hello)", "https://test.com/(hello)"),
            ("https://example.com/path(1)(2)", "https://example.com/path(1)(2)"),
            
            // Edge cases
            ("https://test.com/", "https://test.com/"),
            ("https://example.com", "https://example.com"),
        ];
        
        for (input, expected) in test_cases {
            // Create a minimal terminal for testing
            let term = Term::new(Config::default(), &TermSize::new(80, 24), VoidListener);
            
            // Create a dummy match that spans the entire input
            let start_point = AlacPoint::new(alacritty_terminal::index::Line(0), alacritty_terminal::index::Column(0));
            let end_point = AlacPoint::new(alacritty_terminal::index::Line(0), alacritty_terminal::index::Column(input.len()));
            let dummy_match = alacritty_terminal::term::search::Match::new(start_point, end_point);
            
            let (result, _) = sanitize_url_punctuation(input.to_string(), dummy_match, &term);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_url_periods_sanitization() {
        // Test URLs with trailing periods (sentence punctuation)
        let test_cases = vec![
            // Cases that should be sanitized (trailing periods likely punctuation)
            ("https://example.com.", "https://example.com"),
            ("https://github.com/zed-industries/zed.", "https://github.com/zed-industries/zed"),
            ("https://example.com/path/file.html.", "https://example.com/path/file.html"),
            ("https://example.com/file.pdf.", "https://example.com/file.pdf"),
            ("https://example.com:8080.", "https://example.com:8080"),
            ("https://example.com..", "https://example.com"),
            ("https://en.wikipedia.org/wiki/C.E.O.", "https://en.wikipedia.org/wiki/C.E.O"),
            
            // Cases that should NOT be sanitized (periods are part of URL structure)
            ("https://example.com/v1.0/api", "https://example.com/v1.0/api"),
            ("https://192.168.1.1", "https://192.168.1.1"),
            ("https://sub.domain.com", "https://sub.domain.com"),
        ];
        
        for (input, expected) in test_cases {
            // Create a minimal terminal for testing
            let term = Term::new(Config::default(), &TermSize::new(80, 24), VoidListener);
            
            // Create a dummy match that spans the entire input
            let start_point = AlacPoint::new(alacritty_terminal::index::Line(0), alacritty_terminal::index::Column(0));
            let end_point = AlacPoint::new(alacritty_terminal::index::Line(0), alacritty_terminal::index::Column(input.len()));
            let dummy_match = alacritty_terminal::term::search::Match::new(start_point, end_point);
            
            // This test should initially fail since we haven't implemented period sanitization yet
            let (result, _) = sanitize_url_punctuation(input.to_string(), dummy_match, &term);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
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

    // We use custom columns in many tests to workaround this issue by ensuring a wrapped
    // line never ends on a wide char:
    //
    // <https://github.com/alacritty/alacritty/issues/8586>
    //
    // This issue was recently fixed, as soon as we update to a version containing the fix we
    // can remove all the custom columns from these tests.
    //
    macro_rules! test_hyperlink {
        ($($lines:expr),+; $hyperlink_kind:ident) => { {
            use crate::terminal_hyperlinks::tests::line_cells_count;
            use std::cmp;

            let test_lines = vec![$($lines),+];
            let (total_cells, longest_line_cells) =
                test_lines.iter().copied()
                    .map(line_cells_count)
                    .fold((0, 0), |state, cells| (state.0 + cells, cmp::max(state.1, cells)));

            test_hyperlink!(
                // Alacritty has issues with 2 columns, use 3 as the minimum for now.
                [3, longest_line_cells / 2, longest_line_cells + 1];
                total_cells;
                test_lines.iter().copied();
                $hyperlink_kind
            )
        } };

        ($($columns:literal),+; $($lines:expr),+; $hyperlink_kind:ident) => { {
            use crate::terminal_hyperlinks::tests::line_cells_count;

            let test_lines = vec![$($lines),+];
            let total_cells = test_lines.iter().copied().map(line_cells_count).sum();

            test_hyperlink!(
                [ $($columns),+ ]; total_cells; test_lines.iter().copied(); $hyperlink_kind
            )
        } };

        ([ $($columns:expr),+ ]; $total_cells:expr; $lines:expr; $hyperlink_kind:ident) => { {
            use crate::terminal_hyperlinks::tests::{ test_hyperlink, HyperlinkKind };

            let source_location = format!("{}:{}", std::file!(), std::line!());
            for columns in vec![ $($columns),+] {
                test_hyperlink(columns, $total_cells, $lines, HyperlinkKind::$hyperlink_kind,
                    &source_location);
            }
        } };
    }

    mod path {
        /// 👉 := **hovered** on following char
        ///
        /// 👈 := **hovered** on wide char spacer of previous full width char
        ///
        /// **`‹›`** := expected **hyperlink** match
        ///
        /// **`«»`** := expected **path**, **row**, and **column** capture groups
        ///
        /// [**`c₀, c₁, …, cₙ;`**]ₒₚₜ := use specified terminal widths of `c₀, c₁, …, cₙ` **columns**
        /// (defaults to `3, longest_line_cells / 2, longest_line_cells + 1;`)
        ///
        macro_rules! test_path {
            ($($lines:literal),+) => { test_hyperlink!($($lines),+; Path) };
            ($($columns:literal),+; $($lines:literal),+) => {
                test_hyperlink!($($columns),+; $($lines),+; Path)
            };
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
            #[cfg_attr(not(target_os = "windows"), should_panic(expected = "Path = «例»"))]
            #[cfg_attr(target_os = "windows", should_panic(expected = r#"Path = «C:\\例»"#))]
            // <https://github.com/alacritty/alacritty/issues/8586>
            fn issue_alacritty_8586() {
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
                    expected = "Path = «test/controllers/template_items_controller_test.rb», line = 20, at grid cells (0, 0)..=(17, 1)"
                )
            )]
            #[cfg_attr(
                target_os = "windows",
                should_panic(
                    expected = r#"Path = «test\\controllers\\template_items_controller_test.rb», line = 20, at grid cells (0, 0)..=(17, 1)"#
                )
            )]
            // <https://github.com/zed-industries/zed/issues/28194>
            //
            // #28194 was closed, but the link includes the description part (":in" here), which
            // seems wrong...
            fn issue_28194() {
                test_path!(
                    "‹«test/c👉ontrollers/template_items_controller_test.rb»:«20»›:in 'block (2 levels) in <class:TemplateItemsControllerTest>'"
                );
                test_path!(
                    "‹«test/controllers/template_items_controller_test.rb»:«19»›:i👉n 'block in <class:TemplateItemsControllerTest>'"
                );
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

        #[cfg(target_os = "windows")]
        mod windows {
            // Lots of fun to be had with long file paths (verbatim) and UNC paths on Windows.
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
                    expected = r#"Path = «C:\\test\\cool.rs», at grid cells (0, 0)..=(6, 0)"#
                )]
                fn issue_verbatim() {
                    test_path!(r#"‹«\\?\C:\👉test\cool.rs»›"#);
                    test_path!(r#"‹«\\?\C:\test\cool👉.rs»›"#);
                }

                #[test]
                #[should_panic(
                    expected = r#"Path = «\\\\server\\share\\test\\cool.rs», at grid cells (0, 0)..=(10, 2)"#
                )]
                fn issue_verbatim_unc() {
                    test_path!(r#"‹«\\?\UNC\server\share\👉test\cool.rs»›"#);
                    test_path!(r#"‹«\\?\UNC\server\share\test\cool👉.rs»›"#);
                }
            }
        }
    }

    mod file_iri {
        // File IRIs have a ton of use cases, most of which we currently do not support. A few of
        // those cases are documented here as tests which are expected to fail.
        // See https://en.wikipedia.org/wiki/File_URI_scheme

        /// [**`c₀, c₁, …, cₙ;`**]ₒₚₜ := use specified terminal widths of `c₀, c₁, …, cₙ` **columns**
        /// (defaults to `3, longest_line_cells / 2, longest_line_cells + 1;`)
        ///
        macro_rules! test_file_iri {
            ($file_iri:literal) => { { test_hyperlink!(concat!("‹«👉", $file_iri, "»›"); FileIri) } };
            ($($columns:literal),+; $file_iri:literal) => { {
                test_hyperlink!($($columns),+; concat!("‹«👉", $file_iri, "»›"); FileIri)
            } };
        }

        #[cfg(not(target_os = "windows"))]
        #[test]
        fn absolute_file_iri() {
            test_file_iri!("file:///test/cool/index.rs");
            test_file_iri!("file:///test/cool/");
        }

        mod issues {
            #[cfg(not(target_os = "windows"))]
            #[test]
            #[should_panic(expected = "Path = «/test/Ῥόδος/», at grid cells (0, 0)..=(15, 1)")]
            fn issue_file_iri_with_percent_encoded_characters() {
                // Non-space characters
                // file:///test/Ῥόδος/
                test_file_iri!("file:///test/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82/"); // URI

                // Spaces
                test_file_iri!("file:///te%20st/co%20ol/index.rs");
                test_file_iri!("file:///te%20st/co%20ol/");
            }
        }

        #[cfg(target_os = "windows")]
        mod windows {
            mod issues {
                // The test uses Url::to_file_path(), but it seems that the Url crate doesn't
                // support relative file IRIs.
                #[test]
                #[should_panic(
                    expected = r#"Failed to interpret file IRI `file:/test/cool/index.rs` as a path"#
                )]
                fn issue_relative_file_iri() {
                    test_file_iri!("file:/test/cool/index.rs");
                    test_file_iri!("file:/test/cool/");
                }

                // See https://en.wikipedia.org/wiki/File_URI_scheme
                #[test]
                #[should_panic(
                    expected = r#"Path = «C:\\test\\cool\\index.rs», at grid cells (0, 0)..=(9, 1)"#
                )]
                fn issue_absolute_file_iri() {
                    test_file_iri!("file:///C:/test/cool/index.rs");
                    test_file_iri!("file:///C:/test/cool/");
                }

                #[test]
                #[should_panic(
                    expected = r#"Path = «C:\\test\\Ῥόδος\\», at grid cells (0, 0)..=(16, 1)"#
                )]
                fn issue_file_iri_with_percent_encoded_characters() {
                    // Non-space characters
                    // file:///test/Ῥόδος/
                    test_file_iri!("file:///C:/test/%E1%BF%AC%CF%8C%CE%B4%CE%BF%CF%82/"); // URI

                    // Spaces
                    test_file_iri!("file:///C:/te%20st/co%20ol/index.rs");
                    test_file_iri!("file:///C:/te%20st/co%20ol/");
                }
            }
        }
    }

    mod iri {
        /// [**`c₀, c₁, …, cₙ;`**]ₒₚₜ := use specified terminal widths of `c₀, c₁, …, cₙ` **columns**
        /// (defaults to `3, longest_line_cells / 2, longest_line_cells + 1;`)
        ///
        macro_rules! test_iri {
            ($iri:literal) => { { test_hyperlink!(concat!("‹«👉", $iri, "»›"); Iri) } };
            ($($columns:literal),+; $iri:literal) => { {
                test_hyperlink!($($columns),+; concat!("‹«👉", $iri, "»›"); Iri)
            } };
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

        #[test]
        fn wide_chars() {
            // In the order they appear in URL_REGEX, except 'file://' which is treated as a path
            test_iri!(4, 20; "ipfs://例🏃🦀/cool.ipfs");
            test_iri!(4, 20; "ipns://例🏃🦀/cool.ipns");
            test_iri!(6, 20; "magnet://例🏃🦀/cool.git");
            test_iri!(4, 20; "mailto:someone@somewhere.here");
            test_iri!(4, 20; "gemini://somewhere.here");
            test_iri!(4, 20; "gopher://somewhere.here");
            test_iri!(4, 20; "http://例🏃🦀/cool/index.html");
            test_iri!(4, 20; "http://10.10.10.10:1111/cool.html");
            test_iri!(4, 20; "http://例🏃🦀/cool/index.html?amazing=1");
            test_iri!(4, 20; "http://例🏃🦀/cool/index.html#right%20here");
            test_iri!(4, 20; "http://例🏃🦀/cool/index.html?amazing=1#right%20here");
            test_iri!(4, 20; "https://例🏃🦀/cool/index.html");
            test_iri!(4, 20; "https://10.10.10.10:1111/cool.html");
            test_iri!(4, 20; "https://例🏃🦀/cool/index.html?amazing=1");
            test_iri!(4, 20; "https://例🏃🦀/cool/index.html#right%20here");
            test_iri!(4, 20; "https://例🏃🦀/cool/index.html?amazing=1#right%20here");
            test_iri!(4, 20; "news://例🏃🦀/cool.news");
            test_iri!(5, 20; "git://例/cool.git");
            test_iri!(5, 20; "ssh://user@somewhere.over.here:12345/例🏃🦀/cool.git");
            test_iri!(7, 20; "ftp://例🏃🦀/cool.ftp");
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
        #[should_panic(expected = "Expected a path, but was a iri")]
        fn file_is_a_path() {
            test_iri!("file://test/cool/index.rs");
        }
    }

    #[derive(Debug, PartialEq)]
    enum HyperlinkKind {
        FileIri,
        Iri,
        Path,
    }

    struct ExpectedHyperlink {
        hovered_grid_point: AlacPoint,
        hovered_char: char,
        hyperlink_kind: HyperlinkKind,
        iri_or_path: String,
        row: Option<u32>,
        column: Option<u32>,
        hyperlink_match: RangeInclusive<AlacPoint>,
    }

    /// Converts to Windows style paths on Windows, like path!(), but at runtime for improved test
    /// readability.
    fn build_term_from_test_lines<'a>(
        hyperlink_kind: HyperlinkKind,
        term_size: TermSize,
        test_lines: impl Iterator<Item = &'a str>,
    ) -> (Term<VoidListener>, ExpectedHyperlink) {
        #[derive(Default, Eq, PartialEq)]
        enum HoveredState {
            #[default]
            HoveredScan,
            HoveredNextChar,
            Done,
        }

        #[derive(Default, Eq, PartialEq)]
        enum MatchState {
            #[default]
            MatchScan,
            MatchNextChar,
            Match(AlacPoint),
            Done,
        }

        #[derive(Default, Eq, PartialEq)]
        enum CapturesState {
            #[default]
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

        let mut hovered_grid_point: Option<AlacPoint> = None;
        let mut hyperlink_match = AlacPoint::default()..=AlacPoint::default();
        let mut iri_or_path = String::default();
        let mut row = None;
        let mut column = None;
        let mut prev_input_point = AlacPoint::default();
        let mut hovered_state = HoveredState::default();
        let mut match_state = MatchState::default();
        let mut captures_state = CapturesState::default();
        let mut term = Term::new(Config::default(), &term_size, VoidListener);

        for text in test_lines {
            let chars: Box<dyn Iterator<Item = char>> =
                if cfg!(windows) && hyperlink_kind == HyperlinkKind::Path {
                    Box::new(text.chars().map(|c| if c == '/' { '\\' } else { c })) as _
                } else {
                    Box::new(text.chars()) as _
                };
            let mut chars = chars.peekable();
            while let Some(c) = chars.next() {
                match c {
                    '👉' => {
                        hovered_state = HoveredState::HoveredNextChar;
                    }
                    '👈' => {
                        hovered_grid_point = Some(prev_input_point.add(&term, Boundary::Grid, 1));
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
                    _ => {
                        if let CapturesState::Row(number) | CapturesState::Column(number) =
                            &mut captures_state
                        {
                            number.push(c)
                        }

                        let is_windows_abs_path_start = captures_state
                            == CapturesState::PathNextChar
                            && cfg!(windows)
                            && hyperlink_kind == HyperlinkKind::Path
                            && c == '\\'
                            && chars.peek().is_some_and(|c| *c != '\\');

                        if is_windows_abs_path_start {
                            // Convert Unix abs path start into Windows abs path start so that the
                            // same test can be used for both OSes.
                            term.input('C');
                            prev_input_point = prev_input_point_from_term(&term);
                            term.input(':');
                            term.input(c);
                        } else {
                            term.input(c);
                            prev_input_point = prev_input_point_from_term(&term);
                        }

                        if hovered_state == HoveredState::HoveredNextChar {
                            hovered_grid_point = Some(prev_input_point);
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

        if hyperlink_kind == HyperlinkKind::FileIri {
            let Ok(url) = Url::parse(&iri_or_path) else {
                panic!("Failed to parse file IRI `{iri_or_path}`");
            };
            let Ok(path) = url.to_file_path() else {
                panic!("Failed to interpret file IRI `{iri_or_path}` as a path");
            };
            iri_or_path = path.to_string_lossy().to_string();
        }

        if cfg!(windows) {
            // Handle verbatim and UNC paths for Windows
            if let Some(stripped) = iri_or_path.strip_prefix(r#"\\?\UNC\"#) {
                iri_or_path = format!(r#"\\{stripped}"#);
            } else if let Some(stripped) = iri_or_path.strip_prefix(r#"\\?\"#) {
                iri_or_path = stripped.to_string();
            }
        }

        let hovered_grid_point = hovered_grid_point.expect("Missing hovered point (👉 or 👈)");
        let hovered_char = term.grid().index(hovered_grid_point).c;
        (
            term,
            ExpectedHyperlink {
                hovered_grid_point,
                hovered_char,
                hyperlink_kind,
                iri_or_path,
                row,
                column,
                hyperlink_match,
            },
        )
    }

    fn line_cells_count(line: &str) -> usize {
        // This avoids taking a dependency on the unicode-width crate
        fn width(c: char) -> usize {
            match c {
                // Fullwidth unicode characters used in tests
                '例' | '🏃' | '🦀' | '🔥' => 2,
                _ => 1,
            }
        }
        const CONTROL_CHARS: &str = "‹«👉👈»›";
        line.chars()
            .filter(|c| !CONTROL_CHARS.contains(*c))
            .map(width)
            .sum::<usize>()
    }

    struct CheckHyperlinkMatch<'a> {
        term: &'a Term<VoidListener>,
        expected_hyperlink: &'a ExpectedHyperlink,
        source_location: &'a str,
    }

    impl<'a> CheckHyperlinkMatch<'a> {
        fn new(
            term: &'a Term<VoidListener>,
            expected_hyperlink: &'a ExpectedHyperlink,
            source_location: &'a str,
        ) -> Self {
            Self {
                term,
                expected_hyperlink,
                source_location,
            }
        }

        fn check_path_with_position_and_match(
            &self,
            path_with_position: PathWithPosition,
            hyperlink_match: &Match,
        ) {
            let format_path_with_position_and_match =
                |path_with_position: &PathWithPosition, hyperlink_match: &Match| {
                    let mut result =
                        format!("Path = «{}»", &path_with_position.path.to_string_lossy());
                    if let Some(row) = path_with_position.row {
                        result += &format!(", line = {row}");
                        if let Some(column) = path_with_position.column {
                            result += &format!(", column = {column}");
                        }
                    }

                    result += &format!(
                        ", at grid cells {}",
                        Self::format_hyperlink_match(hyperlink_match)
                    );
                    result
                };

            assert_ne!(
                self.expected_hyperlink.hyperlink_kind,
                HyperlinkKind::Iri,
                "\n    at {}\nExpected a path, but was a iri:\n{}",
                self.source_location,
                self.format_renderable_content()
            );

            assert_eq!(
                format_path_with_position_and_match(
                    &PathWithPosition {
                        path: PathBuf::from(self.expected_hyperlink.iri_or_path.clone()),
                        row: self.expected_hyperlink.row,
                        column: self.expected_hyperlink.column
                    },
                    &self.expected_hyperlink.hyperlink_match
                ),
                format_path_with_position_and_match(&path_with_position, hyperlink_match),
                "\n    at {}:\n{}",
                self.source_location,
                self.format_renderable_content()
            );
        }

        fn check_iri_and_match(&self, iri: String, hyperlink_match: &Match) {
            let format_iri_and_match = |iri: &String, hyperlink_match: &Match| {
                format!(
                    "Url = «{iri}», at grid cells {}",
                    Self::format_hyperlink_match(hyperlink_match)
                )
            };

            assert_eq!(
                self.expected_hyperlink.hyperlink_kind,
                HyperlinkKind::Iri,
                "\n    at {}\nExpected a iri, but was a path:\n{}",
                self.source_location,
                self.format_renderable_content()
            );

            assert_eq!(
                format_iri_and_match(
                    &self.expected_hyperlink.iri_or_path,
                    &self.expected_hyperlink.hyperlink_match
                ),
                format_iri_and_match(&iri, hyperlink_match),
                "\n    at {}:\n{}",
                self.source_location,
                self.format_renderable_content()
            );
        }

        fn format_hyperlink_match(hyperlink_match: &Match) -> String {
            format!(
                "({}, {})..=({}, {})",
                hyperlink_match.start().line.0,
                hyperlink_match.start().column.0,
                hyperlink_match.end().line.0,
                hyperlink_match.end().column.0
            )
        }

        fn format_renderable_content(&self) -> String {
            let mut result = format!("\nHovered on '{}'\n", self.expected_hyperlink.hovered_char);

            let mut first_header_row = String::new();
            let mut second_header_row = String::new();
            let mut marker_header_row = String::new();
            for index in 0..self.term.columns() {
                let remainder = index % 10;
                first_header_row.push_str(
                    &(index > 0 && remainder == 0)
                        .then_some((index / 10).to_string())
                        .unwrap_or(" ".into()),
                );
                second_header_row += &remainder.to_string();
                if index == self.expected_hyperlink.hovered_grid_point.column.0 {
                    marker_header_row.push('↓');
                } else {
                    marker_header_row.push(' ');
                }
            }

            result += &format!("\n      [{}]\n", first_header_row);
            result += &format!("      [{}]\n", second_header_row);
            result += &format!("       {}", marker_header_row);

            let spacers: Flags = Flags::LEADING_WIDE_CHAR_SPACER | Flags::WIDE_CHAR_SPACER;
            for cell in self
                .term
                .renderable_content()
                .display_iter
                .filter(|cell| !cell.flags.intersects(spacers))
            {
                if cell.point.column.0 == 0 {
                    let prefix =
                        if cell.point.line == self.expected_hyperlink.hovered_grid_point.line {
                            '→'
                        } else {
                            ' '
                        };
                    result += &format!("\n{prefix}[{:>3}] ", cell.point.line.to_string());
                }

                result.push(cell.c);
            }

            result
        }
    }

    fn test_hyperlink<'a>(
        columns: usize,
        total_cells: usize,
        test_lines: impl Iterator<Item = &'a str>,
        hyperlink_kind: HyperlinkKind,
        source_location: &str,
    ) {
        thread_local! {
            static TEST_REGEX_SEARCHES: RefCell<RegexSearches> = RefCell::new(RegexSearches::new());
        }

        let term_size = TermSize::new(columns, total_cells / columns + 2);
        let (term, expected_hyperlink) =
            build_term_from_test_lines(hyperlink_kind, term_size, test_lines);
        let hyperlink_found = TEST_REGEX_SEARCHES.with(|regex_searches| {
            find_from_grid_point(
                &term,
                expected_hyperlink.hovered_grid_point,
                &mut regex_searches.borrow_mut(),
            )
        });
        let check_hyperlink_match =
            CheckHyperlinkMatch::new(&term, &expected_hyperlink, source_location);
        match hyperlink_found {
            Some((hyperlink_word, false, hyperlink_match)) => {
                check_hyperlink_match.check_path_with_position_and_match(
                    PathWithPosition::parse_str(&hyperlink_word),
                    &hyperlink_match,
                );
            }
            Some((hyperlink_word, true, hyperlink_match)) => {
                check_hyperlink_match.check_iri_and_match(hyperlink_word, &hyperlink_match);
            }
            _ => {
                assert!(
                    false,
                    "No hyperlink found\n     at {source_location}:\n{}",
                    check_hyperlink_match.format_renderable_content()
                )
            }
        }
    }
}
