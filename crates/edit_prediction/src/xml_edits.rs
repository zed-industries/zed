use anyhow::{Context as _, Result};
use language::{Anchor, BufferSnapshot, OffsetRangeExt as _, Point};
use std::{cmp, ops::Range, path::Path, sync::Arc};

const EDITS_TAG_NAME: &'static str = "edits";
const OLD_TEXT_TAG_NAME: &'static str = "old_text";
const NEW_TEXT_TAG_NAME: &'static str = "new_text";
const XML_TAGS: &[&str] = &[EDITS_TAG_NAME, OLD_TEXT_TAG_NAME, NEW_TEXT_TAG_NAME];

pub async fn parse_xml_edits<'a>(
    input: &'a str,
    get_buffer: impl Fn(&Path) -> Option<(&'a BufferSnapshot, &'a [Range<Anchor>])> + Send,
) -> Result<(&'a BufferSnapshot, Vec<(Range<Anchor>, Arc<str>)>)> {
    parse_xml_edits_inner(input, get_buffer)
        .await
        .with_context(|| format!("Failed to parse XML edits:\n{input}"))
}

async fn parse_xml_edits_inner<'a>(
    input: &'a str,
    get_buffer: impl Fn(&Path) -> Option<(&'a BufferSnapshot, &'a [Range<Anchor>])> + Send,
) -> Result<(&'a BufferSnapshot, Vec<(Range<Anchor>, Arc<str>)>)> {
    let xml_edits = extract_xml_replacements(input)?;

    let (buffer, context_ranges) = get_buffer(xml_edits.file_path.as_ref())
        .with_context(|| format!("no buffer for file {}", xml_edits.file_path))?;

    let mut all_edits = vec![];
    for (old_text, new_text) in xml_edits.replacements {
        let match_range = fuzzy_match_in_ranges(old_text, buffer, context_ranges)?;
        let matched_old_text = buffer
            .text_for_range(match_range.clone())
            .collect::<String>();
        let edits_within_hunk = language::text_diff(&matched_old_text, new_text);
        all_edits.extend(
            edits_within_hunk
                .into_iter()
                .map(move |(inner_range, inner_text)| {
                    (
                        buffer.anchor_after(match_range.start + inner_range.start)
                            ..buffer.anchor_before(match_range.start + inner_range.end),
                        inner_text,
                    )
                }),
        );
    }

    Ok((buffer, all_edits))
}

fn fuzzy_match_in_ranges(
    old_text: &str,
    buffer: &BufferSnapshot,
    context_ranges: &[Range<Anchor>],
) -> Result<Range<usize>> {
    let mut state = FuzzyMatcher::new(buffer, old_text);
    let mut best_match = None;
    let mut tie_match_range = None;

    for range in context_ranges {
        let best_match_cost = best_match.as_ref().map(|(score, _)| *score);
        match (best_match_cost, state.match_range(range.to_offset(buffer))) {
            (Some(lowest_cost), Some((new_cost, new_range))) => {
                if new_cost == lowest_cost {
                    tie_match_range = Some(new_range);
                } else if new_cost < lowest_cost {
                    tie_match_range.take();
                    best_match = Some((new_cost, new_range));
                }
            }
            (None, Some(new_match)) => {
                best_match = Some(new_match);
            }
            (None, None) | (Some(_), None) => {}
        };
    }

    if let Some((_, best_match_range)) = best_match {
        if let Some(tie_match_range) = tie_match_range {
            anyhow::bail!(
                "Multiple ambiguous matches:\n{:?}:\n{}\n\n{:?}:\n{}",
                best_match_range.clone(),
                buffer.text_for_range(best_match_range).collect::<String>(),
                tie_match_range.clone(),
                buffer.text_for_range(tie_match_range).collect::<String>()
            );
        }
        return Ok(best_match_range);
    }

    anyhow::bail!(
        "Failed to fuzzy match `old_text`:\n{}\nin:\n```\n{}\n```",
        old_text,
        context_ranges
            .iter()
            .map(|range| buffer.text_for_range(range.clone()).collect::<String>())
            .collect::<Vec<String>>()
            .join("```\n```")
    );
}

#[derive(Debug)]
struct XmlEdits<'a> {
    file_path: &'a str,
    /// Vec of (old_text, new_text) pairs
    replacements: Vec<(&'a str, &'a str)>,
}

fn extract_xml_replacements(input: &str) -> Result<XmlEdits<'_>> {
    let mut cursor = 0;

    let (edits_body_start, edits_attrs) =
        find_tag_open(input, &mut cursor, EDITS_TAG_NAME)?.context("No edits tag found")?;

    let file_path = edits_attrs
        .trim_start()
        .strip_prefix("path")
        .context("no path attribute on edits tag")?
        .trim_end()
        .strip_prefix('=')
        .context("no value for path attribute")?
        .trim()
        .trim_start_matches('"')
        .trim_end_matches('"');

    cursor = edits_body_start;
    let mut edits_list = Vec::new();

    while let Some((old_body_start, _)) = find_tag_open(input, &mut cursor, OLD_TEXT_TAG_NAME)? {
        let old_body_end = find_tag_close(input, &mut cursor)?;
        let old_text = trim_surrounding_newlines(&input[old_body_start..old_body_end]);

        let (new_body_start, _) = find_tag_open(input, &mut cursor, NEW_TEXT_TAG_NAME)?
            .context("no new_text tag following old_text")?;
        let new_body_end = find_tag_close(input, &mut cursor)?;
        let new_text = trim_surrounding_newlines(&input[new_body_start..new_body_end]);

        edits_list.push((old_text, new_text));
    }

    Ok(XmlEdits {
        file_path,
        replacements: edits_list,
    })
}

/// Trims a single leading and trailing newline
fn trim_surrounding_newlines(input: &str) -> &str {
    let start = input.strip_prefix('\n').unwrap_or(input);
    let end = start.strip_suffix('\n').unwrap_or(start);
    end
}

fn find_tag_open<'a>(
    input: &'a str,
    cursor: &mut usize,
    expected_tag: &str,
) -> Result<Option<(usize, &'a str)>> {
    let mut search_pos = *cursor;

    while search_pos < input.len() {
        let Some(tag_start) = input[search_pos..].find("<") else {
            break;
        };
        let tag_start = search_pos + tag_start;
        if !input[tag_start + 1..].starts_with(expected_tag) {
            search_pos = search_pos + tag_start + 1;
            continue;
        };

        let after_tag_name = tag_start + expected_tag.len() + 1;
        let close_bracket = input[after_tag_name..]
            .find('>')
            .with_context(|| format!("missing > after <{}", expected_tag))?;
        let attrs_end = after_tag_name + close_bracket;
        let body_start = attrs_end + 1;

        let attributes = input[after_tag_name..attrs_end].trim();
        *cursor = body_start;

        return Ok(Some((body_start, attributes)));
    }

    Ok(None)
}

fn find_tag_close(input: &str, cursor: &mut usize) -> Result<usize> {
    let mut depth = 1;
    let mut search_pos = *cursor;

    while search_pos < input.len() && depth > 0 {
        let Some(bracket_offset) = input[search_pos..].find('<') else {
            break;
        };
        let bracket_pos = search_pos + bracket_offset;

        if input[bracket_pos..].starts_with("</")
            && let Some(close_end) = input[bracket_pos + 2..].find('>')
        {
            let close_start = bracket_pos + 2;
            let tag_name = input[close_start..close_start + close_end].trim();

            if XML_TAGS.contains(&tag_name) {
                depth -= 1;
                if depth == 0 {
                    *cursor = close_start + close_end + 1;
                    return Ok(bracket_pos);
                }
            }
            search_pos = close_start + close_end + 1;
            continue;
        } else if let Some(close_bracket_offset) = input[bracket_pos..].find('>') {
            let close_bracket_pos = bracket_pos + close_bracket_offset;
            let tag_name = &input[bracket_pos + 1..close_bracket_pos].trim();
            if XML_TAGS.contains(&tag_name) {
                depth += 1;
            }
        }

        search_pos = bracket_pos + 1;
    }

    anyhow::bail!("no closing tag found")
}

const REPLACEMENT_COST: u32 = 1;
const INSERTION_COST: u32 = 3;
const DELETION_COST: u32 = 10;

/// A fuzzy matcher that can process text chunks incrementally
/// and return the best match found so far at each step.
struct FuzzyMatcher<'a> {
    snapshot: &'a BufferSnapshot,
    query_lines: Vec<&'a str>,
    matrix: SearchMatrix,
}

impl<'a> FuzzyMatcher<'a> {
    fn new(snapshot: &'a BufferSnapshot, old_text: &'a str) -> Self {
        let query_lines = old_text.lines().collect();
        Self {
            snapshot,
            query_lines,
            matrix: SearchMatrix::new(0),
        }
    }

    fn match_range(&mut self, range: Range<usize>) -> Option<(u32, Range<usize>)> {
        let point_range = range.to_point(&self.snapshot);
        let buffer_line_count = (point_range.end.row - point_range.start.row + 1) as usize;

        self.matrix
            .reset(self.query_lines.len() + 1, buffer_line_count + 1);
        let query_line_count = self.query_lines.len();

        for row in 0..query_line_count {
            let query_line = self.query_lines[row].trim();
            let leading_deletion_cost = (row + 1) as u32 * DELETION_COST;

            self.matrix.set(
                row + 1,
                0,
                SearchState::new(leading_deletion_cost, SearchDirection::Up),
            );

            let mut buffer_lines = self.snapshot.text_for_range(range.clone()).lines();

            let mut col = 0;
            while let Some(buffer_line) = buffer_lines.next() {
                let buffer_line = buffer_line.trim();
                let up = SearchState::new(
                    self.matrix
                        .get(row, col + 1)
                        .cost
                        .saturating_add(DELETION_COST),
                    SearchDirection::Up,
                );
                let left = SearchState::new(
                    self.matrix
                        .get(row + 1, col)
                        .cost
                        .saturating_add(INSERTION_COST),
                    SearchDirection::Left,
                );
                let diagonal = SearchState::new(
                    if query_line == buffer_line {
                        self.matrix.get(row, col).cost
                    } else if fuzzy_eq(query_line, buffer_line) {
                        self.matrix.get(row, col).cost + REPLACEMENT_COST
                    } else {
                        self.matrix
                            .get(row, col)
                            .cost
                            .saturating_add(DELETION_COST + INSERTION_COST)
                    },
                    SearchDirection::Diagonal,
                );
                self.matrix
                    .set(row + 1, col + 1, up.min(left).min(diagonal));
                col += 1;
            }
        }

        // Find all matches with the best cost
        let mut best_cost = u32::MAX;
        let mut matches_with_best_cost = Vec::new();

        for col in 1..=buffer_line_count {
            let cost = self.matrix.get(query_line_count, col).cost;
            if cost < best_cost {
                best_cost = cost;
                matches_with_best_cost.clear();
                matches_with_best_cost.push(col as u32);
            } else if cost == best_cost {
                matches_with_best_cost.push(col as u32);
            }
        }

        // Find ranges for the matches
        for &match_end_col in &matches_with_best_cost {
            let mut matched_lines = 0;
            let mut query_row = query_line_count;
            let mut match_start_col = match_end_col;
            while query_row > 0 && match_start_col > 0 {
                let current = self.matrix.get(query_row, match_start_col as usize);
                match current.direction {
                    SearchDirection::Diagonal => {
                        query_row -= 1;
                        match_start_col -= 1;
                        matched_lines += 1;
                    }
                    SearchDirection::Up => {
                        query_row -= 1;
                    }
                    SearchDirection::Left => {
                        match_start_col -= 1;
                    }
                }
            }

            let buffer_row_start = match_start_col + point_range.start.row;
            let buffer_row_end = match_end_col + point_range.start.row;

            let matched_buffer_row_count = buffer_row_end - buffer_row_start;
            let matched_ratio = matched_lines as f32
                / (matched_buffer_row_count as f32).max(query_line_count as f32);
            if matched_ratio >= 0.8 {
                let buffer_start_ix = self
                    .snapshot
                    .point_to_offset(Point::new(buffer_row_start, 0));
                let buffer_end_ix = self.snapshot.point_to_offset(Point::new(
                    buffer_row_end - 1,
                    self.snapshot.line_len(buffer_row_end - 1),
                ));
                return Some((best_cost, buffer_start_ix..buffer_end_ix));
            }
        }

        None
    }
}

fn fuzzy_eq(left: &str, right: &str) -> bool {
    const THRESHOLD: f64 = 0.8;

    let min_levenshtein = left.len().abs_diff(right.len());
    let min_normalized_levenshtein =
        1. - (min_levenshtein as f64 / cmp::max(left.len(), right.len()) as f64);
    if min_normalized_levenshtein < THRESHOLD {
        return false;
    }

    strsim::normalized_levenshtein(left, right) >= THRESHOLD
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SearchDirection {
    Up,
    Left,
    Diagonal,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SearchState {
    cost: u32,
    direction: SearchDirection,
}

impl SearchState {
    fn new(cost: u32, direction: SearchDirection) -> Self {
        Self { cost, direction }
    }
}

struct SearchMatrix {
    cols: usize,
    rows: usize,
    data: Vec<SearchState>,
}

impl SearchMatrix {
    fn new(cols: usize) -> Self {
        SearchMatrix {
            cols,
            rows: 0,
            data: Vec::new(),
        }
    }

    fn reset(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;
        self.data
            .fill(SearchState::new(0, SearchDirection::Diagonal));
        self.data.resize(
            self.rows * self.cols,
            SearchState::new(0, SearchDirection::Diagonal),
        );
    }

    fn get(&self, row: usize, col: usize) -> SearchState {
        debug_assert!(row < self.rows);
        debug_assert!(col < self.cols);
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, state: SearchState) {
        debug_assert!(row < self.rows && col < self.cols);
        self.data[row * self.cols + col] = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::Point;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    #[test]
    fn test_extract_xml_edits() {
        let input = indoc! {r#"
            <edits path="test.rs">
            <old_text>
            old content
            </old_text>
            <new_text>
            new content
            </new_text>
            </edits>
        "#};

        let result = extract_xml_replacements(input).unwrap();
        assert_eq!(result.file_path, "test.rs");
        assert_eq!(result.replacements.len(), 1);
        assert_eq!(result.replacements[0].0, "old content");
        assert_eq!(result.replacements[0].1, "new content");
    }

    #[test]
    fn test_extract_xml_edits_with_wrong_closing_tags() {
        let input = indoc! {r#"
            <edits path="test.rs">
            <old_text>
            old content
            </new_text>
            <new_text>
            new content
            </old_text>
            </ edits >
        "#};

        let result = extract_xml_replacements(input).unwrap();
        assert_eq!(result.file_path, "test.rs");
        assert_eq!(result.replacements.len(), 1);
        assert_eq!(result.replacements[0].0, "old content");
        assert_eq!(result.replacements[0].1, "new content");
    }

    #[test]
    fn test_extract_xml_edits_with_xml_like_content() {
        let input = indoc! {r#"
            <edits path="component.tsx">
            <old_text>
            <foo><bar></bar></foo>
            </old_text>
            <new_text>
            <foo><bar><baz></baz></bar></foo>
            </new_text>
            </edits>
        "#};

        let result = extract_xml_replacements(input).unwrap();
        assert_eq!(result.file_path, "component.tsx");
        assert_eq!(result.replacements.len(), 1);
        assert_eq!(result.replacements[0].0, "<foo><bar></bar></foo>");
        assert_eq!(
            result.replacements[0].1,
            "<foo><bar><baz></baz></bar></foo>"
        );
    }

    #[test]
    fn test_extract_xml_edits_with_conflicting_content() {
        let input = indoc! {r#"
            <edits path="component.tsx">
            <old_text>
            <new_text></new_text>
            </old_text>
            <new_text>
            <old_text></old_text>
            </new_text>
            </edits>
        "#};

        let result = extract_xml_replacements(input).unwrap();
        assert_eq!(result.file_path, "component.tsx");
        assert_eq!(result.replacements.len(), 1);
        assert_eq!(result.replacements[0].0, "<new_text></new_text>");
        assert_eq!(result.replacements[0].1, "<old_text></old_text>");
    }

    #[test]
    fn test_extract_xml_edits_multiple_pairs() {
        let input = indoc! {r#"
            Some reasoning before edits. Lots of thinking going on here

            <edits path="test.rs">
            <old_text>
            first old
            </old_text>
            <new_text>
            first new
            </new_text>
            <old_text>
            second old
            </edits>
            <new_text>
            second new
            </old_text>
            </edits>
        "#};

        let result = extract_xml_replacements(input).unwrap();
        assert_eq!(result.file_path, "test.rs");
        assert_eq!(result.replacements.len(), 2);
        assert_eq!(result.replacements[0].0, "first old");
        assert_eq!(result.replacements[0].1, "first new");
        assert_eq!(result.replacements[1].0, "second old");
        assert_eq!(result.replacements[1].1, "second new");
    }

    #[test]
    fn test_extract_xml_edits_unexpected_eof() {
        let input = indoc! {r#"
            <edits path="test.rs">
            <old_text>
            first old
            </
        "#};

        extract_xml_replacements(input).expect_err("Unexpected end of file");
    }

    #[gpui::test]
    async fn test_parse_xml_edits(cx: &mut TestAppContext) {
        let fs = init_test(cx);

        let buffer_1_text = indoc! {r#"
            one two three four
            five six seven eight
            nine ten eleven twelve
            thirteen fourteen fifteen
            sixteen seventeen eighteen
        "#};

        fs.insert_tree(
            path!("/root"),
            json!({
                "file1": buffer_1_text,
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/root").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/root/file1"), cx)
            })
            .await
            .unwrap();
        let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());

        let edits = indoc! {r#"
            <edits path="root/file1">
            <old_text>
            nine ten eleven twelve
            </old_text>
            <new_text>
            nine TEN eleven twelve!
            </new_text>
            </edits>
        "#};

        let included_ranges = [(buffer_snapshot.anchor_before(Point::new(1, 0))..Anchor::MAX)];
        let (buffer, edits) = parse_xml_edits(edits, |_path| {
            Some((&buffer_snapshot, included_ranges.as_slice()))
        })
        .await
        .unwrap();

        let edits = edits
            .into_iter()
            .map(|(range, text)| (range.to_point(&buffer), text))
            .collect::<Vec<_>>();
        assert_eq!(
            edits,
            &[
                (Point::new(2, 5)..Point::new(2, 8), "TEN".into()),
                (Point::new(2, 22)..Point::new(2, 22), "!".into())
            ]
        );
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<FakeFs> {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
        });

        FakeFs::new(cx.background_executor.clone())
    }
}
