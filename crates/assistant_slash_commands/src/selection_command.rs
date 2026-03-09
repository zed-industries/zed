use anyhow::{Result, anyhow};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandContent, SlashCommandEvent,
    SlashCommandOutputSection, SlashCommandResult,
};
use editor::{BufferOffset, Editor, MultiBufferSnapshot};
use futures::StreamExt;
use gpui::{App, SharedString, Task, WeakEntity, Window};
use language::{BufferSnapshot, CodeLabel, LspAdapterDelegate};

use rope::Point;
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use ui::IconName;
use workspace::Workspace;

use crate::file_command::codeblock_fence_for_path;

pub struct SelectionCommand;

impl SlashCommand for SelectionCommand {
    fn name(&self) -> String {
        "selection".into()
    }

    fn label(&self, _cx: &App) -> CodeLabel {
        CodeLabel::plain(self.name(), None)
    }

    fn description(&self) -> String {
        "Insert editor selection".into()
    }

    fn icon(&self) -> IconName {
        IconName::Quote
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn accepts_arguments(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let mut events = vec![];

        let Some(creases) = workspace
            .update(cx, |workspace, cx| {
                let editor = workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))?;

                editor.update(cx, |editor, cx| {
                    let selection_ranges = editor
                        .selections
                        .all_adjusted(&editor.display_snapshot(cx))
                        .iter()
                        .map(|selection| selection.range())
                        .collect::<Vec<_>>();
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    Some(selections_creases(selection_ranges, snapshot, cx))
                })
            })
            .unwrap_or_else(|e| {
                events.push(Err(e));
                None
            })
        else {
            return Task::ready(Err(anyhow!("no active selection")));
        };

        for (text, title) in creases {
            events.push(Ok(SlashCommandEvent::StartSection {
                icon: IconName::TextSnippet,
                label: SharedString::from(title),
                metadata: None,
            }));
            events.push(Ok(SlashCommandEvent::Content(SlashCommandContent::Text {
                text,
                run_commands_in_text: false,
            })));
            events.push(Ok(SlashCommandEvent::EndSection));
            events.push(Ok(SlashCommandEvent::Content(SlashCommandContent::Text {
                text: "\n".to_string(),
                run_commands_in_text: false,
            })));
        }

        let result = futures::stream::iter(events).boxed();

        Task::ready(Ok(result))
    }
}

pub fn selections_creases(
    selection_ranges: Vec<Range<Point>>,
    snapshot: MultiBufferSnapshot,
    cx: &App,
) -> Vec<(String, String)> {
    let mut creases = Vec::new();
    for range in selection_ranges {
        let buffer_ranges = snapshot.range_to_buffer_ranges(range.clone());

        if buffer_ranges.is_empty() {
            creases.extend(crease_for_range(range, &snapshot, cx));
            continue;
        }

        for (buffer_snapshot, buffer_range, _excerpt_id) in buffer_ranges {
            creases.extend(crease_for_buffer_range(buffer_snapshot, buffer_range, cx));
        }
    }
    creases
}

/// Creates a crease for a range within a specific buffer (excerpt).
/// This is used when we know the exact buffer and range within it.
fn crease_for_buffer_range(
    buffer: &BufferSnapshot,
    Range { start, end }: Range<BufferOffset>,
    cx: &App,
) -> Option<(String, String)> {
    let selected_text: String = buffer.text_for_range(start.0..end.0).collect();

    if selected_text.is_empty() {
        return None;
    }

    let start_point = buffer.offset_to_point(start.0);
    let end_point = buffer.offset_to_point(end.0);
    let start_buffer_row = start_point.row;
    let end_buffer_row = end_point.row;

    let language = buffer.language_at(start.0);
    let language_name_arc = language.map(|l| l.code_fence_block_name());
    let language_name = language_name_arc.as_deref().unwrap_or_default();

    let filename = buffer
        .file()
        .map(|file| file.full_path(cx).to_string_lossy().into_owned());

    let text = if language_name == "markdown" {
        selected_text
            .lines()
            .map(|line| format!("> {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        let start_symbols = buffer.symbols_containing(start, None);
        let end_symbols = buffer.symbols_containing(end, None);

        let outline_text = if !start_symbols.is_empty() && !end_symbols.is_empty() {
            Some(
                start_symbols
                    .into_iter()
                    .zip(end_symbols)
                    .take_while(|(a, b)| a == b)
                    .map(|(a, _)| a.text)
                    .collect::<Vec<_>>()
                    .join(" > "),
            )
        } else {
            None
        };

        let line_comment_prefix =
            language.and_then(|l| l.default_scope().line_comment_prefixes().first().cloned());

        let fence =
            codeblock_fence_for_path(filename.as_deref(), Some(start_buffer_row..=end_buffer_row));

        if let Some((line_comment_prefix, outline_text)) = line_comment_prefix.zip(outline_text) {
            let breadcrumb = format!("{line_comment_prefix}Excerpt from: {outline_text}\n");
            format!("{fence}{breadcrumb}{selected_text}\n```")
        } else {
            format!("{fence}{selected_text}\n```")
        }
    };

    let crease_title = if let Some(path) = filename {
        let start_line = start_buffer_row + 1;
        let end_line = end_buffer_row + 1;
        if start_line == end_line {
            format!("{path}, Line {start_line}")
        } else {
            format!("{path}, Lines {start_line} to {end_line}")
        }
    } else {
        "Quoted selection".to_string()
    };

    Some((text, crease_title))
}

/// Fallback function to create a crease from a multibuffer range when we can't split by excerpt.
fn crease_for_range(
    range: Range<Point>,
    snapshot: &MultiBufferSnapshot,
    cx: &App,
) -> Option<(String, String)> {
    let selected_text = snapshot.text_for_range(range.clone()).collect::<String>();
    if selected_text.is_empty() {
        return None;
    }

    // Get actual file line numbers (not multibuffer row numbers)
    let start_buffer_row = snapshot
        .point_to_buffer_point(range.start)
        .map(|(_, point, _)| point.row)
        .unwrap_or(range.start.row);
    let end_buffer_row = snapshot
        .point_to_buffer_point(range.end)
        .map(|(_, point, _)| point.row)
        .unwrap_or(range.end.row);

    let start_language = snapshot.language_at(range.start);
    let end_language = snapshot.language_at(range.end);
    let language_name = if start_language == end_language {
        start_language.map(|language| language.code_fence_block_name())
    } else {
        None
    };
    let language_name = language_name.as_deref().unwrap_or("");

    let filename = snapshot
        .file_at(range.start)
        .map(|file| file.full_path(cx).to_string_lossy().into_owned());

    let text = if language_name == "markdown" {
        selected_text
            .lines()
            .map(|line| format!("> {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        let start_symbols = snapshot
            .symbols_containing(range.start, None)
            .map(|(_, symbols)| symbols);
        let end_symbols = snapshot
            .symbols_containing(range.end, None)
            .map(|(_, symbols)| symbols);

        let outline_text =
            if let Some((start_symbols, end_symbols)) = start_symbols.zip(end_symbols) {
                Some(
                    start_symbols
                        .into_iter()
                        .zip(end_symbols)
                        .take_while(|(a, b)| a == b)
                        .map(|(a, _)| a.text)
                        .collect::<Vec<_>>()
                        .join(" > "),
                )
            } else {
                None
            };

        let line_comment_prefix =
            start_language.and_then(|l| l.default_scope().line_comment_prefixes().first().cloned());

        let fence =
            codeblock_fence_for_path(filename.as_deref(), Some(start_buffer_row..=end_buffer_row));

        if let Some((line_comment_prefix, outline_text)) = line_comment_prefix.zip(outline_text) {
            let breadcrumb = format!("{line_comment_prefix}Excerpt from: {outline_text}\n");
            format!("{fence}{breadcrumb}{selected_text}\n```")
        } else {
            format!("{fence}{selected_text}\n```")
        }
    };

    let crease_title = if let Some(path) = filename {
        let start_line = start_buffer_row + 1;
        let end_line = end_buffer_row + 1;
        if start_line == end_line {
            format!("{path}, Line {start_line}")
        } else {
            format!("{path}, Lines {start_line} to {end_line}")
        }
    } else {
        "Quoted selection".to_string()
    };

    Some((text, crease_title))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use multi_buffer::MultiBuffer;

    #[gpui::test]
    fn test_selections_creases_single_excerpt(cx: &mut TestAppContext) {
        let buffer = cx.update(|cx| {
            MultiBuffer::build_multi(
                [("a\nb\nc\n", vec![Point::new(0, 0)..Point::new(3, 0)])],
                cx,
            )
        });
        let creases = cx.update(|cx| {
            let snapshot = buffer.read(cx).snapshot(cx);
            selections_creases(vec![Point::new(0, 0)..Point::new(2, 1)], snapshot, cx)
        });
        assert_eq!(creases.len(), 1);
        assert_eq!(creases[0].0, "```untitled:1-3\na\nb\nc\n```");
        assert_eq!(creases[0].1, "Quoted selection");
    }

    #[gpui::test]
    fn test_selections_creases_spans_multiple_excerpts(cx: &mut TestAppContext) {
        let buffer = cx.update(|cx| {
            MultiBuffer::build_multi(
                [
                    ("aaa\nbbb\n", vec![Point::new(0, 0)..Point::new(2, 0)]),
                    ("111\n222\n", vec![Point::new(0, 0)..Point::new(2, 0)]),
                ],
                cx,
            )
        });
        let creases = cx.update(|cx| {
            let snapshot = buffer.read(cx).snapshot(cx);
            let end = snapshot.offset_to_point(snapshot.len());
            selections_creases(vec![Point::new(0, 0)..end], snapshot, cx)
        });
        assert_eq!(creases.len(), 2);
        assert!(creases[0].0.contains("aaa") && !creases[0].0.contains("111"));
        assert!(creases[1].0.contains("111") && !creases[1].0.contains("aaa"));
    }
}
