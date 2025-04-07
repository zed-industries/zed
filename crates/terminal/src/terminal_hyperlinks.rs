use crate::{MaybeNavigationTarget, PathLikeTarget};
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

    pub(super) fn find_from_grid_point<'a, T: EventListener>(
        &mut self,
        point: AlacPoint,
        term: &Term<T>,
    ) -> Option<(MaybeNavigationTarget, Match)> {
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
            let target = if is_url {
                // Treat "file://" URLs like file paths to ensure
                // that line numbers at the end of the path are
                // handled correctly
                if let Some(path) = maybe_url_or_path.strip_prefix("file://") {
                    MaybeNavigationTarget::PathLike(PathLikeTarget {
                        maybe_path: path.to_string(),
                        terminal_dir: None,
                    })
                } else {
                    MaybeNavigationTarget::Url(maybe_url_or_path.clone())
                }
            } else {
                MaybeNavigationTarget::PathLike(PathLikeTarget {
                    maybe_path: maybe_url_or_path.clone(),
                    terminal_dir: None,
                })
            };
            (target, word_match)
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
// TODO(davewa): I don't (yet?) understand why we do this? Shouldn't this only search the line of the cell that was
// clicked on? That would also perform better, especially for terminals with a lot of lines: tiny font vertical
// montior people I'm looking at you ;), which could easily be multiple 100s of lines, then we always add another
// 200 beyond that--why not always only search the one line?
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
    use std::{cmp, iter, ops::RangeInclusive, path::PathBuf};
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

    #[derive(Debug)]
    struct ExpectedHyperlink {
        hovered_grid_point: AlacPoint,
        hovered_grid_char: char,
        hyperlink_match: RangeInclusive<AlacPoint>,
        // It is only used in derive(Debug)
        #[allow(dead_code)]
        path_capture_group: RangeInclusive<AlacPoint>,
        path_with_position: PathWithPosition,
    }

    fn build_term_from_test_lines<'a>(
        term_size: TermSize,
        test_lines: &(impl IntoIterator<Item = &'a str> + Clone),
    ) -> (Term<VoidListener>, ExpectedHyperlink) {
        let mut term = Term::new(Config::default(), &term_size, VoidListener);

        enum MatchState {
            MatchScan,
            Match(AlacPoint),
            Done,
        }

        enum CapturesState {
            PathScan,
            Path(AlacPoint),
            RowScan,
            ColumnScan,
            Done,
        }

        let mut hovered_grid_point = AlacPoint::default();
        let mut hyperlink_match =
            AlacPoint::new(Line(0), Column(0))..=AlacPoint::new(Line(0), Column(0));
        let mut path_capture_group =
            AlacPoint::new(Line(0), Column(0))..=AlacPoint::new(Line(0), Column(0));
        let mut path_with_position = PathWithPosition::from_path(PathBuf::new());
        let mut match_state = MatchState::MatchScan;
        let mut captures_state = CapturesState::PathScan;
        let mut last_input_point = AlacPoint::new(Line(0), Column(0));
        for text in test_lines.clone().into_iter() {
            let text = text.chars().collect_vec();

            let parse_u32 = |index: &mut usize,
                             last_input_point: &mut AlacPoint,
                             term: Option<&mut Term<VoidListener>>|
             -> u32 {
                *index += 1;
                let mut number = String::new();
                for c in &text[*index..] {
                    if c.is_digit(10) {
                        number.push(*c);
                    } else {
                        break;
                    }
                }
                *index += number.len();
                if let Some(term) = term {
                    for c in number.chars() {
                        if !term.grid().cursor.input_needs_wrap {
                            *last_input_point = term.grid().cursor.point;
                        } else {
                            *last_input_point = AlacPoint::new(
                                Line(term.grid().cursor.point.line.0 + 1),
                                Column(0),
                            );
                        }
                        term.input(c)
                    }
                }
                number.parse::<u32>().unwrap()
            };

            let mut index = 0;
            while index < text.len() {
                match text[index] {
                    '«' | '»' => {
                        captures_state = match captures_state {
                            CapturesState::PathScan => {
                                let hovered_grid_offset =
                                    parse_u32(&mut index, &mut last_input_point, None) as usize;
                                hovered_grid_point = term.grid().cursor.point.add(
                                    &term,
                                    Boundary::Grid,
                                    hovered_grid_offset,
                                );
                                let next_input_point = if !term.grid().cursor.input_needs_wrap {
                                    term.grid().cursor.point
                                } else {
                                    AlacPoint::new(
                                        Line(term.grid().cursor.point.line.0 + 1),
                                        Column(0),
                                    )
                                };
                                CapturesState::Path(next_input_point)
                            }
                            CapturesState::Path(start_point) => {
                                path_capture_group = start_point..=last_input_point;
                                path_with_position = PathWithPosition::from_path(PathBuf::from(
                                    &term.bounds_to_string(
                                        path_capture_group.start().clone(),
                                        path_capture_group.end().clone(),
                                    ),
                                ));
                                index += 1;
                                CapturesState::RowScan
                            }
                            CapturesState::RowScan => {
                                path_with_position.row = Some(parse_u32(
                                    &mut index,
                                    &mut last_input_point,
                                    Some(&mut term),
                                ));
                                index += 1;
                                CapturesState::ColumnScan
                            }
                            CapturesState::ColumnScan => {
                                path_with_position.column = Some(parse_u32(
                                    &mut index,
                                    &mut last_input_point,
                                    Some(&mut term),
                                ));
                                index += 1;
                                CapturesState::Done
                            }
                            CapturesState::Done => {
                                panic!("Extra '«', '»'")
                            }
                        }
                    }
                    '‹' | '›' => {
                        match_state = match match_state {
                            MatchState::MatchScan => {
                                index += 1;
                                let next_input_point = if !term.grid().cursor.input_needs_wrap {
                                    term.grid().cursor.point
                                } else {
                                    AlacPoint::new(
                                        Line(term.grid().cursor.point.line.0 + 1),
                                        Column(0),
                                    )
                                };
                                MatchState::Match(next_input_point)
                            }
                            MatchState::Match(start_point) => {
                                hyperlink_match = start_point..=last_input_point;
                                index += 1;
                                MatchState::Done
                            }
                            MatchState::Done => {
                                panic!("Extra '‹', '›'")
                            }
                        }
                    }
                    _ => {
                        if !term.grid().cursor.input_needs_wrap {
                            last_input_point = term.grid().cursor.point;
                        } else {
                            last_input_point = AlacPoint::new(
                                Line(term.grid().cursor.point.line.0 + 1),
                                Column(0),
                            );
                        }
                        term.input(text[index]);
                        index += 1
                    }
                }
            }
            term.input('\n');
        }

        let hovered_grid_char = term.grid().index(hovered_grid_point).c;

        (
            term,
            ExpectedHyperlink {
                hovered_grid_point,
                hovered_grid_char,
                hyperlink_match,
                path_capture_group,
                path_with_position,
            },
        )
    }

    fn print_renderable_content(term: &Term<VoidListener>) {
        let header = format!(
            "      {}",
            String::from_iter(iter::repeat_n('=', term.columns()))
        );
        print!("{}", header);
        for cell in term.renderable_content().display_iter {
            if cell.point.column.0 == 0 {
                print!("\n[{:>3}] ", cell.point.line.to_string());
            }
            if cell
                .flags
                .intersects(Flags::LEADING_WIDE_CHAR_SPACER | Flags::WIDE_CHAR_SPACER)
            {
                continue;
            }
            print!("{}", cell.c);
        }
        println!("\n{}", header);
    }

    fn print_hyperlink_and_match(path_with_position: &PathWithPosition, path_match: &Match) {
        print!("Path «{}»", &path_with_position.path.to_string_lossy());
        if let Some(row) = path_with_position.row {
            print!(", line = {row}");
            if let Some(column) = path_with_position.column {
                print!(", column = {column}");
            }
        }

        println!(
            ", at grid cells ({}, {})..=({}, {})",
            path_match.start().line.0,
            path_match.start().column.0,
            path_match.end().line.0,
            path_match.end().column.0,
        )
    }

    /// **`‹›`** := **hyperlink** match
    ///
    /// **`«NNaaaaa»`** := **path** capture group
    ///
    ///   - Where `NN` is the **grid cell offset** from the start of the capture to the hovered cell.
    ///
    /// **`«NN»`** := **row** or **column** capture group
    macro_rules! test_hyperlink {
        ($($line:literal),+) => {
            test_hyperlink!(2 @ $($line),+)
        };
        ($mininum_columns:literal @ $($line:literal),+) => {{
            let test_lines = vec![$($line),+];
            let (total_chars, longest_line_chars) = test_lines
                .iter()
                .fold((0, 0), |state, line| {
                    let line_chars = line.chars().filter(|c| "‹«»›".find(*c).is_none()).count();
                    (state.0 + line_chars, cmp::max(state.1, line_chars))
                });
            for columns in $mininum_columns..longest_line_chars + 1 {
                println!("\n\nTesting with {columns} terminal columns:");
                let screen_lines = total_chars / columns + 2;
                let (mut term, expected_hyperlink) = build_term_from_test_lines(
                    TermSize::new(columns, screen_lines), &test_lines);
                print_renderable_content(&term);
                println!("Hovered at ({}, {}) ({})",
                    expected_hyperlink.hovered_grid_point.line.0,
                    expected_hyperlink.hovered_grid_point.column.0,
                    expected_hyperlink.hovered_grid_char
                );
                print!("Expected: ");
                print_hyperlink_and_match(&expected_hyperlink.path_with_position, &expected_hyperlink.hyperlink_match);
                let mut hyperlink_finder = HyperlinkFinder::new();
                if let Some((
                    MaybeNavigationTarget::PathLike(PathLikeTarget { maybe_path, .. }),
                    hyperlink_match,
                )) = hyperlink_finder
                    .find_from_grid_point(expected_hyperlink.hovered_grid_point, &mut term)
                {
                    print!("Actual  : ");
                    let path_with_position = PathWithPosition::parse_str(&maybe_path);
                    print_hyperlink_and_match(&path_with_position, &hyperlink_match);
                    assert_eq!(path_with_position, expected_hyperlink.path_with_position);
                    assert_eq!(hyperlink_match, expected_hyperlink.hyperlink_match);
                } else {
                    assert!(false, "No hyperlink found")
                }
            }
        }};
    }

    // TODO: More tests
    // - [ ] Resize the terminal down to a few columns, to test matches that span multiple lines
    // - [ ] MSBuild-style(line,column)
    // - [ ] Windows paths

    #[test]
    fn simple() {
        // Rust paths
        test_hyperlink!("‹«1/test/cool.rs»›");
        test_hyperlink!("‹«1/test/cool.rs»›");
        test_hyperlink!("‹«1/test/cool.rs»:«4»›");
        test_hyperlink!("‹«1/test/cool.rs»:«4»:«2»›");

        // TODO: Not yet supported, should be enabled by the fix for #12338
        // test_hyperlink!("‹«4/🏃/🦀.rs»›");
        // test_hyperlink!("‹«5/🏃/🦀.rs»›");
        // test_hyperlink!("‹«4/🏃/🦀.rs»:«4»›");
        // test_hyperlink!("‹«4/🏃/🦀.rs»:«4»:«2»›");

        // Cargo output
        test_hyperlink!("    Compiling Cool (‹«1/test/Cool»›)");
        test_hyperlink!("    Compiling Cool (‹«2/test/Cool»›)");

        // Python
        test_hyperlink!(3 @ "‹«3awesome.py»›");
        // TODO(davewa): Do we really want the hyperlink under `File `?
        test_hyperlink!("    ‹File \"«4/awesome.py»\", line «42»›: Wat?"); // Hovered on: 's'
    }

    #[test]
    // TODO(davewa): We use higher minimum columns in this test because basically any wide char at a line
    // wrap is buggy in alacritty. I feel like this really needs fixing, even if a lot of people haven't
    // reported it. It is most likely in the category of people experiencing failures, but somewhat
    // randomly and not really understanding what situation is causing it to work or not work, which
    // isn't a great experience, even though it might not have been reported as an actual issue with
    // a clear repro case.
    fn wide_chars() {
        // Rust paths
        test_hyperlink!(4 @ "‹«1/例/cool.rs»›");
        test_hyperlink!(4 @ "‹«1/例/cool.rs»›");
        test_hyperlink!(4 @ "‹«1/例/cool.rs»:«4»›");
        test_hyperlink!(4 @ "‹«1/例/cool.rs»:«4»:«2»›");

        // TODO: Not yet supported, should be enabled by the fix for #12338
        // test_hyperlink!("‹«4/🏃/🦀.rs»›");
        // test_hyperlink!("‹«5/🏃/🦀.rs»›");
        // test_hyperlink!("‹«4/🏃/🦀.rs»:«4»›");
        // test_hyperlink!("‹«4/🏃/🦀.rs»:«4»:«2»›");

        // Cargo output
        test_hyperlink!(25 @ "    Compiling Cool (‹«1/例/Cool»›)");
        test_hyperlink!(25 @ "    Compiling Cool (‹«2/例/Cool»›)");

        // Python
        test_hyperlink!(4 @ "‹«3例wesome.py»›");
        // TODO(davewa): Do we really want the hyperlink under `File `?
        test_hyperlink!(15 @ "    ‹File \"«4/例wesome.py»\", line «42»›: Wat?"); // Hovered on: 's'
    }

    #[test]
    // TOOD(davewa): Possible alacritty_terminal bugs with matching content at the end of the grid, even just
    // plain ascii?
    #[should_panic(expected = "assertion `left == right` failed")]
    fn issue_alacritty_bugs_with_few_columns() {
        // Python
        test_hyperlink!("‹«3awesome.py»›");
    }

    #[test]
    // TOOD(davewa): Possible alacritty_terminal bugs with wide chars and fewer than 4 columns?
    #[should_panic(expected = "assertion `left == right` failed")]
    fn issue_alacritty_bugs_with_wide_char_at_line_wrap() {
        // Rust paths
        test_hyperlink!("‹«1/test/cool.rs»›");
        test_hyperlink!("‹«1/例/cool.rs»›");
        test_hyperlink!("‹«1/例/cool.rs»:«4»›");
        test_hyperlink!("‹«1/例/cool.rs»:«4»:«2»›");

        // Cargo output
        test_hyperlink!("    Compiling Cool (‹«1/例/Cool»›)");
        test_hyperlink!("    Compiling Cool (‹«2/例/Cool»›)");

        // Python
        test_hyperlink!("‹«3例wesome.py»›");
        // TODO(davewa): Do we really want the hyperlink under `File `?
        test_hyperlink!("    ‹File \"«4/例wesome.py»\", line «42»›: Wat?"); // Hovered on: 's'
    }

    #[test]
    // TODO(davewa): See comment on `wide_chars` test above.
    fn mojo() {
        // Mojo diagnostic message
        // TODO(davewa): Do we really want the hyperlink under `File `?
        // TODO(davewa): I haven't ever run Mojo, this is assuming it uses the same format as Python.
        test_hyperlink!(25 @ "    ‹File \"«4/awesome.🔥»\", line «42»›: Wat?"); // Hovered on: 's'
        test_hyperlink!(25 @ "    ‹File \"«8/awesome.🔥»\", line «42»›: Wat?"); // Hovered on: '.'
        test_hyperlink!(25 @ "    ‹File \"«9/awesome.🔥»\", line «42»›: Wat?"); // Hovered on: '🔥' (WIDE_CHAR cell)
        test_hyperlink!(25 @ "    ‹File \"«10/awesome.🔥»\", line «42»›: Wat?"); // Hovered on: '🔥' (WIDE_CHAR_SPACER cell)
    }

    #[test]
    // TODO: Fix <https://github.com/zed-industries/zed/issues/12338>
    #[should_panic(expected = "No hyperlink found")]
    fn issue_12338() {
        test_hyperlink!(".rw-r--r--     0     staff 05-27 14:03 ‹«4test、2.txt»›");
        test_hyperlink!(".rw-r--r--     0     staff 05-27 14:03 ‹«5test、2.txt»›");
        test_hyperlink!(".rw-r--r--     0     staff 05-27 14:03 ‹«4test。3.txt»›");
        test_hyperlink!(".rw-r--r--     0     staff 05-27 14:03 ‹«5test。3.txt»›");
    }

    #[test]
    // TODO: Mojo should also be fixed by the fix for <https://github.com/zed-industries/zed/issues/12338>
    #[should_panic(expected = "assertion `left == right` failed")]
    fn issue_broken_mojo() {
        // Match ends on a wide char
        test_hyperlink!("‹«4/awesome.🔥»› is some good Mojo!"); // Hovered on: 's'
        test_hyperlink!("‹«8/awesome.🔥»› is some good Mojo!"); // Hovered on: '.'
        test_hyperlink!("‹«9/awesome.🔥»› is some good Mojo!"); // Hovered on: '🔥' (WIDE_CHAR cell)
        test_hyperlink!("‹«10/awesome.🔥»› is some good Mojo!"); // Hovered on: '🔥' (WIDE_CHAR_SPACER cell)
    }
}
