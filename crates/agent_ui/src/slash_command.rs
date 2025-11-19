use crate::text_thread_editor::TextThreadEditor;
use anyhow::Result;
pub use assistant_slash_command::SlashCommand;
use assistant_slash_command::{AfterCompletion, SlashCommandLine, SlashCommandWorkingSet};
use editor::{CompletionProvider, Editor, ExcerptId};
use fuzzy::{StringMatchCandidate, match_strings};
use gpui::{App, AppContext as _, Context, Entity, Task, WeakEntity, Window};
use language::{Anchor, Buffer, ToPoint};
use parking_lot::Mutex;
use project::{
    CompletionDisplayOptions, CompletionIntent, CompletionSource,
    lsp_store::CompletionDocumentation,
};
use rope::Point;
use std::{
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
};
use workspace::Workspace;

pub struct SlashCommandCompletionProvider {
    cancel_flag: Mutex<Arc<AtomicBool>>,
    slash_commands: Arc<SlashCommandWorkingSet>,
    editor: Option<WeakEntity<TextThreadEditor>>,
    workspace: Option<WeakEntity<Workspace>>,
}

impl SlashCommandCompletionProvider {
    pub fn new(
        slash_commands: Arc<SlashCommandWorkingSet>,
        editor: Option<WeakEntity<TextThreadEditor>>,
        workspace: Option<WeakEntity<Workspace>>,
    ) -> Self {
        Self {
            cancel_flag: Mutex::new(Arc::new(AtomicBool::new(false))),
            slash_commands,
            editor,
            workspace,
        }
    }

    fn complete_command_name(
        &self,
        command_name: &str,
        command_range: Range<Anchor>,
        name_range: Range<Anchor>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Vec<project::CompletionResponse>>> {
        let slash_commands = self.slash_commands.clone();
        let candidates = slash_commands
            .command_names(cx)
            .into_iter()
            .enumerate()
            .map(|(ix, def)| StringMatchCandidate::new(ix, &def))
            .collect::<Vec<_>>();
        let command_name = command_name.to_string();
        let editor = self.editor.clone();
        let workspace = self.workspace.clone();
        window.spawn(cx, async move |cx| {
            let matches = match_strings(
                &candidates,
                &command_name,
                true,
                true,
                usize::MAX,
                &Default::default(),
                cx.background_executor().clone(),
            )
            .await;

            cx.update(|_, cx| {
                let completions = matches
                    .into_iter()
                    .filter_map(|mat| {
                        let command = slash_commands.command(&mat.string, cx)?;
                        let mut new_text = mat.string.clone();
                        let requires_argument = command.requires_argument();
                        let accepts_arguments = command.accepts_arguments();
                        if requires_argument || accepts_arguments {
                            new_text.push(' ');
                        }

                        let confirm =
                            editor
                                .clone()
                                .zip(workspace.clone())
                                .map(|(editor, workspace)| {
                                    let command_name = mat.string.clone();
                                    let command_range = command_range.clone();
                                    Arc::new(
                                            move |intent: CompletionIntent,
                                            window: &mut Window,
                                            cx: &mut App| {
                                                if !requires_argument
                                                && (!accepts_arguments || intent.is_complete())
                                                {
                                                    editor
                                                        .update(cx, |editor, cx| {
                                                            editor.run_command(
                                                                command_range.clone(),
                                                                &command_name,
                                                                &[],
                                                                true,
                                                                workspace.clone(),
                                                                window,
                                                                cx,
                                                            );
                                                        })
                                                        .ok();
                                                    false
                                                } else {
                                                    requires_argument || accepts_arguments
                                                }
                                            },
                                        ) as Arc<_>
                                });

                        Some(project::Completion {
                            replace_range: name_range.clone(),
                            documentation: Some(CompletionDocumentation::SingleLine(
                                command.description().into(),
                            )),
                            new_text,
                            label: command.label(cx),
                            icon_path: None,
                            match_start: None,
                            snippet_deduplication_key: None,
                            insert_text_mode: None,
                            confirm,
                            source: CompletionSource::Custom,
                        })
                    })
                    .collect();

                vec![project::CompletionResponse {
                    completions,
                    display_options: CompletionDisplayOptions::default(),
                    is_incomplete: false,
                }]
            })
        })
    }

    fn complete_command_argument(
        &self,
        command_name: &str,
        arguments: &[String],
        command_range: Range<Anchor>,
        argument_range: Range<Anchor>,
        last_argument_range: Range<Anchor>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Vec<project::CompletionResponse>>> {
        let new_cancel_flag = Arc::new(AtomicBool::new(false));
        let mut flag = self.cancel_flag.lock();
        flag.store(true, SeqCst);
        *flag = new_cancel_flag.clone();
        if let Some(command) = self.slash_commands.command(command_name, cx) {
            let completions = command.complete_argument(
                arguments,
                new_cancel_flag,
                self.workspace.clone(),
                window,
                cx,
            );
            let command_name: Arc<str> = command_name.into();
            let editor = self.editor.clone();
            let workspace = self.workspace.clone();
            let arguments = arguments.to_vec();
            cx.background_spawn(async move {
                let completions = completions
                    .await?
                    .into_iter()
                    .map(|new_argument| {
                        let confirm =
                            editor
                                .clone()
                                .zip(workspace.clone())
                                .map(|(editor, workspace)| {
                                    Arc::new({
                                        let mut completed_arguments = arguments.clone();
                                        if new_argument.replace_previous_arguments {
                                            completed_arguments.clear();
                                        } else {
                                            completed_arguments.pop();
                                        }
                                        completed_arguments.push(new_argument.new_text.clone());

                                        let command_range = command_range.clone();
                                        let command_name = command_name.clone();
                                        move |intent: CompletionIntent,
                                              window: &mut Window,
                                              cx: &mut App| {
                                            if new_argument.after_completion.run()
                                                || intent.is_complete()
                                            {
                                                editor
                                                    .update(cx, |editor, cx| {
                                                        editor.run_command(
                                                            command_range.clone(),
                                                            &command_name,
                                                            &completed_arguments,
                                                            true,
                                                            workspace.clone(),
                                                            window,
                                                            cx,
                                                        );
                                                    })
                                                    .ok();
                                                false
                                            } else {
                                                !new_argument.after_completion.run()
                                            }
                                        }
                                    }) as Arc<_>
                                });

                        let mut new_text = new_argument.new_text.clone();
                        if new_argument.after_completion == AfterCompletion::Continue {
                            new_text.push(' ');
                        }

                        project::Completion {
                            replace_range: if new_argument.replace_previous_arguments {
                                argument_range.clone()
                            } else {
                                last_argument_range.clone()
                            },
                            label: new_argument.label,
                            icon_path: None,
                            new_text,
                            documentation: None,
                            match_start: None,
                            snippet_deduplication_key: None,
                            confirm,
                            insert_text_mode: None,
                            source: CompletionSource::Custom,
                        }
                    })
                    .collect();

                Ok(vec![project::CompletionResponse {
                    completions,
                    display_options: CompletionDisplayOptions::default(),
                    // TODO: Could have slash commands indicate whether their completions are incomplete.
                    is_incomplete: true,
                }])
            })
        } else {
            Task::ready(Ok(vec![project::CompletionResponse {
                completions: Vec::new(),
                display_options: CompletionDisplayOptions::default(),
                is_incomplete: true,
            }]))
        }
    }
}

impl CompletionProvider for SlashCommandCompletionProvider {
    fn completions(
        &self,
        _excerpt_id: ExcerptId,
        buffer: &Entity<Buffer>,
        buffer_position: Anchor,
        _: editor::CompletionContext,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<project::CompletionResponse>>> {
        let Some((name, arguments, command_range, last_argument_range)) =
            buffer.update(cx, |buffer, _cx| {
                let position = buffer_position.to_point(buffer);
                let line_start = Point::new(position.row, 0);
                let mut lines = buffer.text_for_range(line_start..position).lines();
                let line = lines.next()?;
                let call = SlashCommandLine::parse(line)?;

                let command_range_start = Point::new(position.row, call.name.start as u32 - 1);
                let command_range_end = Point::new(
                    position.row,
                    call.arguments.last().map_or(call.name.end, |arg| arg.end) as u32,
                );
                let command_range = buffer.anchor_before(command_range_start)
                    ..buffer.anchor_after(command_range_end);

                let name = line[call.name.clone()].to_string();
                let (arguments, last_argument_range) = if let Some(argument) = call.arguments.last()
                {
                    let last_arg_start =
                        buffer.anchor_before(Point::new(position.row, argument.start as u32));
                    let first_arg_start = call.arguments.first().expect("we have the last element");
                    let first_arg_start = buffer
                        .anchor_before(Point::new(position.row, first_arg_start.start as u32));
                    let arguments = call
                        .arguments
                        .into_iter()
                        .filter_map(|argument| Some(line.get(argument)?.to_string()))
                        .collect::<Vec<_>>();
                    let argument_range = first_arg_start..buffer_position;
                    (
                        Some((arguments, argument_range)),
                        last_arg_start..buffer_position,
                    )
                } else {
                    let start =
                        buffer.anchor_before(Point::new(position.row, call.name.start as u32));
                    (None, start..buffer_position)
                };

                Some((name, arguments, command_range, last_argument_range))
            })
        else {
            return Task::ready(Ok(vec![project::CompletionResponse {
                completions: Vec::new(),
                display_options: CompletionDisplayOptions::default(),
                is_incomplete: false,
            }]));
        };

        if let Some((arguments, argument_range)) = arguments {
            self.complete_command_argument(
                &name,
                &arguments,
                command_range,
                argument_range,
                last_argument_range,
                window,
                cx,
            )
        } else {
            self.complete_command_name(&name, command_range, last_argument_range, window, cx)
        }
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        _text: &str,
        _trigger_in_words: bool,
        _menu_is_open: bool,
        cx: &mut Context<Editor>,
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

    fn sort_completions(&self) -> bool {
        false
    }
}
