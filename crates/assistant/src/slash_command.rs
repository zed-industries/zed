use anyhow::Result;
use collections::HashMap;
use editor::{CompletionProvider, Editor};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{AppContext, Model, Task, ViewContext};
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

mod file_command;
mod prompt_command;

pub(crate) struct SlashCommandCompletionProvider {
    commands: Arc<HashMap<String, Box<dyn SlashCommand>>>,
    cancel_flag: Mutex<Arc<AtomicBool>>,
    project: Model<Project>,
}

trait SlashCommand: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn complete_argument(
        &self,
        query: String,
        cancel: Arc<AtomicBool>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>>;
}

struct SlashCommandInvocation {
    name: (String, Point),
    argument: Option<(String, Point)>,
}

impl SlashCommandCompletionProvider {
    pub fn new(project: Model<Project>, cx: &mut AppContext) -> Self {
        let mut commands: HashMap<String, Box<dyn SlashCommand>> = HashMap::default();

        let file_command = Box::new(file_command::FileSlashCommand::new(project.clone()));
        let prompt_command = Box::new(prompt_command::PromptSlashCommand::new());
        commands.insert(file_command.name(), file_command);
        commands.insert(prompt_command.name(), prompt_command);

        Self {
            project,
            cancel_flag: Mutex::new(Arc::new(AtomicBool::new(false))),
            commands: Arc::new(commands),
        }
    }

    fn current_slash_command(
        &self,
        buffer: &Model<Buffer>,
        buffer_position: Anchor,
        cx: &mut ViewContext<Editor>,
    ) -> Option<SlashCommandInvocation> {
        buffer.update(cx, |buffer, _| {
            let mut command: Option<SlashCommandInvocation> = None;
            let position = buffer_position.to_point(buffer);
            let line_start = Point::new(position.row, 0);
            let mut column = 0;
            for chunk in buffer.text_for_range(line_start..position) {
                for (ix, c) in chunk.char_indices() {
                    if let Some(cmd) = &mut command {
                        if cmd.name.0.is_empty() {
                            if c.is_alphabetic() {
                                cmd.name.0.push(c);
                            } else {
                                return None;
                            }
                        } else if let Some((arg, arg_position)) = &mut cmd.argument {
                            if arg.is_empty() {
                                if !c.is_whitespace() {
                                    *arg_position = Point::new(position.row, (column + ix) as u32);
                                    arg.push(c);
                                }
                            } else {
                                arg.push(c);
                            }
                        } else {
                            if c.is_whitespace() {
                                cmd.argument = Some((
                                    String::new(),
                                    Point::new(position.row, (column + ix) as u32),
                                ));
                            } else {
                                cmd.name.0.push(c);
                            }
                        }
                    } else {
                        if c == '/' {
                            command = Some(SlashCommandInvocation {
                                name: (
                                    String::new(),
                                    Point::new(position.row, (column + ix + 1) as u32),
                                ),
                                argument: None,
                            });
                        } else if !c.is_whitespace() {
                            return None;
                        }
                    }
                }
                column += chunk.len();
            }
            command
        })
    }

    fn complete_command_name(
        &self,
        command_name: String,
        range: Range<Anchor>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<project::Completion>>> {
        let candidates = self
            .commands
            .keys()
            .enumerate()
            .map(|(ix, def)| StringMatchCandidate {
                id: ix,
                string: def.clone(),
                char_bag: def.as_str().into(),
            })
            .collect::<Vec<_>>();
        let commands = self.commands.clone();

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
                .map(|mat| project::Completion {
                    old_range: range.clone(),
                    label: CodeLabel::plain(mat.string.clone(), None),
                    new_text: mat.string.clone(),
                    documentation: commands
                        .get(&mat.string)
                        .map(|command| Documentation::SingleLine(command.description())),
                    server_id: LanguageServerId(0),
                    lsp_completion: Default::default(),
                })
                .collect())
        })
    }

    fn complete_command_argument(
        &self,
        command_name: String,
        argument: String,
        range: Range<Anchor>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<project::Completion>>> {
        let new_cancel_flag = Arc::new(AtomicBool::new(false));
        let mut flag = self.cancel_flag.lock();
        flag.store(true, SeqCst);
        *flag = new_cancel_flag.clone();

        if let Some(command) = self.commands.get(&command_name) {
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
        let Some(command) = self.current_slash_command(buffer, buffer_position, cx) else {
            return Task::ready(Ok(vec![]));
        };

        let buffer = buffer.read(cx);
        let (name, name_position) = command.name;
        if let Some((argument, argument_position)) = command.argument {
            let start = buffer.anchor_after(argument_position);
            self.complete_command_argument(name, argument, start..buffer_position, cx)
        } else {
            let start = buffer.anchor_after(name_position);
            self.complete_command_name(name, start..buffer_position, cx)
        }
    }

    fn resolve_completions(
        &self,
        buffer: Model<Buffer>,
        completion_indices: Vec<usize>,
        completions: Arc<RwLock<Box<[project::Completion]>>>,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<bool>> {
        Task::ready(Ok(true))
    }

    fn apply_additional_edits_for_completion(
        &self,
        buffer: Model<Buffer>,
        completion: project::Completion,
        push_to_history: bool,
        cx: &mut ViewContext<Editor>,
    ) -> Task<Result<Option<language::Transaction>>> {
        Task::ready(Ok(None))
    }
}
