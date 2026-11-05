//! Full-text search for the "Text" tab, built directly on `Project::search`.

use std::{
    ops::Range,
    sync::atomic::{AtomicBool, Ordering},
};

use futures::StreamExt as _;
use gpui::{AsyncApp, Entity, SharedString};
use language::{Anchor, Buffer, Point, ToPoint as _};
use project::{
    Project, ProjectPath, SearchResults,
    search::{SearchQuery, SearchResult},
};
use util::paths::PathMatcher;

/// Longest line excerpt we render for a single text match.
const MAX_LINE_CHARS: usize = 160;

#[derive(Clone)]
pub struct TextMatch {
    pub project_path: ProjectPath,
    pub range: Range<Point>,
    /// The (trimmed) line containing the match.
    pub line: SharedString,
    /// Byte offsets into `line` to highlight.
    pub positions: Vec<usize>,
}

/// Runs a plain-text, case-insensitive project search and returns at most
/// `limit` matches (one entry per occurrence, like IntelliJ's Text tab).
pub async fn search(
    project: Entity<Project>,
    query: String,
    limit: usize,
    cancel_flag: &AtomicBool,
    cx: &mut AsyncApp,
) -> anyhow::Result<Vec<TextMatch>> {
    let search_query = SearchQuery::text(
        query,
        false,
        false,
        false,
        PathMatcher::default(),
        PathMatcher::default(),
        false,
        None,
    )?;
    // Dropping `_task_handle` cancels the search, so keep it alive while reading.
    let SearchResults {
        rx,
        task_handle: _task_handle,
    } = project.update(cx, |project, cx| project.search(search_query, cx));
    futures::pin_mut!(rx);

    let mut matches = Vec::new();
    while let Some(result) = rx.next().await {
        if cancel_flag.load(Ordering::Acquire) {
            break;
        }
        let (buffer, ranges) = match result {
            SearchResult::Buffer { buffer, ranges } => (buffer, ranges),
            SearchResult::LimitReached => break,
            SearchResult::WaitingForScan | SearchResult::Searching => continue,
        };
        let remaining = limit.saturating_sub(matches.len());
        matches.extend(buffer.read_with(cx, |buffer, cx| {
            matches_in_buffer(buffer, &ranges, remaining, cx)
        }));
        if matches.len() >= limit {
            break;
        }
    }
    Ok(matches)
}

fn matches_in_buffer(
    buffer: &Buffer,
    ranges: &[Range<Anchor>],
    limit: usize,
    cx: &gpui::App,
) -> Vec<TextMatch> {
    let Some(file) = buffer.file() else {
        return Vec::new();
    };
    let project_path = ProjectPath {
        worktree_id: file.worktree_id(cx),
        path: file.path().clone(),
    };
    let snapshot = buffer.snapshot();
    ranges
        .iter()
        .take(limit)
        .map(|range| {
            let start = range.start.to_point(&snapshot);
            let end = range.end.to_point(&snapshot);
            let row = start.row;
            let line_len = snapshot.line_len(row);
            let line: String = snapshot
                .text_for_range(Point::new(row, 0)..Point::new(row, line_len))
                .collect();

            let match_end_column = if end.row == row {
                end.column as usize
            } else {
                line.len()
            };
            let (excerpt, positions) = excerpt(&line, start.column as usize..match_end_column);

            TextMatch {
                project_path: project_path.clone(),
                range: start..end,
                line: excerpt.into(),
                positions,
            }
        })
        .collect()
}

/// Trims the line's indentation, caps its length and returns the byte
/// offsets (into the returned excerpt) of every char inside `matched`.
fn excerpt(line: &str, matched: Range<usize>) -> (String, Vec<usize>) {
    let indent = line.len() - line.trim_start().len();
    let line = &line[indent..];
    let matched = matched.start.saturating_sub(indent)..matched.end.saturating_sub(indent);

    let mut excerpt = String::new();
    let mut positions = Vec::new();
    for (char_count, (offset, ch)) in line.char_indices().enumerate() {
        if char_count >= MAX_LINE_CHARS {
            excerpt.push('…');
            break;
        }
        if matched.contains(&offset) {
            positions.push(excerpt.len());
        }
        excerpt.push(ch);
    }
    (excerpt, positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excerpt_trims_indentation_and_shifts_highlights() {
        let (text, positions) = excerpt("    let needle = 1;", 8..14);
        assert_eq!(text, "let needle = 1;");
        assert_eq!(positions, vec![4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn excerpt_handles_multibyte_chars() {
        let (text, positions) = excerpt("é jehla", 3..8);
        assert_eq!(text, "é jehla");
        assert_eq!(positions, vec![3, 4, 5, 6, 7]);
    }

    #[test]
    fn excerpt_truncates_long_lines() {
        let line = "x".repeat(MAX_LINE_CHARS + 50);
        let (text, _) = excerpt(&line, 0..1);
        assert_eq!(text.chars().count(), MAX_LINE_CHARS + 1);
        assert!(text.ends_with('…'));
    }
}
