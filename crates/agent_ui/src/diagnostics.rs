use anyhow::Result;
use gpui::{App, AppContext as _, Entity, Task};
use language::{Anchor, BufferSnapshot, DiagnosticEntryRef, DiagnosticSeverity, ToOffset};
use project::{DiagnosticSummary, Project};
use rope::Point;
use std::{fmt::Write, ops::RangeInclusive, path::Path};
use text::OffsetRangeExt;
use util::ResultExt;
use util::paths::PathMatcher;

pub fn codeblock_fence_for_path(
    path: Option<&str>,
    row_range: Option<RangeInclusive<u32>>,
) -> String {
    let mut text = String::new();
    write!(text, "```").unwrap();

    if let Some(path) = path {
        if let Some(extension) = Path::new(path).extension().and_then(|ext| ext.to_str()) {
            write!(text, "{} ", extension).unwrap();
        }

        write!(text, "{path}").unwrap();
    } else {
        write!(text, "untitled").unwrap();
    }

    if let Some(row_range) = row_range {
        write!(text, ":{}-{}", row_range.start() + 1, row_range.end() + 1).unwrap();
    }

    text.push('\n');
    text
}

pub struct DiagnosticsOptions {
    pub include_errors: bool,
    pub include_warnings: bool,
    pub path_matcher: Option<PathMatcher>,
}

/// Collects project diagnostics into a formatted string.
///
/// Returns `None` if no matching diagnostics were found.
pub fn collect_diagnostics(
    project: Entity<Project>,
    options: DiagnosticsOptions,
    cx: &mut App,
) -> Task<Result<Option<String>>> {
    let path_style = project.read(cx).path_style(cx);
    let glob_is_exact_file_match = if let Some(path) = options
        .path_matcher
        .as_ref()
        .and_then(|pm| pm.sources().next())
    {
        project
            .read(cx)
            .find_project_path(Path::new(path), cx)
            .is_some()
    } else {
        false
    };

    let project_handle = project.downgrade();
    let diagnostic_summaries: Vec<_> = project
        .read(cx)
        .diagnostic_summaries(false, cx)
        .flat_map(|(path, _, summary)| {
            let worktree = project.read(cx).worktree_for_id(path.worktree_id, cx)?;
            let full_path = worktree.read(cx).root_name().join(&path.path);
            Some((path, full_path, summary))
        })
        .collect();

    cx.spawn(async move |cx| {
        let error_source = if let Some(path_matcher) = &options.path_matcher {
            debug_assert_eq!(path_matcher.sources().count(), 1);
            Some(path_matcher.sources().next().unwrap_or_default())
        } else {
            None
        };

        let mut text = String::new();
        if let Some(error_source) = error_source.as_ref() {
            writeln!(text, "diagnostics: {}", error_source).unwrap();
        } else {
            writeln!(text, "diagnostics").unwrap();
        }

        let mut found_any_diagnostics = false;
        let mut project_summary = DiagnosticSummary::default();
        for (project_path, path, summary) in diagnostic_summaries {
            if let Some(path_matcher) = &options.path_matcher
                && !path_matcher.is_match(&path)
            {
                continue;
            }

            let has_errors = options.include_errors && summary.error_count > 0;
            let has_warnings = options.include_warnings && summary.warning_count > 0;
            if !has_errors && !has_warnings {
                continue;
            }

            if options.include_errors {
                project_summary.error_count += summary.error_count;
            }
            if options.include_warnings {
                project_summary.warning_count += summary.warning_count;
            }

            let file_path = path.display(path_style).to_string();
            if !glob_is_exact_file_match {
                writeln!(&mut text, "{file_path}").unwrap();
            }

            if let Some(buffer) = project_handle
                .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                .await
                .log_err()
            {
                let snapshot = cx.read_entity(&buffer, |buffer, _| buffer.snapshot());
                if collect_buffer_diagnostics(
                    &mut text,
                    &snapshot,
                    options.include_warnings,
                    options.include_errors,
                ) {
                    found_any_diagnostics = true;
                }
            }
        }

        if !found_any_diagnostics {
            return Ok(None);
        }

        let mut label = String::new();
        label.push_str("Diagnostics");
        if let Some(source) = error_source {
            write!(label, " ({})", source).unwrap();
        }

        if project_summary.error_count > 0 || project_summary.warning_count > 0 {
            label.push(':');

            if project_summary.error_count > 0 {
                write!(label, " {} errors", project_summary.error_count).unwrap();
                if project_summary.warning_count > 0 {
                    label.push(',');
                }
            }

            if project_summary.warning_count > 0 {
                write!(label, " {} warnings", project_summary.warning_count).unwrap();
            }
        }

        // Prepend the summary label to the output.
        text.insert_str(0, &format!("{label}\n"));

        Ok(Some(text))
    })
}

/// Collects diagnostics from a buffer snapshot into the text output.
///
/// Returns `true` if any diagnostics were written.
fn collect_buffer_diagnostics(
    text: &mut String,
    snapshot: &BufferSnapshot,
    include_warnings: bool,
    include_errors: bool,
) -> bool {
    let mut found_any = false;
    for (_, group) in snapshot.diagnostic_groups(None) {
        let entry = &group.entries[group.primary_ix];
        if collect_diagnostic(text, entry, snapshot, include_warnings, include_errors) {
            found_any = true;
        }
    }
    found_any
}

/// Formats a single diagnostic entry as a code excerpt with the diagnostic message.
///
/// Returns `true` if the diagnostic was written (i.e. it matched severity filters).
fn collect_diagnostic(
    text: &mut String,
    entry: &DiagnosticEntryRef<'_, Anchor>,
    snapshot: &BufferSnapshot,
    include_warnings: bool,
    include_errors: bool,
) -> bool {
    const EXCERPT_EXPANSION_SIZE: u32 = 2;
    const MAX_MESSAGE_LENGTH: usize = 2000;

    let ty = match entry.diagnostic.severity {
        DiagnosticSeverity::WARNING => {
            if !include_warnings {
                return false;
            }
            "warning"
        }
        DiagnosticSeverity::ERROR => {
            if !include_errors {
                return false;
            }
            "error"
        }
        _ => return false,
    };

    let range = entry.range.to_point(snapshot);
    let diagnostic_row_number = range.start.row + 1;

    let start_row = range.start.row.saturating_sub(EXCERPT_EXPANSION_SIZE);
    let end_row = (range.end.row + EXCERPT_EXPANSION_SIZE).min(snapshot.max_point().row) + 1;
    let excerpt_range =
        Point::new(start_row, 0).to_offset(snapshot)..Point::new(end_row, 0).to_offset(snapshot);

    text.push_str("```");
    if let Some(language_name) = snapshot.language().map(|l| l.code_fence_block_name()) {
        text.push_str(&language_name);
    }
    text.push('\n');

    let mut buffer_text = String::new();
    for chunk in snapshot.text_for_range(excerpt_range) {
        buffer_text.push_str(chunk);
    }

    for (i, line) in buffer_text.lines().enumerate() {
        let line_number = start_row + i as u32 + 1;
        writeln!(text, "{}", line).unwrap();

        if line_number == diagnostic_row_number {
            text.push_str("//");
            let marker_start = text.len();
            write!(text, " {}: ", ty).unwrap();
            let padding = text.len() - marker_start;

            let message = util::truncate(&entry.diagnostic.message, MAX_MESSAGE_LENGTH)
                .replace('\n', format!("\n//{:padding$}", "").as_str());

            writeln!(text, "{message}").unwrap();
        }
    }

    writeln!(text, "```").unwrap();
    true
}
