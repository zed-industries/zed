use crate::assistant_panel::ContextEditor;
use anyhow::Result;
use assistant_slash_command::AfterCompletion;
pub use assistant_slash_command::SlashCommand;
use assistant_slash_command::SlashCommandWorkingSet;
use editor::{CompletionProvider, Editor};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{Model, Task, ViewContext, WeakView, WindowContext};
use language::{Anchor, Buffer, Documentation, LanguageServerId, ToPoint};
use parking_lot::Mutex;
use project::CompletionIntent;
use rope::Point;
use std::{
    cell::RefCell,
    ops::Range,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use workspace::Workspace;

pub(crate) struct SlashCommandCompletionProvider {
    cancel_flag: Mutex<Arc<AtomicBool>>,
    slash_commands: Arc<SlashCommandWorkingSet>,
    editor: Option<WeakView<ContextEditor>>,
    workspace: Option<WeakView<Workspace>>,
}

pub(crate) struct SlashCommandLine {
    /// The range within the line containing the command name.
    pub name: Range<usize>,
    /// Ranges within the line containing the command arguments.
    pub arguments: Vec<Range<usize>>,
}

impl SlashCommandCompletionProvider {
    pub fn new(
        slash_commands: Arc<SlashCommandWorkingSet>,
        editor: Option<WeakView<ContextEditor>>,
        workspace: Option<WeakView<Workspace>>,
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
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<project::Completion>>> {
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
                                    let editor = editor.clone();
                                    let workspace = workspace.clone();
                                    Arc::new(
                                        move |intent: CompletionIntent, cx: &mut WindowContext| {
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
                            old_range: name_range.clone(),
                            documentation: Some(Documentation::SingleLine(command.description())),
                            new_text,
                            label: command.label(cx),
                            server_id: LanguageServerId(0),
                            lsp_completion: Default::default(),
                            confirm,
                            resolved: true,
                        })
                    })
                    .collect()
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
        cx: &mut WindowContext,
    ) -> Task<Result<Vec<project::Completion>>> {
        let new_cancel_flag = Arc::new(AtomicBool::new(false));
        let mut flag = self.cancel_flag.lock();
        flag.store(true, SeqCst);
        *flag = new_cancel_flag.clone();
        if let Some(command) = self.slash_commands.command(command_name, cx) {
            let completions = command.complete_argument(
                arguments,
                new_cancel_flag.clone(),
                self.workspace.clone(),
                cx,
            );
            let command_name: Arc<str> = command_name.into();
            let editor = self.editor.clone();
            let workspace = self.workspace.clone();
            let arguments = arguments.to_vec();
            cx.background_executor().spawn(async move {
                Ok(completions
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
                                        move |intent: CompletionIntent, cx: &mut WindowContext| {
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
                            old_range: if new_argument.replace_previous_arguments {
                                argument_range.clone()
                            } else {
                                last_argument_range.clone()
                            },
                            label: new_argument.label,
                            new_text,
                            documentation: None,
                            server_id: LanguageServerId(0),
                            lsp_completion: Default::default(),
                            confirm,
                            resolved: true,
                        }
                    })
                    .collect())
            })
        } else {
            Task::ready(Ok(Vec::new()))
        }
    }
}

impl CompletionProvider for SlashCommandCompletionProvider {
    fn completions(
        &self,
        buffer: &Model<Buffer>,
        buffer_position: Anchor,
        _: editor::CompletionContext,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<Vec<project::Completion>>> {
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
                let command_range = buffer.anchor_after(command_range_start)
                    ..buffer.anchor_after(command_range_end);

                let name = line[call.name.clone()].to_string();
                let (arguments, last_argument_range) = if let Some(argument) = call.arguments.last()
                {
                    let last_arg_start =
                        buffer.anchor_after(Point::new(position.row, argument.start as u32));
                    let first_arg_start = call.arguments.first().expect("we have the last element");
                    let first_arg_start =
                        buffer.anchor_after(Point::new(position.row, first_arg_start.start as u32));
                    let arguments = call
                        .arguments
                        .iter()
                        .filter_map(|argument| Some(line.get(argument.clone())?.to_string()))
                        .collect::<Vec<_>>();
                    let argument_range = first_arg_start..buffer_position;
                    (
                        Some((arguments, argument_range)),
                        last_arg_start..buffer_position,
                    )
                } else {
                    let start =
                        buffer.anchor_after(Point::new(position.row, call.name.start as u32));
                    (None, start..buffer_position)
                };

                Some((name, arguments, command_range, last_argument_range))
            })
        else {
            return Task::ready(Ok(Vec::new()));
        };

        if let Some((arguments, argument_range)) = arguments {
            self.complete_command_argument(
                &name,
                &arguments,
                command_range,
                argument_range,
                last_argument_range,
                cx,
            )
        } else {
            self.complete_command_name(&name, command_range, last_argument_range, cx)
        }
    }

    fn resolve_completions(
        &self,
        _: Model<Buffer>,
        _: Vec<usize>,
        _: Rc<RefCell<Box<[project::Completion]>>>,
        _: &mut ViewContext<Editor>,
    ) -> Task<Result<bool>> {
        Task::ready(Ok(true))
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

    fn sort_completions(&self) -> bool {
        false
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
                if let Some(argument) = call.arguments.last_mut() {
                    if c.is_whitespace() {
                        if (*argument).is_empty() {
                            argument.start = next_ix;
                            argument.end = next_ix;
                        } else {
                            argument.end = ix;
                            call.arguments.push(next_ix..next_ix);
                        }
                    } else {
                        argument.end = next_ix;
                    }
                }
                // The command name ends at the first whitespace character.
                else if !call.name.is_empty() {
                    if c.is_whitespace() {
                        call.arguments = vec![next_ix..next_ix];
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
                    arguments: Vec::new(),
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
