use alacritty_terminal::{
    Term,
    event::EventListener,
    grid::Dimensions,
    index::{Boundary, Column, Direction as AlacDirection, Point as AlacPoint},
    term::{
        cell::Flags,
        search::{Match, RegexIter, RegexSearch},
    },
};
use log::{info, warn};
use regex::Regex;
use std::{
    iter::{once, once_with},
    ops::{Index, Range},
    time::{Duration, Instant},
};
use url::Url;
use util::paths::{PathStyle, UrlExt};

const URL_REGEX: &str = r#"(ipfs:|ipns:|magnet:|mailto:|gemini://|gopher://|https://|http://|news:|file://|git://|ssh:|ftp://)[^\u{0000}-\u{001F}\u{007F}-\u{009F}<>"\s{-}\^⟨⟩`']+"#;
const WIDE_CHAR_SPACERS: Flags =
    Flags::from_bits(Flags::LEADING_WIDE_CHAR_SPACER.bits() | Flags::WIDE_CHAR_SPACER.bits())
        .unwrap();

pub(super) struct RegexSearches {
    url_regex: RegexSearch,
    path_hyperlink_regexes: Vec<Regex>,
    path_hyperlink_timeout: Duration,
}

impl Default for RegexSearches {
    fn default() -> Self {
        Self {
            url_regex: RegexSearch::new(URL_REGEX).unwrap(),
            path_hyperlink_regexes: Vec::default(),
            path_hyperlink_timeout: Duration::default(),
        }
    }
}
impl RegexSearches {
    pub(super) fn new(
        path_hyperlink_regexes: impl IntoIterator<Item: AsRef<str>>,
        path_hyperlink_timeout_ms: u64,
    ) -> Self {
        Self {
            url_regex: RegexSearch::new(URL_REGEX).unwrap(),
            path_hyperlink_regexes: path_hyperlink_regexes
                .into_iter()
                .filter_map(|regex| {
                    Regex::new(regex.as_ref())
                        .inspect_err(|error| {
                            warn!(
                                concat!(
                                    "Ignoring path hyperlink regex specified in ",
                                    "`terminal.path_hyperlink_regexes`:\n\n\t{}\n\nError: {}",
                                ),
                                regex.as_ref(),
                                error
                            );
                        })
                        .ok()
                })
                .collect(),
            path_hyperlink_timeout: Duration::from_millis(path_hyperlink_timeout_ms),
        }
    }
}

pub(super) fn find_from_grid_point<T: EventListener>(
    term: &Term<T>,
    point: AlacPoint,
    regex_searches: &mut RegexSearches,
    path_style: PathStyle,
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
    } else {
        let (line_start, line_end) = (term.line_search_left(point), term.line_search_right(point));
        if let Some((url, url_match)) = RegexIter::new(
            line_start,
            line_end,
            AlacDirection::Right,
            term,
            &mut regex_searches.url_regex,
        )
        .find(|rm| rm.contains(&point))
        .map(|url_match| {
            let url = term.bounds_to_string(*url_match.start(), *url_match.end());
            sanitize_url_punctuation(url, url_match, term)
        }) {
            Some((url, true, url_match))
        } else {
            path_match(
                &term,
                line_start,
                line_end,
                point,
                &mut regex_searches.path_hyperlink_regexes,
                regex_searches.path_hyperlink_timeout,
            )
            .map(|(path, path_match)| (path, false, path_match))
        }
    };

    found_word.map(|(maybe_url_or_path, is_url, word_match)| {
        if is_url {
            // Treat "file://" IRIs like file paths to ensure
            // that line numbers at the end of the path are
            // handled correctly.
            // Use Url::to_file_path() to properly handle Windows drive letters
            // (e.g., file:///C:/path -> C:\path)
            if maybe_url_or_path.starts_with("file://") {
                if let Ok(url) = Url::parse(&maybe_url_or_path) {
                    if let Ok(path) = url.to_file_path_ext(path_style) {
                        return (path.to_string_lossy().into_owned(), false, word_match);
                    }
                }
                // Fallback: strip file:// prefix if URL parsing fails
                let path = maybe_url_or_path
                    .strip_prefix("file://")
                    .unwrap_or(&maybe_url_or_path);
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

    // Count parentheses in the URL
    let (open_parens, mut close_parens) =
        sanitized_url
            .chars()
            .fold((0, 0), |(opens, closes), c| match c {
                '(' => (opens + 1, closes),
                ')' => (opens, closes + 1),
                _ => (opens, closes),
            });

    // Remove trailing characters that shouldn't be at the end of URLs
    while let Some(last_char) = sanitized_url.chars().last() {
        let should_remove = match last_char {
            // These may be part of a URL but not at the end. It's not that the spec
            // doesn't allow them, but they are frequently used in plain text as delimiters
            // where they're not meant to be part of the URL.
            '.' | ',' | ':' | ';' => true,
            '(' => true,
            ')' if close_parens > open_parens => {
                close_parens -= 1;

                true
            }
            _ => false,
        };

        if should_remove {
            sanitized_url.pop();
            chars_trimmed += 1;
        } else {
            break;
        }
    }

    if chars_trimmed > 0 {
        let new_end = url_match.end().sub(term, Boundary::Grid, chars_trimmed);
        let sanitized_match = Match::new(*url_match.start(), new_end);
        (sanitized_url, sanitized_match)
    } else {
        (sanitized_url, url_match)
    }
}

fn path_match<T>(
    term: &Term<T>,
    line_start: AlacPoint,
    line_end: AlacPoint,
    hovered: AlacPoint,
    path_hyperlink_regexes: &mut Vec<Regex>,
    path_hyperlink_timeout: Duration,
) -> Option<(String, Match)> {
    if path_hyperlink_regexes.is_empty() || path_hyperlink_timeout.as_millis() == 0 {
        return None;
    }
    debug_assert!(line_start <= hovered);
    debug_assert!(line_end >= hovered);
    let search_start_time = Instant::now();

    let timed_out = || {
        let elapsed_time = Instant::now().saturating_duration_since(search_start_time);
        (elapsed_time > path_hyperlink_timeout)
            .then_some((elapsed_time.as_millis(), path_hyperlink_timeout.as_millis()))
    };

    // This used to be: `let line = term.bounds_to_string(line_start, line_end)`, however, that
    // api compresses tab characters into a single space, whereas we require a cell accurate
    // string representation of the line. The below algorithm does this, but seems a bit odd.
    // Maybe there is a clean api for doing this, but I couldn't find it.
    let mut line = String::with_capacity(
        (line_end.line.0 - line_start.line.0 + 1) as usize * term.grid().columns(),
    );
    let first_cell = &term.grid()[line_start];
    let mut prev_len = 0;
    line.push(first_cell.c);
    let mut prev_char_is_space = first_cell.c == ' ';
    let mut hovered_point_byte_offset = None;
    let mut hovered_word_start_offset = None;
    let mut hovered_word_end_offset = None;

    if line_start == hovered {
        hovered_point_byte_offset = Some(0);
        if first_cell.c != ' ' {
            hovered_word_start_offset = Some(0);
        }
    }

    for cell in term.grid().iter_from(line_start) {
        if cell.point > line_end {
            break;
        }

        if !cell.flags.intersects(WIDE_CHAR_SPACERS) {
            prev_len = line.len();
            match cell.c {
                ' ' | '\t' => {
                    if hovered_point_byte_offset.is_some() && !prev_char_is_space {
                        if hovered_word_end_offset.is_none() {
                            hovered_word_end_offset = Some(line.len());
                        }
                    }
                    line.push(' ');
                    prev_char_is_space = true;
                }
                c @ _ => {
                    if hovered_point_byte_offset.is_none() && prev_char_is_space {
                        hovered_word_start_offset = Some(line.len());
                    }
                    line.push(c);
                    prev_char_is_space = false;
                }
            }
        }

        if cell.point == hovered {
            debug_assert!(hovered_point_byte_offset.is_none());
            hovered_point_byte_offset = Some(prev_len);
        }
    }
    let line = line.trim_ascii_end();
    let hovered_point_byte_offset = hovered_point_byte_offset?;
    let hovered_word_range = {
        let word_start_offset = hovered_word_start_offset.unwrap_or(0);
        (word_start_offset != 0)
            .then_some(word_start_offset..hovered_word_end_offset.unwrap_or(line.len()))
    };
    if line.len() <= hovered_point_byte_offset {
        return None;
    }
    let found_from_range = |path_range: Range<usize>,
                            link_range: Range<usize>,
                            position: Option<(u32, Option<u32>)>| {
        let advance_point_by_str = |mut point: AlacPoint, s: &str| {
            for _ in s.chars() {
                point = term
                    .expand_wide(point, AlacDirection::Right)
                    .add(term, Boundary::Grid, 1);
            }

            // There does not appear to be an alacritty api that is
            // "move to start of current wide char", so we have to do it ourselves.
            let flags = term.grid().index(point).flags;
            if flags.contains(Flags::LEADING_WIDE_CHAR_SPACER) {
                AlacPoint::new(point.line + 1, Column(0))
            } else if flags.contains(Flags::WIDE_CHAR_SPACER) {
                AlacPoint::new(point.line, point.column - 1)
            } else {
                point
            }
        };

        let link_start = advance_point_by_str(line_start, &line[..link_range.start]);
        let link_end = advance_point_by_str(link_start, &line[link_range]);
        let link_match = link_start
            ..=term
                .expand_wide(link_end, AlacDirection::Left)
                .sub(term, Boundary::Grid, 1);

        (
            {
                let mut path = line[path_range].to_string();
                position.inspect(|(line, column)| {
                    path += &format!(":{line}");
                    column.inspect(|column| path += &format!(":{column}"));
                });
                path
            },
            link_match,
        )
    };

    for regex in path_hyperlink_regexes {
        let mut path_found = false;

        for (line_start_offset, captures) in once(
            regex
                .captures_iter(&line)
                .next()
                .map(|captures| (0, captures)),
        )
        .chain(once_with(|| {
            if let Some(hovered_word_range) = &hovered_word_range {
                regex
                    .captures_iter(&line[hovered_word_range.clone()])
                    .next()
                    .map(|captures| (hovered_word_range.start, captures))
            } else {
                None
            }
        }))
        .flatten()
        {
            path_found = true;
            let match_range = captures.get(0).unwrap().range();
            let (mut path_range, line_column) = if let Some(path) = captures.name("path") {
                let parse = |name: &str| {
                    captures
                        .name(name)
                        .and_then(|capture| capture.as_str().parse().ok())
                };

                (
                    path.range(),
                    parse("line").map(|line| (line, parse("column"))),
                )
            } else {
                (match_range.clone(), None)
            };
            let mut link_range = captures
                .name("link")
                .map_or_else(|| match_range.clone(), |link| link.range());

            path_range.start += line_start_offset;
            path_range.end += line_start_offset;
            link_range.start += line_start_offset;
            link_range.end += line_start_offset;

            if !link_range.contains(&hovered_point_byte_offset) {
                // No match, just skip.
                continue;
            }
            let found = found_from_range(path_range, link_range, line_column);

            if found.1.contains(&hovered) {
                return Some(found);
            }
        }

        if path_found {
            return None;
        }

        if let Some((timed_out_ms, timeout_ms)) = timed_out() {
            warn!("Timed out processing path hyperlink regexes after {timed_out_ms}ms");
            info!("{timeout_ms}ms time out specified in `terminal.path_hyperlink_timeout_ms`");
            return None;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::terminal_settings::TerminalSettings;

    use super::*;
    use alacritty_terminal::{
        event::VoidListener,
        grid::Dimensions,
        index::{Boundary, Column, Line, Point as AlacPoint},
        term::{Config, cell::Flags, test::TermSize},
        vte::ansi::Handler,
    };
    use regex::Regex;
    use settings::{self, Settings, SettingsContent};
    use std::{cell::RefCell, ops::RangeInclusive, path::PathBuf, rc::Rc};
    use url::Url;
    use util::paths::PathWithPosition;

    fn re_test(re: &str, hay: &str, expected: Vec<&str>) {
        let results: Vec<_> = Regex::new(re)
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
            ("https://test.com/(((", "https://test.com/"),
            ("https://test.com/(test)(", "https://test.com/(test)"),
            // Cases that should NOT be sanitized (balanced parentheses)
            (
                "https://en.wikipedia.org/wiki/Example_(disambiguation)",
                "https://en.wikipedia.org/wiki/Example_(disambiguation)",
            ),
            ("https://test.com/(hello)", "https://test.com/(hello)"),
            (
                "https://example.com/path(1)(2)",
                "https://example.com/path(1)(2)",
            ),
            // Edge cases
            ("https://test.com/", "https://test.com/"),
            ("https://example.com", "https://example.com"),
        ];

        for (input, expected) in test_cases {
            // Create a minimal terminal for testing
            let term = Term::new(Config::default(), &TermSize::new(80, 24), VoidListener);

            // Create a dummy match that spans the entire input
            let start_point = AlacPoint::new(Line(0), Column(0));
            let end_point = AlacPoint::new(Line(0), Column(input.len()));
            let dummy_match = Match::new(start_point, end_point);

            let (result, _) = sanitize_url_punctuation(input.to_string(), dummy_match, &term);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_url_punctuation_sanitization() {
        // Test URLs with trailing punctuation (sentence/text punctuation)
        // The sanitize_url_punctuation function removes ., ,, :, ;, from the end
        let test_cases = vec![
            ("https://example.com.", "https://example.com"),
            (
                "https://github.com/zed-industries/zed.",
                "https://github.com/zed-industries/zed",
            ),
            (
                "https://example.com/path/file.html.",
                "https://example.com/path/file.html",
            ),
            (
                "https://example.com/file.pdf.",
                "https://example.com/file.pdf",
            ),
            ("https://example.com:8080.", "https://example.com:8080"),
            ("https://example.com..", "https://example.com"),
            (
                "https://en.wikipedia.org/wiki/C.E.O.",
                "https://en.wikipedia.org/wiki/C.E.O",
            ),
            ("https://example.com,", "https://example.com"),
            ("https://example.com/path,", "https://example.com/path"),
            ("https://example.com,,", "https://example.com"),
            ("https://example.com:", "https://example.com"),
            ("https://example.com/path:", "https://example.com/path"),
            ("https://example.com::", "https://example.com"),
            ("https://example.com;", "https://example.com"),
            ("https://example.com/path;", "https://example.com/path"),
            ("https://example.com;;", "https://example.com"),
            ("https://example.com.,", "https://example.com"),
            ("https://example.com.:;", "https://example.com"),
            ("https://example.com!.", "https://example.com!"),
            ("https://example.com/).", "https://example.com/"),
            ("https://example.com/);", "https://example.com/"),
            ("https://example.com/;)", "https://example.com/"),
            (
                "https://example.com/v1.0/api",
                "https://example.com/v1.0/api",
            ),
            ("https://192.168.1.1", "https://192.168.1.1"),
            ("https://sub.domain.com", "https://sub.domain.com"),
            (
                "https://example.com?query=value",
                "https://example.com?query=value",
            ),
            ("https://example.com?a=1&b=2", "https://example.com?a=1&b=2"),
            (
                "https://example.com/path:8080",
                "https://example.com/path:8080",
            ),
        ];

        for (input, expected) in test_cases {
            // Create a minimal terminal for testing
            let term = Term::new(Config::default(), &TermSize::new(80, 24), VoidListener);

            // Create a dummy match that spans the entire input
            let start_point = AlacPoint::new(Line(0), Column(0));
            let end_point = AlacPoint::new(Line(0), Column(input.len()));
            let dummy_match = Match::new(start_point, end_point);

            let (result, _) = sanitize_url_punctuation(input.to_string(), dummy_match, &term);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    macro_rules! test_hyperlink {
        ($($lines:expr),+; $hyperlink_kind:ident) => { {
            use crate::terminal_hyperlinks::tests::line_cells_count;
            use std::cmp;

            let test_lines = vec![$($lines),+];
            let (total_cells, longest_line_cells) =
                test_lines.iter().copied()
                    .map(line_cells_count)
                    .fold((0, 0), |state, cells| (state.0 + cells, cmp::max(state.1, cells)));
            let contains_tab_char = test_lines.iter().copied()
                .map(str::chars).flatten().find(|&c| c == '\t');
            let columns = if contains_tab_char.is_some() {
                // This avoids tabs at end of lines causing whitespace-eating line wraps...
                vec![longest_line_cells + 1]
            } else {
                // Alacritty has issues with 2 columns, use 3 as the minimum for now.
                vec![3, longest_line_cells / 2, longest_line_cells + 1]
            };
            test_hyperlink!(
                columns;
                total_cells;
                test_lines.iter().copied();
                $hyperlink_kind
            )
        } };

        ($columns:expr; $total_cells:expr; $lines:expr; $hyperlink_kind:ident) => { {
            use crate::terminal_hyperlinks::tests::{ test_hyperlink, HyperlinkKind };

            let source_location = format!("{}:{}", std::file!(), std::line!());
            for columns in $columns {
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
            test_path!("‹«/👉test/cool.rs»:(«4»,«2»)›:");
            test_path!("‹«/test/cool.rs»:(«4»,«2»👉)›:");
            test_path!("‹«/👉test/cool.rs»:(«4»:«2»)›:");
            test_path!("‹«/test/cool.rs»:(«4»:«2»👉)›:");
            test_path!("/test/cool.rs:4:2👉:", "What is this?");
            test_path!("/test/cool.rs(4,2)👉:", "What is this?");

            // path, line, column, and description
            test_path!("‹«/test/co👉ol.rs»:«4»:«2»›:Error!");
            test_path!("‹«/test/co👉ol.rs»(«4»,«2»)›:Error!");

            // Cargo output
            test_path!("    Compiling Cool 👉(/test/Cool)");
            test_path!("    Compiling Cool (‹«/👉test/Cool»›)");
            test_path!("    Compiling Cool (/test/Cool👉)");

            // Python
            test_path!("‹«awe👉some.py»›");
            test_path!("‹«👉a»› ");

            test_path!("    ‹F👉ile \"«/awesome.py»\", line «42»›: Wat?");
            test_path!("    ‹File \"«/awe👉some.py»\", line «42»›");
            test_path!("    ‹File \"«/awesome.py»👉\", line «42»›: Wat?");
            test_path!("    ‹File \"«/awesome.py»\", line «4👉2»›");
        }

        #[test]
        fn simple_with_descriptions() {
            // path, line, column and description
            test_path!("‹«/👉test/cool.rs»:«4»:«2»›:例Desc例例例");
            test_path!("‹«/test/cool.rs»:«4»:«👉2»›:例Desc例例例");
            test_path!("‹«/👉test/cool.rs»(«4»,«2»)›:例Desc例例例");
            test_path!("‹«/test/cool.rs»(«4»👉,«2»)›:例Desc例例例");

            // path, line, column and description w/extra colons
            test_path!("‹«/👉test/cool.rs»:«4»:«2»›::例Desc例例例");
            test_path!("‹«/test/cool.rs»:«4»:«👉2»›::例Desc例例例");
            test_path!("‹«/👉test/cool.rs»(«4»,«2»)›::例Desc例例例");
            test_path!("‹«/test/cool.rs»(«4»,«2»👉)›::例Desc例例例");
        }

        #[test]
        fn multiple_same_line() {
            test_path!("‹«/👉test/cool.rs»› /test/cool.rs");
            test_path!("/test/cool.rs ‹«/👉test/cool.rs»›");

            test_path!(
                "‹«🦀 multiple_👉same_line 🦀» 🚣«4» 🏛️«2»›: 🦀 multiple_same_line 🦀 🚣4 🏛️2:"
            );

            // ls output (tab separated)
            test_path!(
                "‹«Carg👉o.toml»›\t\texperiments\t\tnotebooks\t\trust-toolchain.toml\ttooling"
            );
            test_path!(
                "Cargo.toml\t\t‹«exper👉iments»›\t\tnotebooks\t\trust-toolchain.toml\ttooling"
            );
            test_path!(
                "Cargo.toml\t\texperiments\t\t‹«note👉books»›\t\trust-toolchain.toml\ttooling"
            );
            test_path!(
                "Cargo.toml\t\texperiments\t\tnotebooks\t\t‹«rust-t👉oolchain.toml»›\ttooling"
            );
            test_path!(
                "Cargo.toml\t\texperiments\t\tnotebooks\t\trust-toolchain.toml\t‹«too👉ling»›"
            );
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
            test_path!("‹«/test/co👉ol.rs»(«1»,«618»)›::");
        }

        #[test]
        fn quotes_and_brackets() {
            test_path!("\"‹«/test/co👉ol.rs»:«4»›\"");
            test_path!("'‹«/test/co👉ol.rs»:«4»›'");
            test_path!("`‹«/test/co👉ol.rs»:«4»›`");

            test_path!("[‹«/test/co👉ol.rs»:«4»›]");
            test_path!("(‹«/test/co👉ol.rs»:«4»›)");
            test_path!("{‹«/test/co👉ol.rs»:«4»›}");
            test_path!("<‹«/test/co👉ol.rs»:«4»›>");

            test_path!("[\"‹«/test/co👉ol.rs»:«4»›\"]");
            test_path!("'(‹«/test/co👉ol.rs»:«4»›)'");

            test_path!("\"‹«/test/co👉ol.rs»:«4»:«2»›\"");
            test_path!("'‹«/test/co👉ol.rs»:«4»:«2»›'");
            test_path!("`‹«/test/co👉ol.rs»:«4»:«2»›`");

            test_path!("[‹«/test/co👉ol.rs»:«4»:«2»›]");
            test_path!("(‹«/test/co👉ol.rs»:«4»:«2»›)");
            test_path!("{‹«/test/co👉ol.rs»:«4»:«2»›}");
            test_path!("<‹«/test/co👉ol.rs»:«4»:«2»›>");

            test_path!("[\"‹«/test/co👉ol.rs»:«4»:«2»›\"]");

            test_path!("\"‹«/test/co👉ol.rs»(«4»)›\"");
            test_path!("'‹«/test/co👉ol.rs»(«4»)›'");
            test_path!("`‹«/test/co👉ol.rs»(«4»)›`");

            test_path!("[‹«/test/co👉ol.rs»(«4»)›]");
            test_path!("(‹«/test/co👉ol.rs»(«4»)›)");
            test_path!("{‹«/test/co👉ol.rs»(«4»)›}");
            test_path!("<‹«/test/co👉ol.rs»(«4»)›>");

            test_path!("[\"‹«/test/co👉ol.rs»(«4»)›\"]");

            test_path!("\"‹«/test/co👉ol.rs»(«4»,«2»)›\"");
            test_path!("'‹«/test/co👉ol.rs»(«4»,«2»)›'");
            test_path!("`‹«/test/co👉ol.rs»(«4»,«2»)›`");

            test_path!("[‹«/test/co👉ol.rs»(«4»,«2»)›]");
            test_path!("(‹«/test/co👉ol.rs»(«4»,«2»)›)");
            test_path!("{‹«/test/co👉ol.rs»(«4»,«2»)›}");
            test_path!("<‹«/test/co👉ol.rs»(«4»,«2»)›>");

            test_path!("[\"‹«/test/co👉ol.rs»(«4»,«2»)›\"]");

            // Imbalanced
            test_path!("([‹«/test/co👉ol.rs»:«4»›] was here...)");
            test_path!("[Here's <‹«/test/co👉ol.rs»:«4»›>]");
            test_path!("('‹«/test/co👉ol.rs»:«4»›' was here...)");
            test_path!("[Here's `‹«/test/co👉ol.rs»:«4»›`]");
        }

        #[test]
        fn trailing_punctuation() {
            test_path!("‹«/test/co👉ol.rs»›:,..");
            test_path!("/test/cool.rs:,👉..");
            test_path!("‹«/test/co👉ol.rs»:«4»›:,");
            test_path!("/test/cool.rs:4:👉,");
            test_path!("[\"‹«/test/co👉ol.rs»:«4»›\"]:,");
            test_path!("'(‹«/test/co👉ol.rs»:«4»›),,'...");
            test_path!("('‹«/test/co👉ol.rs»:«4»›'::: was here...)");
            test_path!("[Here's <‹«/test/co👉ol.rs»:«4»›>]::: ");
        }

        #[test]
        fn word_wide_chars() {
            // Rust paths
            test_path!("‹«/👉例/cool.rs»›");
            test_path!("‹«/例👈/cool.rs»›");
            test_path!("‹«/例/cool.rs»:«👉4»›");
            test_path!("‹«/例/cool.rs»:«4»:«👉2»›");

            // Cargo output
            test_path!("    Compiling Cool (‹«/👉例/Cool»›)");
            test_path!("    Compiling Cool (‹«/例👈/Cool»›)");

            test_path!("    Compiling Cool (‹«/👉例/Cool Spaces»›)");
            test_path!("    Compiling Cool (‹«/例👈/Cool Spaces»›)");
            test_path!("    Compiling Cool (‹«/👉例/Cool Spaces»:«4»:«2»›)");
            test_path!("    Compiling Cool (‹«/例👈/Cool Spaces»(«4»,«2»)›)");

            test_path!("    --> ‹«/👉例/Cool Spaces»›");
            test_path!("    ::: ‹«/例👈/Cool Spaces»›");
            test_path!("    --> ‹«/👉例/Cool Spaces»:«4»:«2»›");
            test_path!("    ::: ‹«/例👈/Cool Spaces»(«4»,«2»)›");
            test_path!("    panicked at ‹«/👉例/Cool Spaces»:«4»:«2»›:");
            test_path!("    panicked at ‹«/例👈/Cool Spaces»(«4»,«2»)›:");
            test_path!("    at ‹«/👉例/Cool Spaces»:«4»:«2»›");
            test_path!("    at ‹«/例👈/Cool Spaces»(«4»,«2»)›");

            // Python
            test_path!("‹«👉例wesome.py»›");
            test_path!("‹«例👈wesome.py»›");
            test_path!("    ‹File \"«/👉例wesome.py»\", line «42»›: Wat?");
            test_path!("    ‹File \"«/例👈wesome.py»\", line «42»›: Wat?");
        }

        #[test]
        fn non_word_wide_chars() {
            // Mojo diagnostic message
            test_path!("    ‹File \"«/awe👉some.🔥»\", line «42»›: Wat?");
            test_path!("    ‹File \"«/awesome👉.🔥»\", line «42»›: Wat?");
            test_path!("    ‹File \"«/awesome.👉🔥»\", line «42»›: Wat?");
            test_path!("    ‹File \"«/awesome.🔥👈»\", line «42»›: Wat?");
        }

        /// These likely rise to the level of being worth fixing.
        mod issues {
            #[test]
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
            // <https://github.com/zed-industries/zed/issues/12338>
            fn issue_12338_regex() {
                // Issue #12338
                test_path!(".rw-r--r--     0     staff 05-27 14:03 ‹«'test file 👉1.txt'»›");
                test_path!(".rw-r--r--     0     staff 05-27 14:03 ‹«👉'test file 1.txt'»›");
            }

            #[test]
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
            // <https://github.com/zed-industries/zed/issues/40202>
            fn issue_40202() {
                // Elixir
                test_path!("[‹«lib/blitz_apex_👉server/stats/aggregate_rank_stats.ex»:«35»›: BlitzApexServer.Stats.AggregateRankStats.update/2]
                1 #=> 1");
            }

            #[test]
            // <https://github.com/zed-industries/zed/issues/28194>
            fn issue_28194() {
                test_path!(
                    "‹«test/c👉ontrollers/template_items_controller_test.rb»:«20»›:in 'block (2 levels) in <class:TemplateItemsControllerTest>'"
                );
            }

            #[test]
            #[cfg_attr(
                not(target_os = "windows"),
                should_panic(
                    expected = "Path = «/test/cool.rs:4:NotDesc», at grid cells (0, 1)..=(7, 2)"
                )
            )]
            #[cfg_attr(
                target_os = "windows",
                should_panic(
                    expected = r#"Path = «C:\\test\\cool.rs:4:NotDesc», at grid cells (0, 1)..=(8, 1)"#
                )
            )]
            // PathWithPosition::parse_str considers "/test/co👉ol.rs:4:NotDesc" invalid input, but
            // still succeeds and truncates the part after the position. Ideally this would be
            // parsed as the path "/test/co👉ol.rs:4:NotDesc" with no position.
            fn path_with_position_parse_str() {
                test_path!("`‹«/test/co👉ol.rs:4:NotDesc»›`");
                test_path!("<‹«/test/co👉ol.rs:4:NotDesc»›>");

                test_path!("'‹«(/test/co👉ol.rs:4:2)»›'");
                test_path!("'‹«(/test/co👉ol.rs(4))»›'");
                test_path!("'‹«(/test/co👉ol.rs(4,2))»›'");
            }
        }

        /// Minor issues arguably not important enough to fix/workaround...
        mod nits {
            #[test]
            fn alacritty_bugs_with_two_columns() {
                test_path!("‹«/👉test/cool.rs»(«4»)›");
                test_path!("‹«/test/cool.rs»(«👉4»)›");
                test_path!("‹«/test/cool.rs»(«4»,«👉2»)›");

                // Python
                test_path!("‹«awe👉some.py»›");
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
            #[cfg_attr(
                not(target_os = "windows"),
                should_panic(expected = "Path = «/te:st/co:ol.r:s:4:2::::::»")
            )]
            #[cfg_attr(
                target_os = "windows",
                should_panic(expected = r#"Path = «C:\\te:st\\co:ol.r:s:4:2::::::»"#)
            )]
            fn many_trailing_colons_should_be_parsed_as_part_of_the_path() {
                test_path!("‹«/te:st/👉co:ol.r:s:4:2::::::»›");
                test_path!("/test/cool.rs:::👉:");
            }
        }

        mod windows {
            // Lots of fun to be had with long file paths (verbatim) and UNC paths on Windows.
            // See <https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation>
            // See <https://users.rust-lang.org/t/understanding-windows-paths/58583>
            // See <https://github.com/rust-lang/cargo/issues/13919>

            #[test]
            fn default_prompts() {
                // Windows command prompt
                test_path!(r#"‹«C:\Users\someone\👉test»›>"#);
                test_path!(r#"C:\Users\someone\test👉>"#);

                // Windows PowerShell
                test_path!(r#"PS ‹«C:\Users\someone\👉test\cool.rs»›>"#);
                test_path!(r#"PS C:\Users\someone\test\cool.rs👉>"#);
            }

            #[test]
            fn unc() {
                test_path!(r#"‹«\\server\share\👉test\cool.rs»›"#);
                test_path!(r#"‹«\\server\share\test\cool👉.rs»›"#);
            }

            mod issues {
                #[test]
                fn issue_verbatim() {
                    test_path!(r#"‹«\\?\C:\👉test\cool.rs»›"#);
                    test_path!(r#"‹«\\?\C:\test\cool👉.rs»›"#);
                }

                #[test]
                fn issue_verbatim_unc() {
                    test_path!(r#"‹«\\?\UNC\server\share\👉test\cool.rs»›"#);
                    test_path!(r#"‹«\\?\UNC\server\share\test\cool👉.rs»›"#);
                }
            }
        }

        mod perf {
            use super::super::*;
            use crate::TerminalSettings;
            use alacritty_terminal::{
                event::VoidListener,
                grid::Scroll,
                index::{Column, Point as AlacPoint},
                term::test::mock_term,
                term::{Term, search::Match},
            };
            use settings::{self, Settings, SettingsContent};
            use std::{cell::RefCell, rc::Rc};
            use util_macros::perf;

            fn build_test_term(
                line: &str,
                repeat: usize,
                hover_offset_column: usize,
            ) -> (Term<VoidListener>, AlacPoint) {
                let content = line.repeat(repeat);
                let mut term = mock_term(&content);
                term.resize(TermSize {
                    columns: 1024,
                    screen_lines: 10,
                });
                term.scroll_display(Scroll::Top);
                let point =
                    AlacPoint::new(Line(term.topmost_line().0 + 3), Column(hover_offset_column));
                (term, point)
            }

            #[perf]
            pub fn cargo_hyperlink_benchmark() {
                const LINE: &str = "    Compiling terminal v0.1.0 (/Hyperlinks/Bench/Source/zed-hyperlinks/crates/terminal)\r\n";
                thread_local! {
                    static TEST_TERM_AND_POINT: (Term<VoidListener>, AlacPoint) =
                        build_test_term(LINE, 500, 50);
                }
                TEST_TERM_AND_POINT.with(|(term, point)| {
                    assert_eq!(
                        find_from_grid_point_bench(term, *point)
                            .map(|(path, ..)| path)
                            .unwrap_or_default(),
                        "/Hyperlinks/Bench/Source/zed-hyperlinks/crates/terminal",
                        "Hyperlink should have been found"
                    );
                });
            }

            #[perf]
            pub fn rust_hyperlink_benchmark() {
                const LINE: &str = "    --> /Hyperlinks/Bench/Source/zed-hyperlinks/crates/terminal/terminal.rs:1000:42\r\n";
                thread_local! {
                    static TEST_TERM_AND_POINT: (Term<VoidListener>, AlacPoint) =
                        build_test_term(LINE, 500, 50);
                }
                TEST_TERM_AND_POINT.with(|(term, point)| {
                    assert_eq!(
                        find_from_grid_point_bench(term, *point)
                            .map(|(path, ..)| path)
                            .unwrap_or_default(),
                        "/Hyperlinks/Bench/Source/zed-hyperlinks/crates/terminal/terminal.rs:1000:42",
                        "Hyperlink should have been found"
                    );
                });
            }

            #[perf]
            pub fn ls_hyperlink_benchmark() {
                const LINE: &str = "Cargo.toml        experiments        notebooks        rust-toolchain.toml    tooling\r\n";
                thread_local! {
                    static TEST_TERM_AND_POINT: (Term<VoidListener>, AlacPoint) =
                        build_test_term(LINE, 500, 60);
                }
                TEST_TERM_AND_POINT.with(|(term, point)| {
                    assert_eq!(
                        find_from_grid_point_bench(term, *point)
                            .map(|(path, ..)| path)
                            .unwrap_or_default(),
                        "rust-toolchain.toml",
                        "Hyperlink should have been found"
                    );
                });
            }

            #[perf]
            // https://github.com/zed-industries/zed/pull/44407
            pub fn pr_44407_hyperlink_benchmark() {
                const LINE: &str = "-748, 706, 163, 222, -980, 949, 381, -568, 199, 501, 760, -821, 90, -451, 183, 867, -351, -810, -762, -109, 423, 84, 14, -77, -820, -345, 74, -791, 930, -618, -900, 862, -959, 289, -19, 471, -757, 793, 155, -554, 249, 830, 402, 732, -731, -866, -720, -703, -257, -439, 731, 872, -489, 676, -167, 613, -698, 415, -80, -453, -896, 333, -511, 621, -450, 624, -309, -575, 177, 141, 891, -104, -97, -367, -599, -675, 607, -225, -760, 552, -465, 804, 55, 282, 104, -929, -252,\
-311, 900, 550, 599, -80, 774, 553, 837, -395, 541, 953, 154, -396, -596, -111, -802, -221, -337, -633, -73, -527, -82, -658, -264, 222, 375, 434, 204, -756, -703, 303, 239, -257, -365, -351, 904, 364, -743, -484, 655, -542, 446, 888, 632, -167, -260, 716, 150, 806, 723, 513, -118, -323, -683, 983, -564, 358, -16, -287, 277, -607, 87, 365, -1, 164, 401, 257, 369, -893, 145, -969, 375, -53, 541, -408, -865, 753, 258, 337, -886, 593, -378, -528, 191, 204, 566, -61, -621, 769, 524, -628, 6,\
249, 896, -785, -776, 321, -681, 604, -740, 886, 426, -480, -983, 23, -247, 125, -666, 913, 842, -460, -797, -483, -58, -565, -587, -206, 197, 715, 764, -97, 457, -149, -226, 261, 194, -390, 431, 180, -778, 829, -657, -668, 397, 859, 152, -178, 677, -18, 687, -247, 96, 466, -572, 478, 622, -143, -25, -471, 265, 335, 957, 152, -951, -647, 670, 57, 152, -115, 206, 87, 629, -798, -125, -725, -31, 844, 398, -876, 44, 963, -211, 518, -8, -103, -999, 948, 823, 149, -803, 769, -236, -683, 527,\
-108, -36, 18, -437, 687, -305, -526, 972, -965, 276, 420, -259, -379, -142, -747, 600, -578, 197, 673, 890, 324, -931, 755, -765, -422, 785, -369, -110, -505, 532, -208, -438, 713, 110, 853, 996, -360, 823, 289, -699, 629, -661, 560, -329, -323, 439, 571, -537, 644, -84, 25, -536, -161, 112, 169, -922, -537, -734, -423, 37, 451, -149, 408, 18, -672, 206, -784, 444, 593, -241, 502, -259, -798, -352, -658, 712, -675, -734, 627, -620, 64, -554, 999, -537, -160, -641, 464, 894, 29, 322, 566,\
-510, -749, 982, 204, 967, -261, -986, -136, 251, -598, 995, -831, 891, 22, 761, -783, -415, 125, 470, -919, -97, -668, 85, 205, -175, -550, 502, 652, -468, 798, 775, -216, 89, -433, -24, -621, 877, -126, 951, 809, 782, 156, -618, -841, -463, 19, -723, -904, 550, 263, 991, -758, -114, 446, -731, -623, -634, 462, 48, 851, 333, -846, 480, 892, -966, -910, -436, 317, -711, -341, -294, 124, 238, -214, -281, 467, -950, -342, 913, -90, -388, -573, 740, -883, -451, 493, -500, 863, 930, 127, 530,\
-810, 540, 541, -664, -951, -227, -420, -476, -581, -534, 549, 253, 984, -985, -84, -521, 538, 484, -440, 371, 784, -306, -850, 530, -133, 251, -799, 446, -170, -243, -674, 769, 646, 778, -680, -714, -442, 804, 901, -774, 69, 307, -293, 755, 443, 224, -918, -771, 723, 40, 132, 568, -847, -47, 844, 69, 986, -293, -459, 313, 155, 331, 69, 280, -637, 569, 104, -119, -988, 252, 857, -590, 810, -891, 484, 566, -934, -587, -290, 566, 587, 489, 870, 280, 454, -252, 613, -701, -278, 195, -198,\
683, 533, -372, 707, -152, 371, 866, 609, -5, -372, -30, -694, 552, 192, 452, -663, 350, -985, 10, 884, 813, -592, -331, -470, 711, -941, 928, 379, -339, 220, 999, 376, 507, 179, 916, 84, 104, 392, 192, 299, -860, 218, -698, -919, -452, 37, 850, 5, -874, 287, 123, -746, -575, 776, -909, 118, 903, -275, 450, -996, -591, -920, -850, 453, -896, 73, 83, -535, -20, 287, -765, 442, 808, 45, 445, 202, 917, -208, 783, 790, -534, 373, -129, 556, -757, -69, 459, -163, -59, 265, -563, -889, 635,\
-583, -261, -790, 799, 826, 953, 85, 619, 334, 842, 672, -869, -4, -833, 315, 942, -524, 579, 926, 628, -404, 128, -629, 161, 568, -117, -526, 223, -876, 906, 176, -549, -317, 381, 375, -801, -416, 647, 335, 253, -386, -375, -254, 635, 352, 317, 398, -422, 111, 201, 220, 554, -972, 853, 378, 956, 942, -857, -289, -333, -180, 488, -814, -42, -595, 721, 39, 644, 721, -242, -44, 643, -457, -419, 560, -863, 974, 458, 222, -882, 526, -243, -318, -343, -707, -401, 117, 677, -489, 546, -903,\
-960, -881, -684, 125, -928, -995, -692, -773, 647, -718, -862, -814, 671, 664, -130, -856, -674, 653, 711, 194, -685, -160, 138, -27, -128, -671, -242, 526, 494, -674, 424, -921, -778, 313, -237, 332, 913, 252, 808, -936, 289, 755, 52, -139, 57, -19, -827, -775, -561, -14, 107, -84, 622, -303, -747, 258, -942, 290, 211, -919, -207, 797, 95, 794, -830, -181, -788, 757, 75, -946, -949, -988, 152, 340, 732, 886, -891, -642, -666, 321, -910, 841, 632, 298, 55, -349, 498, 287, -711, 97, 305,\
-974, -987, 790, -64, 605, -583, -821, 345, 887, -861, 548, 894, 288, 452, 556, -448, 813, 420, 545, 967, 127, -947, 19, -314, -607, -513, -851, 254, -290, -938, -783, -93, 474, 368, -485, -935, -539, 81, 404, -283, 779, 345, -164, 53, 563, -771, 911, -323, 522, -998, 315, 415, 460, 58, -541, -878, -152, -886, 201, -446, -810, 549, -142, -575, -632, 521, 549, 209, -681, 998, 798, -611, -919, -708, -4, 677, -172, 588, 750, -435, 508, 609, 498, -535, -691, -738, 85, 615, 705, 169, 425,\
-669, -491, -783, 73, -847, 228, -981, -812, -229, 950, -904, 175, -438, 632, -556, 910, 173, 576, -751, -53, -169, 635, 607, -944, -13, -84, 105, -644, 984, 935, 259, -445, 620, -405, 832, 167, 114, 209, -181, -944, -496, 693, -473, 137, 38, -873, -334, -353, -57, 397, 944, 698, 811, -401, 712, -667, 905, 276, -653, 368, -543, -349, 414, 287, 894, 935, 461, 55, 741, -623, -660, -773, 617, 834, 278, -121, 52, 495, -855, -440, -210, -99, 279, -661, 540, 934, 540, 784, 895, 268, -503, 513,\
-484, -352, 528, 341, -451, 885, -71, 799, -195, -885, -585, -233, 92, 453, 994, 464, 694, 190, -561, -116, 675, -775, -236, 556, -110, -465, 77, -781, 507, -960, -410, 229, -632, 717, 597, 429, 358, -430, -692, -825, 576, 571, 758, -891, 528, -267, 190, -869, 132, -811, 796, 750, -596, -681, 870, 360, 969, 860, -412, -567, 694, -86, -498, 38, -178, -583, -778, 412, 842, -586, 722, -192, 350, 363, 81, -677, -163, 564, 543, 671, 110, 314, 739, -552, -224, -644, 922, 685, 134, 613, 793,\
-363, -244, -284, -257, -561, 418, 988, 333, 110, -966, 790, 927, 536, -620, -309, -358, 895, -867, -796, -357, 308, -740, 287, -732, -363, -969, 658, 711, 511, 256, 590, -574, 815, -845, -84, 546, -581, -71, -334, -890, 652, -959, 320, -236, 445, -851, 825, -756, -4, 877, 308, 573, -117, 293, 686, -483, 391, 342, -550, -982, 713, 886, 552, 474, -673, 283, -591, -383, 988, 435, -131, 708, -326, -884, 87, 680, -818, -408, -486, 813, -307, -799, 23, -497, 802, -146, -100, 541, 7, -493, 577,\
50, -270, 672, 834, 111, -788, 247, 337, 628, -33, -964, -519, 683, 54, -703, 633, -127, -448, 759, -975, 696, 2, -870, -760, 67, 696, 306, 750, 615, 155, -933, -568, 399, 795, 164, -460, 205, 439, -526, -691, 35, -136, -481, -63, 73, -598, 748, 133, 874, -29, 4, -73, 472, 389, 962, 231, -328, 240, 149, 959, 46, -207, 72, -514, -608, 0, -14, 32, 374, -478, -806, 919, -729, -286, 652, 109, 509, -879, -979, -865, 584, -92, -346, -992, 781, 401, 575, 993, -746, -33, 684, -683, 750, -105,\
-425, -508, -627, 27, 770, -45, 338, 921, -139, -392, -933, 634, 563, 224, -780, 921, 991, 737, 22, 64, 414, -249, -687, 869, 50, 759, -97, 515, 20, -775, -332, 957, 138, -542, -835, 591, -819, 363, -715, -146, -950, -641, -35, -435, -407, -548, -984, 383, -216, -559, 853, 4, -410, -319, -831, -459, -628, -819, -324, 755, 696, -192, 238, -234, -724, -445, 915, 302, -708, 484, 224, -641, 25, -771, 528, -106, -744, -588, 913, -554, -515, -239, -843, -812, -171, 721, 543, -269, 440, 151,\
996, -723, -557, -522, -280, -514, -593, 208, 715, 404, 353, 270, -483, -785, 318, -313, 798, 638, 764, 748, -929, -827, -318, -56, 389, -546, -958, -398, 463, -700, 461, 311, -787, -488, 877, 456, 166, 535, -995, -189, -715, 244, 40, 484, 212, -329, -351, 638, -69, -446, -292, 801, -822, 490, -486, -185, 790, 370, -340, 401, -656, 584, 561, -749, 269, -19, -294, -111, 975, 874, -73, 851, 231, -331, -684, 460, 765, -654, -76, 10, 733, 520, 521, 416, -958, -202, -186, -167, 175, 343, -50,\
673, -763, -854, -977, -17, -853, -122, -25, 180, 149, 268, 874, -816, -745, 747, -303, -959, 390, 509, 18, -66, 275, -277, 9, 837, -124, 989, -542, -649, -845, 894, 926, 997, -847, -809, -579, -96, -372, 766, 238, -251, 503, 559, 276, -281, -102, -735, 815, 109, 175, -10, 128, 543, -558, -707, 949, 996, -422, -506, 252, 702, -930, 552, -961, 584, -79, -177, 341, -275, 503, -21, 677, -545, 8, -956, -795, -870, -254, 170, -502, -880, 106, 174, 459, 603, -600, -963, 164, -136, -641, -309,\
-380, -707, -727, -10, 727, 952, 997, -731, -133, 269, 287, 855, 716, -650, 479, 299, -839, -308, -782, 769, 545, 663, -536, -115, 904, -986, -258, -562, 582, 664, 408, -525, -889, 471, -370, -534, -220, 310, 766, 931, -193, -897, -192, -74, -365, -256, -359, -328, 658, -691, -431, 406, 699, 425, 713, -584, -45, -588, 289, 658, -290, -880, -987, -444, 371, 904, -155, 81, -278, -708, -189, -78, 655, 342, -998, -647, -734, -218, 726, 619, 663, 744, 518, 60, -409, 561, -727, -961, -306,\
-147, -550, 240, -218, -393, 267, 724, 791, -548, 480, 180, -631, 825, -170, 107, 227, -691, 905, -909, 359, 227, 287, 909, 632, -89, -522, 80, -429, 37, 561, -732, -474, 565, -798, -460, 188, 507, -511, -654, 212, -314, -376, -997, -114, -708, 512, -848, 781, 126, -956, -298, 354, -400, -121, 510, 445, 926, 27, -708, 676, 248, 834, 542, 236, -105, -153, 102, 128, 96, -348, -626, 598, 8, 978, -589, -461, -38, 381, -232, -817, 467, 356, -151, -460, 429, -408, 425, 618, -611, -247, 819,\
963, -160, 1000, 141, -647, -875, 108, 790, -127, 463, -37, -195, -542, 12, 845, -384, 770, -129, 315, 826, -942, 430, 146, -170, -583, -903, -489, 497, -559, -401, -29, -129, -411, 166, 942, -646, -862, -404, 785, 777, -111, -481, -738, 490, 741, -398, 846, -178, -509, -661, 748, 297, -658, -567, 531, 427, -201, -41, -808, -668, 782, -860, -324, 249, 835, -234, 116, 542, -201, 328, 675, 480, -906, 188, 445, 63, -525, 811, 277, 133, 779, -680, 950, -477, -306, -64, 552, -890, -956, 169,\
442, 44, -169, -243, -242, 423, -884, -757, -403, 739, -350, 383, 429, 153, -702, -725, 51, 310, 857, -56, 538, 46, -311, 132, -620, -297, -124, 534, 884, -629, -117, 506, -837, -100, -27, -381, -735, 262, 843, 703, 260, -457, 834, 469, 9, 950, 59, 127, -820, 518, 64, -783, 659, -608, -676, 802, 30, 589, 246, -369, 361, 347, 534, -376, 68, 941, 709, 264, 384, 481, 628, 199, -568, -342, -337, 853, -804, -858, -169, -270, 641, -344, 112, 530, -773, -349, -135, -367, -350, -756, -911, 180,\
-660, 116, -478, -265, -581, 510, 520, -986, 935, 219, 522, 744, 47, -145, 917, 638, 301, 296, 858, -721, 511, -816, 328, 473, 441, 697, -260, -673, -379, 893, 458, 154, 86, 905, 590, 231, -717, -179, 79, 272, -439, -192, 178, -200, 51, 717, -256, -358, -626, -518, -314, -825, -325, 588, 675, -892, -798, 448, -518, 603, -23, 668, -655, 845, -314, 783, -347, -496, 921, 893, -163, -748, -906, 11, -143, -64, 300, 336, 882, 646, 533, 676, -98, -148, -607, -952, -481, -959, -874, 764, 537,\
736, -347, 646, -843, 966, -916, -718, -391, -648, 740, 755, 919, -608, 388, -655, 68, 201, 675, -855, 7, -503, 881, 760, 669, 831, 721, -564, -445, 217, 331, 970, 521, 486, -254, 25, -259, 336, -831, 252, -995, 908, -412, -240, 123, -478, 366, 264, -504, -843, 632, -288, 896, 301, 423, 185, 318, 380, 457, -450, -162, -313, 673, -963, 570, 433, -548, 107, -39, -142, -98, -884, -3, 599, -486, -926, 923, -82, 686, 290, 99, -382, -789, 16, 495, 570, 284, 474, -504, -201, -178, -1, 592, 52,\
827, -540, -151, -991, 130, 353, -420, -467, -661, 417, -690, 942, 936, 814, -566, -251, -298, 341, -139, 786, 129, 525, -861, 680, 955, -245, -50, 331, 412, -38, -66, 611, -558, 392, -629, -471, -68, -535, 744, 495, 87, 558, 695, 260, -308, 215, -464, 239, -50, 193, -540, 184, -8, -194, 148, 898, -557, -21, 884, 644, -785, -689, -281, -737, 267, 50, 206, 292, 265, 380, -511, 310, 53, 375, -497, -40, 312, -606, -395, 142, 422, 662, -584, 72, 144, 40, -679, -593, 581, 689, -829, 442, 822,\
977, -832, -134, -248, -207, 248, 29, 259, 189, 592, -834, -866, 102, 0, 340, 25, -354, -239, 420, -730, -992, -925, -314, 420, 914, 607, -296, -415, -30, 813, 866, 153, -90, 150, -81, 636, -392, -222, -835, 482, -631, -962, -413, -727, 280, 686, -382, 157, -404, -511, -432, 455, 58, 108, -408, 290, -829, -252, 113, 550, -935, 925, 422, 38, 789, 361, 487, -460, -769, -963, -285, 206, -799, -488, -233, 416, 143, -456, 753, 520, 599, 621, -168, 178, -841, 51, 952, 374, 166, -300, -576, 844,\
-656, 90, 780, 371, 730, -896, -895, -386, -662, 467, -61, 130, -362, -675, -113, 135, -761, -55, 408, 822, 675, -347, 725, 114, 952, -510, -972, 390, -413, -277, -52, 315, -80, 401, -712, 147, -202, 84, 214, -178, 970, -571, -210, 525, -887, -863, 504, 192, 837, -594, 203, -876, -209, 305, -826, 377, 103, -928, -803, -956, 949, -868, -547, 824, -994, 516, 93, -524, -866, -890, -988, -501, 15, -6, 413, -825, 304, -818, -223, 525, 176, 610, 828, 391, 940, 540, -831, 650, 438, 589, 941, 57,\
523, 126, 221, 860, -282, -262, -226, 764, 743, -640, 390, 384, -434, 608, -983, 566, -446, 618, 456, -176, -278, 215, 871, -180, 444, -931, -200, -781, 404, 881, 780, -782, 517, -739, -548, -811, 201, -95, -249, -228, 491, -299, 700, 964, -550, 108, 334, -653, 245, -293, -552, 350, -685, -415, -818, 216, -194, -255, 295, 249, 408, 351, 287, 379, 682, 231, -693, 902, -902, 574, 937, -708, -402, -460, 827, -268, 791, 343, -780, -150, -738, 920, -430, -88, -361, -588, -727, -47, -297, 662,\
-840, -637, -635, 916, -857, 938, 132, -553, 391, -522, 640, 626, 690, 833, 867, -555, 577, 226, 686, -44, 0, -965, 651, -1, 909, 595, -646, 740, -821, -648, -962, 927, -193, 159, 490, 594, -189, 707, -884, 759, -278, -160, -566, -340, 19, 862, -440, 445, -598, 341, 664, -311, 309, -159, 19, -672, 705, -646, 976, 247, 686, -830, -27, -667, 81, 399, -423, -567, 945, 38, 51, 740, 621, 204, -199, -908, -593, 424, 250, -561, 695, 9, 520, 878, 120, -109, 42, -375, -635, -711, -687, 383, -278,\
36, 970, 925, 864, 836, 309, 117, 89, 654, -387, 346, -53, 617, -164, -624, 184, -45, 852, 498, -513, 794, -682, -576, 13, -147, 285, -776, -886, -96, 483, 994, -188, 346, -629, -848, 738, 51, 128, -898, -753, -906, 270, -203, -577, 48, -243, -210, 666, 353, 636, -954, 862, 560, -944, -877, -137, 440, -945, -316, 274, -211, -435, 615, -635, -468, 744, 948, -589, 525, 757, -191, -431, 42, 451, -160, -827, -991, 324, 697, 342, -610, 894, -787, -384, 872, 734, 878, 70, -260, 57, 397, -518,\
629, -510, -94, 207, 214, -625, 106, -882, -575, 908, -650, 723, -154, 45, 108, -69, -565, 927, -68, -351, 707, -282, 429, -889, -596, 848, 578, -492, 41, -822, -992, 168, -286, -780, 970, 597, -293, -12, 367, 708, -415, 194, -86, -390, 224, 69, -368, -674, 1000, -672, 356, -202, -169, 826, 476, -285, 29, -448, 545, 186, 319, 67, 705, 412, 225, -212, -351, -391, -783, -9, 875, -59, -159, -123, -151, -296, 871, -638, 359, 909, -945, 345, -16, -562, -363, -183, -625, -115, -571, -329, 514,\
99, 263, 463, -39, 597, -652, -349, 246, 77, -127, -563, -879, -30, 756, 777, -865, 675, -813, -501, 871, -406, -627, 834, -609, -205, -812, 643, -204, 291, -251, -184, -584, -541, 410, -573, -600, 908, -871, -687, 296, -713, -139, -778, -790, 347, -52, -400, 407, -653, 670, 39, -856, 904, 433, 392, 590, -271, -144, -863, 443, 353, 468, -544, 486, -930, 458, -596, -890, 163, 822, 768, 980, -783, -792, 126, 386, 367, -264, 603, -61, 728, 160, -4, -837, 832, 591, 436, 518, 796, -622, -867,\
-669, -947, 253, 100, -792, 841, 413, 833, -249, -550, 282, -825, 936, -348, 898, -451, -283, 818, -237, 630, 216, -499, -637, -511, 767, -396, 221, 958, -586, -920, 401, -313, -580, -145, -270, 118, 497, 426, -975, 480, -445, -150, -721, -929, 439, -893, 902, 960, -525, -793, 924, 563, 683, -727, -86, 309, 432, -762, -345, 371, -617, 149, -215, -228, 505, 593, -20, -292, 704, -999, 149, -104, 819, -414, -443, 517, -599, -5, 145, -24, -993, -283, 904, 174, -112, -276, -860, 44, -257,\
-931, -821, -667, 540, 421, 485, 531, 407, 833, 431, -415, 878, 503, -901, 639, -608, 896, 860, 927, 424, 113, -808, -323, 729, 382, -922, 548, -791, -379, 207, 203, 559, 537, 137, 999, -913, -240, 942, 249, 616, 775, -4, 915, 855, -987, -234, -384, 948, -310, -542, 125, -289, -599, 967, -492, -349, -552, 562, -926, 632, -164, 217, -165, -496, 847, 684, -884, 457, -748, -745, -38, 93, 961, 934, 588, 366, -130, 851, -803, -811, -211, 428, 183, -469, 888, 596, -475, -899, -681, 508, 184,\
921, 863, -610, -416, -119, -966, -686, 210, 733, 715, -889, -925, -434, -566, -455, 596, -514, 983, 755, -194, -802, -313, 91, -541, 808, -834, 243, -377, 256, 966, -402, -773, -308, -605, 266, 866, 118, -425, -531, 498, 666, 813, -267, 830, 69, -869, -496, 735, 28, 488, -645, -493, -689, 170, -940, 532, 844, -658, -617, 408, -200, 764, -665, 568, 342, 621, 908, 471, 280, 859, 709, 898, 81, -547, 406, 514, -595, 43, -824, -696, -746, -429, -59, -263, -813, 233, 279, -125, 687, -418,\
-530, 409, 614, 803, -407, 78, -676, -39, -887, -141, -292, 270, -343, 400, 907, 588, 668, 899, 973, 103, -101, -11, 397, -16, 165, 705, -410, -585, 316, 391, -346, -336, 957, -118, -538, -441, -845, 121, 591, -359, -188, -362, -208, 27, -925, -157, -495, -177, -580, 9, 531, -752, 94, 107, 820, 769, -500, 852, 617, 145, 355, 34, -463, -265, -709, -111, -855, -405, 560, 470, 3, -177, -164, -249, 450, 662, 841, -689, -509, 987, -33, 769, 234, -2, 203, 780, 744, -895, 497, -432, -406, -264,\
-71, 124, 778, -897, 495, 127, -76, 52, -768, 205, 464, -992, 801, -83, -806, 545, -316, 146, 772, 786, 289, -936, 145, -30, -722, -455, 270, 444, 427, -482, 383, -861, 36, 630, -404, 83, 864, 743, -351, -846, 315, -837, 357, -195, 450, -715, 227, -942, 740, -519, 476, 716, 713, 169, 492, -112, -49, -931, 866, 95, -725, 198, -50, -17, -660, 356, -142, -781, 53, 431, 720, 143, -416, 446, -497, 490, -96, 157, 239, 487, -337, -224, -445, 813, 92, -22, 603, 424, 952, -632, -367, 898, -927,\
884, -277, -187, -777, 537, -575, -313, 347, -33, 800, 672, -919, -541, 5, -270, -94, -265, -793, -183, -761, -516, -608, -218, 57, -889, -912, 508, 93, -90, 34, 530, 201, 999, -37, -186, -62, -980, 239, 902, 983, -287, -634, 524, -772, 470, -961, 32, 162, 315, -411, 400, -235, -283, -787, -703, 869, 792, 543, -274, 239, 733, -439, 306, 349, 579, -200, -201, -824, 384, -246, 133, -508, 770, -102, 957, -825, 740, 748, -376, 183, -426, 46, 668, -886, -43, -174, 672, -419, 390, 927, 1000,\
318, 886, 47, 908, -540, -825, -5, 314, -999, 354, -603, 966, -633, -689, 985, 534, -290, 167, -652, -797, -612, -79, 488, 622, -464, -950, 595, 897, 704, -238, -395, 125, 831, -180, 226, -379, 310, 564, 56, -978, 895, -61, 686, -251, 434, -417, 161, -512, 752, 528, -589, -425, 66, -925, -157, 1000, 96, 256, -239, -784, -882, -464, -909, 663, -177, -678, -441, 669, -564, -201, -121, -743, 187, -107, -768, -682, 355, 161, 411, 984, -954, 166, -842, -755, 267, -709, 372, -699, -272, -850,\
403, -839, 949, 622, -62, 51, 917, 70, 528, -558, -632, 832, 276, 61, -445, -195, 960, 846, -474, 764, 879, -411, 948, -62, -592, -123, -96, -551, -555, -724, 849, 250, -808, -732, 797, -839, -554, 306, -919, 888, 484, -728, 152, -122, -287, 16, -345, -396, -268, -963, -500, 433, 343, 418, -480, 828, 594, 821, -9, 933, -230, 707, -847, -610, -748, -234, 688, 935, 713, 865, -743, 293, -143, -20, 928, -906, -762, 528, 722, 412, -70, 622, -245, 539, -686, 730, -866, -705, 28, -916, -623,\
-768, -614, -915, -123, -183, 680, -223, 515, -37, -235, -5, 260, 347, -239, -322, -861, -848, -936, 945, 721, -580, -639, 780, -153, -26, 685, 177, 587, 307, -915, 435, 658, 539, -229, -719, -171, -858, 162, 734, -539, -437, 246, 639, 765, -477, -342, -209, -284, -779, -414, -452, 914, 338, -83, 759, 567, 266, -485, 14, 225, 347, -432, -242, 997, -365, -764, 119, -641, -416, -388, -436, -388, -54, -649, -571, -920, -477, 714, -363, 836, 369, 702, 869, 503, -287, -679, 46, -666, -202,\
-602, 71, -259, 967, 601, -571, -830, -993, -271, 281, -494, 482, -180, 572, 587, -651, -566, -448, -228, 511, -924, 832, -52, -712, 402, -644, -533, -865, 269, 965, 56, 675, 179, -338, -272, 614, 602, -283, 303, -70, 909, -942, 117, 839, 468, 813, -765, 884, -697, -813, 352, 374, -705, -295, 633, 211, -754, 597, -941, -142, -393, -469, -653, 688, 996, 911, 214, 431, 453, -141, 874, -81, -258, -735, -3, -110, -338, -929, -182, -306, -104, -840, -588, -759, -157, -801, 848, -698, 627, 914,\
-33, -353, 425, 150, -798, 553, 934, -778, -196, -132, 808, 745, -894, 144, 213, 662, 273, -79, 454, -60, -467, 48, -15, -807, 69, -930, 749, 559, -867, -103, 258, -677, 750, -303, 846, -227, -936, 744, -770, 770, -434, 594, -477, 589, -612, 535, 357, -623, 683, 369, 905, 980, -410, -663, 762, -888, -563, -845, 843, 353, -491, 996, -255, -336, -132, 695, -823, 289, -143, 365, 916, 877, 245, -530, -848, -804, -118, -108, 847, 620, -355, 499, 881, 92, -640, 542, 38, 626, -260, -34, -378,\
598, 890, 305, -118, 711, -385, 600, -570, 27, -129, -893, 354, 459, 374, 816, 470, 356, 661, 877, 735, -286, -780, 620, 943, -169, -888, 978, 441, -667, -399, 662, 249, 137, 598, -863, -453, 722, -815, -251, -995, -294, -707, 901, 763, 977, 137, 431, -994, 905, 593, 694, 444, -626, -816, 252, 282, 616, 841, 360, -932, 817, -908, 50, 394, -120, -786, -338, 499, -982, -95, -454, 838, -312, 320, -127, -653, 53, 16, 988, -968, -151, -369, -836, 293, -271, 483, 18, 724, -204, -965, 245, 310,\
987, 552, -835, -912, -861, 254, 560, 124, 145, 798, 178, 476, 138, -311, 151, -907, -886, -592, 728, -43, -489, 873, -422, -439, -489, 375, -703, -459, 338, 418, -25, 332, -454, 730, -604, -800, 37, -172, -197, -568, -563, -332, 228, -182, 994, -123, 444, -567, 98, 78, 0, -504, -150, 88, -936, 199, -651, -776, 192, 46, 526, -727, -991, 534, -659, -738, 256, -894, 965, -76, 816, 435, -418, 800, 838, 67, -733, 570, 112, -514, -416\r\
";
                thread_local! {
                    static TEST_TERM_AND_POINT: (Term<VoidListener>, AlacPoint) =
                        build_test_term(&LINE, 5, 50);
                }
                TEST_TERM_AND_POINT.with(|(term, point)| {
                    assert_eq!(
                        find_from_grid_point_bench(term, *point)
                            .map(|(path, ..)| path)
                            .unwrap_or_default(),
                        "392",
                        "Hyperlink should have been found"
                    );
                });
            }

            #[perf]
            // https://github.com/zed-industries/zed/issues/44510
            pub fn issue_44510_hyperlink_benchmark() {
                const LINE: &str = "..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
..............................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\
...............................................E.\r\
";
                thread_local! {
                    static TEST_TERM_AND_POINT: (Term<VoidListener>, AlacPoint) =
                        build_test_term(&LINE, 5, 50);
                }
                TEST_TERM_AND_POINT.with(|(term, point)| {
                    assert_eq!(
                        find_from_grid_point_bench(term, *point)
                            .map(|(path, ..)| path)
                            .unwrap_or_default(),
                        LINE.trim_end_matches(['.', '\r', '\n']),
                        "Hyperlink should have been found"
                    );
                });
            }

            pub fn find_from_grid_point_bench(
                term: &Term<VoidListener>,
                point: AlacPoint,
            ) -> Option<(String, bool, Match)> {
                const PATH_HYPERLINK_TIMEOUT_MS: u64 = 1000;

                thread_local! {
                    static TEST_REGEX_SEARCHES: RefCell<RegexSearches> =
                        RefCell::new({
                            let default_settings_content: Rc<SettingsContent> =
                                settings::parse_json_with_comments(&settings::default_settings())
                                    .unwrap();
                            let default_terminal_settings =
                                TerminalSettings::from_settings(&default_settings_content);

                            RegexSearches::new(
                                &default_terminal_settings.path_hyperlink_regexes,
                                PATH_HYPERLINK_TIMEOUT_MS
                            )
                        });
                }

                TEST_REGEX_SEARCHES.with(|regex_searches| {
                    find_from_grid_point(
                        &term,
                        point,
                        &mut regex_searches.borrow_mut(),
                        PathStyle::local(),
                    )
                })
            }
        }
    }

    mod file_iri {
        // File IRIs have a ton of use cases. Absolute file URIs are supported on all platforms,
        // including Windows drive letters (e.g., file:///C:/path) and percent-encoded characters.
        // Some cases like relative file IRIs are not supported.
        // See https://en.wikipedia.org/wiki/File_URI_scheme

        /// [**`c₀, c₁, …, cₙ;`**]ₒₚₜ := use specified terminal widths of `c₀, c₁, …, cₙ` **columns**
        /// (defaults to `3, longest_line_cells / 2, longest_line_cells + 1;`)
        ///
        macro_rules! test_file_iri {
            ($file_iri:literal) => { { test_hyperlink!(concat!("‹«👉", $file_iri, "»›"); FileIri) } };
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
                // https://github.com/zed-industries/zed/issues/39189
                #[test]
                fn issue_39189() {
                    test_file_iri!("file:///C:/test/cool/index.rs");
                    test_file_iri!("file:///C:/test/cool/");
                }

                #[test]
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
            test_iri!("ipfs://例🏃🦀/cool.ipfs");
            test_iri!("ipns://例🏃🦀/cool.ipns");
            test_iri!("magnet://例🏃🦀/cool.git");
            test_iri!("mailto:someone@somewhere.here");
            test_iri!("gemini://somewhere.here");
            test_iri!("gopher://somewhere.here");
            test_iri!("http://例🏃🦀/cool/index.html");
            test_iri!("http://10.10.10.10:1111/cool.html");
            test_iri!("http://例🏃🦀/cool/index.html?amazing=1");
            test_iri!("http://例🏃🦀/cool/index.html#right%20here");
            test_iri!("http://例🏃🦀/cool/index.html?amazing=1#right%20here");
            test_iri!("https://例🏃🦀/cool/index.html");
            test_iri!("https://10.10.10.10:1111/cool.html");
            test_iri!("https://例🏃🦀/cool/index.html?amazing=1");
            test_iri!("https://例🏃🦀/cool/index.html#right%20here");
            test_iri!("https://例🏃🦀/cool/index.html?amazing=1#right%20here");
            test_iri!("news://例🏃🦀/cool.news");
            test_iri!("git://例/cool.git");
            test_iri!("ssh://user@somewhere.over.here:12345/例🏃🦀/cool.git");
            test_iri!("ftp://例🏃🦀/cool.ftp");
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
                point = point.sub(term, Boundary::Grid, 1);
            }

            if grid.index(point).flags.contains(Flags::WIDE_CHAR_SPACER) {
                point.column -= 1;
            }

            point
        }

        fn end_point_from_prev_input_point(
            term: &Term<VoidListener>,
            prev_input_point: AlacPoint,
        ) -> AlacPoint {
            if term
                .grid()
                .index(prev_input_point)
                .flags
                .contains(Flags::WIDE_CHAR)
            {
                prev_input_point.add(term, Boundary::Grid, 1)
            } else {
                prev_input_point
            }
        }

        fn process_input(term: &mut Term<VoidListener>, c: char) {
            match c {
                '\t' => term.put_tab(1),
                c @ _ => term.input(c),
            }
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
                                iri_or_path = term.bounds_to_string(
                                    start_point,
                                    end_point_from_prev_input_point(&term, prev_input_point),
                                );
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
                                hyperlink_match = start_point
                                    ..=end_point_from_prev_input_point(&term, prev_input_point);
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
                            process_input(&mut term, c);
                        } else {
                            process_input(&mut term, c);
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
            iri_or_path = path.to_string_lossy().into_owned();
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
                '\t' => 8, // it's really 0-8, use the max always
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
                if index > 0 && remainder == 0 {
                    first_header_row.push_str(&format!("{:>10}", (index / 10)));
                }
                second_header_row += &remainder.to_string();
                if index == self.expected_hyperlink.hovered_grid_point.column.0 {
                    marker_header_row.push('↓');
                } else {
                    marker_header_row.push(' ');
                }
            }

            let remainder = (self.term.columns() - 1) % 10;
            if remainder != 0 {
                first_header_row.push_str(&" ".repeat(remainder));
            }

            result += &format!("\n      [ {}]\n", first_header_row);
            result += &format!("      [{}]\n", second_header_row);
            result += &format!("       {}", marker_header_row);

            for cell in self
                .term
                .renderable_content()
                .display_iter
                .filter(|cell| !cell.flags.intersects(WIDE_CHAR_SPACERS))
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

                match cell.c {
                    '\t' => result.push(' '),
                    c @ _ => result.push(c),
                }
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
        const CARGO_DIR_REGEX: &str =
            r#"\s+(Compiling|Checking|Documenting) [^(]+\((?<link>(?<path>.+))\)"#;
        const RUST_DIAGNOSTIC_REGEX: &str = r#"\s+(-->|:::|at) (?<link>(?<path>.+?))(:$|$)"#;
        const ISSUE_12338_REGEX: &str =
            r#"[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2} (?<link>(?<path>.+))"#;
        const MULTIPLE_SAME_LINE_REGEX: &str =
            r#"(?<link>(?<path>🦀 multiple_same_line 🦀) 🚣(?<line>[0-9]+) 🏛(?<column>[0-9]+)):"#;
        const PATH_HYPERLINK_TIMEOUT_MS: u64 = 1000;

        thread_local! {
            static TEST_REGEX_SEARCHES: RefCell<RegexSearches> =
                RefCell::new({
                    let default_settings_content: Rc<SettingsContent> =
                        settings::parse_json_with_comments(&settings::default_settings()).unwrap();
                    let default_terminal_settings = TerminalSettings::from_settings(&default_settings_content);

                    RegexSearches::new([
                        RUST_DIAGNOSTIC_REGEX,
                        CARGO_DIR_REGEX,
                        ISSUE_12338_REGEX,
                        MULTIPLE_SAME_LINE_REGEX,
                    ]
                        .into_iter()
                        .chain(default_terminal_settings.path_hyperlink_regexes
                            .iter()
                            .map(AsRef::as_ref)),
                    PATH_HYPERLINK_TIMEOUT_MS)
                });
        }

        let term_size = TermSize::new(columns, total_cells / columns + 2);
        let (term, expected_hyperlink) =
            build_term_from_test_lines(hyperlink_kind, term_size, test_lines);
        let hyperlink_found = TEST_REGEX_SEARCHES.with(|regex_searches| {
            find_from_grid_point(
                &term,
                expected_hyperlink.hovered_grid_point,
                &mut regex_searches.borrow_mut(),
                PathStyle::local(),
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
            None => {
                if expected_hyperlink.hyperlink_match.start()
                    != expected_hyperlink.hyperlink_match.end()
                {
                    assert!(
                        false,
                        "No hyperlink found\n     at {source_location}:\n{}",
                        check_hyperlink_match.format_renderable_content()
                    )
                }
            }
        }
    }
}
