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
use fancy_regex::Regex;
use log::{info, warn};
use std::{
    ops::{Index, Range},
    time::{Duration, Instant},
};

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

    // First, handle parentheses balancing using single traversal
    let (open_parens, close_parens) =
        sanitized_url
            .chars()
            .fold((0, 0), |(opens, closes), c| match c {
                '(' => (opens + 1, closes),
                ')' => (opens, closes + 1),
                _ => (opens, closes),
            });

    // Trim unbalanced closing parentheses
    if close_parens > open_parens {
        let mut remaining_close = close_parens;
        while sanitized_url.ends_with(')') && remaining_close > open_parens {
            sanitized_url.pop();
            chars_trimmed += 1;
            remaining_close -= 1;
        }
    }

    // Handle trailing periods
    if sanitized_url.ends_with('.') {
        let trailing_periods = sanitized_url
            .chars()
            .rev()
            .take_while(|&c| c == '.')
            .count();

        if trailing_periods > 1 {
            sanitized_url.truncate(sanitized_url.len() - trailing_periods);
            chars_trimmed += trailing_periods;
        } else if trailing_periods == 1
            && let Some(second_last_char) = sanitized_url.chars().rev().nth(1)
            && (second_last_char.is_alphanumeric() || second_last_char == '/')
        {
            sanitized_url.pop();
            chars_trimmed += 1;
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
    line.push(first_cell.c);
    let mut start_offset = 0;
    let mut hovered_point_byte_offset = None;

    if !first_cell.flags.intersects(WIDE_CHAR_SPACERS) {
        start_offset += first_cell.c.len_utf8();
        if line_start == hovered {
            hovered_point_byte_offset = Some(0);
        }
    }

    for cell in term.grid().iter_from(line_start) {
        if cell.point > line_end {
            break;
        }
        let is_spacer = cell.flags.intersects(WIDE_CHAR_SPACERS);
        if cell.point == hovered {
            debug_assert!(hovered_point_byte_offset.is_none());
            if start_offset > 0 && cell.flags.contains(Flags::WIDE_CHAR_SPACER) {
                // If we hovered on a trailing spacer, back up to the end of the previous char's bytes.
                start_offset -= 1;
            }
            hovered_point_byte_offset = Some(start_offset);
        } else if cell.point < hovered && !is_spacer {
            start_offset += cell.c.len_utf8();
        }

        if !is_spacer {
            line.push(match cell.c {
                '\t' => ' ',
                c @ _ => c,
            });
        }
    }
    let line = line.trim_ascii_end();
    let hovered_point_byte_offset = hovered_point_byte_offset?;
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

        for captures in regex.captures_iter(&line) {
            let captures = match captures {
                Ok(captures) => captures,
                Err(error) => {
                    warn!("Error '{error}' searching for path hyperlinks in line: {line}");
                    info!(
                        "Skipping match from path hyperlinks with regex: {}",
                        regex.as_str()
                    );
                    continue;
                }
            };
            path_found = true;
            let match_range = captures.get(0).unwrap().range();
            let (path_range, line_column) = if let Some(path) = captures.name("path") {
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
            let link_range = captures
                .name("link")
                .map_or_else(|| match_range.clone(), |link| link.range());

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
    use fancy_regex::Regex;
    use settings::{self, Settings, SettingsContent};
    use std::{cell::RefCell, ops::RangeInclusive, path::PathBuf, rc::Rc};
    use url::Url;
    use util::paths::PathWithPosition;

    fn re_test(re: &str, hay: &str, expected: Vec<&str>) {
        let results: Vec<_> = Regex::new(re)
            .unwrap()
            .find_iter(hay)
            .map(|m| m.unwrap().as_str())
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
    fn test_url_periods_sanitization() {
        // Test URLs with trailing periods (sentence punctuation)
        let test_cases = vec![
            // Cases that should be sanitized (trailing periods likely punctuation)
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
            // Cases that should NOT be sanitized (periods are part of URL structure)
            (
                "https://example.com/v1.0/api",
                "https://example.com/v1.0/api",
            ),
            ("https://192.168.1.1", "https://192.168.1.1"),
            ("https://sub.domain.com", "https://sub.domain.com"),
        ];

        for (input, expected) in test_cases {
            // Create a minimal terminal for testing
            let term = Term::new(Config::default(), &TermSize::new(80, 24), VoidListener);

            // Create a dummy match that spans the entire input
            let start_point = AlacPoint::new(Line(0), Column(0));
            let end_point = AlacPoint::new(Line(0), Column(input.len()));
            let dummy_match = Match::new(start_point, end_point);

            // This test should initially fail since we haven't implemented period sanitization yet
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
            test_path!("/test/cool.rs:4:2👉:Error!");
            test_path!("/test/cool.rs:4:2:👉Error!");
            test_path!("‹«/test/co👉ol.rs»:«4»:«2»›:Error!");
            test_path!("‹«/test/co👉ol.rs»(«4»,«2»)›:Error!");

            // Cargo output
            test_path!("    Compiling Cool 👉(/test/Cool)");
            test_path!("    Compiling Cool (‹«/👉test/Cool»›)");
            test_path!("    Compiling Cool (/test/Cool👉)");

            // Python
            test_path!("‹«awe👉some.py»›");

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
            test_path!("/test/cool.rs:4:2:例Desc例👉例例");
            test_path!("‹«/👉test/cool.rs»(«4»,«2»)›:例Desc例例例");
            test_path!("‹«/test/cool.rs»(«4»👉,«2»)›:例Desc例例例");
            test_path!("/test/cool.rs(4,2):例Desc例👉例例");

            // path, line, column and description w/extra colons
            test_path!("‹«/👉test/cool.rs»:«4»:«2»›::例Desc例例例");
            test_path!("‹«/test/cool.rs»:«4»:«👉2»›::例Desc例例例");
            test_path!("/test/cool.rs:4:2::例Desc例👉例例");
            test_path!("‹«/👉test/cool.rs»(«4»,«2»)›::例Desc例例例");
            test_path!("‹«/test/cool.rs»(«4»,«2»👉)›::例Desc例例例");
            test_path!("/test/cool.rs(4,2)::例Desc例👉例例");
        }

        #[test]
        fn multiple_same_line() {
            test_path!("‹«/👉test/cool.rs»› /test/cool.rs");
            test_path!("/test/cool.rs ‹«/👉test/cool.rs»›");

            test_path!(
                "‹«🦀 multiple_👉same_line 🦀» 🚣«4» 🏛️«2»›: 🦀 multiple_same_line 🦀 🚣4 🏛️2:"
            );
            test_path!(
                "🦀 multiple_same_line 🦀 🚣4 🏛️2 ‹«🦀 multiple_👉same_line 🦀» 🚣«4» 🏛️«2»›:"
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
            test_path!("‹«/test/co👉ol.rs:4:2»(«1»,«618»)›");
            test_path!("‹«/test/co👉ol.rs:4:2»(«1»,«618»)›:");
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
            test_path!("'‹«(/test/co👉ol.rs:4)»›'");

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
            test_path!("'‹«(/test/co👉ol.rs:4),,»›'..");
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
                test_path!(
                    "test/controllers/template_items_controller_test.rb:19:i👉n 'block in <class:TemplateItemsControllerTest>'"
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
                grid::Dimensions,
                index::{Column, Point as AlacPoint},
                term::test::mock_term,
                term::{Term, search::Match},
            };
            use settings::{self, Settings, SettingsContent};
            use std::{cell::RefCell, rc::Rc};
            use util_macros::perf;

            fn build_test_term(line: &str) -> (Term<VoidListener>, AlacPoint) {
                let content = line.repeat(500);
                let term = mock_term(&content);
                let point = AlacPoint::new(
                    term.grid().bottommost_line() - 1,
                    Column(term.grid().last_column().0 / 2),
                );

                (term, point)
            }

            #[perf]
            pub fn cargo_hyperlink_benchmark() {
                const LINE: &str = "    Compiling terminal v0.1.0 (/Hyperlinks/Bench/Source/zed-hyperlinks/crates/terminal)\r\n";
                thread_local! {
                    static TEST_TERM_AND_POINT: (Term<VoidListener>, AlacPoint) =
                        build_test_term(LINE);
                }
                TEST_TERM_AND_POINT.with(|(term, point)| {
                    assert!(
                        find_from_grid_point_bench(term, *point).is_some(),
                        "Hyperlink should have been found"
                    );
                });
            }

            #[perf]
            pub fn rust_hyperlink_benchmark() {
                const LINE: &str = "    --> /Hyperlinks/Bench/Source/zed-hyperlinks/crates/terminal/terminal.rs:1000:42\r\n";
                thread_local! {
                    static TEST_TERM_AND_POINT: (Term<VoidListener>, AlacPoint) =
                        build_test_term(LINE);
                }
                TEST_TERM_AND_POINT.with(|(term, point)| {
                    assert!(
                        find_from_grid_point_bench(term, *point).is_some(),
                        "Hyperlink should have been found"
                    );
                });
            }

            #[perf]
            pub fn ls_hyperlink_benchmark() {
                const LINE: &str = "Cargo.toml        experiments        notebooks        rust-toolchain.toml    tooling\r\n";
                thread_local! {
                    static TEST_TERM_AND_POINT: (Term<VoidListener>, AlacPoint) =
                        build_test_term(LINE);
                }
                TEST_TERM_AND_POINT.with(|(term, point)| {
                    assert!(
                        find_from_grid_point_bench(term, *point).is_some(),
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
                    find_from_grid_point(&term, point, &mut regex_searches.borrow_mut())
                })
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
                // https://github.com/zed-industries/zed/issues/39189
                #[test]
                #[should_panic(
                    expected = r#"Path = «C:\\test\\cool\\index.rs», at grid cells (0, 0)..=(9, 1)"#
                )]
                fn issue_39189() {
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
