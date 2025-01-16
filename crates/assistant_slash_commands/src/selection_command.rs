use anyhow::{anyhow, Result};
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandContent, SlashCommandEvent,
    SlashCommandOutputSection, SlashCommandResult,
};
use editor::Editor;
use futures::StreamExt;
use gpui::{AppContext, Task, WeakView};
use gpui::{SharedString, ViewContext, WindowContext};
use language::{BufferSnapshot, CodeLabel, LspAdapterDelegate};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use ui::IconName;
use workspace::Workspace;

use crate::file_command::codeblock_fence_for_path;

pub struct SelectionCommand;

impl SlashCommand for SelectionCommand {
    fn name(&self) -> String {
        "selection".into()
    }

    fn label(&self, _cx: &AppContext) -> CodeLabel {
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
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
        let mut events = vec![];

        let Some(creases) = workspace
            .update(cx, selections_creases)
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
    workspace: &mut workspace::Workspace,
    cx: &mut ViewContext<Workspace>,
) -> Option<Vec<(String, String)>> {
    let editor = workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))?;

    let mut creases = vec![];
    editor.update(cx, |editor, cx| {
        let selections = editor.selections.all_adjusted(cx);
        let buffer = editor.buffer().read(cx).snapshot(cx);
        for selection in selections {
            let range = editor::ToOffset::to_offset(&selection.start, &buffer)
                ..editor::ToOffset::to_offset(&selection.end, &buffer);
            let selected_text = buffer.text_for_range(range.clone()).collect::<String>();
            if selected_text.is_empty() {
                continue;
            }
            let start_language = buffer.language_at(range.start);
            let end_language = buffer.language_at(range.end);
            let language_name = if start_language == end_language {
                start_language.map(|language| language.code_fence_block_name())
            } else {
                None
            };
            let language_name = language_name.as_deref().unwrap_or("");
            let filename = buffer
                .file_at(selection.start)
                .map(|file| file.full_path(cx));
            let text = if language_name == "markdown" {
                selected_text
                    .lines()
                    .map(|line| format!("> {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                let start_symbols = buffer
                    .symbols_containing(selection.start, None)
                    .map(|(_, symbols)| symbols);
                let end_symbols = buffer
                    .symbols_containing(selection.end, None)
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
                    Some(selection.start.row..=selection.end.row),
                );

                if let Some((line_comment_prefix, outline_text)) =
                    line_comment_prefix.zip(outline_text)
                {
                    let breadcrumb = format!("{line_comment_prefix}Excerpt from: {outline_text}\n");
                    format!("{fence}{breadcrumb}{selected_text}\n```")
                } else {
                    format!("{fence}{selected_text}\n```")
                }
            };
            let crease_title = if let Some(path) = filename {
                let start_line = selection.start.row + 1;
                let end_line = selection.end.row + 1;
                if start_line == end_line {
                    format!("{}, Line {}", path.display(), start_line)
                } else {
                    format!("{}, Lines {} to {}", path.display(), start_line, end_line)
                }
            } else {
                "Quoted selection".to_string()
            };
            creases.push((text, crease_title));
        }
    });
    Some(creases)
}
