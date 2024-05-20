use anyhow::Result;
use editor::{CompletionProvider, Editor};
use fuzzy::{match_strings, PathMatch, StringMatchCandidate};
use gpui::{AppContext, Model, Task, ViewContext};
use language::{Anchor, Buffer, CodeLabel, LanguageServerId, ToPoint};
use parking_lot::{Mutex, RwLock};
use project::{PathMatchCandidateSet, Project};
use rope::Point;
use std::{
    ops::Range,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};

pub(crate) struct SlashCommandCompletionProvider {
    command_definitions: Vec<SlashCommandDefinition>,
    cancel_flag: Mutex<Arc<AtomicBool>>,
    project: Model<Project>,
}

struct SlashCommandDefinition {
    name: String,
    description: String,
}

struct SlashCommand {
    position: Point,
    name: String,
    argument: Option<(String, Point)>,
}

impl SlashCommandCompletionProvider {
    pub fn new(project: Model<Project>, cx: &mut AppContext) -> Self {
        Self {
            project,
            cancel_flag: Mutex::new(Arc::new(AtomicBool::new(false))),
            command_definitions: vec![
                SlashCommandDefinition {
                    name: "file".into(),
                    description: "insert a file".into(),
                },
                SlashCommandDefinition {
                    name: "current-file".into(),
                    description: "insert the current file".into(),
                },
                SlashCommandDefinition {
                    name: "prompt".into(),
                    description: "insert a prompt from the library".into(),
                },
            ],
        }
    }

    fn current_slash_command(
        &self,
        buffer: &Model<Buffer>,
        buffer_position: Anchor,
        cx: &mut ViewContext<Editor>,
    ) -> Option<SlashCommand> {
        buffer.update(cx, |buffer, _| {
            let mut command: Option<SlashCommand> = None;
            let position = buffer_position.to_point(buffer);
            let line_start = Point::new(position.row, 0);
            let mut column = 0;
            for chunk in buffer.text_for_range(line_start..position) {
                for (ix, c) in chunk.char_indices() {
                    if let Some(cmd) = &mut command {
                        if cmd.name.is_empty() {
                            if c.is_alphabetic() {
                                cmd.name.push(c);
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
                                cmd.name.push(c);
                            }
                        }
                    } else {
                        if c == '/' {
                            command = Some(SlashCommand {
                                position: Point::new(position.row, (column + ix + 1) as u32),
                                name: String::new(),
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
            .command_definitions
            .iter()
            .enumerate()
            .map(|(ix, def)| StringMatchCandidate {
                id: ix,
                string: def.name.clone(),
                char_bag: def.name.as_str().into(),
            })
            .collect::<Vec<_>>();

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
                    documentation: None,
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
        match command_name.as_str() {
            "file" => {
                let paths = self.search_paths(argument, cx);
                cx.background_executor().spawn(async move {
                    Ok(paths
                        .await
                        .into_iter()
                        .map(|path_match| {
                            let path = format!(
                                "{}{}",
                                path_match.path_prefix,
                                path_match.path.to_string_lossy()
                            );
                            project::Completion {
                                old_range: range.clone(),
                                new_text: path.clone(),
                                label: CodeLabel::plain(path.clone(), None),
                                server_id: LanguageServerId(0),
                                documentation: None,
                                lsp_completion: Default::default(),
                            }
                        })
                        .collect())
                })
            }
            _ => {
                cx.background_executor().spawn(async move {
                    //
                    Ok(Vec::new())
                })
            }
        }
    }

    fn search_paths(&self, query: String, cx: &mut AppContext) -> Task<Vec<PathMatch>> {
        let worktrees = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .collect::<Vec<_>>();
        let include_root_name = worktrees.len() > 1;
        let candidate_sets = worktrees
            .into_iter()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                PathMatchCandidateSet {
                    snapshot: worktree.snapshot(),
                    include_ignored: worktree
                        .root_entry()
                        .map_or(false, |entry| entry.is_ignored),
                    include_root_name,
                    directories_only: false,
                }
            })
            .collect::<Vec<_>>();

        let new_cancel_flag = Arc::new(AtomicBool::new(false));
        let mut flag = self.cancel_flag.lock();
        flag.store(true, SeqCst);
        *flag = new_cancel_flag.clone();
        let executor = cx.background_executor().clone();
        cx.foreground_executor().spawn(async move {
            fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                query.as_str(),
                None,
                false,
                100,
                &new_cancel_flag,
                executor,
            )
            .await
        })
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
        if let Some((argument, argument_position)) = command.argument {
            let start = buffer.anchor_after(argument_position);
            self.complete_command_argument(command.name, argument, start..buffer_position, cx)
        } else {
            let start = buffer.anchor_after(command.position);
            self.complete_command_name(command.name, start..buffer_position, cx)
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
