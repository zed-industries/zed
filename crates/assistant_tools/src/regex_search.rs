use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use futures::StreamExt;
use gpui::{App, Entity, Task};
use language::{OffsetRangeExt, Point};

fn matches_regex(text: String, pattern: &str) -> bool {
    // Safely check if pattern exists in text
    if pattern.is_empty() {
        return false;
    }
    text.contains(pattern)
}

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
        const CONTEXT_LINES: usize = 2;

        let input = match serde_json::from_value::<RegexSearchToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };
        let offset = input.offset.unwrap_or(0);
        let regex_str = input.regex;

        let query = match SearchQuery::regex(
            &regex_str,
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
                                let context_lines_u32 = CONTEXT_LINES as u32;
                                point_range.start.row = point_range.start.row.saturating_sub(context_lines_u32);
                                point_range.start.column = 0;
                                point_range.end.row = cmp::min(
                                    buffer.max_point().row,
                                    point_range.end.row + (CONTEXT_LINES as u32),
                                );
                                point_range.end.column = buffer.line_len(point_range.end.row);
                                point_range
                            })
                            .peekable();

                        while let Some(mut range) = ranges.next() {
                            if matches_found >= RESULTS_PER_PAGE {
                                has_more_matches = true;
                                return Ok(());
                            }

                            if skips_remaining > 0 {
                                skips_remaining -= 1;
                                continue;
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
                                let row_u32 = row as u32;
                                let line_len = buffer.line_len(row_u32);
                                if (line_len as usize) > MAX_LINE_LENGTH {
                                    let line_range = Point::new(row_u32, 0)..Point::new(row_u32, line_len);
                                    let line_text = buffer.text_for_range(line_range).collect::<String>();

                                    if matches_regex(line_text.clone(), &regex_str) {
                                        if skips_remaining == 0 {
                                            // Show each match in the long line with limited context
                                            for (match_start, match_end) in find_matches(line_text.clone(), &regex_str) {
                                                let start_char = match_start.saturating_sub(LONG_LINE_CONTEXT);
                                                let end_char = (match_end + LONG_LINE_CONTEXT).min(line_len as usize);
                                                writeln!(output, "\n# Line {}, chars {}-{}\n```", row_u32 + 1, start_char, end_char)?;
                                                output.push_str(&line_text[start_char..end_char]);
                                                output.push_str("\n```\n");
                                            }
                                            matches_found += 1;
                                        } else {
                                            skips_remaining -= 1;
                                        }

                                        processed_lines.insert(row_u32);
                                    }
                                }
                            }

                            // Second pass: handle regular lines with context
                            let mut row = range.start.row;
                            while row <= range.end.row {
                                let row_u32 = row as u32;
                                if processed_lines.contains(&row_u32) {
                                    row += 1;
                                    continue;
                                }

                                let line_len = buffer.line_len(row_u32);
                                let line_range = Point::new(row_u32, 0)..Point::new(row_u32, line_len);
                                let line_text = buffer.text_for_range(line_range).collect::<String>();

                                if matches_regex(line_text.clone(), &regex_str) {
                                    if skips_remaining > 0 {
                                        skips_remaining -= 1;
                                        row += 1;
                                        continue;
                                    }

                                    // Show the match with context lines
                                    let context_start = (row as usize).saturating_sub(CONTEXT_LINES) as u32;
                                    let context_end = ((row as usize + CONTEXT_LINES) as u32).min(buffer.max_point().row);
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
