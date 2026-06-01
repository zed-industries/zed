use log::{info, warn};
use regex::Regex;
use std::{
    ops::Range as StdRange,
    time::{Duration, Instant},
};
use url::Url;
use util::paths::{PathStyle, UrlExt};

use crate::{Content, IndexedCell, Point, Range};

const URL_REGEX: &str = r#"(ipfs:|ipns:|magnet:|mailto:|gemini://|gopher://|https://|http://|news:|file://|git://|ssh:|ftp://)[^\u{0000}-\u{001F}\u{007F}-\u{009F}<>"\s{-}\^⟨⟩`']+"#;

pub(super) struct RegexSearches {
    content_url_regex: Option<Regex>,
    path_hyperlink_regexes: Vec<Regex>,
    path_hyperlink_timeout: Duration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct HyperlinkMatch {
    pub(super) text: String,
    pub(super) is_url: bool,
    pub(super) range: Range,
}

impl Default for RegexSearches {
    fn default() -> Self {
        Self::new(Vec::<String>::new(), 0)
    }
}

impl RegexSearches {
    pub(super) fn new(
        path_hyperlink_regexes: impl IntoIterator<Item: AsRef<str>>,
        path_hyperlink_timeout_ms: u64,
    ) -> Self {
        Self {
            content_url_regex: Regex::new(URL_REGEX).ok(),
            path_hyperlink_regexes: Self::path_hyperlink_regexes(path_hyperlink_regexes),
            path_hyperlink_timeout: Duration::from_millis(path_hyperlink_timeout_ms),
        }
    }

    fn path_hyperlink_regexes(
        path_hyperlink_regexes: impl IntoIterator<Item: AsRef<str>>,
    ) -> Vec<Regex> {
        path_hyperlink_regexes
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
            .collect()
    }
}

pub(super) fn find_from_content_point(
    content: &Content,
    point: Point,
    regex_searches: &mut RegexSearches,
    path_style: PathStyle,
) -> Option<HyperlinkMatch> {
    if let Some((text, is_url, range)) = content_cell_hyperlink(content, point) {
        return Some(normalize_hyperlink_match(text, is_url, range, path_style));
    }

    let (line, points, hovered_point_byte_offset) = content_line_text(content, point)?;

    let found_word = regex_searches
        .content_url_regex
        .as_ref()
        .and_then(|content_url_regex| {
            content_url_regex
                .find_iter(&line)
                .find(|url_match| url_match.range().contains(&hovered_point_byte_offset))
                .and_then(|url_match| {
                    sanitize_content_url_punctuation(
                        url_match.as_str().to_string(),
                        url_match.range(),
                        &points,
                    )
                })
                .map(|(url, url_match)| (url, true, url_match))
        })
        .or_else(|| {
            content_path_match(
                &line,
                &points,
                hovered_point_byte_offset,
                point,
                &mut regex_searches.path_hyperlink_regexes,
                regex_searches.path_hyperlink_timeout,
            )
            .map(|(path, path_match)| (path, false, path_match))
        });

    found_word
        .map(|(text, is_url, range)| normalize_hyperlink_match(text, is_url, range, path_style))
}

fn content_cell_hyperlink(content: &Content, point: Point) -> Option<(String, bool, Range)> {
    let index = content.cells.iter().position(|cell| cell.point == point)?;
    let link = content.cells[index].hyperlink()?;
    let mut start_index = index;
    while start_index > 0 && content.cells[start_index - 1].hyperlink() == Some(link) {
        start_index -= 1;
    }

    let mut end_index = index;
    while content
        .cells
        .get(end_index + 1)
        .and_then(|cell| cell.hyperlink())
        == Some(link)
    {
        end_index += 1;
    }

    Some((
        link.uri().to_string(),
        true,
        Range::new(
            content.cells[start_index].point,
            content.cells[end_index].point,
        ),
    ))
}

fn normalize_hyperlink_match(
    maybe_url_or_path: String,
    is_url: bool,
    range: Range,
    path_style: PathStyle,
) -> HyperlinkMatch {
    if is_url {
        if maybe_url_or_path.starts_with("file://") {
            if let Ok(url) = Url::parse(&maybe_url_or_path) {
                if let Ok(path) = url.to_file_path_ext(path_style) {
                    return HyperlinkMatch {
                        text: path.to_string_lossy().into_owned(),
                        is_url: false,
                        range,
                    };
                } else if let Some(path) = try_osc8_url_to_path(url)
                    && path_style.is_posix()
                {
                    return HyperlinkMatch {
                        text: path,
                        is_url: false,
                        range,
                    };
                }
            }

            let path = maybe_url_or_path
                .strip_prefix("file://")
                .unwrap_or(&maybe_url_or_path);
            HyperlinkMatch {
                text: path.to_string(),
                is_url: false,
                range,
            }
        } else {
            HyperlinkMatch {
                text: maybe_url_or_path,
                is_url: true,
                range,
            }
        }
    } else {
        HyperlinkMatch {
            text: maybe_url_or_path,
            is_url: false,
            range,
        }
    }
}

fn content_line_text(
    content: &Content,
    point: Point,
) -> Option<(String, Vec<(usize, Point)>, usize)> {
    let mut line = String::new();
    let mut points = Vec::new();
    let mut hovered_point_byte_offset = None;
    let (start_line, end_line) = content_logical_line_bounds(content, point.line)?;

    for line_number in start_line..=end_line {
        for cell in content_cells_for_line(content, line_number) {
            if cell.is_wide_char_spacer_or_leading() {
                if cell.point == point {
                    hovered_point_byte_offset = points.last().map(|(byte_offset, _)| *byte_offset);
                }
                continue;
            }

            let byte_offset = line.len();
            if cell.point == point {
                hovered_point_byte_offset = Some(byte_offset);
            }
            points.push((byte_offset, cell.point));
            match cell.character() {
                ' ' | '\t' => line.push(' '),
                character => line.push(character),
            }

            if let Some(characters) = cell.zerowidth() {
                for character in characters {
                    points.push((line.len(), cell.point));
                    line.push(*character);
                }
            }
        }
    }

    let trimmed_len = line.trim_ascii_end().len();
    line.truncate(trimmed_len);
    let hovered_point_byte_offset = hovered_point_byte_offset?;
    (line.len() > hovered_point_byte_offset).then_some((line, points, hovered_point_byte_offset))
}

fn content_logical_line_bounds(content: &Content, line: i32) -> Option<(i32, i32)> {
    let mut start_line = line;
    let mut end_line = line;
    let top_line = content.cells.first()?.point.line;
    let bottom_line = content.cells.last()?.point.line;

    while start_line > top_line && content_line_is_soft_wrapped(content, start_line - 1) {
        start_line -= 1;
    }
    while end_line < bottom_line && content_line_is_soft_wrapped(content, end_line) {
        end_line += 1;
    }

    Some((start_line, end_line))
}

fn content_line_is_soft_wrapped(content: &Content, line: i32) -> bool {
    content.soft_wrapped_lines.binary_search(&line).is_ok()
}

fn content_cells_for_line(content: &Content, line: i32) -> &[IndexedCell] {
    let start = content.cells.partition_point(|cell| cell.point.line < line);
    let end = start + content.cells[start..].partition_point(|cell| cell.point.line == line);
    &content.cells[start..end]
}

fn sanitize_content_url_punctuation(
    url: String,
    url_match_range: StdRange<usize>,
    points: &[(usize, Point)],
) -> Option<(String, Range)> {
    let mut sanitized_url = url;
    let mut bytes_trimmed = 0;

    let (open_parens, mut close_parens) =
        sanitized_url
            .chars()
            .fold((0, 0), |(opens, closes), character| match character {
                '(' => (opens + 1, closes),
                ')' => (opens, closes + 1),
                _ => (opens, closes),
            });

    while let Some(last_char) = sanitized_url.chars().last() {
        let should_remove = match last_char {
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
            bytes_trimmed += last_char.len_utf8();
        } else {
            break;
        }
    }

    let sanitized_range = url_match_range.start..url_match_range.end.checked_sub(bytes_trimmed)?;
    let sanitized_match = content_match_from_byte_range(points, sanitized_range)?;
    Some((sanitized_url, sanitized_match))
}

fn content_path_match(
    line: &str,
    points: &[(usize, Point)],
    hovered_point_byte_offset: usize,
    hovered: Point,
    path_hyperlink_regexes: &mut Vec<Regex>,
    path_hyperlink_timeout: Duration,
) -> Option<(String, Range)> {
    if path_hyperlink_regexes.is_empty() || path_hyperlink_timeout.as_millis() == 0 {
        return None;
    }
    let search_start_time = Instant::now();

    let timed_out = || {
        let elapsed_time = Instant::now().saturating_duration_since(search_start_time);
        (elapsed_time > path_hyperlink_timeout)
            .then_some((elapsed_time.as_millis(), path_hyperlink_timeout.as_millis()))
    };

    for regex in path_hyperlink_regexes {
        let mut path_found = false;

        for captures in regex.captures_iter(line) {
            path_found = true;
            let Some(full_match) = captures.get(0) else {
                continue;
            };
            let match_range = full_match.range();
            let (mut path_range, line_column) = if let Some(path) = captures.name("path") {
                let parse = |name: &str| -> Option<u32> {
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

            if let Some(trim) = first_unbalanced_open_paren(&line[path_range.clone()]) {
                path_range.start += trim;
                link_range.start = link_range.start.max(path_range.start);
            }

            if !link_range.contains(&hovered_point_byte_offset) {
                continue;
            }

            let link_match = content_match_from_byte_range(points, link_range.clone())?;
            if !link_match.contains(hovered) {
                continue;
            }

            let mut path = line[path_range].to_string();
            if let Some((line, column)) = line_column {
                path += &format!(":{line}");
                if let Some(column) = column {
                    path += &format!(":{column}");
                }
            }

            return Some((path, link_match));
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

fn content_match_from_byte_range(
    points: &[(usize, Point)],
    range: StdRange<usize>,
) -> Option<Range> {
    if range.is_empty() {
        return None;
    }

    let start_index = points.partition_point(|(byte_offset, _)| *byte_offset < range.start);
    let end_index = points
        .partition_point(|(byte_offset, _)| *byte_offset < range.end)
        .checked_sub(1)?;
    let start = points.get(start_index)?.1;
    let end = points.get(end_index)?.1;
    Some(Range::new(start, end))
}

fn try_osc8_url_to_path(url: url::Url) -> Option<String> {
    use percent_encoding::percent_decode;
    if url.scheme() != "file" {
        return None;
    }

    let bytes = url
        .path_segments()?
        .skip(1)
        .flat_map(|segment| percent_decode(segment.as_bytes()))
        .collect::<Vec<u8>>();
    bytes.try_into().ok()
}

fn first_unbalanced_open_paren(s: &str) -> Option<usize> {
    let mut balance: i32 = 0;
    let mut first_unmatched = None;
    for (index, character) in s.char_indices() {
        match character {
            '(' => {
                if balance == 0 {
                    first_unmatched = Some(index + character.len_utf8());
                }
                balance += 1;
            }
            ')' => {
                balance -= 1;
                if balance <= 0 {
                    balance = 0;
                    first_unmatched = None;
                }
            }
            _ => {}
        }
    }
    first_unmatched.filter(|_| balance > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cell, CellFlags, Color, Content, NamedColor};

    fn content_from_lines(lines: &[&str], soft_wrapped_lines: Vec<i32>) -> Content {
        let mut cells = Vec::new();
        for (line, text) in lines.iter().enumerate() {
            for (column, character) in text.chars().enumerate() {
                let mut cell = Cell::default();
                cell.set_character(character);
                cells.push(IndexedCell {
                    point: Point::new(line as i32, column),
                    cell,
                });
            }
        }

        Content {
            cells,
            soft_wrapped_lines,
            ..Default::default()
        }
    }

    fn test_cell(character: char, flags: CellFlags) -> Cell {
        Cell::new(
            character,
            Color::Named(NamedColor::Foreground),
            Color::Named(NamedColor::Background),
            flags,
        )
    }

    #[test]
    fn finds_plain_url_across_soft_wrap() {
        let content = content_from_lines(&["https://zed.", "dev/docs"], vec![0]);
        let mut regex_searches = RegexSearches::default();

        let found = find_from_content_point(
            &content,
            Point::new(1, 1),
            &mut regex_searches,
            PathStyle::local(),
        )
        .expect("expected wrapped URL match");

        assert_eq!(found.text, "https://zed.dev/docs");
        assert!(found.is_url);
        assert_eq!(found.range, Range::new(Point::new(0, 0), Point::new(1, 7)));
    }

    #[test]
    fn finds_path_regex_match_across_soft_wrap() {
        let content = content_from_lines(&["/tmp/wrapped/", "path.rs:42"], vec![0]);
        let mut regex_searches = RegexSearches::new(
            [r"(?P<link>(?P<path>/tmp/wrapped/path\.rs):(?P<line>\d+))"],
            1000,
        );

        let found = find_from_content_point(
            &content,
            Point::new(1, 2),
            &mut regex_searches,
            PathStyle::local(),
        )
        .expect("expected wrapped path match");

        assert_eq!(found.text, "/tmp/wrapped/path.rs:42");
        assert!(!found.is_url);
        assert_eq!(found.range, Range::new(Point::new(0, 0), Point::new(1, 9)));
    }

    #[test]
    fn finds_plain_url_when_hovering_wide_spacer() {
        let mut cells = Vec::new();
        for (column, character) in "https://".chars().enumerate() {
            cells.push(IndexedCell {
                point: Point::new(0, column),
                cell: test_cell(character, CellFlags::empty()),
            });
        }
        cells.push(IndexedCell {
            point: Point::new(0, 8),
            cell: test_cell('例', CellFlags::WIDE_CHAR),
        });
        cells.push(IndexedCell {
            point: Point::new(0, 9),
            cell: test_cell(' ', CellFlags::WIDE_CHAR_SPACER),
        });
        for (offset, character) in ".com".chars().enumerate() {
            cells.push(IndexedCell {
                point: Point::new(0, 10 + offset),
                cell: test_cell(character, CellFlags::empty()),
            });
        }

        let content = Content {
            cells,
            ..Default::default()
        };
        let mut regex_searches = RegexSearches::default();

        let found = find_from_content_point(
            &content,
            Point::new(0, 9),
            &mut regex_searches,
            PathStyle::local(),
        )
        .expect("expected URL match from wide spacer");

        assert_eq!(found.text, "https://例.com");
        assert_eq!(found.range, Range::new(Point::new(0, 0), Point::new(0, 13)));
    }
}
