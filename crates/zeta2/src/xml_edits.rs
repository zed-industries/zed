use anyhow::{Context as _, Result, anyhow};
use language::{Anchor, BufferSnapshot, OffsetRangeExt as _, Point, TextBufferSnapshot};
use std::{cmp, ops::Range, path::Path, sync::Arc};

pub async fn parse_xml_edits<'a>(
    input: &'a str,
    get_buffer: impl Fn(&Path) -> Option<(&'a BufferSnapshot, &'a [Range<Anchor>])> + Send,
) -> Result<(&'a BufferSnapshot, Vec<(Range<Anchor>, Arc<str>)>)> {
    parse_xml_edits_inner(input, get_buffer)
        .await
        .with_context(|| format!("Failed to parse XML edits:\n{input}"))
}

async fn parse_xml_edits_inner<'a>(
    mut input: &'a str,
    get_buffer: impl Fn(&Path) -> Option<(&'a BufferSnapshot, &'a [Range<Anchor>])> + Send,
) -> Result<(&'a BufferSnapshot, Vec<(Range<Anchor>, Arc<str>)>)> {
    let edits_tag = parse_tag(&mut input, "edits")?.context("No edits tag")?;

    input = edits_tag.body;

    let file_path = edits_tag
        .attributes
        .trim_start()
        .strip_prefix("path")
        .context("no file attribute on edits tag")?
        .trim_end()
        .strip_prefix('=')
        .context("no value for path attribute")?
        .trim()
        .trim_start_matches('"')
        .trim_end_matches('"');

    let (buffer, context_ranges) = get_buffer(file_path.as_ref())
        .with_context(|| format!("no buffer for file {file_path}"))?;

    let mut edits = vec![];
    while let Some(old_text_tag) = parse_tag(&mut input, "old_text")? {
        let new_text_tag =
            parse_tag(&mut input, "new_text")?.context("no new_text tag following old_text")?;
        if let Some(match_range) = fuzzy_match_range(old_text_tag.body, buffer, context_ranges) {
            let old_text = buffer
                .text_for_range(match_range.clone())
                .collect::<String>();
            eprintln!(
                "{}",
                pretty_assertions::StrComparison::new(&old_text, new_text_tag.body)
            );
            let edits_within_hunk = language::text_diff(&old_text, &new_text_tag.body);
            edits.extend(
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

        // edits.extend(resolve_new_text_old_text_in_buffer(
        //     new_text_tag.body,
        //     old_text_tag.body,
        //     buffer,
        //     context_ranges,
        // )?);
    }

    Ok((buffer, edits))
}

fn fuzzy_match_range(
    old_text: &str,
    buffer: &BufferSnapshot,
    context_ranges: &[Range<Anchor>],
) -> Option<Range<usize>> {
    let mut state = StreamingFuzzyMatcher::new(buffer, old_text);
    let mut best_match = None;
    for range in context_ranges {
        let best_match_score = best_match.as_ref().map(|(score, _)| *score);
        best_match = match (best_match_score, state.match_range(range.to_offset(buffer))) {
            (Some(best_score), Some((new_score, _))) if best_score > new_score => best_match,
            (Some(best_score), Some(new_match @ (new_score, _))) if best_score < new_score => {
                Some(new_match)
            }
            (None, Some(new_match)) => Some(new_match),
            _ => best_match,
        };
    }

    best_match.map(|(_, range)| range)
}

fn resolve_new_text_old_text_in_buffer(
    new_text: &str,
    old_text: &str,
    buffer: &TextBufferSnapshot,
    ranges: &[Range<Anchor>],
) -> Result<impl Iterator<Item = (Range<Anchor>, Arc<str>)>, anyhow::Error> {
    if ranges.is_empty() {
        anyhow::bail!("no ranges provided");
    }
    let context_offset = if old_text.is_empty() {
        Ok(0)
    } else {
        let mut offset = None;
        for range in ranges {
            let range = range.to_offset(buffer);
            let text = buffer.text_for_range(range.clone()).collect::<String>();
            for (match_offset, _) in text.match_indices(old_text) {
                if let Some(offset) = offset {
                    let offset_match_point = buffer.offset_to_point(offset);
                    let second_match_point = buffer.offset_to_point(range.start + match_offset);
                    anyhow::bail!(
                        "old_text is not unique enough:\n{}\nFound at {:?} and {:?}",
                        old_text,
                        offset_match_point,
                        second_match_point
                    );
                }
                offset = Some(range.start + match_offset);
            }
        }
        offset.ok_or_else(|| {
            let mut closest_match = String::new();

            if cfg!(any(debug_assertions, feature = "eval-support"))
                && let Some(closest_old_text_match) =
                    closest_old_text_match(buffer, ranges, old_text)
            {
                closest_match = format!(
                    "\nClosest `old_text` match in provided ranges: {}",
                    pretty_assertions::StrComparison::new(old_text, &closest_old_text_match)
                );
            }
            anyhow!("Failed to match old_text:\n{}{}", old_text, closest_match)
        })
    }?;

    let edits_within_hunk = language::text_diff(&old_text, &new_text);
    Ok(edits_within_hunk
        .into_iter()
        .map(move |(inner_range, inner_text)| {
            (
                buffer.anchor_after(context_offset + inner_range.start)
                    ..buffer.anchor_before(context_offset + inner_range.end),
                inner_text,
            )
        }))
}

fn closest_old_text_match(
    buffer: &TextBufferSnapshot,
    ranges: &[Range<Anchor>],
    old_text: &str,
) -> Option<String> {
    let len = old_text.len();

    if len == 0 {
        return None;
    }

    let mut min_score = usize::MAX;
    let mut min_match = String::new();

    for range in ranges {
        let buffer_text = buffer.text_for_range(range.clone()).collect::<String>();
        let mut cursor = 0;
        while cursor <= buffer_text.len() {
            let candidate = &buffer_text[cursor..usize::min(buffer_text.len(), cursor + len)];

            let score = strsim::levenshtein(candidate, old_text);
            if score < min_score {
                min_score = score;
                min_match = candidate.to_string();
            }

            if cursor + len > buffer_text.len() {
                break;
            }

            cursor += 1;
        }
    }

    if min_score != usize::MAX {
        Some(min_match)
    } else {
        None
    }
}

struct ParsedTag<'a> {
    attributes: &'a str,
    body: &'a str,
}

fn parse_tag<'a>(input: &mut &'a str, tag: &str) -> Result<Option<ParsedTag<'a>>> {
    let open_tag = format!("<{}", tag);
    let close_tag = format!("</{}>", tag);
    let Some(start_ix) = input.find(&open_tag) else {
        return Ok(None);
    };
    let start_ix = start_ix + open_tag.len();
    let closing_bracket_ix = start_ix
        + input[start_ix..]
            .find('>')
            .with_context(|| format!("missing > after {tag}"))?;
    let attributes = &input[start_ix..closing_bracket_ix].trim();
    let end_ix = closing_bracket_ix
        + input[closing_bracket_ix..]
            .find(&close_tag)
            .with_context(|| format!("no `{close_tag}` tag"))?;
    let body = &input[closing_bracket_ix + '>'.len_utf8()..end_ix];
    let body = body.strip_prefix('\n').unwrap_or(body);
    *input = &input[end_ix + close_tag.len()..];
    Ok(Some(ParsedTag { attributes, body }))
}

const REPLACEMENT_COST: u32 = 1;
const INSERTION_COST: u32 = 3;
const DELETION_COST: u32 = 10;

/// A streaming fuzzy matcher that can process text chunks incrementally
/// and return the best match found so far at each step.
struct StreamingFuzzyMatcher<'a> {
    snapshot: &'a BufferSnapshot,
    query_lines: Vec<String>,
    line_hint: Option<u32>,
    incomplete_line: String,
    matches: Vec<Range<usize>>,
    matrix: SearchMatrix,
}

impl<'a> StreamingFuzzyMatcher<'a> {
    fn new(snapshot: &'a BufferSnapshot, old_text: &str) -> Self {
        let query_lines = old_text.lines().map(String::from).collect();
        Self {
            snapshot,
            query_lines,
            line_hint: None,
            incomplete_line: String::new(),
            matches: Vec::new(),
            matrix: SearchMatrix::new(0),
        }
    }

    fn match_range(&mut self, range: Range<usize>) -> Option<(u32, Range<usize>)> {
        let buffer_line_count = ((self.snapshot.offset_to_point(range.end).row
            - self.snapshot.offset_to_point(range.start).row)
            + 1) as usize;

        self.matrix
            .reset(self.query_lines.len() + 1, buffer_line_count + 1);
        // todo! rename
        let new_query_line_count = self.query_lines.len();
        let old_query_line_count = 0;

        // Process only the new query lines
        for row in old_query_line_count..new_query_line_count {
            let query_line = self.query_lines[row].trim();
            let leading_deletion_cost = (row + 1) as u32 * DELETION_COST;

            self.matrix.set(
                row + 1,
                0,
                SearchState::new(leading_deletion_cost, SearchDirection::Up),
            );

            dbg!(
                self.snapshot
                    .text_for_range(range.clone())
                    .collect::<String>()
            );
            let mut buffer_lines = self.snapshot.text_for_range(range.clone()).lines();

            let mut col = 0;
            while let Some(buffer_line) = buffer_lines.next() {
                dbg!(buffer_line);
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
            let cost = self.matrix.get(new_query_line_count, col).cost;
            if cost < best_cost {
                best_cost = cost;
                matches_with_best_cost.clear();
                matches_with_best_cost.push(col as u32);
            } else if cost == best_cost {
                matches_with_best_cost.push(col as u32);
            }
        }

        // Find ranges for the matches
        let mut valid_matches = Vec::new();
        for &buffer_row_end in &matches_with_best_cost {
            let mut matched_lines = 0;
            let mut query_row = new_query_line_count;
            let mut buffer_row_start = buffer_row_end;
            while query_row > 0 && buffer_row_start > 0 {
                let current = self.matrix.get(query_row, buffer_row_start as usize);
                match current.direction {
                    SearchDirection::Diagonal => {
                        query_row -= 1;
                        buffer_row_start -= 1;
                        matched_lines += 1;
                    }
                    SearchDirection::Up => {
                        query_row -= 1;
                    }
                    SearchDirection::Left => {
                        buffer_row_start -= 1;
                    }
                }
            }

            let matched_buffer_row_count = buffer_row_end - buffer_row_start;
            let matched_ratio = matched_lines as f32
                / (matched_buffer_row_count as f32).max(new_query_line_count as f32);
            if matched_ratio >= 0.8 {
                let buffer_start_ix = self
                    .snapshot
                    .point_to_offset(Point::new(buffer_row_start, 0));
                let buffer_end_ix = self.snapshot.point_to_offset(Point::new(
                    buffer_row_end - 1,
                    self.snapshot.line_len(buffer_row_end - 1),
                ));
                dbg!(
                    self.snapshot
                        .text_for_range(buffer_start_ix..buffer_end_ix)
                        .collect::<String>()
                );
                valid_matches.push((buffer_row_start, buffer_start_ix..buffer_end_ix));
            }
        }

        valid_matches.first().cloned()
    }

    /// Returns the query lines.
    fn query_lines(&self) -> &[String] {
        &self.query_lines
    }

    /// Push a new chunk of text and get the best match found so far.
    ///
    /// This method accumulates text chunks and processes complete lines.
    /// Partial lines are buffered internally until a newline is received.
    ///
    /// # Returns
    ///
    /// Returns `Some(range)` if a match has been found with the accumulated
    /// query so far, or `None` if no suitable match exists yet.
    fn push(&mut self, chunk: &str, line_hint: Option<u32>) -> Option<Range<usize>> {
        if line_hint.is_some() {
            self.line_hint = line_hint;
        }

        // Add the chunk to our incomplete line buffer
        self.incomplete_line.push_str(chunk);
        self.line_hint = line_hint;

        if let Some((last_pos, _)) = self.incomplete_line.match_indices('\n').next_back() {
            let complete_part = &self.incomplete_line[..=last_pos];

            // Split into lines and add to query_lines
            for line in complete_part.lines() {
                self.query_lines.push(line.to_string());
            }

            self.incomplete_line.replace_range(..last_pos + 1, "");

            self.matches = self.resolve_location_fuzzy();
        }

        let best_match = self.select_best_match();
        best_match.or_else(|| self.matches.first().cloned())
    }

    /// Finish processing and return the final best match(es).
    ///
    /// This processes any remaining incomplete line before returning the final
    /// match result.
    fn finish(&mut self) -> Vec<Range<usize>> {
        // Process any remaining incomplete line
        if !self.incomplete_line.is_empty() {
            self.query_lines.push(self.incomplete_line.clone());
            self.incomplete_line.clear();
            self.matches = self.resolve_location_fuzzy();
        }
        self.matches.clone()
    }

    fn resolve_location_fuzzy(&mut self) -> Vec<Range<usize>> {
        let new_query_line_count = self.query_lines.len();
        let old_query_line_count = self.matrix.rows.saturating_sub(1);
        if new_query_line_count == old_query_line_count {
            return Vec::new();
        }

        self.matrix.resize_rows(new_query_line_count + 1);

        // Process only the new query lines
        for row in old_query_line_count..new_query_line_count {
            let query_line = self.query_lines[row].trim();
            let leading_deletion_cost = (row + 1) as u32 * DELETION_COST;

            self.matrix.set(
                row + 1,
                0,
                SearchState::new(leading_deletion_cost, SearchDirection::Up),
            );

            let mut buffer_lines = self.snapshot.as_rope().chunks().lines();
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
        let buffer_line_count = self.snapshot.max_point().row as usize + 1;
        let mut best_cost = u32::MAX;
        let mut matches_with_best_cost = Vec::new();

        for col in 1..=buffer_line_count {
            let cost = self.matrix.get(new_query_line_count, col).cost;
            if cost < best_cost {
                best_cost = cost;
                matches_with_best_cost.clear();
                matches_with_best_cost.push(col as u32);
            } else if cost == best_cost {
                matches_with_best_cost.push(col as u32);
            }
        }

        // Find ranges for the matches
        let mut valid_matches = Vec::new();
        for &buffer_row_end in &matches_with_best_cost {
            let mut matched_lines = 0;
            let mut query_row = new_query_line_count;
            let mut buffer_row_start = buffer_row_end;
            while query_row > 0 && buffer_row_start > 0 {
                let current = self.matrix.get(query_row, buffer_row_start as usize);
                match current.direction {
                    SearchDirection::Diagonal => {
                        query_row -= 1;
                        buffer_row_start -= 1;
                        matched_lines += 1;
                    }
                    SearchDirection::Up => {
                        query_row -= 1;
                    }
                    SearchDirection::Left => {
                        buffer_row_start -= 1;
                    }
                }
            }

            let matched_buffer_row_count = buffer_row_end - buffer_row_start;
            let matched_ratio = matched_lines as f32
                / (matched_buffer_row_count as f32).max(new_query_line_count as f32);
            if matched_ratio >= 0.8 {
                let buffer_start_ix = self
                    .snapshot
                    .point_to_offset(Point::new(buffer_row_start, 0));
                let buffer_end_ix = self.snapshot.point_to_offset(Point::new(
                    buffer_row_end - 1,
                    self.snapshot.line_len(buffer_row_end - 1),
                ));
                valid_matches.push((buffer_row_start, buffer_start_ix..buffer_end_ix));
            }
        }

        valid_matches.into_iter().map(|(_, range)| range).collect()
    }

    /// Return the best match with starting position close enough to line_hint.
    fn select_best_match(&self) -> Option<Range<usize>> {
        // Allow line hint to be off by that many lines.
        // Higher values increase probability of applying edits to a wrong place,
        // Lower values increase edits failures and overall conversation length.
        const LINE_HINT_TOLERANCE: u32 = 200;

        if self.matches.is_empty() {
            return None;
        }

        if self.matches.len() == 1 {
            return self.matches.first().cloned();
        }

        let Some(line_hint) = self.line_hint else {
            // Multiple ambiguous matches
            return None;
        };

        let mut best_match = None;
        let mut best_distance = u32::MAX;

        for range in &self.matches {
            let start_point = self.snapshot.offset_to_point(range.start);
            let start_line = start_point.row;
            let distance = start_line.abs_diff(line_hint);

            if distance <= LINE_HINT_TOLERANCE && distance < best_distance {
                best_distance = distance;
                best_match = Some(range.clone());
            }
        }

        best_match
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

    fn resize_rows(&mut self, needed_rows: usize) {
        debug_assert!(needed_rows > self.rows);
        self.rows = needed_rows;
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

// #[cfg(test)]
// mod fuzzy_match_tests {
//     use super::*;
//     use indoc::indoc;
//     use language::{BufferId, TextBuffer};
//     use rand::prelude::*;
//     use text::ReplicaId;
//     use util::test::{generate_marked_text, marked_text_ranges};

//     #[test]
//     fn test_empty_query() {
//         let buffer = TextBuffer::new(
//             ReplicaId::LOCAL,
//             BufferId::new(1).unwrap(),
//             "Hello world\nThis is a test\nFoo bar baz",
//         );
//         let snapshot = buffer.snapshot();

//         let mut finder = StreamingFuzzyMatcher::new(snapshot);
//         assert_eq!(push(&mut finder, ""), None);
//         assert_eq!(finish(finder), None);
//     }

//     #[test]
//     fn test_streaming_exact_match() {
//         let buffer = TextBuffer::new(
//             ReplicaId::LOCAL,
//             BufferId::new(1).unwrap(),
//             "Hello world\nThis is a test\nFoo bar baz",
//         );
//         let snapshot = buffer.snapshot();

//         let mut finder = StreamingFuzzyMatcher::new(snapshot);

//         // Push partial query
//         assert_eq!(push(&mut finder, "This"), None);

//         // Complete the line
//         assert_eq!(
//             push(&mut finder, " is a test\n"),
//             Some("This is a test".to_string())
//         );

//         // Finish should return the same result
//         assert_eq!(finish(finder), Some("This is a test".to_string()));
//     }

//     #[test]
//     fn test_streaming_fuzzy_match() {
//         let buffer = TextBuffer::new(
//             ReplicaId::LOCAL,
//             BufferId::new(1).unwrap(),
//             indoc! {"
//                 function foo(a, b) {
//                     return a + b;
//                 }

//                 function bar(x, y) {
//                     return x * y;
//                 }
//             "},
//         );
//         let snapshot = buffer.snapshot();

//         let mut finder = StreamingFuzzyMatcher::new(snapshot);

//         // Push a fuzzy query that should match the first function
//         assert_eq!(
//             push(&mut finder, "function foo(a, c) {\n").as_deref(),
//             Some("function foo(a, b) {")
//         );
//         assert_eq!(
//             push(&mut finder, "    return a + c;\n}\n").as_deref(),
//             Some(concat!(
//                 "function foo(a, b) {\n",
//                 "    return a + b;\n",
//                 "}"
//             ))
//         );
//     }

//     #[test]
//     fn test_incremental_improvement() {
//         let buffer = TextBuffer::new(
//             ReplicaId::LOCAL,
//             BufferId::new(1).unwrap(),
//             "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
//         );
//         let snapshot = buffer.snapshot();

//         let mut finder = StreamingFuzzyMatcher::new(snapshot);

//         // No match initially
//         assert_eq!(push(&mut finder, "Lin"), None);

//         // Get a match when we complete a line
//         assert_eq!(push(&mut finder, "e 3\n"), Some("Line 3".to_string()));

//         // The match might change if we add more specific content
//         assert_eq!(
//             push(&mut finder, "Line 4\n"),
//             Some("Line 3\nLine 4".to_string())
//         );
//         assert_eq!(finish(finder), Some("Line 3\nLine 4".to_string()));
//     }

//     #[test]
//     fn test_incomplete_lines_buffering() {
//         let buffer = TextBuffer::new(
//             ReplicaId::LOCAL,
//             BufferId::new(1).unwrap(),
//             indoc! {"
//                 The quick brown fox
//                 jumps over the lazy dog
//                 Pack my box with five dozen liquor jugs
//             "},
//         );
//         let snapshot = buffer.snapshot();

//         let mut finder = StreamingFuzzyMatcher::new(snapshot);

//         // Push text in small chunks across line boundaries
//         assert_eq!(push(&mut finder, "jumps "), None); // No newline yet
//         assert_eq!(push(&mut finder, "over the"), None); // Still no newline
//         assert_eq!(push(&mut finder, " lazy"), None); // Still incomplete

//         // Complete the line
//         assert_eq!(
//             push(&mut finder, " dog\n"),
//             Some("jumps over the lazy dog".to_string())
//         );
//     }

//     #[test]
//     fn test_multiline_fuzzy_match() {
//         let buffer = TextBuffer::new(
//             ReplicaId::LOCAL,
//             BufferId::new(1).unwrap(),
//             indoc! {r#"
//                 impl Display for User {
//                     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//                         write!(f, "User: {} ({})", self.name, self.email)
//                     }
//                 }

//                 impl Debug for User {
//                     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//                         f.debug_struct("User")
//                             .field("name", &self.name)
//                             .field("email", &self.email)
//                             .finish()
//                     }
//                 }
//             "#},
//         );
//         let snapshot = buffer.snapshot();

//         let mut finder = StreamingFuzzyMatcher::new(snapshot);

//         assert_eq!(
//             push(&mut finder, "impl Debug for User {\n"),
//             Some("impl Debug for User {".to_string())
//         );
//         assert_eq!(
//             push(
//                 &mut finder,
//                 "    fn fmt(&self, f: &mut Formatter) -> Result {\n"
//             )
//             .as_deref(),
//             Some(concat!(
//                 "impl Debug for User {\n",
//                 "    fn fmt(&self, f: &mut Formatter) -> fmt::Result {"
//             ))
//         );
//         assert_eq!(
//             push(&mut finder, "        f.debug_struct(\"User\")\n").as_deref(),
//             Some(concat!(
//                 "impl Debug for User {\n",
//                 "    fn fmt(&self, f: &mut Formatter) -> fmt::Result {\n",
//                 "        f.debug_struct(\"User\")"
//             ))
//         );
//         assert_eq!(
//             push(
//                 &mut finder,
//                 "            .field(\"name\", &self.username)\n"
//             )
//             .as_deref(),
//             Some(concat!(
//                 "impl Debug for User {\n",
//                 "    fn fmt(&self, f: &mut Formatter) -> fmt::Result {\n",
//                 "        f.debug_struct(\"User\")\n",
//                 "            .field(\"name\", &self.name)"
//             ))
//         );
//         assert_eq!(
//             finish(finder).as_deref(),
//             Some(concat!(
//                 "impl Debug for User {\n",
//                 "    fn fmt(&self, f: &mut Formatter) -> fmt::Result {\n",
//                 "        f.debug_struct(\"User\")\n",
//                 "            .field(\"name\", &self.name)"
//             ))
//         );
//     }

//     #[gpui::test(iterations = 100)]
//     fn test_resolve_location_single_line(mut rng: StdRng) {
//         assert_location_resolution(
//             concat!(
//                 "    Lorem\n",
//                 "«    ipsum»\n",
//                 "    dolor sit amet\n",
//                 "    consecteur",
//             ),
//             "ipsum",
//             &mut rng,
//         );
//     }

//     #[gpui::test(iterations = 100)]
//     fn test_resolve_location_multiline(mut rng: StdRng) {
//         assert_location_resolution(
//             concat!(
//                 "    Lorem\n",
//                 "«    ipsum\n",
//                 "    dolor sit amet»\n",
//                 "    consecteur",
//             ),
//             "ipsum\ndolor sit amet",
//             &mut rng,
//         );
//     }

//     #[gpui::test(iterations = 100)]
//     fn test_resolve_location_function_with_typo(mut rng: StdRng) {
//         assert_location_resolution(
//             indoc! {"
//                 «fn foo1(a: usize) -> usize {
//                     40
//                 }»

//                 fn foo2(b: usize) -> usize {
//                     42
//                 }
//             "},
//             "fn foo1(a: usize) -> u32 {\n40\n}",
//             &mut rng,
//         );
//     }

//     #[gpui::test(iterations = 100)]
//     fn test_resolve_location_class_methods(mut rng: StdRng) {
//         assert_location_resolution(
//             indoc! {"
//                 class Something {
//                     one() { return 1; }
//                 «    two() { return 2222; }
//                     three() { return 333; }
//                     four() { return 4444; }
//                     five() { return 5555; }
//                     six() { return 6666; }»
//                     seven() { return 7; }
//                     eight() { return 8; }
//                 }
//             "},
//             indoc! {"
//                 two() { return 2222; }
//                 four() { return 4444; }
//                 five() { return 5555; }
//                 six() { return 6666; }
//             "},
//             &mut rng,
//         );
//     }

//     #[gpui::test(iterations = 100)]
//     fn test_resolve_location_imports_no_match(mut rng: StdRng) {
//         assert_location_resolution(
//             indoc! {"
//                 use std::ops::Range;
//                 use std::sync::Mutex;
//                 use std::{
//                     collections::HashMap,
//                     env,
//                     ffi::{OsStr, OsString},
//                     fs,
//                     io::{BufRead, BufReader},
//                     mem,
//                     path::{Path, PathBuf},
//                     process::Command,
//                     sync::LazyLock,
//                     time::SystemTime,
//                 };
//             "},
//             indoc! {"
//                 use std::collections::{HashMap, HashSet};
//                 use std::ffi::{OsStr, OsString};
//                 use std::fmt::Write as _;
//                 use std::fs;
//                 use std::io::{BufReader, Read, Write};
//                 use std::mem;
//                 use std::path::{Path, PathBuf};
//                 use std::process::Command;
//                 use std::sync::Arc;
//             "},
//             &mut rng,
//         );
//     }

//     #[gpui::test(iterations = 100)]
//     fn test_resolve_location_nested_closure(mut rng: StdRng) {
//         assert_location_resolution(
//             indoc! {"
//                 impl Foo {
//                     fn new() -> Self {
//                         Self {
//                             subscriptions: vec![
//                                 cx.observe_window_activation(window, |editor, window, cx| {
//                                     let active = window.is_window_active();
//                                     editor.blink_manager.update(cx, |blink_manager, cx| {
//                                         if active {
//                                             blink_manager.enable(cx);
//                                         } else {
//                                             blink_manager.disable(cx);
//                                         }
//                                     });
//                                 }),
//                             ];
//                         }
//                     }
//                 }
//             "},
//             concat!(
//                 "                    editor.blink_manager.update(cx, |blink_manager, cx| {\n",
//                 "                        blink_manager.enable(cx);\n",
//                 "                    });",
//             ),
//             &mut rng,
//         );
//     }

//     #[gpui::test(iterations = 100)]
//     fn test_resolve_location_tool_invocation(mut rng: StdRng) {
//         assert_location_resolution(
//             indoc! {r#"
//                 let tool = cx
//                     .update(|cx| working_set.tool(&tool_name, cx))
//                     .map_err(|err| {
//                         anyhow!("Failed to look up tool '{}': {}", tool_name, err)
//                     })?;

//                 let Some(tool) = tool else {
//                     return Err(anyhow!("Tool '{}' not found", tool_name));
//                 };

//                 let project = project.clone();
//                 let action_log = action_log.clone();
//                 let messages = messages.clone();
//                 let tool_result = cx
//                     .update(|cx| tool.run(invocation.input, &messages, project, action_log, cx))
//                     .map_err(|err| anyhow!("Failed to start tool '{}': {}", tool_name, err))?;

//                 tasks.push(tool_result.output);
//             "#},
//             concat!(
//                 "let tool_result = cx\n",
//                 "    .update(|cx| tool.run(invocation.input, &messages, project, action_log, cx))\n",
//                 "    .output;",
//             ),
//             &mut rng,
//         );
//     }

//     #[gpui::test]
//     fn test_line_hint_selection() {
//         let text = indoc! {r#"
//             fn first_function() {
//                 return 42;
//             }

//             fn second_function() {
//                 return 42;
//             }

//             fn third_function() {
//                 return 42;
//             }
//         "#};

//         let buffer = TextBuffer::new(
//             ReplicaId::LOCAL,
//             BufferId::new(1).unwrap(),
//             text.to_string(),
//         );
//         let snapshot = buffer.snapshot();
//         let mut matcher = StreamingFuzzyMatcher::new(snapshot.clone());

//         // Given a query that matches all three functions
//         let query = "return 42;\n";

//         // Test with line hint pointing to second function (around line 5)
//         let best_match = matcher.push(query, Some(5)).expect("Failed to match query");

//         let matched_text = snapshot
//             .text_for_range(best_match.clone())
//             .collect::<String>();
//         assert!(matched_text.contains("return 42;"));
//         assert_eq!(
//             best_match,
//             63..77,
//             "Expected to match `second_function` based on the line hint"
//         );

//         let mut matcher = StreamingFuzzyMatcher::new(snapshot);
//         matcher.push(query, None);
//         matcher.finish();
//         let best_match = matcher.select_best_match();
//         assert!(
//             best_match.is_none(),
//             "Best match should be None when query cannot be uniquely resolved"
//         );
//     }

//     #[track_caller]
//     fn assert_location_resolution(text_with_expected_range: &str, query: &str, rng: &mut StdRng) {
//         let (text, expected_ranges) = marked_text_ranges(text_with_expected_range, false);
//         let buffer = TextBuffer::new(ReplicaId::LOCAL, BufferId::new(1).unwrap(), text.clone());
//         let snapshot = buffer.snapshot();

//         let mut matcher = StreamingFuzzyMatcher::new(snapshot);

//         // Split query into random chunks
//         let chunks = to_random_chunks(rng, query);

//         // Push chunks incrementally
//         for chunk in &chunks {
//             matcher.push(chunk, None);
//         }

//         let actual_ranges = matcher.finish();

//         // If no expected ranges, we expect no match
//         if expected_ranges.is_empty() {
//             assert!(
//                 actual_ranges.is_empty(),
//                 "Expected no match for query: {:?}, but found: {:?}",
//                 query,
//                 actual_ranges
//             );
//         } else {
//             let text_with_actual_range = generate_marked_text(&text, &actual_ranges, false);
//             pretty_assertions::assert_eq!(
//                 text_with_actual_range,
//                 text_with_expected_range,
//                 indoc! {"
//                     Query: {:?}
//                     Chunks: {:?}
//                     Expected marked text: {}
//                     Actual marked text: {}
//                     Expected ranges: {:?}
//                     Actual ranges: {:?}"
//                 },
//                 query,
//                 chunks,
//                 text_with_expected_range,
//                 text_with_actual_range,
//                 expected_ranges,
//                 actual_ranges
//             );
//         }
//     }

//     fn to_random_chunks(rng: &mut StdRng, input: &str) -> Vec<String> {
//         let chunk_count = rng.random_range(1..=cmp::min(input.len(), 50));
//         let mut chunk_indices = (0..input.len()).choose_multiple(rng, chunk_count);
//         chunk_indices.sort();
//         chunk_indices.push(input.len());

//         let mut chunks = Vec::new();
//         let mut last_ix = 0;
//         for chunk_ix in chunk_indices {
//             chunks.push(input[last_ix..chunk_ix].to_string());
//             last_ix = chunk_ix;
//         }
//         chunks
//     }

//     fn push(finder: &mut StreamingFuzzyMatcher, chunk: &str) -> Option<String> {
//         finder
//             .push(chunk, None)
//             .map(|range| finder.snapshot.text_for_range(range).collect::<String>())
//     }

//     fn finish(mut finder: StreamingFuzzyMatcher) -> Option<String> {
//         let snapshot = finder.snapshot.clone();
//         let matches = finder.finish();
//         matches
//             .first()
//             .map(|range| snapshot.text_for_range(range.clone()).collect::<String>())
//     }
// }

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
    fn test_parse_tags() {
        let mut input = indoc! {r#"
            Prelude
            <tag attr="foo">
            tag value
            </tag>
            "# };
        let parsed = parse_tag(&mut input, "tag").unwrap().unwrap();
        assert_eq!(parsed.attributes, "attr=\"foo\"");
        assert_eq!(parsed.body, "tag value\n");
        assert_eq!(input, "\n");
    }

    #[gpui::test]
    async fn test_parse_xml_edits(cx: &mut TestAppContext) {
        let fs = init_test(cx);

        let buffer_1_text = indoc! {r#"
            one two three four
            five six seven eight
            nine ten eleven twelve
        "# };

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
            five six seven eight
            </old_text>
            <new_text>
            five SIX seven eight!
            </new_text>
            </edits>
        "#};

        let (buffer, edits) = parse_xml_edits(edits, |_path| {
            Some((&buffer_snapshot, &[(Anchor::MIN..Anchor::MAX)] as &[_]))
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
                (Point::new(1, 5)..Point::new(1, 8), "SIX".into()),
                (Point::new(1, 20)..Point::new(1, 20), "!".into())
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
