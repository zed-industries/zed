use anyhow::Result;
use gpui::{AsyncApp, Entity};
use language::{Buffer, OutlineItem};
use regex::Regex;
use std::fmt::Write;
use text::Point;

/// For files over this size, instead of reading them (or including them in context),
/// we automatically provide the file's symbol outline instead, with line numbers.
pub const AUTO_OUTLINE_SIZE: usize = 16384;

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
    path: Option<&str>,
    cx: &AsyncApp,
) -> Result<BufferContent> {
    let file_size = buffer.read_with(cx, |buffer, _| buffer.text().len())?;

    if file_size > AUTO_OUTLINE_SIZE {
        // For large files, use outline instead of full content
        // Wait until the buffer has been fully parsed, so we can read its outline
        buffer
            .read_with(cx, |buffer, _| buffer.parsing_idle())?
            .await;

        let outline_items = buffer.read_with(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            snapshot
                .outline(None)
                .items
                .into_iter()
                .map(|item| item.to_point(&snapshot))
                .collect::<Vec<_>>()
        })?;

        // If no outline exists, fall back to first 1KB so the agent has some context
        if outline_items.is_empty() {
            let text = buffer.read_with(cx, |buffer, _| {
                let snapshot = buffer.snapshot();
                let len = snapshot.len().min(snapshot.as_rope().floor_char_boundary(1024));
                let content = snapshot.text_for_range(0..len).collect::<String>();
                if let Some(path) = path {
                    format!("# First 1KB of {path} (file too large to show full content, and no outline available)\n\n{content}")
                } else {
                    format!("# First 1KB of file (file too large to show full content, and no outline available)\n\n{content}")
                }
            })?;

            return Ok(BufferContent {
                text,
                is_outline: false,
            });
        }

        let outline_text = render_outline(outline_items, None, 0, usize::MAX).await?;

        let text = if let Some(path) = path {
            format!(
                "# File outline for {path} (file too large to show full content)\n\n{outline_text}",
            )
        } else {
            format!("# File outline (file too large to show full content)\n\n{outline_text}",)
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

async fn render_outline(
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

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::TestAppContext;
    use project::Project;
    use settings::SettingsStore;

    #[gpui::test]
    async fn test_large_file_fallback_to_subset(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
        });

        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;

        let content = "⚡".repeat(100 * 1024); // 100KB
        let content_len = content.len();
        let buffer = project
            .update(cx, |project, cx| project.create_buffer(true, cx))
            .await
            .expect("failed to create buffer");

        buffer.update(cx, |buffer, cx| buffer.set_text(content, cx));

        let result = cx
            .spawn(|cx| async move { get_buffer_content_or_outline(buffer, None, &cx).await })
            .await
            .unwrap();

        // Should contain some of the actual file content
        assert!(
            result.text.contains("⚡⚡⚡⚡⚡⚡⚡"),
            "Result did not contain content subset"
        );

        // Should be marked as not an outline (it's truncated content)
        assert!(
            !result.is_outline,
            "Large file without outline should not be marked as outline"
        );

        // Should be reasonably sized (much smaller than original)
        assert!(
            result.text.len() < 50 * 1024,
            "Result size {} should be smaller than 50KB",
            result.text.len()
        );

        // Should be significantly smaller than the original content
        assert!(
            result.text.len() < content_len / 10,
            "Result should be much smaller than original content"
        );
    }
}
