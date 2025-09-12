use action_log::ActionLog;
use anyhow::{Context as _, Result};
use gpui::{AsyncApp, Entity};
use language::{Buffer, OutlineItem, ParseStatus};
use project::Project;
use regex::Regex;
use std::fmt::Write;
use std::path::Path;
use text::Point;

/// For files over this size, instead of reading them (or including them in context),
/// we automatically provide the file's symbol outline instead, with line numbers.
pub const AUTO_OUTLINE_SIZE: usize = 16384;

pub async fn file_outline(
    project: Entity<Project>,
    path: String,
    action_log: Entity<ActionLog>,
    regex: Option<Regex>,
    cx: &mut AsyncApp,
) -> anyhow::Result<String> {
    let buffer = {
        let project_path = project.read_with(cx, |project, cx| {
            project
                .find_project_path(&path, cx)
                .with_context(|| format!("Path {path} not found in project"))
        })??;

        project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))?
            .await?
    };

    action_log.update(cx, |action_log, cx| {
        action_log.buffer_read(buffer.clone(), cx);
    })?;

    // Wait until the buffer has been fully parsed, so that we can read its outline.
    let mut parse_status = buffer.read_with(cx, |buffer, _| buffer.parse_status())?;
    while *parse_status.borrow() != ParseStatus::Idle {
        parse_status.changed().await?;
    }

    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
    let outline = snapshot.outline(None);

    render_outline(
        outline
            .items
            .into_iter()
            .map(|item| item.to_point(&snapshot)),
        regex,
        0,
        usize::MAX,
    )
    .await
}

pub async fn render_outline(
    items: impl IntoIterator<Item = OutlineItem<Point>>,
    regex: Option<Regex>,
    offset: usize,
    results_per_page: usize,
) -> Result<String> {
    let mut items = items.into_iter().skip(offset);

    let entries = items
        .by_ref()
        .filter(|item| {
            regex
                .as_ref()
                .is_none_or(|regex| regex.is_match(&item.text))
        })
        .take(results_per_page)
        .collect::<Vec<_>>();
    let has_more = items.next().is_some();

    let mut output = String::new();
    let entries_rendered = render_entries(&mut output, entries);

    // Calculate pagination information
    let page_start = offset + 1;
    let page_end = offset + entries_rendered;
    let total_symbols = if has_more {
        format!("more than {}", page_end)
    } else {
        page_end.to_string()
    };

    // Add pagination information
    if has_more {
        writeln!(&mut output, "\nShowing symbols {page_start}-{page_end} (there were more symbols found; use offset: {page_end} to see next page)",
        )
    } else {
        writeln!(
            &mut output,
            "\nShowing symbols {page_start}-{page_end} (total symbols: {total_symbols})",
        )
    }
    .ok();

    Ok(output)
}

fn render_entries(
    output: &mut String,
    items: impl IntoIterator<Item = OutlineItem<Point>>,
) -> usize {
    let mut entries_rendered = 0;

    for item in items {
        // Indent based on depth ("" for level 0, "  " for level 1, etc.)
        for _ in 0..item.depth {
            output.push(' ');
        }
        output.push_str(&item.text);

        // Add position information - convert to 1-based line numbers for display
        let start_line = item.range.start.row + 1;
        let end_line = item.range.end.row + 1;

        if start_line == end_line {
            writeln!(output, " [L{}]", start_line).ok();
        } else {
            writeln!(output, " [L{}-{}]", start_line, end_line).ok();
        }
        entries_rendered += 1;
    }

    entries_rendered
}

/// Result of getting buffer content, which can be either full content or an outline.
pub struct BufferContent {
    /// The actual content (either full text or outline)
    pub text: String,
    /// Whether this is an outline (true) or full content (false)
    pub is_outline: bool,
}

/// Returns either the full content of a buffer or its outline, depending on size.
/// For files larger than AUTO_OUTLINE_SIZE, returns an outline with a header.
/// For smaller files, returns the full content.
pub async fn get_buffer_content_or_outline(
    buffer: Entity<Buffer>,
    path: Option<&Path>,
    cx: &AsyncApp,
) -> Result<BufferContent> {
    let file_size = buffer.read_with(cx, |buffer, _| buffer.text().len())?;

    if file_size > AUTO_OUTLINE_SIZE {
        // For large files, use outline instead of full content
        // Wait until the buffer has been fully parsed, so we can read its outline
        let mut parse_status = buffer.read_with(cx, |buffer, _| buffer.parse_status())?;
        while *parse_status.borrow() != ParseStatus::Idle {
            parse_status.changed().await?;
        }

        let outline_items = buffer.read_with(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot
                .outline(None)
                .items
                .into_iter()
                .map(|item| item.to_point(&snapshot))
                .collect::<Vec<_>>()
        })?;

        let outline_text = render_outline(outline_items, None, 0, usize::MAX).await?;

        let text = if let Some(path) = path {
            format!(
                "# File outline for {} (file too large to show full content)\n\n{}",
                path.display(),
                outline_text
            )
        } else {
            format!(
                "# File outline (file too large to show full content)\n\n{}",
                outline_text
            )
        };
        Ok(BufferContent {
            text,
            is_outline: true,
        })
    } else {
        // File is small enough, return full content
        let text = buffer.read_with(cx, |buffer, _| buffer.text())?;
        Ok(BufferContent {
            text,
            is_outline: false,
        })
    }
}
