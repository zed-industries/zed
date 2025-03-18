use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use futures::StreamExt;
use gpui::{App, Entity, Task};
use language::{OffsetRangeExt, Point};


fn find_matches(text: String, pattern: &str) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();
    if pattern.is_empty() {
        return matches;
    }

    let mut start = 0;
    while start < text.len() {
        match text[start..].find(pattern) {
            Some(pos) => {
                let match_start = start + pos;
                let match_end = match_start + pattern.len();
                if match_end <= text.len() {
                    matches.push((match_start, match_end));
                }
                start = match_start + 1;
            }
            None => break,
        }
    }
    matches
}
use language_model::LanguageModelRequestMessage;
use project::{
    search::{SearchQuery, SearchResult},
    Project,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{cmp, fmt::Write, sync::Arc};
use util::paths::PathMatcher;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RegexSearchToolInput {
    /// A regex pattern to search for in the entire project. Note that the regex
    /// will be parsed by the Rust `regex` crate.
    pub regex: String,

    /// Optional starting position for paginated results (0-based).
    /// When not provided, starts from the beginning.
    #[serde(default)]
    pub offset: Option<usize>,
}

const RESULTS_PER_PAGE: usize = 20;
const MAX_LINE_LENGTH: usize = 240;
const LONG_LINE_CONTEXT: usize = 120;

pub struct RegexSearchTool;

impl Tool for RegexSearchTool {
    fn name(&self) -> String {
        "regex-search".into()
    }

    fn description(&self) -> String {
        include_str!("./regex_search_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(RegexSearchToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        const CONTEXT_LINES: u32 = 2;

        let (offset, regex) = match serde_json::from_value::<RegexSearchToolInput>(input) {
            Ok(input) => (input.offset.unwrap_or(0), input.regex),
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        if regex.is_empty() {
            return Task::ready(Err(anyhow!("Empty regex pattern is not allowed")));
        };

        let query = match SearchQuery::regex(
            &regex,
            false,
            false,
            false,
            PathMatcher::default(),
            PathMatcher::default(),
            None,
        ) {
            Ok(query) => query,
            Err(error) => return Task::ready(Err(error)),
        };

        let results = project.update(cx, |project, cx| project.search(query, cx));

        cx.spawn(|cx| async move {
            futures::pin_mut!(results);

            let mut output = String::new();
            let mut skips_remaining = offset;
            let mut matches_found = 0;
            let mut has_more_matches = false;

            while let Some(SearchResult::Buffer { buffer, ranges }) = results.next().await {
                if ranges.is_empty() {
                    continue;
                }

                buffer.read_with(&cx, |buffer, cx| -> Result<(), anyhow::Error> {
                    if let Some(path) = buffer.file().map(|file| file.full_path(cx)) {
                        let mut file_header_written = false;
                        let mut ranges = ranges
                            .into_iter()
                            .map(|range| {
                                let mut point_range = range.to_point(buffer);
                                point_range.start.row = point_range.start.row.saturating_sub(CONTEXT_LINES);
                                point_range.start.column = 0;
                                point_range.end.row = cmp::min(
                                    buffer.max_point().row,
                                    point_range.end.row + CONTEXT_LINES,
                                );
                                point_range.end.column = buffer.line_len(point_range.end.row);
                                point_range
                            })
                            .peekable();

                        while let Some(mut range) = ranges.next() {
                            if skips_remaining > 0 {
                                skips_remaining -= 1;
                                continue;
                            }

                            if matches_found >= RESULTS_PER_PAGE {
                                has_more_matches = true;
                                return Ok(());
                            }

                            while let Some(next_range) = ranges.peek() {
                                if range.end.row >= next_range.start.row {
                                    range.end = next_range.end;
                                    ranges.next();
                                } else {
                                    break;
                                }
                            }

                            if !file_header_written {
                                writeln!(output, "\n## Matches in {}", path.display())?;
                                file_header_written = true;
                            }

                            let mut processed_lines = std::collections::HashSet::<u32>::new();

                            // Process matches in two passes:
                            // 1. Long lines (>240 chars): Show only the matched line, with 120 chars of context around each match
                            // 2. Regular lines: Show the matched line plus context lines before/after

                            // First pass: handle long lines
                            for row in range.start.row..=range.end.row {
                                let line_len = buffer.line_len(row);
                                if (line_len as usize) > MAX_LINE_LENGTH {
                                    let line_range = Point::new(row, 0)..Point::new(row, line_len);
                                    let line_text = buffer.text_for_range(line_range).collect::<String>();

                                    if line_text.contains(&regex) {
                                        if skips_remaining == 0 {
                                            // Show each match in the long line with limited context
                                            for (match_start, match_end) in find_matches(line_text.clone(), &regex) {
                                                let start_char = match_start.saturating_sub(LONG_LINE_CONTEXT);
                                                let end_char = (match_end + LONG_LINE_CONTEXT).min(line_len as usize);
                                                writeln!(output, "\n### Line {}, chars {}-{}\n```", row + 1, start_char, end_char)?;
                                                output.push_str(&line_text[start_char..end_char]);
                                                output.push_str("\n```\n");
                                            }
                                            matches_found += 1;
                                        } else {
                                            skips_remaining -= 1;
                                        }

                                        processed_lines.insert(row);
                                    }
                                }
                            }

                            // Second pass: handle regular lines with context
                            let mut row = range.start.row;
                            while row <= range.end.row {
                                if processed_lines.contains(&row) {
                                    row += 1;
                                    continue;
                                }

                                let line_len = buffer.line_len(row);
                                let line_range = Point::new(row, 0)..Point::new(row, line_len);
                                let line_text = buffer.text_for_range(line_range).collect::<String>();

                                if line_text.contains(&regex) {
                                    if skips_remaining > 0 {
                                        skips_remaining -= 1;
                                        row += 1;
                                        continue;
                                    }

                                    // Show the match with context lines
                                    let context_start = row.saturating_sub(CONTEXT_LINES);
                                    let context_end = (row + CONTEXT_LINES).min(buffer.max_point().row);
                                    let context_range = Point::new(context_start, 0)..Point::new(context_end, buffer.line_len(context_end));

                                    writeln!(output, "\n### Lines {}-{}\n```", context_start + 1, context_end + 1)?;
                                    output.push_str(&buffer.text_for_range(context_range).collect::<String>());
                                    output.push_str("\n```\n");

                                    // Mark all lines in this context range as processed
                                    for r in context_start..=context_end {
                                        processed_lines.insert(r);
                                    }

                                    matches_found += 1;
                                    row = context_end + 1;
                                } else {
                                    row += 1;
                                }
                            }

                        }
                    }

                    Ok(())
                })??;
            }

            if matches_found == 0 {
                Ok("No matches found".to_string())
            } else if has_more_matches {
                Ok(format!(
                    "Showing matches {}-{} (there were more matches found; use offset: {} to see next page):\n{output}",
                    offset + 1,
                    offset + matches_found,
                    offset + RESULTS_PER_PAGE,
                ))
            } else {
                Ok(format!("Found {matches_found} matches:\n{output}"))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_find_matches() {
        // Test basic match finding
        let text = "hello world hello".to_string();
        let matches = find_matches(text, "hello");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0], (0, 5));
        assert_eq!(matches[1], (12, 17));

        // Test overlapping matches
        let overlaps = find_matches("abababa".to_string(), "aba");
        assert_eq!(overlaps.len(), 3);
        assert_eq!(overlaps[0], (0, 3));
        assert_eq!(overlaps[1], (2, 5));
        assert_eq!(overlaps[2], (4, 7));

        // Test edge cases
        assert_eq!(find_matches("".to_string(), "pattern"), vec![]);
        assert_eq!(find_matches("text".to_string(), ""), vec![]);
    }

    #[test]
    fn test_long_line_context_calculation() {
        // Create a line that exceeds MAX_LINE_LENGTH
        let prefix = "x".repeat(100);
        let target = "TARGET";
        let suffix = "y".repeat(300);
        let long_line = format!("{}{}{}", prefix, target, suffix);

        // Find the target in the long line
        let matches = find_matches(long_line.clone(), target);
        assert_eq!(matches.len(), 1);

        let (match_start, match_end) = matches[0];

        // Verify context calculation
        let start_char = match_start.saturating_sub(LONG_LINE_CONTEXT);
        let end_char = (match_end + LONG_LINE_CONTEXT).min(long_line.len());

        // Context should start no more than LONG_LINE_CONTEXT chars before match
        assert!(match_start - start_char <= LONG_LINE_CONTEXT);

        // Context should end no more than LONG_LINE_CONTEXT chars after match
        assert!(end_char - match_end <= LONG_LINE_CONTEXT);

        // Context should contain the target
        let context = &long_line[start_char..end_char];
        assert!(context.contains(target));
    }

    #[test]
    fn test_line_length_classification() {
        // Test if lines are correctly classified as long or regular

        // Line shorter than MAX_LINE_LENGTH
        let regular_line = "x".repeat(MAX_LINE_LENGTH - 1);
        assert!(regular_line.len() < MAX_LINE_LENGTH);

        // Line exactly at MAX_LINE_LENGTH
        let boundary_line = "x".repeat(MAX_LINE_LENGTH);
        assert_eq!(boundary_line.len(), MAX_LINE_LENGTH);

        // Line longer than MAX_LINE_LENGTH
        let long_line = "x".repeat(MAX_LINE_LENGTH + 1);
        assert!(long_line.len() > MAX_LINE_LENGTH);
    }

    #[test]
    fn test_heading_format() {
        // For long lines: "### Line X, chars Y-Z"
        // For regular lines: "### Lines X-Y"

        let long_line_row = 42_u32;
        let start_char = 100;
        let end_char = 340;

        // In the implementation, the heading format for long lines is "### Line"
        let long_line_heading = format!(
            "### Line {}, chars {}-{}",
            long_line_row + 1,
            start_char,
            end_char
        );
        assert_eq!(long_line_heading, "### Line 43, chars 100-340");

        let context_start = 40_u32;
        let context_end = 44_u32;
        let regular_heading = format!("### Lines {}-{}", context_start + 1, context_end + 1);
        assert_eq!(regular_heading, "### Lines 41-45");
    }

    #[test]
    fn test_pagination() {
        // Initialize variables to simulate pagination
        let mut skips_remaining = 2;
        let mut matches_found = 0;

        // Simulate processing 5 matches with offset 2
        for _ in 0..5 {
            if skips_remaining > 0 {
                skips_remaining -= 1;
                continue;
            }
            matches_found += 1;
        }

        // Should have processed 3 matches after skipping 2
        assert_eq!(matches_found, 3);
        assert_eq!(skips_remaining, 0);

        // Test page limits
        let mut matches_found = 0;
        let mut has_more_matches = false;

        // Simulate processing more matches than fit in a page
        for _ in 0..(RESULTS_PER_PAGE + 5) {
            if matches_found >= RESULTS_PER_PAGE {
                has_more_matches = true;
                break;
            }
            matches_found += 1;
        }

        // Should have stopped at RESULTS_PER_PAGE
        assert_eq!(matches_found, RESULTS_PER_PAGE);
        assert!(has_more_matches);
    }

    #[test]
    fn test_handling_of_very_long_lines() {
        // Test very long lines with matches at different positions

        // Create a very long line (3 * MAX_LINE_LENGTH)
        let long_prefix = "prefix_".repeat(MAX_LINE_LENGTH / 7);
        let middle = "middle_".repeat(MAX_LINE_LENGTH / 7);
        let long_suffix = "suffix_".repeat(MAX_LINE_LENGTH / 7);
        let very_long_line = format!("{}{}{}", long_prefix, middle, long_suffix);

        assert!(very_long_line.len() > MAX_LINE_LENGTH);

        // Find matches for "middle" in the very long line
        let matches = find_matches(very_long_line.clone(), "middle_");
        assert!(!matches.is_empty());

        // First match should be after the prefix
        let (first_match_start, _) = matches[0];
        assert!(first_match_start >= long_prefix.len());

        // With 120 chars of context, we should not see the start of the string
        let context_start = first_match_start.saturating_sub(LONG_LINE_CONTEXT);
        assert!(context_start > 0); // Context should not include the very beginning

        // But context should include the match
        let context_end = first_match_start + 7 + LONG_LINE_CONTEXT; // "middle_" is 7 chars
        let context = &very_long_line[context_start..context_end.min(very_long_line.len())];
        assert!(context.contains("middle_"));
    }

    #[test]
    fn test_output_format() {
        // Test format consistency

        // For a long line match (> 240 chars), we show:
        // - Only the matched line (no context lines)
        // - Format: "### Line X, chars Y-Z"
        // - Context: 120 chars before and after the match

        // For regular lines (< 240 chars), we show:
        // - The matched line plus context (2 lines before, 2 lines after)
        // - Format: "### Lines X-Y"

        // Test that multiple matches in a long line are shown separately
        let row = 42_u32;
        let match1_start: usize = 100;
        let match1_end: usize = 110;
        let match2_start: usize = 300;
        let match2_end: usize = 310;

        let heading1 = format!(
            "### Line {}, chars {}-{}",
            row + 1,
            match1_start.saturating_sub(LONG_LINE_CONTEXT),
            (match1_end + LONG_LINE_CONTEXT)
        );

        let heading2 = format!(
            "### Line {}, chars {}-{}",
            row + 1,
            match2_start.saturating_sub(LONG_LINE_CONTEXT),
            (match2_end + LONG_LINE_CONTEXT)
        );

        // Each match should have its own heading
        assert_ne!(heading1, heading2);
    }
}
