use super::{SlashCommand, SlashCommandCleanup, SlashCommandInvocation};
use anyhow::Result;
use futures::channel::oneshot;
use fuzzy::PathMatch;
use gpui::{AppContext, Model, Task};
use language::LspAdapterDelegate;
use project::{PathMatchCandidateSet, Project};
use std::{
    path::Path,
    sync::{atomic::AtomicBool, Arc},
};

pub(crate) struct FileSlashCommand {
    project: Model<Project>,
}

impl FileSlashCommand {
    pub fn new(project: Model<Project>) -> Self {
        Self { project }
    }

    fn search_paths(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        cx: &mut AppContext,
    ) -> Task<Vec<PathMatch>> {
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

        let executor = cx.background_executor().clone();
        cx.foreground_executor().spawn(async move {
            fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                query.as_str(),
                None,
                false,
                100,
                &cancellation_flag,
                executor,
            )
            .await
        })
    }
}

impl SlashCommand for FileSlashCommand {
    fn name(&self) -> String {
        "file".into()
    }

    fn description(&self) -> String {
        "insert an entire file".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        cx: &mut AppContext,
    ) -> gpui::Task<Result<Vec<String>>> {
        let paths = self.search_paths(query, cancellation_flag, cx);
        cx.background_executor().spawn(async move {
            Ok(paths
                .await
                .into_iter()
                .map(|path_match| {
                    format!(
                        "{}{}",
                        path_match.path_prefix,
                        path_match.path.to_string_lossy()
                    )
                })
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut AppContext,
    ) -> SlashCommandInvocation {
        let project = self.project.read(cx);
        let Some(argument) = argument else {
            return SlashCommandInvocation {
                output: Task::ready(Err(anyhow::anyhow!("missing path"))),
                invalidated: oneshot::channel().1,
                cleanup: SlashCommandCleanup::default(),
            };
        };

        let path = Path::new(argument);
        let abs_path = project.worktrees().find_map(|worktree| {
            let worktree = worktree.read(cx);
            worktree.entry_for_path(path)?;
            worktree.absolutize(path).ok()
        });

        let Some(abs_path) = abs_path else {
            return SlashCommandInvocation {
                output: Task::ready(Err(anyhow::anyhow!("missing path"))),
                invalidated: oneshot::channel().1,
                cleanup: SlashCommandCleanup::default(),
            };
        };

        let fs = project.fs().clone();
        let argument = argument.to_string();
        let output = cx.background_executor().spawn(async move {
            let content = fs.load(&abs_path).await?;
            let mut output = String::with_capacity(argument.len() + content.len() + 9);
            output.push_str("```");
            output.push_str(&argument);
            output.push('\n');
            output.push_str(&content);
            if !output.ends_with('\n') {
                output.push('\n');
            }
            output.push_str("```");
            Ok(output)
        });
        SlashCommandInvocation {
            output,
            invalidated: oneshot::channel().1,
            cleanup: SlashCommandCleanup::default(),
        }
    }
}
