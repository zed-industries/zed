use anyhow::Result;
use collections::HashMap;
use editor::{CompletionProvider, Editor};
use futures::channel::oneshot;
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{AppContext, Model, Task, ViewContext, WindowHandle};
use language::{Anchor, Buffer, CodeLabel, Documentation, LanguageServerId, ToPoint};
use parking_lot::{Mutex, RwLock};
use project::Project;
use rope::Point;
use std::{
    ops::Range,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
use workspace::Workspace;

use crate::PromptLibrary;

mod current_file_command;
mod file_command;
mod prompt_command;

pub(crate) struct SlashCommandCompletionProvider {
    commands: Arc<SlashCommandRegistry>,
    cancel_flag: Mutex<Arc<AtomicBool>>,
}

#[derive(Default)]
pub(crate) struct SlashCommandRegistry {
    commands: HashMap<String, Box<dyn SlashCommand>>,
}

pub(crate) trait SlashCommand: 'static + Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn complete_argument(
        &self,
        query: String,
        cancel: Arc<AtomicBool>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>>;
    fn requires_argument(&self) -> bool;
    fn run(&self, argument: Option<&str>, cx: &mut AppContext) -> SlashCommandInvocation;
}

pub(crate) struct SlashCommandInvocation {
    pub output: Task<Result<String>>,
    pub invalidated: oneshot::Receiver<()>,
    pub cleanup: SlashCommandCleanup,
}

#[derive(Default)]
pub(crate) struct SlashCommandCleanup(Option<Box<dyn FnOnce()>>);

impl SlashCommandCleanup {
    pub fn new(cleanup: impl FnOnce() + 'static) -> Self {
        Self(Some(Box::new(cleanup)))
    }
}

impl Drop for SlashCommandCleanup {
    fn drop(&mut self) {
        if let Some(cleanup) = self.0.take() {
            cleanup();
        }
    }
}

pub(crate) struct SlashCommandLine {
    /// The range within the line containing the command name.
    pub name: Range<usize>,
    /// The range within the line containing the command argument.
    pub argument: Option<Range<usize>>,
}

impl SlashCommandRegistry {
    pub fn new(
        project: Model<Project>,
        prompt_library: Arc<PromptLibrary>,
        window: Option<WindowHandle<Workspace>>,
    ) -> Arc<Self> {
        let mut this = Self {
            commands: HashMap::default(),
        };

        this.register_command(file_command::FileSlashCommand::new(project));
        this.register_command(prompt_command::PromptSlashCommand::new(prompt_library));
        if let Some(window) = window {
            this.register_command(current_file_command::CurrentFileSlashCommand::new(window));
        }

        Arc::new(this)
    }

    fn register_command(&mut self, command: impl SlashCommand) {
        self.commands.insert(command.name(), Box::new(command));
    }

    fn command_names(&self) -> impl Iterator<Item = &String> {
        self.commands.keys()
    }

    pub(crate) fn command(&self, name: &str) -> Option<&dyn SlashCommand> {
        self.commands.get(name).map(|b| &**b)
    }
}

impl SlashCommandCompletionProvider {
    pub fn new(commands: Arc<SlashCommandRegistry>) -> Self {
        Self {
            cancel_flag: Mutex::new(Arc::new(AtomicBool::new(false))),
            commands,
        }
    }

    fn complete_command_name(
        &self,
        command_name: &str,
        range: Range<Anchor>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<project::Completion>>> {
        let candidates = self
            .commands
            .command_names()
            .enumerate()
            .map(|(ix, def)| StringMatchCandidate {
                id: ix,
                string: def.clone(),
                char_bag: def.as_str().into(),
            })
            .collect::<Vec<_>>();
        let commands = self.commands.clone();
        let command_name = command_name.to_string();
        let executor = cx.background_executor().clone();
        executor.clone().spawn(async move {
            let matches = match_strings(
                &candidates,
                &command_name,
                true,
                usize::MAX,
                &Default::default(),
                executor,
            )
            .await;

            Ok(matches
                .into_iter()
                .filter_map(|mat| {
                    let command = commands.command(&mat.string)?;
                    let mut new_text = mat.string.clone();
                    if command.requires_argument() {
                        new_text.push(' ');
                    }

                    Some(project::Completion {
                        old_range: range.clone(),
                        documentation: Some(Documentation::SingleLine(command.description())),
                        new_text,
                        label: CodeLabel::plain(mat.string, None),
                        server_id: LanguageServerId(0),
                        lsp_completion: Default::default(),
                    })
                })
                .collect())
        })
    }

    fn complete_command_argument(
        &self,
        command_name: &str,
        argument: String,
        range: Range<Anchor>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<project::Completion>>> {
        let new_cancel_flag = Arc::new(AtomicBool::new(false));
        let mut flag = self.cancel_flag.lock();
        flag.store(true, SeqCst);
        *flag = new_cancel_flag.clone();

        if let Some(command) = self.commands.command(command_name) {
            let completions = command.complete_argument(argument, new_cancel_flag.clone(), cx);
            cx.background_executor().spawn(async move {
                Ok(completions
                    .await?
                    .into_iter()
                    .map(|arg| project::Completion {
                        old_range: range.clone(),
                        label: CodeLabel::plain(arg.clone(), None),
                        new_text: arg.clone(),
                        documentation: None,
                        server_id: LanguageServerId(0),
                        lsp_completion: Default::default(),
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
        let task = buffer.update(cx, |buffer, cx| {
            let position = buffer_position.to_point(buffer);
            let line_start = Point::new(position.row, 0);
            let mut lines = buffer.text_for_range(line_start..position).lines();
            let line = lines.next()?;
            let call = SlashCommandLine::parse(line)?;

            let name = &line[call.name.clone()];
            if let Some(argument) = call.argument {
                let start = buffer.anchor_after(Point::new(position.row, argument.start as u32));
                let argument = line[argument.clone()].to_string();
                Some(self.complete_command_argument(name, argument, start..buffer_position, cx))
            } else {
                let start = buffer.anchor_after(Point::new(position.row, call.name.start as u32));
                Some(self.complete_command_name(name, start..buffer_position, cx))
            }
        });

        task.unwrap_or_else(|| Task::ready(Ok(Vec::new())))
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
