use anyhow::{Result, anyhow};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandContent, SlashCommandEvent,
    SlashCommandOutputSection, SlashCommandResult,
};
use editor::{Editor, MultiBufferSnapshot};
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
                        .all_adjusted(cx)
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
        let selected_text = snapshot.text_for_range(range.clone()).collect::<String>();
        if selected_text.is_empty() {
            continue;
        }
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

            let line_comment_prefix = start_language
                .and_then(|l| l.default_scope().line_comment_prefixes().first().cloned());

            let fence = codeblock_fence_for_path(
                filename.as_deref(),
                Some(range.start.row..=range.end.row),
            );

            if let Some((line_comment_prefix, outline_text)) = line_comment_prefix.zip(outline_text)
            {
                let breadcrumb = format!("{line_comment_prefix}Excerpt from: {outline_text}\n");
                format!("{fence}{breadcrumb}{selected_text}\n```")
            } else {
                format!("{fence}{selected_text}\n```")
            }
        };
        let crease_title = if let Some(path) = filename {
            let start_line = range.start.row + 1;
            let end_line = range.end.row + 1;
            if start_line == end_line {
                format!("{path}, Line {start_line}")
            } else {
                format!("{path}, Lines {start_line} to {end_line}")
            }
        } else {
            "Quoted selection".to_string()
        };
        creases.push((text, crease_title));
    }
    creases
}
