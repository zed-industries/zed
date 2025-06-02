use language::{Point, TextBufferSnapshot};
use std::{cmp, ops::Range};

const REPLACEMENT_COST: u32 = 1;
const INSERTION_COST: u32 = 3;
const DELETION_COST: u32 = 10;

/// A streaming fuzzy matcher that can process text chunks incrementally
/// and return the best match found so far at each step.
pub struct StreamingFuzzyMatcher {
    snapshot: TextBufferSnapshot,
    query_lines: Vec<String>,
    incomplete_line: String,
    best_match: Option<Range<usize>>,
    matrix: SearchMatrix,
}

impl StreamingFuzzyMatcher {
    pub fn new(snapshot: TextBufferSnapshot) -> Self {
        let buffer_line_count = snapshot.max_point().row as usize + 1;
        Self {
            snapshot,
            query_lines: Vec::new(),
            incomplete_line: String::new(),
            best_match: None,
            matrix: SearchMatrix::new(buffer_line_count + 1),
        }
    }

    /// Returns the query lines.
    pub fn query_lines(&self) -> &[String] {
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
    pub fn push(&mut self, chunk: &str) -> Option<Range<usize>> {
        // Add the chunk to our incomplete line buffer
        self.incomplete_line.push_str(chunk);

        if let Some((last_pos, _)) = self.incomplete_line.match_indices('\n').next_back() {
            let complete_part = &self.incomplete_line[..=last_pos];

            // Split into lines and add to query_lines
            for line in complete_part.lines() {
                self.query_lines.push(line.to_string());
            }

            self.incomplete_line.replace_range(..last_pos + 1, "");

            self.best_match = self.resolve_location_fuzzy();
        }

        self.best_match.clone()
    }

    /// Finish processing and return the final best match.
    ///
    /// This processes any remaining incomplete line before returning the final
    /// match result.
    pub fn finish(&mut self) -> Option<Range<usize>> {
        // Process any remaining incomplete line
        if !self.incomplete_line.is_empty() {
            self.query_lines.push(self.incomplete_line.clone());
            self.best_match = self.resolve_location_fuzzy();
        }

        self.best_match.clone()
    }

    fn resolve_location_fuzzy(&mut self) -> Option<Range<usize>> {
        let new_query_line_count = self.query_lines.len();
        let old_query_line_count = self.matrix.rows.saturating_sub(1);
        if new_query_line_count == old_query_line_count {
            return None;
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

        // Traceback to find the best match
        let buffer_line_count = self.snapshot.max_point().row as usize + 1;
        let mut buffer_row_end = buffer_line_count as u32;
        let mut best_cost = u32::MAX;
        for col in 1..=buffer_line_count {
            let cost = self.matrix.get(new_query_line_count, col).cost;
            if cost < best_cost {
                best_cost = cost;
                buffer_row_end = col as u32;
            }
        }

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
            Some(buffer_start_ix..buffer_end_ix)
        } else {
            None
        }
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

    fn resize_rows(&mut self, needed_rows: usize) {
        debug_assert!(needed_rows > self.rows);
        self.rows = needed_rows;
        self.data.resize(
            self.rows * self.cols,
            SearchState::new(0, SearchDirection::Diagonal),
        );
    }

    fn get(&self, row: usize, col: usize) -> SearchState {
        debug_assert!(row < self.rows && col < self.cols);
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
    use indoc::indoc;
    use language::{BufferId, TextBuffer};
    use rand::prelude::*;
    use util::test::{generate_marked_text, marked_text_ranges};

    #[test]
    fn test_empty_query() {
        let buffer = TextBuffer::new(
            0,
            BufferId::new(1).unwrap(),
            "Hello world\nThis is a test\nFoo bar baz",
        );
        let snapshot = buffer.snapshot();

        let mut finder = StreamingFuzzyMatcher::new(snapshot.clone());
        assert_eq!(push(&mut finder, ""), None);
        assert_eq!(finish(finder), None);
    }

    #[test]
    fn test_streaming_exact_match() {
        let buffer = TextBuffer::new(
            0,
            BufferId::new(1).unwrap(),
            "Hello world\nThis is a test\nFoo bar baz",
        );
        let snapshot = buffer.snapshot();

        let mut finder = StreamingFuzzyMatcher::new(snapshot.clone());

        // Push partial query
        assert_eq!(push(&mut finder, "This"), None);

        // Complete the line
        assert_eq!(
            push(&mut finder, " is a test\n"),
            Some("This is a test".to_string())
        );

        // Finish should return the same result
        assert_eq!(finish(finder), Some("This is a test".to_string()));
    }

    #[test]
    fn test_streaming_fuzzy_match() {
        let buffer = TextBuffer::new(
            0,
            BufferId::new(1).unwrap(),
            indoc! {"
                function foo(a, b) {
                    return a + b;
                }

                function bar(x, y) {
                    return x * y;
                }
            "},
        );
        let snapshot = buffer.snapshot();

        let mut finder = StreamingFuzzyMatcher::new(snapshot.clone());

        // Push a fuzzy query that should match the first function
        assert_eq!(
            push(&mut finder, "function foo(a, c) {\n").as_deref(),
            Some("function foo(a, b) {")
        );
        assert_eq!(
            push(&mut finder, "    return a + c;\n}\n").as_deref(),
            Some(concat!(
                "function foo(a, b) {\n",
                "    return a + b;\n",
                "}"
            ))
        );
    }

    #[test]
    fn test_incremental_improvement() {
        let buffer = TextBuffer::new(
            0,
            BufferId::new(1).unwrap(),
            "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
        );
        let snapshot = buffer.snapshot();

        let mut finder = StreamingFuzzyMatcher::new(snapshot.clone());

        // No match initially
        assert_eq!(push(&mut finder, "Lin"), None);

        // Get a match when we complete a line
        assert_eq!(push(&mut finder, "e 3\n"), Some("Line 3".to_string()));

        // The match might change if we add more specific content
        assert_eq!(
            push(&mut finder, "Line 4\n"),
            Some("Line 3\nLine 4".to_string())
        );
        assert_eq!(finish(finder), Some("Line 3\nLine 4".to_string()));
    }

    #[test]
    fn test_incomplete_lines_buffering() {
        let buffer = TextBuffer::new(
            0,
            BufferId::new(1).unwrap(),
            indoc! {"
                The quick brown fox
                jumps over the lazy dog
                Pack my box with five dozen liquor jugs
            "},
        );
        let snapshot = buffer.snapshot();

        let mut finder = StreamingFuzzyMatcher::new(snapshot.clone());

        // Push text in small chunks across line boundaries
        assert_eq!(push(&mut finder, "jumps "), None); // No newline yet
        assert_eq!(push(&mut finder, "over the"), None); // Still no newline
        assert_eq!(push(&mut finder, " lazy"), None); // Still incomplete

        // Complete the line
        assert_eq!(
            push(&mut finder, " dog\n"),
            Some("jumps over the lazy dog".to_string())
        );
    }

    #[test]
    fn test_multiline_fuzzy_match() {
        let buffer = TextBuffer::new(
            0,
            BufferId::new(1).unwrap(),
            indoc! {r#"
                impl Display for User {
                    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                        write!(f, "User: {} ({})", self.name, self.email)
                    }
                }

                impl Debug for User {
                    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                        f.debug_struct("User")
                            .field("name", &self.name)
                            .field("email", &self.email)
                            .finish()
                    }
                }
            "#},
        );
        let snapshot = buffer.snapshot();

        let mut finder = StreamingFuzzyMatcher::new(snapshot.clone());

        assert_eq!(
            push(&mut finder, "impl Debug for User {\n"),
            Some("impl Debug for User {".to_string())
        );
        assert_eq!(
            push(
                &mut finder,
                "    fn fmt(&self, f: &mut Formatter) -> Result {\n"
            )
            .as_deref(),
            Some(concat!(
                "impl Debug for User {\n",
                "    fn fmt(&self, f: &mut Formatter) -> fmt::Result {"
            ))
        );
        assert_eq!(
            push(&mut finder, "        f.debug_struct(\"User\")\n").as_deref(),
            Some(concat!(
                "impl Debug for User {\n",
                "    fn fmt(&self, f: &mut Formatter) -> fmt::Result {\n",
                "        f.debug_struct(\"User\")"
            ))
        );
        assert_eq!(
            push(
                &mut finder,
                "            .field(\"name\", &self.username)\n"
            )
            .as_deref(),
            Some(concat!(
                "impl Debug for User {\n",
                "    fn fmt(&self, f: &mut Formatter) -> fmt::Result {\n",
                "        f.debug_struct(\"User\")\n",
                "            .field(\"name\", &self.name)"
            ))
        );
        assert_eq!(
            finish(finder).as_deref(),
            Some(concat!(
                "impl Debug for User {\n",
                "    fn fmt(&self, f: &mut Formatter) -> fmt::Result {\n",
                "        f.debug_struct(\"User\")\n",
                "            .field(\"name\", &self.name)"
            ))
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_resolve_location_single_line(mut rng: StdRng) {
        assert_location_resolution(
            concat!(
                "    Lorem\n",
                "«    ipsum»\n",
                "    dolor sit amet\n",
                "    consecteur",
            ),
            "ipsum",
            &mut rng,
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_resolve_location_multiline(mut rng: StdRng) {
        assert_location_resolution(
            concat!(
                "    Lorem\n",
                "«    ipsum\n",
                "    dolor sit amet»\n",
                "    consecteur",
            ),
            "ipsum\ndolor sit amet",
            &mut rng,
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_resolve_location_function_with_typo(mut rng: StdRng) {
        assert_location_resolution(
            indoc! {"
                «fn foo1(a: usize) -> usize {
                    40
                }»

                fn foo2(b: usize) -> usize {
                    42
                }
            "},
            "fn foo1(a: usize) -> u32 {\n40\n}",
            &mut rng,
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_resolve_location_class_methods(mut rng: StdRng) {
        assert_location_resolution(
            indoc! {"
                class Something {
                    one() { return 1; }
                «    two() { return 2222; }
                    three() { return 333; }
                    four() { return 4444; }
                    five() { return 5555; }
                    six() { return 6666; }»
                    seven() { return 7; }
                    eight() { return 8; }
                }
            "},
            indoc! {"
                two() { return 2222; }
                four() { return 4444; }
                five() { return 5555; }
                six() { return 6666; }
            "},
            &mut rng,
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_resolve_location_imports_no_match(mut rng: StdRng) {
        assert_location_resolution(
            indoc! {"
                use std::ops::Range;
                use std::sync::Mutex;
                use std::{
                    collections::HashMap,
                    env,
                    ffi::{OsStr, OsString},
                    fs,
                    io::{BufRead, BufReader},
                    mem,
                    path::{Path, PathBuf},
                    process::Command,
                    sync::LazyLock,
                    time::SystemTime,
                };
            "},
            indoc! {"
                use std::collections::{HashMap, HashSet};
                use std::ffi::{OsStr, OsString};
                use std::fmt::Write as _;
                use std::fs;
                use std::io::{BufReader, Read, Write};
                use std::mem;
                use std::path::{Path, PathBuf};
                use std::process::Command;
                use std::sync::Arc;
            "},
            &mut rng,
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_resolve_location_nested_closure(mut rng: StdRng) {
        assert_location_resolution(
            indoc! {"
                impl Foo {
                    fn new() -> Self {
                        Self {
                            subscriptions: vec![
                                cx.observe_window_activation(window, |editor, window, cx| {
                                    let active = window.is_window_active();
                                    editor.blink_manager.update(cx, |blink_manager, cx| {
                                        if active {
                                            blink_manager.enable(cx);
                                        } else {
                                            blink_manager.disable(cx);
                                        }
                                    });
                                }),
                            ];
                        }
                    }
                }
            "},
            concat!(
                "                    editor.blink_manager.update(cx, |blink_manager, cx| {\n",
                "                        blink_manager.enable(cx);\n",
                "                    });",
            ),
            &mut rng,
        );
    }

    #[gpui::test(iterations = 100)]
    fn test_resolve_location_tool_invocation(mut rng: StdRng) {
        assert_location_resolution(
            indoc! {r#"
                let tool = cx
                    .update(|cx| working_set.tool(&tool_name, cx))
                    .map_err(|err| {
                        anyhow!("Failed to look up tool '{}': {}", tool_name, err)
                    })?;

                let Some(tool) = tool else {
                    return Err(anyhow!("Tool '{}' not found", tool_name));
                };

                let project = project.clone();
                let action_log = action_log.clone();
                let messages = messages.clone();
                let tool_result = cx
                    .update(|cx| tool.run(invocation.input, &messages, project, action_log, cx))
                    .map_err(|err| anyhow!("Failed to start tool '{}': {}", tool_name, err))?;

                tasks.push(tool_result.output);
            "#},
            concat!(
                "let tool_result = cx\n",
                "    .update(|cx| tool.run(invocation.input, &messages, project, action_log, cx))\n",
                "    .output;",
            ),
            &mut rng,
        );
    }

    #[track_caller]
    fn assert_location_resolution(text_with_expected_range: &str, query: &str, rng: &mut StdRng) {
        let (text, expected_ranges) = marked_text_ranges(text_with_expected_range, false);
        let buffer = TextBuffer::new(0, BufferId::new(1).unwrap(), text.clone());
        let snapshot = buffer.snapshot();

        let mut matcher = StreamingFuzzyMatcher::new(snapshot.clone());

        // Split query into random chunks
        let chunks = to_random_chunks(rng, query);

        // Push chunks incrementally
        for chunk in &chunks {
            matcher.push(chunk);
        }

        let result = matcher.finish();

        // If no expected ranges, we expect no match
        if expected_ranges.is_empty() {
            assert_eq!(
                result, None,
                "Expected no match for query: {:?}, but found: {:?}",
                query, result
            );
        } else {
            let mut actual_ranges = Vec::new();
            if let Some(range) = result {
                actual_ranges.push(range);
            }

            let text_with_actual_range = generate_marked_text(&text, &actual_ranges, false);
            pretty_assertions::assert_eq!(
                text_with_actual_range,
                text_with_expected_range,
                "Query: {:?}, Chunks: {:?}",
                query,
                chunks
            );
        }
    }

    fn to_random_chunks(rng: &mut StdRng, input: &str) -> Vec<String> {
        let chunk_count = rng.gen_range(1..=cmp::min(input.len(), 50));
        let mut chunk_indices = (0..input.len()).choose_multiple(rng, chunk_count);
        chunk_indices.sort();
        chunk_indices.push(input.len());

        let mut chunks = Vec::new();
        let mut last_ix = 0;
        for chunk_ix in chunk_indices {
            chunks.push(input[last_ix..chunk_ix].to_string());
            last_ix = chunk_ix;
        }
        chunks
    }

    fn push(finder: &mut StreamingFuzzyMatcher, chunk: &str) -> Option<String> {
        finder
            .push(chunk)
            .map(|range| finder.snapshot.text_for_range(range).collect::<String>())
    }

    fn finish(mut finder: StreamingFuzzyMatcher) -> Option<String> {
        let snapshot = finder.snapshot.clone();
        finder
            .finish()
            .map(|range| snapshot.text_for_range(range).collect::<String>())
    }
}
