use crate::assistant_panel::ConversationEditor;
use anyhow::Result;
pub use assistant_slash_command::{SlashCommand, SlashCommandOutput, SlashCommandRegistry};
use editor::{CompletionProvider, Editor};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{Model, Task, ViewContext, WeakView, WindowContext};
use language::{Anchor, Buffer, CodeLabel, Documentation, LanguageServerId, ToPoint};
use parking_lot::{Mutex, RwLock};
use rope::Point;
use std::{
    ops::Range,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use workspace::Workspace;

pub mod active_command;
pub mod file_command;
pub mod project_command;
pub mod prompt_command;
pub mod rustdoc_command;
pub mod search_command;
pub mod tabs_command;

pub(crate) struct SlashCommandCompletionProvider {
    editor: WeakView<ConversationEditor>,
    commands: Arc<SlashCommandRegistry>,
    cancel_flag: Mutex<Arc<AtomicBool>>,
    workspace: WeakView<Workspace>,
}

pub(crate) struct SlashCommandLine {
    /// The range within the line containing the command name.
    pub name: Range<usize>,
    /// The range within the line containing the command argument.
    pub argument: Option<Range<usize>>,
}

impl SlashCommandCompletionProvider {
    pub fn new(
        editor: WeakView<ConversationEditor>,
        commands: Arc<SlashCommandRegistry>,
        workspace: WeakView<Workspace>,
    ) -> Self {
        Self {
            cancel_flag: Mutex::new(Arc::new(AtomicBool::new(false))),
            editor,
            commands,
            workspace,
        }
    }

    fn complete_command_name(
        &self,
        command_name: &str,
        command_range: Range<Anchor>,
        name_range: Range<Anchor>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<project::Completion>>> {
        let candidates = self
            .commands
            .command_names()
            .into_iter()
            .enumerate()
            .map(|(ix, def)| StringMatchCandidate {
                id: ix,
                string: def.to_string(),
                char_bag: def.as_ref().into(),
            })
            .collect::<Vec<_>>();
        let commands = self.commands.clone();
        let command_name = command_name.to_string();
        let editor = self.editor.clone();
        let workspace = self.workspace.clone();
        cx.spawn(|mut cx| async move {
            let matches = match_strings(
                &candidates,
                &command_name,
                true,
                usize::MAX,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            cx.update(|cx| {
                matches
                    .into_iter()
                    .filter_map(|mat| {
                        let command = commands.command(&mat.string)?;
                        let mut new_text = mat.string.clone();
                        let requires_argument = command.requires_argument();
                        if requires_argument {
                            new_text.push(' ');
                        }

                        Some(project::Completion {
                            old_range: name_range.clone(),
                            documentation: Some(Documentation::SingleLine(command.description())),
                            new_text,
                            label: command.label(cx),
                            server_id: LanguageServerId(0),
                            lsp_completion: Default::default(),
                            show_new_completions_on_confirm: requires_argument,
                            confirm: (!requires_argument).then(|| {
                                let command_name = mat.string.clone();
                                let command_range = command_range.clone();
                                let editor = editor.clone();
                                let workspace = workspace.clone();
                                Arc::new(move |cx: &mut WindowContext| {
                                    editor
                                        .update(cx, |editor, cx| {
                                            editor.run_command(
                                                command_range.clone(),
                                                &command_name,
                                                None,
                                                workspace.clone(),
                                                cx,
                                            );
                                        })
                                        .ok();
                                }) as Arc<_>
                            }),
                        })
                    })
                    .collect()
            })
        })
    }

    fn complete_command_argument(
        &self,
        command_name: &str,
        argument: String,
        command_range: Range<Anchor>,
        argument_range: Range<Anchor>,
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<project::Completion>>> {
        let new_cancel_flag = Arc::new(AtomicBool::new(false));
        let mut flag = self.cancel_flag.lock();
        flag.store(true, SeqCst);
        *flag = new_cancel_flag.clone();

        if let Some(command) = self.commands.command(command_name) {
            let completions = command.complete_argument(
                argument,
                new_cancel_flag.clone(),
                self.workspace.clone(),
                cx,
            );
            let command_name: Arc<str> = command_name.into();
            let editor = self.editor.clone();
            let workspace = self.workspace.clone();
            cx.background_executor().spawn(async move {
                Ok(completions
                    .await?
                    .into_iter()
                    .map(|arg| project::Completion {
                        old_range: argument_range.clone(),
                        label: CodeLabel::plain(arg.clone(), None),
                        new_text: arg.clone(),
                        documentation: None,
                        server_id: LanguageServerId(0),
                        lsp_completion: Default::default(),
                        show_new_completions_on_confirm: false,
                        confirm: Some(Arc::new({
                            let command_name = command_name.clone();
                            let command_range = command_range.clone();
                            let editor = editor.clone();
                            let workspace = workspace.clone();
                            move |cx| {
                                editor
                                    .update(cx, |editor, cx| {
                                        editor.run_command(
                                            command_range.clone(),
                                            &command_name,
                                            Some(&arg),
                                            workspace.clone(),
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        })),
                    })
                    .collect())
            })
        } else {
            cx.background_executor()
                .spawn(async move { Ok(Vec::new()) })
        }
    }
}

impl CompletionProvider for SlashCommandCompletionProvider {
    fn completions(
        &self,
        buffer: &Model<Buffer>,
        buffer_position: Anchor,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<Vec<project::Completion>>> {
        let Some((name, argument, command_range, argument_range)) =
            buffer.update(cx, |buffer, _cx| {
                let position = buffer_position.to_point(buffer);
                let line_start = Point::new(position.row, 0);
                let mut lines = buffer.text_for_range(line_start..position).lines();
                let line = lines.next()?;
                let call = SlashCommandLine::parse(line)?;

                let command_range_start = Point::new(position.row, call.name.start as u32 - 1);
                let command_range_end = Point::new(
                    position.row,
                    call.argument.as_ref().map_or(call.name.end, |arg| arg.end) as u32,
                );
                let command_range = buffer.anchor_after(command_range_start)
                    ..buffer.anchor_after(command_range_end);

                let name = line[call.name.clone()].to_string();

                Some(if let Some(argument) = call.argument {
                    let start =
                        buffer.anchor_after(Point::new(position.row, argument.start as u32));
                    let argument = line[argument.clone()].to_string();
                    (name, Some(argument), command_range, start..buffer_position)
                } else {
                    let start =
                        buffer.anchor_after(Point::new(position.row, call.name.start as u32));
                    (name, None, command_range, start..buffer_position)
                })
            })
        else {
            return Task::ready(Ok(Vec::new()));
        };

        if let Some(argument) = argument {
            self.complete_command_argument(&name, argument, command_range, argument_range, cx)
        } else {
            self.complete_command_name(&name, command_range, argument_range, cx)
        }
    }

    fn resolve_completions(
        &self,
        _: Model<Buffer>,
        _: Vec<usize>,
        _: Arc<RwLock<Box<[project::Completion]>>>,
        _: &mut ViewContext<Editor>,
    ) -> Task<Result<bool>> {
        Task::ready(Ok(true))
    }

    fn apply_additional_edits_for_completion(
        &self,
        _: Model<Buffer>,
        _: project::Completion,
        _: bool,
        _: &mut ViewContext<Editor>,
    ) -> Task<Result<Option<language::Transaction>>> {
        Task::ready(Ok(None))
    }

    fn is_completion_trigger(
        &self,
        buffer: &Model<Buffer>,
        position: language::Anchor,
        _text: &str,
        _trigger_in_words: bool,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        let buffer = buffer.read(cx);
        let position = position.to_point(buffer);
        let line_start = Point::new(position.row, 0);
        let mut lines = buffer.text_for_range(line_start..position).lines();
        if let Some(line) = lines.next() {
            SlashCommandLine::parse(line).is_some()
        } else {
            false
        }
    }
}

impl SlashCommandLine {
    pub(crate) fn parse(line: &str) -> Option<Self> {
        let mut call: Option<Self> = None;
        let mut ix = 0;
        for c in line.chars() {
            let next_ix = ix + c.len_utf8();
            if let Some(call) = &mut call {
                // The command arguments start at the first non-whitespace character
                // after the command name, and continue until the end of the line.
                if let Some(argument) = &mut call.argument {
                    if (*argument).is_empty() && c.is_whitespace() {
                        argument.start = next_ix;
                    }
                    argument.end = next_ix;
                }
                // The command name ends at the first whitespace character.
                else if !call.name.is_empty() {
                    if c.is_whitespace() {
                        call.argument = Some(next_ix..next_ix);
                    } else {
                        call.name.end = next_ix;
                    }
                }
                // The command name must begin with a letter.
                else if c.is_alphabetic() {
                    call.name.end = next_ix;
                } else {
                    return None;
                }
            }
            // Commands start with a slash.
            else if c == '/' {
                call = Some(SlashCommandLine {
                    name: next_ix..next_ix,
                    argument: None,
                });
            }
            // The line can't contain anything before the slash except for whitespace.
            else if !c.is_whitespace() {
                return None;
            }
            ix = next_ix;
        }
        call
    }
}
